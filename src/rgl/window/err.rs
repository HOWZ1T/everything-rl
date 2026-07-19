use std::fmt::Display;

#[derive(Debug)]
pub enum WindowError {
    FailedToCreateWindow,
}

impl Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WindowError::FailedToCreateWindow => write!(f, "Failed to create window"),
        }
    }
}