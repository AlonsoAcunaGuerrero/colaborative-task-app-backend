
use actix_web::{http::StatusCode, error::ResponseError};
use chrono::{Duration, TimeDelta, Utc};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Params
};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
    errors::ErrorKind as JWTError
};

use crate::{
    errors::{AuthError, HttpError}, 
    models::{LoginRequest, LoginResponse, User}
};
use super::UserService;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub tt: TokenType
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    ACCESS,
    REFRESH
}

pub struct AuthService;

impl AuthService {
    pub async fn login(request: LoginRequest) -> Result<LoginResponse, HttpError> {
        let user = UserService::get_user_by_email(request.email.as_str())
        .await.map_err(|e| e)?;
    
        let is_valid = Self::is_valid_password(request.password, 
            &user.password).await.map_err(|e| e)?;

        if !is_valid {
            return Err(HttpError{
                code: AuthError::NotValidDataError.status_code(),
                message: AuthError::NotValidDataError.to_string()
            });
        }

        let at_duration = Duration::hours(10);
        let rt_duration = Duration::days(7);

        let access_token =  Self::create_token(user.clone(), at_duration, TokenType::ACCESS).await
        .map_err(|e| e)?;

        let refresh_token =  Self::create_token(user.clone(), rt_duration, TokenType::REFRESH).await
        .map_err(|e| e)?;

        Ok(LoginResponse{
            access_token,
            refresh_token
        })
    }
    
    pub async fn encrypt_password(password: String) -> Result<String, HttpError> {
        dotenv().ok();

        let secret_key = env::var("PASSWORD_ENCRYPTION_KEY").map_err(
            |_|
            HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Error loading environment variables.")
            }
        )?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new_with_secret(secret_key.as_bytes(), 
            argon2::Algorithm::Argon2id, argon2::Version::V0x13, Params::DEFAULT)
            .map_err(|_| HttpError {
                code: AuthError::EncryptionError.status_code(),
                message: AuthError::EncryptionError.to_string()
            })?;
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|_| HttpError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Error Encrypting the User's Password.")
        })?.to_string();

        Ok(password_hash)
    }

    async fn create_token(user: User, duration: TimeDelta, tt: TokenType) -> Result<String, HttpError> {
        dotenv().ok();

        let secret_key = env::var("SECRET_KEY").map_err(
            |_|
            HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Error loading environment variables.")
            }
        )?;

        let expiration_date = Utc::now() + duration;

        let sub = user.user_id.to_string();
        let exp = expiration_date.timestamp() as usize;
        let iat = Utc::now().timestamp() as usize;
        let iss = String::from("Backend Server");

        let my_claims = Claims {
            sub,
            exp,
            iat,
            iss,
            tt
        };

        let token = encode(&Header::new(Algorithm::HS512), &my_claims, 
        &EncodingKey::from_secret(secret_key.as_ref())).map_err(|_| HttpError{
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Error creating auth token.")
        })?;

        Ok(token)
    }

    pub async fn read_token(token: String) -> Result<Claims, HttpError> {
        dotenv().ok();

        let secret_key: String = env::var("SECRET_KEY").map_err(
            |_|
            HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Error loading environment variables.")
            }
        )?;

        let token_claims = decode::<Claims>(&token, 
            &DecodingKey::from_secret(secret_key.as_ref()), &Validation::new(Algorithm::HS512))
            .map_err(|e| 
                match e.into_kind() {
                    JWTError::InvalidToken => HttpError{
                        code: StatusCode::UNAUTHORIZED, 
                        message: String::from("The token is invalid.")},
                    JWTError::ExpiredSignature => HttpError{
                        code: StatusCode::UNAUTHORIZED, 
                        message: String::from("The token expired.")},
                    _ => HttpError{code: StatusCode::UNAUTHORIZED, message: String::from("Error! The token is invalid.")}
                }
            )?;

        let claims = token_claims.claims;
        
        Ok(claims)
    }

    async fn is_valid_password(password: String, hash: &String) -> Result<bool, HttpError> {
        let parsed_hash = PasswordHash::new(hash).map_err(
            |_| HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Error dencrypting the user's password.")
            }
        )?;

        let secret_key: String = env::var("PASSWORD_ENCRYPTION_KEY").map_err(
            |_|
            HttpError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: String::from("Error loading environment variables.")
            }
        )?;

        let argon2 = Argon2::new_with_secret(secret_key.as_bytes(), 
            argon2::Algorithm::Argon2id, argon2::Version::V0x13, Params::DEFAULT)
            .map_err(|_| HttpError {
                code: AuthError::EncryptionError.status_code(),
                message: AuthError::EncryptionError.to_string()
            })?;

        let is_valid: bool = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();

        Ok(is_valid)
    }
}