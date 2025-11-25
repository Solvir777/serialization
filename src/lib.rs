use std::io::Read;
pub use serializeable_derive::Serializeable;

mod primitive_serialize;
mod array_serialize;
mod vec_serialize;

pub trait Serializeable {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E);
    fn deserialize<R: Read>(reader: &mut R) -> Self;

    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec!();
        self.serialize_into(&mut vec);
        vec
    }
}

enum Test{
    A,
    B{a: i32},
    C(bool, u32)
}

enum TestA {
    C { content: String, count: usize },
}


impl Serializeable for Test {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        match self {
            Test::A => {
                0.serialize_into(data);
            }
            Test::B{a, } => {
                1.serialize_into(data);
                a.serialize_into(data);
            }
            Test::C(field0, field1) => {
                2.serialize_into(data);
                field0.serialize_into(data);
                field1.serialize_into(data);
            }
        }
    }

    fn deserialize<R: Read>(reader: &mut R) -> Self {
        let id = u8::deserialize(reader);
        match id {
            0 => Test::A,
            1 => Test::B{a: Serializeable::deserialize(reader), },
            2 => Test::C(Serializeable::deserialize(reader), Serializeable::deserialize(reader), ),
            _ => panic!()
        }
    }
}