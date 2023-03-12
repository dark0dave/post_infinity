use std::fmt::{Debug, Display};

use serde::{ser::SerializeSeq, Serialize, Serializer};

use super::fixed_char_array::FixedCharSlice;

#[repr(C, packed)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct FixedCharNDArray<const N: usize, const M: usize>(pub [FixedCharSlice<N>; M]);

impl<const N: usize, const M: usize> Default for FixedCharNDArray<N, M> {
    fn default() -> Self {
        Self([FixedCharSlice::<N>::default(); M])
    }
}

impl<const N: usize, const M: usize> From<&[u8]> for FixedCharNDArray<N, M> {
    fn from(value: &[u8]) -> Self {
        let mut destination = Self::default();
        for (counter, byte) in value.iter().enumerate() {
            if counter >= N * M {
                // TODO: Throw a warning here
                break;
            }
            destination.0[counter / N].0[counter % N] = *byte;
        }
        destination
    }
}

impl<const N: usize, const M: usize> Display for FixedCharNDArray<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            { self.0 }
                .iter()
                .map(|array| format!("{}", array))
                .reduce(|a, b| format!("{}, {}", a, b))
                .unwrap_or_default()
        )
    }
}

impl<const N: usize, const M: usize> Debug for FixedCharNDArray<N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("\"{}\"", self))
    }
}

impl<const N: usize, const M: usize> Serialize for FixedCharNDArray<N, M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some({ self.0 }.len())).unwrap();
        for char_slice in self.0 {
            let _ = seq.serialize_element(&char_slice);
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn valid_from_bytes() {
        let from = "BALDUR".as_bytes();
        assert_eq!(FixedCharNDArray::<6, 1>::from(from).to_string(), "BALDUR")
    }

    #[test]
    fn valid_longer_from_bytes() {
        let from = "BALDUR".as_bytes();
        assert_eq!(FixedCharNDArray::<7, 1>::from(from).to_string(), "BALDUR\0")
    }

    #[test]
    fn valid_shorter_from_bytes() {
        let from = "BALDUR".as_bytes();
        assert_eq!(FixedCharNDArray::<5, 1>::from(from).to_string(), "BALDU")
    }

    #[test]
    fn valid_from_bytes_2d() {
        let from = "BALDURBALDUR".as_bytes();
        assert_eq!(
            FixedCharNDArray::<6, 2>::from(from).to_string(),
            "BALDUR, BALDUR"
        )
    }

    #[test]
    fn valid_longer_from_bytes_3d() {
        let from = "BALDURBALDURBALDUR".as_bytes();
        assert_eq!(
            FixedCharNDArray::<7, 3>::from(from).to_string(),
            "BALDURB, ALDURBA, LDUR\0\0\0"
        )
    }

    #[test]
    fn valid_shorter_from_bytes_4d() {
        let from = "BALDURBALDURBALDURBALDURBALDUR".as_bytes();
        assert_eq!(
            FixedCharNDArray::<5, 4>::from(from).to_string(),
            "BALDU, RBALD, URBAL, DURBA"
        )
    }
}
