use std::panic::Location; 
use std::backtrace::Backtrace;
use std::fmt::Display;

use reqwest::header::InvalidHeaderValue;

use crate::resp_msg::RESPONSE_MESSAGES;

pub type ProcessResult<T> = std::result::Result<T, FragmentError>; 

#[derive(Debug)]
pub struct FragmentError { 
    error: ErrorsVariants,
    trace: Backtrace, 
    location: Location<'static>

}

impl<T> std::convert::From<T> for FragmentError
where T: Into<ErrorsVariants>  
{ 
    #[track_caller]
    fn from(val: T) -> FragmentError {
        let backtrace = Backtrace::capture(); 
        let location = Location::caller().to_owned(); 
        Self { 
            error: val.into(), 
            trace: backtrace,
            location: location 
        }
    }
}

// impl<T> std::convert::From<T> for FragmentError
// where T: Into<&'static str>  
// { 
//     #[track_caller]
//     fn from(val: T) -> FragmentError {
//         let backtrace = Backtrace::capture(); 
//         let location = Location::caller().to_owned(); 
//         Self { 
//             error: val.into(), 
//             trace: backtrace,
//             location: location 
//         }
//     }
// }


#[derive(Debug, thiserror::Error,)]
pub enum ErrorsVariants { 

    #[error("{0:?}")]
    ProcessingError(RESPONSE_MESSAGES),
    
    #[error("The header value init failed for the value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    #[error("Reqwest Client error: {0}")]
    ReqwestError(#[from] reqwest::Error),


    // #[error("{0:?}")]
    // StringError(#[from] &'static String)
}