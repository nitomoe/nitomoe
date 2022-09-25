#[derive(Debug)]
pub enum Error {
    R2d2Error()
}

impl From<diesel::r2d2::Error> for Error {
    fn from(_error: diesel::r2d2::Error) -> Self {
        Self::R2d2Error()
    }
}