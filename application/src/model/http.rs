use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration: f32,
}
