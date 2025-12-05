use crate::Serializeable;

impl<T: Serializeable> Serializeable for Box<T> {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        (**self).serialize_into(data);
    }

    fn deserialize<R: ::std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        Ok(Box::new(T::deserialize(reader)?))
    }
    #[cfg(feature = "async")]
    fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> impl Future<Output=Result<Self, ::std::io::Error>> {
        async {
            Ok(Box::new(T::async_deserialize(reader).await?))
        }
    }
}