
#![allow(non_camel_case_types)]
use axum::{response::{Response, IntoResponse}};
use http_error_derive::HttpError;
use axum::http::StatusCode;
use std::fmt::Display;

use crate::error_types::{FragmentError, ErrorsVariants}; 



// impl IntoResponse for RESPONSE_MESSAGES { 
//     fn into_response(self) -> axum::http::Response<axum::body::Bytes> {
//         let status_code = StatusCode::from_u16(self.http_code().unwrap_or(500)).unwrap(); 
//         let message = self.http_message().to_bytes();
//         let mut resp = Response::new(message);
//         let status = resp.status_mut();
//         *status = status_code;  
//         resp
//     }
// }


impl std::convert::From<RESPONSE_MESSAGES> for ErrorsVariants { 
    #[track_caller]
    fn from(val: RESPONSE_MESSAGES) -> ErrorsVariants {
        ErrorsVariants::ProcessingError(val)
    }
}


#[derive(Debug, HttpError)]
pub enum RESPONSE_MESSAGES {    

    //StatusCode::INTERNAL_SERVER_ERROR
    #[http(code = 500,  message = "Global Map was not set!")]
    INVALID_GLOBAL_MAP,

    //StatusCode::INTERNAL_SERVER_ERROR
    #[http(code = 503,  message = "Service unavailable on server")]
    SERVICE_UNAVAILABLE,

    //SatusCode::INTERNAL_SERVER_ERROR
    #[http(code = 500 , message = "JWT Secret was not set!")]

    INVALID_JWT,

    //StatusCode::OK
    #[http(code = 200, message = "Login was successful!")]
    DEBUG_LOGIN_WORKS,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "There was an error fetching the data!")]
    FETCH_ROUTE_FAIL,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Captcha error failed!")]
    CAPTCHA_ERROR,

    //statusCode::BAD_REQUEST
    #[http(code = 404, message = "Ulid data is invalid")]
    INVALID_ULID,

    //statusCode::BAD_REQUEST
    #[http(code = 404, message = "Data set empty")]
    EMPTY_SET,

    #[http(code = 404, message = "Ulid character set is invalid")]
    INVALID_ULID_DATA,

    //StatusCode::OK
    #[http(code = 200, message = "Account has been created!")]
    SUCCESS_ACCOUNT_CREATED,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "There was an error converting the UUID!")]
    UUID_CONVERT_FAILED,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "There was an error creating the account")]
    TASK_ACCOUNT_INIT_FAILED,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Work in progress route")]
    WIP_ROUTE,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Username was taken!")]
    USERNAME_TAKEN,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Invalid username parameter")]
    USERNAME_INVALID,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "During the user creation, there was a failure!")]
    USER_REGISTER_FAIL,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "During the auth creation, there was a failure!")]
    AUTH_INSERT_FAIL,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "During the profile creation, there was a failure!")]
    PROFILE_INSERT_FAIL,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Password was too short or must include  uppercase, lowercase, digits, and special characters")]
    INVALID_PASSWORD,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Email is invalid or not safe!")]
    INVALID_EMAIL,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Username is invalid or not safe!")]
    INVALID_USERNAME,

    //StatusCode::BAD_REQUEST
    #[http(code = 404, message = "Username was not found!")]
    USERNAME_NOT_FOUND,

    //StatusCode::INTERNAL_SERVER_ERROR
    #[http(code = 500, message = "Database error from the pool within PlayerDB Module!")]
    DATABASE_ERROR,

    //StatusCode::INTERNAL_SERVER_ERROR
    #[http(code = 500, message = "Failed to get connection from pool")]
    DATABASE_POOL_ERROR,

    //StatusCode::INTERNAL_SERVER_ERROR
    #[http(code = 500, message = "Email is already in our database as a member!")]
    EMAIL_ALREADY_IN_USER,

    //StatusCode::OK
    #[http(code = 200, message = "Email is valid but not in the database")]
    VALID_GUEST_EMAIL,

    //StatusCode::INTERNAL_SERVER_ERROR
    #[http(code = 500, message = "Json Failure! :C")]
    JSON_FAILURE
}


