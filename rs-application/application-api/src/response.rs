use application_kernel::result::Error;
use salvo::http::ParseError;
use salvo::{Scribe, writing::Json};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub request_id: String,
    pub data: Option<D>,
}

impl<D: Serialize> Response<D> {
    pub fn new(code: Option<u16>, message: Option<String>, data: Option<D>) -> Self {
        Response {
            code: code.unwrap_or(0),
            message: message.unwrap_or_else(|| "success".to_string()),
            // request_id is populated automatically in Scribe::render() from response headers
            request_id: String::new(),
            data,
        }
    }

    pub fn success(data: D) -> Self {
        Response::new(None, None, Some(data))
    }

    pub fn error(err: ApiErr) -> Self {
        let (code, message) = err.0.get_code_message();

        Response::new(Some(code), Some(message.to_string()), None)
    }
}

/// Helper function to extract request_id from response headers
fn extract_request_id(res: &salvo::Response) -> String {
    res.headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

impl<D: Serialize + Send> Scribe for Response<D> {
    fn render(mut self, res: &mut salvo::Response) {
        // Inject request_id from response headers (set by RequestId middleware)
        self.request_id = extract_request_id(res);
        res.render(Json(self));
    }
}

pub type Result<D> = std::result::Result<D, ApiErr>;
pub type Resp<D> = Result<Response<D>>;

pub struct ApiErr(pub Error);

impl Scribe for ApiErr {
    fn render(self, res: &mut salvo::Response) {
        let mut response = Response::<String>::error(self);
        // Inject request_id using the shared helper function
        response.request_id = extract_request_id(res);
        res.render(Json(response));
    }
}

impl From<Error> for ApiErr {
    fn from(r: Error) -> Self {
        ApiErr(r)
    }
}

impl From<ParseError> for ApiErr {
    fn from(r: ParseError) -> Self {
        info!("解析 Json 请求失败: {:?}", r);

        ApiErr::from(Error::ParamsJsonInvalid(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_response_success_serialization() {
        let mut response = Response::success("test data");
        response.request_id = "test-request-id-123".to_string();
        let json = serde_json::to_value(&response).unwrap();
        
        assert_eq!(json["code"], 0);
        assert_eq!(json["message"], "success");
        assert_eq!(json["request_id"], "test-request-id-123");
        assert_eq!(json["data"], "test data");
    }

    #[test]
    fn test_response_new_with_request_id() {
        let mut response = Response::<String>::new(
            Some(404),
            Some("Not Found".to_string()),
            None,
        );
        response.request_id = "test-request-id-456".to_string();
        let json = serde_json::to_value(&response).unwrap();
        
        assert_eq!(json["code"], 404);
        assert_eq!(json["message"], "Not Found");
        assert_eq!(json["request_id"], "test-request-id-456");
        assert_eq!(json["data"], serde_json::Value::Null);
    }

    #[test]
    fn test_response_structure() {
        let data = json!({
            "id": 1,
            "name": "test"
        });
        let mut response = Response::success(data.clone());
        response.request_id = "req-123".to_string();
        let json = serde_json::to_value(&response).unwrap();
        
        // Verify the response structure matches the required format
        assert!(json.get("code").is_some());
        assert!(json.get("message").is_some());
        assert!(json.get("request_id").is_some());
        assert!(json.get("data").is_some());
        
        // Verify the order doesn't matter but all fields are present
        let keys: Vec<&str> = json.as_object().unwrap().keys().map(|s| s.as_str()).collect();
        assert_eq!(keys.len(), 4);
        assert!(keys.contains(&"code"));
        assert!(keys.contains(&"message"));
        assert!(keys.contains(&"request_id"));
        assert!(keys.contains(&"data"));
    }

    #[test]
    fn test_json_format_example() {
        // Test that the response format matches the issue requirement
        let data = json!({"user_id": 1, "username": "test"});
        let mut response = Response::success(data);
        response.request_id = "xxxxx".to_string();
        let json_str = serde_json::to_string(&response).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        
        // Verify it matches the expected structure:
        // {
        //     "code": 0,
        //     "message": "success",
        //     "request_id": "xxxxx",
        //     "data": xxx
        // }
        assert_eq!(json_value["code"], 0);
        assert_eq!(json_value["message"], "success");
        assert_eq!(json_value["request_id"], "xxxxx");
        assert!(json_value["data"].is_object());
        assert_eq!(json_value["data"]["user_id"], 1);
        assert_eq!(json_value["data"]["username"], "test");
        
        // Print for manual verification
        println!("\nActual JSON output:");
        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }
}
