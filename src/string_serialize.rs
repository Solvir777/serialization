use std::io::Read;
use crate::Serializeable;

impl Serializeable for String {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        self.clone().into_bytes().serialize_into(data);
    }

    fn deserialize<R: Read>(reader: &mut R) -> Self {
        let vec = <Vec<u8>>::deserialize(reader);
        String::from_utf8(vec).unwrap()
    }
}