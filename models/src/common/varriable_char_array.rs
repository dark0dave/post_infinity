use std::fmt::Display;

use serde::Serialize;

pub const DEFAULT: &VarriableCharArray = &VarriableCharArray(vec![]);

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct VarriableCharArray(pub Vec<u8>);

impl Display for VarriableCharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.0)
                .unwrap_or_default()
                .replace('\0', "")
        )
    }
}

impl From<&str> for VarriableCharArray {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

// TODO: Make this less expensive
impl Clone for VarriableCharArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
