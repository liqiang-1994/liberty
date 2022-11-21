// use std::fmt::{Display, Formatter};
// use std::lazy::Lazy;
// use axum::extract::{FromRequest, RequestParts, TypedHeader};
// use axum::headers::Authorization;
// use axum::headers::authorization::Bearer;
// use axum::http::StatusCode;
// use axum::Json;
// use axum::response::{IntoResponse, Response};
// use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
// use serde_json::json;
// use serde::{Serialize, Deserialize};
// use async_trait::async_trait;
//
// static KEYS: Lazy<Keys> = Lazy::new(|| {
//     let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     Keys::new(secret.as_bytes())
// });
//
// async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
//     if payload.client_id.is_empty() || payload.client_secret.is_empty() {
//         return Err(AuthError::MissingCredentials);
//     }
//     if payload.client_id != "foo" || payload.client_secret != "bar" {
//         return Err(AuthError::WrongCredentials);
//     }
//     let claims = Claims {
//         sub: "liqiang".to_owned(),
//         company: "ACME".to_owned(),
//         exp: 200000,
//     };
//
//     let token = encode(&Header::default(), &claims, &KEYS.encoding)
//         .map_err(|_| AuthError::TokenCreation)?;
//
//     Ok(Json(AuthBody::new(token)))
// }
//
// impl Display for Claims {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Email:{}\n Company:{}", self.sub, self.company)
//     }
// }
//
// impl AuthBody {
//     fn new(access_token: String) -> Self {
//         Self {
//             access_token,
//             token_type: "Bearer".to_string(),
//         }
//     }
// }
//
//
// #[async_trait]
// impl<B> FromRequest<B> for Claims
//     where
//         B:Send
// {
//     type Rejection = AuthError;
//
//     async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
//         let TypedHeader(Authorization(bearer)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req)
//                 .await
//                 .map_err(|_| AuthError::InvalidToken)?;
//         let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
//             .map_err(|_| AuthError::InvalidToken)?;
//         Ok(token_data.claims)
//     }
// }
//
// impl IntoResponse for AuthError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
//             AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
//             AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
//             AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
//         };
//         let body = Json(json!({
//             "error": error_message,
//         }));
//         (status, body).into_response()
//     }
// }
//
// struct Keys {
//     encoding: EncodingKey,
//     decoding: DecodingKey,
// }
//
// impl Keys {
//     fn new(secret: &[u8]) -> Self {
//         Self {
//             encoding: EncodingKey::from_secret(secret),
//             decoding: DecodingKey::from_secret(secret),
//         }
//     }
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     company: String,
//     exp: usize,
// }
//
// #[derive(Debug, Deserialize)]
// struct AuthPayload {
//     client_id: String,
//     client_secret: String,
// }
//
// #[derive(Debug, Deserialize)]
// struct AuthBody {
//     access_token: String,
//     token_type: String,
// }
//
// #[derive(Debug)]
// enum AuthError {
//     WrongCredentials,
//     MissingCredentials,
//     TokenCreation,
//     InvalidToken,
// }