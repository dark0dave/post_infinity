use std::{
    fmt::{Debug, Display},
    slice,
};

use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct SignedFixedCharSlice<const N: usize>(pub [i8; N]);

impl<const N: usize> Default for SignedFixedCharSlice<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> From<&[i8]> for SignedFixedCharSlice<N> {
    fn from(value: &[i8]) -> Self {
        let mut destination = [0; N];
        for (counter, byte) in value.iter().enumerate() {
            if counter >= destination.len() {
                // TODO: Throw a warning here
                break;
            }
            destination[counter] = i8::from_le(*byte);
        }
        Self(destination)
    }
}

impl<const N: usize> From<&[u8]> for SignedFixedCharSlice<N> {
    fn from(value: &[u8]) -> Self {
        let mut destination = [0; N];
        for (counter, byte) in value.iter().enumerate() {
            if counter >= destination.len() {
                // TODO: Throw a warning here
                break;
            }
            destination[counter] = i8::try_from(*byte).unwrap_or(0);
        }
        Self(destination)
    }
}

impl<const N: usize> From<&str> for SignedFixedCharSlice<N> {
    fn from(value: &str) -> Self {
        Self::from(unsafe {
            slice::from_raw_parts(value.as_bytes().as_ptr() as *const i8, value.len())
        })
    }
}

impl<const N: usize> Display for SignedFixedCharSlice<{ N }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(unsafe {
                slice::from_raw_parts({ self.0 }.as_ptr() as *const u8, { self.0 }.len())
            })
            .unwrap_or_default()
            .replace('\0', "")
        )
    }
}

impl<const N: usize> Debug for SignedFixedCharSlice<{ N }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", self))
    }
}

impl<const N: usize> Serialize for SignedFixedCharSlice<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

struct SignedFixedCharSliceVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for SignedFixedCharSliceVisitor<N> {
    type Value = SignedFixedCharSlice<N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct SignedFixedCharSlice")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SignedFixedCharSlice::from(v))
    }
}

impl<'de, const N: usize> Deserialize<'de> for SignedFixedCharSlice<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SignedFixedCharSliceVisitor)
    }
}

#[cfg(test)]
mod tests {

    use std::io::{BufReader, Read, Seek, SeekFrom, Write};

    use super::*;
    #[test]
    fn valid_from_bytes_to_fixed_char_slice() {
        let from = "BALDUR";
        assert_eq!(
            SignedFixedCharSlice::<6>::try_from(from)
                .unwrap_or_default()
                .to_string(),
            from
        )
    }

    #[test]
    fn valid_longer_from_bytes_to_fixed_char_slice() {
        let from = "BALDUR";
        assert_eq!(
            SignedFixedCharSlice::<7>::try_from(from)
                .unwrap_or_default()
                .to_string(),
            "BALDUR"
        )
    }

    #[test]
    fn valid_shorter_from_bytes_to_fixed_char_slice() {
        let from = "BALDUR";
        assert_eq!(
            SignedFixedCharSlice::<5>::try_from(from)
                .unwrap_or_default()
                .to_string(),
            "BALDU"
        )
    }

    #[test]
    fn deserialize_serialize_deserialize() {
        let expected = SignedFixedCharSlice::<6>::from("BALDUR");
        let value = serde_json::to_string(&expected).unwrap();

        let mut file = tempfile::tempfile().unwrap();
        file.write_all(value.as_bytes()).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = vec![];
        let mut reader = BufReader::new(file);
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let result: SignedFixedCharSlice<6> = serde_json::from_slice(&buffer).unwrap();
        assert_eq!(expected, result)
    }
}
