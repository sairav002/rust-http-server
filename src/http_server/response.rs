use std::fmt;



pub enum HTTPCode {
    Ok,
    NotFound,
}


impl std::fmt::Display for HTTPCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value, message) = match self {
            HTTPCode::Ok => (200, "OK"),
            HTTPCode::NotFound => (404, "Not Found"),
        };

        write!(f, "{} {}", value, message)
    }
}