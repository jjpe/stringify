use ioe::IoError;
use std::io;

pub type StringifyResult<T> = Result<T, StringifyError>;

pub enum StringifyError {
    IoError(IoError),
    StyleNotFound { name: &'static str },
}

impl From<io::Error> for StringifyError {
    fn from(err: io::Error) -> StringifyError {
        StringifyError::IoError(IoError::from(err))
    }
}
