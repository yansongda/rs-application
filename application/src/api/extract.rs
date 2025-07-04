use crate::api::response::ApiErr;
use axum::extract::FromRequest;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiErr))]
pub struct Json<T>(pub T);
