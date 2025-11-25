use crate::Read;
use crate::Serializeable;

macro_rules! primitive_derive {
    ( $($x:ty),* ) => {
        $(
        impl Serializeable for $x {
            fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
                data.extend(self.to_be_bytes());
            }
            fn deserialize<R: Read>(reader: &mut R) -> Self {
                let mut buf = [0u8; size_of::<Self>()];
                reader.read_exact(&mut buf).unwrap();
                Self::from_be_bytes(buf)
            }
        }
        )*
    };
}

primitive_derive!(u8, u16, u32, u64, u128);
primitive_derive!(i8, i16, i32, i64, i128);
primitive_derive!(f32, f64);


impl Serializeable for bool {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        data.extend([*self as u8])
    }

    fn deserialize<R: Read>(reader: &mut R) -> Self {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf).unwrap();
        buf[0] != 0
    }
}