
#![allow(non_camel_case_types)]
use axum::response::{Response, IntoResponse};
use http_error_derive::HttpError;



// impl IntoResponse for RESPONSE_MESSAGES { 
//     fn to_response(self) -> Response<String> { 
//         let status = self.code(); 
//         let message = self.message(); 


//     }
// }



#[derive(Debug, thiserror::Error, HttpError;)]
pub enum RESPONSE_MESSAGES {    
    
    #[http(code = StatusCode::INTERNAL_SERVER_ERROR, message = "Global Map was not set!")]
    INVALID_GLOBAL_MAP,

    #[http(code = StatusCode::INTERNAL_SERVER_ERROR, message = "JWT Secret was not set!")]
    INVALID_JWT,

    #[http(code = StatusCode::OK, message = "Login was successful!")]
    DEBUG_LOGIN_WORKS,

    #[http(code = StatusCode::BAD_REQUEST, message = "There was an error fetching the data!")]
    FETCH_ROUTE_FAIL,

    #[http(code = StatusCode::OK, message = "Account has been created!")]
    SUCCESS_ACCOUNT_CREATED,

    #[http(code = StatusCode::BAD_REQUEST, mesage = "There was an error converting the UUID!")]
    UUID_CONVERT_FAILED,

    #[http(code = StatusCode::BAD_REQUEST, message = "There was an error creating the account")]
    TASK_ACCOUNT_INIT_FAILED,

    #[http(code = StatusCode::BAD_REQUEST, message = "There was an error creating the account")]
    TASK_ACCOUNT_INIT_FAILED,

    #[http(code = StatusCode::BAD_REQUEST, message = "Work in progress route")]
    WIP_ROUTE,

    #[http(code = StatusCode::BAD_REQUEST, message = "Username was taken!")]
    USERNAME_TAKEN,

    #[http(code = StatusCode::BAD_REQUEST, message = "During the user creation, there was a failure!")]
    USER_REGISTER_FAIL,

    #[http(code = StatusCode::BAD_REQUEST, message = "During the auth creation, there was a failure!")]
    AUTH_INSERT_FAIL,

    #[http(code = StatusCode::BAD_REQUEST, message = "During the profile creation, there was a failure!")]
    PROFILE_INSERT_FAIL,

    #[http(code = StatusCode::BAD_REQUEST, message = "Password was too short or must include  uppercase, lowercase, digits, and special characters")]
    INVALID_PASSWORD,

    #[http(code = StatusCode::BAD_REQUEST, message = "Email is invalid or not safe!")]
    INVALID_EMAIL,

    #[http(code = StatusCode::BAD_REQUEST, message = "Username is invalid or not safe!")]
    INVALID_USERNAME,

    #[http(code = StatusCode::BAD_REQUEST, message = "Username was not found!")]
    USERNAME_NOT_FOUND,

    #[http(code = StatusCode::INTERNAL_SERVER_ERROR, message = "Database error from the pool within PlayerDB Module!")]
    DATABASE_ERROR,

    #[http(code = StatusCode::INTERNAL_SERVER_ERROR, message = "Email is already in our database as a member!")]
    EMAIL_ALREADY_IN_USER,

    #[http(code = StatusCode::OK, message = "Email is valid but not in the database")]
    VALID_GUEST_EMAIL,

    #[http(code = StatusCode::INTERNAL_SERVER_ERROR, message = "Json Failure! :C")]
    JSON_FAILURE
}