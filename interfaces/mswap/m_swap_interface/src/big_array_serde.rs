#![allow(unused)]
use std::fmt;
use std::marker::PhantomData;
use serde::ser::{Serialize, Serializer, SerializeTuple};
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};

pub fn serialize<S, T, const N: usize>(array: &[T; N], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut seq = serializer.serialize_tuple(N)?;
    for item in array.iter() {
        seq.serialize_element(item)?;
    }
    seq.end()
}

pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

    impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de>,
    {
        type Value = [T; N];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of length {}", N)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<[T; N], A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(N);
            for i in 0..N {
                vec.push(
                    seq.next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?,
                );
            }
            vec.try_into().map_err(|_| serde::de::Error::custom("wrong array length"))
        }
    }

    deserializer.deserialize_tuple(N, ArrayVisitor(PhantomData))
}
