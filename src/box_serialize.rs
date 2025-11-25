use std::io::Read;
use crate::Serializeable;

impl<T: Serializeable> Serializeable for Box<T> {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        (**self).serialize_into(data);
    }

    fn deserialize<R: Read>(reader: &mut R) -> Self {
        Box::new(T::deserialize(reader))
    }
}