use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub struct DbError {
    why: String
}

/// The error type used by db controller
impl DbError {
    pub fn new(why: &str) -> Self {
        Self {
            why: why.to_string()
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl error::Error for DbError {

}



