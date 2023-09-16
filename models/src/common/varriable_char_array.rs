use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use serde::{Serialize, Serializer};

#[derive(PartialEq, Eq)]
pub struct VarriableCharArray(pub Rc<[u8]>);

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

impl Debug for VarriableCharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", self))
    }
}

impl From<&str> for VarriableCharArray {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().into())
    }
}

impl Clone for VarriableCharArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Serialize for VarriableCharArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format!("{}", self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strips_nulls_and_returns() {
        let from = "BALDUR\0";
        let expected = "BALDUR";
        assert_eq!(
            VarriableCharArray(from.as_bytes().into()).to_string(),
            expected
        )
    }
}
