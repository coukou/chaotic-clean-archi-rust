use std::ops::Deref;

use kernel::value::Email;
use serde::{Deserialize, Serialize};

use crate::value::Password;

pub struct Id(String);

#[derive(Clone, Serialize, Deserialize)]
pub struct Props {
    pub email: Email,
    pub password: Password,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    props: Props,
}

impl Account {
    pub fn new(props: Props) -> Self {
        Self { props }
    }
}

impl Deref for Account {
    type Target = Props;
    fn deref(&self) -> &Self::Target {
        &self.props
    }
}
