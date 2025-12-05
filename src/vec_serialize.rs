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

    fn deserialize<R: ::std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let len = u64::deserialize(reader)?;
        (0..len).map(|_| T::deserialize(reader)).collect()
    }

    #[cfg(feature = "async")]
    fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> impl Future<Output=Result<Self, ::std::io::Error>> {
        async {
            let len = u64::async_deserialize(reader).await?;
            let mut v = Vec::with_capacity(len as usize);
            for _ in 0..len {
                v.push(T::async_deserialize(reader).await?);
            }
            Ok(v)
        }
    }
}