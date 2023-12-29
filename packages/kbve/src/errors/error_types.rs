use std::panic::Location; 
use std::backtrace::Backtrace;

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
    StringError(#[from] String)
}