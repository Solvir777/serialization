use std::array;
use std::io::Read;
use crate::Serializeable;

impl <T, const N: usize> Serializeable for [T; N]
where
    T: Serializeable,
{
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        for elem in self.iter() {
            elem.serialize_into(data);
        }
    }

    fn deserialize<R: Read>(reader: &mut R) -> Self {
        array::from_fn(|_| T::deserialize(reader))
    }
}