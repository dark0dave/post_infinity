use std::{
    fmt::{Debug, Display},
    slice,
};

use serde::{ser::SerializeSeq, Serialize, Serializer};

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
            destination[counter] = *byte;
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
        let mut seq = serializer.serialize_seq(Some({ self.0 }.len())).unwrap();
        for i in self.0 {
            seq.serialize_element(&i).unwrap();
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {

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
            "BALDUR\0"
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
}
