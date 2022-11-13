use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref VALIDATION_REGEX: Regex =
        Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$").unwrap();
}

#[derive(Debug, PartialEq, Eq)]
pub enum EmailError {
    Invalid,
}

impl From<EmailError> for crate::Error {
    fn from(error: EmailError) -> Self {
        match error {
            EmailError::Invalid => Self::ParseError,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, EmailError> {
        let email = email.into();
        if !VALIDATION_REGEX.is_match(&email) {
            Err(EmailError::Invalid)?
        }
        Ok(Self(email))
    }
}

#[cfg(test)]
mod test {
    use super::EmailError;

    #[test]
    fn invalid_emails() {
        let cases = vec![
            ("a@gmail.a", EmailError::Invalid),
            ("toto", EmailError::Invalid),
            ("foobar_@_@gmail.com", EmailError::Invalid),
            ("foobar@gmail", EmailError::Invalid),
        ];
        for (value, expected) in cases.iter() {
            assert_eq!(super::Email::new(*value).unwrap_err(), *expected);
        }
    }

    #[test]
    fn valid_emails() {
        let cases = vec!["foobar@a.com", "toto@gmail.com", "foobar@foo.com"];
        for value in cases.iter() {
            super::Email::new(*value).unwrap();
        }
    }
}
