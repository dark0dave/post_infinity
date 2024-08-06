use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableCharArray(pub Rc<[u8]>);

impl Display for VariableCharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap_or_default())
    }
}

impl Debug for VariableCharArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}

impl From<&str> for VariableCharArray {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().into())
    }
}

impl Clone for VariableCharArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Serialize for VariableCharArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

struct VariableCharArrayVisitor;

impl<'de> Visitor<'de> for VariableCharArrayVisitor {
    type Value = VariableCharArray;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct VariableCharArray")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VariableCharArray::from(v))
    }
}

impl<'de> Deserialize<'de> for VariableCharArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(VariableCharArrayVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Read, Seek, SeekFrom, Write};

    #[test]
    fn deserialize_serialize_deserialize() {
        let expected = VariableCharArray::from("BALDUR");
        let value = serde_json::to_string(&expected).unwrap();

        let mut file = tempfile::tempfile().unwrap();
        file.write_all(value.as_bytes()).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(file);
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let result: VariableCharArray = serde_json::from_slice(&buffer).unwrap();
        assert_eq!(expected, result)
    }
}
