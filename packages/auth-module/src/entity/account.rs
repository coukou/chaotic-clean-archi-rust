use std::ops::Deref;

use kernel::value::Email;
use serde::{Deserialize, Serialize};

use crate::value::Password;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Id(kernel::value::Id);

impl Id {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Props {
    pub email: Email,
    pub password: Password,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Account {
    id: Id,

    #[serde(flatten)]
    props: Props,
}

impl Account {
    pub fn new(props: Props) -> Self {
        Self {
            id: Id(kernel::value::Id::new()),
            props,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

impl Deref for Account {
    type Target = Props;
    fn deref(&self) -> &Self::Target {
        &self.props
    }
}
