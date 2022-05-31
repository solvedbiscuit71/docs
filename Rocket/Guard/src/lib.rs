pub mod router;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
pub struct ApiKey(pub String);

#[derive(Debug)]
pub enum ApiKeyError {
    Invalid,
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = req.headers().get_one("api-key");
        match api_key {
            Some(key) if key.len() == 9 => Outcome::Success(ApiKey(key.to_string())),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            None => Outcome::Failure((Status::Forbidden, ApiKeyError::Missing)),
        }
    }
}
