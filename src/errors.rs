use std;
use std::fmt::{ self, Display };
use std::error::Error as StdError;

use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use serde_json;
use actix_web::error::ResponseError as WebError;
use actix_web::{ HttpResponse, http, error };
use failure::{ self, Backtrace, Fail, Context };
use self::http::StatusCode;

pub use failure::ResultExt;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)]
    Io(String),
    #[fail(display = "{}", _0)]
    FromUtf8Error(String),
    #[fail(display = "{}", _0)]
    DbError(String),
    #[fail(display = "Database connection error")]
    DbPoolError,
    #[fail(display = "invalid UTF-8 sequence, {}", _0)]
    InvalidUtf8(String),
    #[fail(display = "This operation is unsupported: {}", _0)]
    Unimplemented(String),
    #[fail(display = "Invalid format!")]
    Format,
    #[fail(display = "Invalid '{}' value, description: {}", _0, _1)]
    BadClientData(String, String),
    #[fail(display = "{}", _0)]
    Custom(String),
    #[fail(display = "FS IO Error")]
    FsError,
    #[fail(display = "Config template error.")]
    ConfigTemplate

}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        let description = err.description().to_string();

        err.context(ErrorKind::Io(description)).into()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}

impl DeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Custom(msg.to_string()).into()
    }
}

impl SerError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Custom(msg.to_string()).into()
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "An error occured"
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl WebError for Error {
    fn error_response(&self) -> HttpResponse {
        #[derive(Debug, Serialize)]
        struct ErrorBody<'a> { field: &'a String, message: &'a String };

        match self.inner.get_context() {
            ErrorKind::BadClientData(field, message) => {
                let ref err =
                    ErrorBody {
                        field: &field.to_string(),
                        message: &message.to_string()
                    };

                // That does not need to be so painful
                let serialized = serde_json::to_string(err).unwrap();
                HttpResponse::with_body(StatusCode::BAD_REQUEST, serialized)
            }

            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
