use std::fmt::{Debug, Display};

use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct FixedCharSlice<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for FixedCharSlice<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> From<&[u8]> for FixedCharSlice<N> {
    fn from(value: &[u8]) -> Self {
        let mut destination = [0; N];
        for (counter, byte) in value.iter().enumerate() {
            if counter >= destination.len() {
                // TODO: Throw a warning here
                break;
            }
            destination[counter] = *byte;
        }
        Self(destination)
    }
}

impl<const N: usize> From<&str> for FixedCharSlice<N> {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl<const N: usize> Display for FixedCharSlice<{ N }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&{ self.0 })
                .unwrap_or_default()
                .replace('\0', "")
        )
    }
}

impl<const N: usize> Debug for FixedCharSlice<{ N }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", self))
    }
}

impl<const N: usize> Serialize for FixedCharSlice<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

struct FixedCharSliceVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for FixedCharSliceVisitor<N> {
    type Value = FixedCharSlice<N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct FixedCharSlice")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(FixedCharSlice::from(v))
    }
}

impl<'de, const N: usize> Deserialize<'de> for FixedCharSlice<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(FixedCharSliceVisitor)
    }
}

#[cfg(test)]
mod tests {

    use std::io::{BufReader, Read, Seek, SeekFrom, Write};

    use super::*;
    #[test]
    fn valid_from_bytes_to_fixed_char_slice() {
        let from = "BALDUR".as_bytes();
        assert_eq!(
            FixedCharSlice::<6>::try_from(from)
                .unwrap_or_default()
                .to_string(),
            "BALDUR"
        )
    }

    #[test]
    fn valid_longer_from_bytes_to_fixed_char_slice() {
        let from = "BALDUR".as_bytes();
        assert_eq!(
            FixedCharSlice::<6>::try_from(from)
                .unwrap_or_default()
                .to_string(),
            "BALDUR"
        )
    }

    #[test]
    fn valid_shorter_from_bytes_to_fixed_char_slice() {
        let from = "BALDUR".as_bytes();
        assert_eq!(
            FixedCharSlice::<5>::try_from(from)
                .unwrap_or_default()
                .to_string(),
            "BALDU"
        )
    }

    #[test]
    fn deserialize_serialize_deserialize() {
        let expected = FixedCharSlice::<6>::from("BALDUR");
        let value = serde_json::to_string(&expected).unwrap();

        let mut file = tempfile::tempfile().unwrap();
        file.write_all(value.as_bytes()).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(file);
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let result: FixedCharSlice<6> = serde_json::from_slice(&buffer).unwrap();
        assert_eq!(expected, result)
    }
}
