use std::fmt::{Debug, Display};

use serde::{Serialize, Serializer};

use super::varriable_char_array::VarriableCharArray;

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
            std::str::from_utf8(&{ self.0 }).unwrap_or_default()
        )
    }
}

impl<const N: usize> Debug for FixedCharSlice<{ N }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", self))
    }
}

impl<const N: usize> TryFrom<&VarriableCharArray> for FixedCharSlice<N> {
    type Error = String;

    fn try_from(value: &VarriableCharArray) -> Result<Self, Self::Error> {
        if value.0.len() != N {
            return Err(
                "Attempt to move a larger vector to a smaller destination aborting".to_string(),
            );
        }
        Ok(Self(unsafe {
            std::ptr::read(value.0.as_ptr() as *const _)
        }))
    }
}

impl<const N: usize> Serialize for FixedCharSlice<N> {
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
            FixedCharSlice::<7>::try_from(from)
                .unwrap_or_default()
                .to_string(),
            "BALDUR\0"
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
}
