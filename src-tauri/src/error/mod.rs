use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct STLError {
    message: String,
}

impl STLError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for STLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<surrealdb::Error> for STLError {
    fn from(value: surrealdb::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<std::io::Error> for STLError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

impl From<base64::DecodeError> for STLError {
    fn from(value: base64::DecodeError) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}
