use std::io::{Error, Read};
use nalgebra::{ArrayStorage, Const, Matrix};
use crate::Serializeable;


impl<T: Serializeable, const ROWS: usize, const COLS: usize, const U_A: usize, const U_B: usize> Serializeable for  Matrix<T, Const<U_A>, Const<U_B>, ArrayStorage<T, ROWS, COLS>>
{
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        self.data.0.serialize_into(data);
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let storage = <ArrayStorage<T, ROWS, COLS>>::deserialize(reader)?;
        unsafe { Ok(Self::from_data_statically_unchecked(storage)) }
    }

    #[cfg(feature = "async")]
    fn async_deserialize<R: ::tokio::io::AsyncReadExt + Unpin>(reader: &mut R) -> impl Future<Output=Result<Self, Error>> {
        async {
            let storage = <ArrayStorage<T, ROWS, COLS>>::async_deserialize(reader).await?;
            unsafe { Ok(Self::from_data_statically_unchecked(storage)) }
        }
    }

}

impl <T: Serializeable, const ROWS: usize, const COLS: usize> Serializeable for ArrayStorage<T, ROWS, COLS> {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        self.0.serialize_into(data);
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let array = <[[T; ROWS]; COLS]>::deserialize(reader)?;
        Ok(ArrayStorage(array))
    }

    #[cfg(feature = "async")]
    fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> impl Future<Output=Result<Self, Error>> {
        async {
            let array = <[[T; ROWS]; COLS]>::async_deserialize(reader).await?;
            Ok(ArrayStorage(array))
        }
    }
}