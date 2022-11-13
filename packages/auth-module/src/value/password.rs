use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordError {
    TooShort,
    HashError(String),
}

impl From<PasswordError> for kernel::Error {
    fn from(error: PasswordError) -> Self {
        match error {
            PasswordError::TooShort => Self::ParseError,
            PasswordError::HashError(_) => Self::ParseError,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn from_hash(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn new(password: impl Into<String>, hash_cost: u32) -> Result<Self, PasswordError> {
        let password = password.into();
        if password.len() < 6 {
            Err(PasswordError::TooShort)?
        }
        let hash = bcrypt::hash(password, hash_cost)
            .map_err(|error| PasswordError::HashError(error.to_string()))?;
        Ok(Self(hash))
    }

    pub fn compare(&self, password: impl Into<String>) -> bool {
        bcrypt::verify(password.into(), self.0.as_str()).unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use super::PasswordError;

    #[test]
    fn invalid_password() {
        let cases = vec![
            ("a", PasswordError::TooShort),
            ("aa", PasswordError::TooShort),
            ("aaa", PasswordError::TooShort),
            ("aaaa", PasswordError::TooShort),
            ("aaaaa", PasswordError::TooShort),
        ];
        for (value, expected) in cases.iter() {
            assert_eq!(super::Password::new(*value, 4).unwrap_err(), *expected)
        }
    }

    #[test]
    fn valid_password() {
        let cases = vec![
            "123456",
            "password",
            "password123",
            "password123@",
            "password123@!.;lpo0-/'",
        ];
        for value in cases.iter() {
            super::Password::new(*value, 4).unwrap();
        }
    }
}
