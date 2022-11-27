use crate::KEY;
use axum::extract::{FromRequest, RequestParts};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Redirect, Response};
use axum::TypedHeader;
use axum::{async_trait, Json};
use headers::authorization::Bearer;
use headers::Authorization;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::{Display, Formatter};

pub struct JwtKey {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    company: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl JwtKey {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\n Company: {}", self.sub, self.company)
    }
}

#[async_trait]
impl<S> FromRequest<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        //let bearer = req

        let TypedHeader(Authorization(bearer)) = req
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEY.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
        };
        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_id != "1" || payload.client_secret != "2" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        // Mandatory expiry time as UTC timestamp
        exp: 2000000000, // May 2033
    };
    let token =
        encode(&Header::default(), &claims, &KEY.encoding).map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody::new(token)))
}

pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!("Welcome to liberty :)\nYour data:\n{}", claims))
}
