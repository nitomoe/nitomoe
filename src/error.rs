#[derive(Debug)]
pub enum Error {
    R2d2Error,
    ActixWebError,
    Diesel(diesel::result::Error),
    BoardDoesNotExist,
    NewThreadMissingFile
}

impl From<diesel::r2d2::Error> for Error {
    fn from(_error: diesel::r2d2::Error) -> Self {
        Self::R2d2Error
    }
}

impl From<actix_web::Error> for Error {
    fn from(_error: actix_web::Error) -> Self {
        Self::ActixWebError
    }
}

impl From<diesel::result::Error> for Error {
    fn from(error: diesel::result::Error) -> Self {
        Self::Diesel(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R2d2Error => write!(f, "R2d2Error"),
            Self::ActixWebError => write!(f, "ActixWebError"),
            Self::Diesel(e) => write!(f, "DieselError: {:?}", e),
            Self::BoardDoesNotExist => write!(f, "Board does not exist"),
            Self::NewThreadMissingFile => write!(f, "You must upload a file when creating a new thread.")
        }
    }
}