use crate::Serializeable;

impl Serializeable for String {
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        self.clone().into_bytes().serialize_into(data);
    }

    fn deserialize<R: ::std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let vec = <Vec<u8>>::deserialize(reader)?;
        match String::from_utf8(vec) {
            Ok(string) => Ok(string),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)),
        }
    }
    #[cfg(feature = "async")]
    async fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> Self {
        let vec = <Vec<u8>>::async_deserialize(reader).await;
        String::from_utf8(vec).unwrap()
    }
}