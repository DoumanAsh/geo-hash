use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};

use crate::codec::GeoHash;

impl Serialize for GeoHash {
    #[inline]
    fn serialize<SER: Serializer>(&self, ser: SER) -> Result<SER::Ok, SER::Error> {
        ser.serialize_str(self.as_str())
    }
}

struct GeoHashVisitor;

impl<'de> serde::de::Visitor<'de> for GeoHashVisitor {
    type Value = GeoHash;

    #[inline(always)]
    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a geohash string with len in range 1..=12")
    }

    #[inline]
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        match Self::Value::from_str(v) {
            Some(result) => Ok(result),
            None => Err(serde::de::Error::custom(format_args!("Exceeds buffer capacity({} bytes)", Self::Value::MAX_LEN)))
        }
    }
}

impl<'a> Deserialize<'a> for GeoHash {
    #[inline]
    fn deserialize<D: Deserializer<'a>>(des: D) -> Result<Self, D::Error> {
        des.deserialize_str(GeoHashVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::GeoHash;

    use serde::de::Deserialize;
    use serde::de::value::{BorrowedStrDeserializer, Error as ValueError};

    #[test]
    fn should_error_one_exceeding_capacity() {
        let des = BorrowedStrDeserializer::<ValueError>::new("1234567891234");
        let res = GeoHash::deserialize(des);
        assert!(res.is_err());
    }

    #[test]
    fn should_ok_within_capacity() {
        let des = BorrowedStrDeserializer::<ValueError>::new("123456789123");
        let res = GeoHash::deserialize(des).expect("Unexpected fail");
        assert_eq!(res.as_str(), "123456789123");
    }
}
