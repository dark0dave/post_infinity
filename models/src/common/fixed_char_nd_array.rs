use std::fmt::{Debug, Display};

use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Serialize, Serializer};

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
                .replace('\0', "")
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
        // TODO: Fix this crap
        let mut seq = serializer.serialize_seq(Some({ self.0 }.len())).unwrap();
        for char_slice in self.0 {
            let _ = seq.serialize_element(&char_slice);
        }
        seq.end()
    }
}

struct FixedCharNDArrayVisitor<const N: usize, const M: usize>;

impl<'de, const N: usize, const M: usize> Visitor<'de> for FixedCharNDArrayVisitor<N, M> {
    type Value = FixedCharNDArray<N, M>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "struct FixedCharNDArray")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut destination = FixedCharNDArray::<N, M>::default();
        let mut counter = 0;
        while let Ok(Some(item)) = seq.next_element::<FixedCharSlice<N>>() {
            destination.0[counter] = item;
            counter += 1;
        }
        Ok(destination)
    }
}

impl<'de, const N: usize, const M: usize> Deserialize<'de> for FixedCharNDArray<N, M> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(FixedCharNDArrayVisitor)
    }
}

#[cfg(test)]
mod tests {

    use std::io::{BufReader, Read, Seek, SeekFrom, Write};

    use super::*;
    #[test]
    fn valid_from_bytes() {
        let from = "BALDUR".as_bytes();
        assert_eq!(FixedCharNDArray::<6, 1>::from(from).to_string(), "BALDUR")
    }

    #[test]
    fn valid_longer_from_bytes() {
        let from = "BALDUR".as_bytes();
        assert_eq!(FixedCharNDArray::<7, 1>::from(from).to_string(), "BALDUR")
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
            "BALDURB, ALDURBA, LDUR"
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

    #[test]
    fn deserialize_serialize_deserialize() {
        let from = "BALDUR".as_bytes();
        let expected = FixedCharNDArray::<3, 2>::from(from);
        let value = serde_json::to_string(&expected).unwrap();

        let mut file = tempfile::tempfile().unwrap();
        file.write_all(value.as_bytes()).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        let mut buffer = Vec::new();
        let mut reader = BufReader::new(file);
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let result: FixedCharNDArray<3, 2> = serde_json::from_slice(&buffer).unwrap();
        assert_eq!(expected, result)
    }
}
