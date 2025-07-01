use application_kernel::result::Error;
use axum::extract::FromRequest;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(Error))]
pub struct Json<T>(pub T);
