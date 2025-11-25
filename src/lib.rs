use std::io::Read;
pub use serializeable_derive::Serializeable;

mod primitive_serialize;
mod array_serialize;
mod vec_serialize;
mod box_serialize;
mod string_serialize;

pub trait Serializeable {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E);
    fn deserialize<R: Read>(reader: &mut R) -> Self;

    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec!();
        self.serialize_into(&mut vec);
        vec
    }
}