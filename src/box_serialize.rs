use crate::Serializeable;

impl<T: Serializeable> Serializeable for Box<T> {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        (**self).serialize_into(data);
    }

    fn deserialize<R: ::std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        Ok(Box::new(T::deserialize(reader)?))
    }
    #[cfg(feature = "async")]
    async fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> Self {
        Box::new(T::async_deserialize(reader).await)
    }
}