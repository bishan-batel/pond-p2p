use std::{fmt::Display, ops::Deref, sync::Arc};

use serde::{ser::SerializeStruct, Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct Username(String);

impl From<String> for Username {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Hash)]
pub struct User {
    name: Username,
}

impl User {
    #[must_use]
    pub fn new(name: Username) -> Self {
        Self { name }
    }
}
