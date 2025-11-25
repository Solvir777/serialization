use std::io::Read;
use crate::Serializeable;

impl<T> Serializeable for Vec<T>
where
    T: Serializeable
{
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        (self.len() as u64).serialize_into(data);
        self.iter().for_each(
            |elem| elem.serialize_into(data)
        )
    }

    fn deserialize<R: Read>(reader: &mut R) -> Self {
        let len = u64::deserialize(reader);
        (0..len).map(|_| T::deserialize(reader)).collect()
    }
}