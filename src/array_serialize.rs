use std::io::Error;
use crate::Serializeable;

impl <T, const N: usize> Serializeable for [T; N]
where
    T: Serializeable,
{
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E) {
        for elem in self.iter() {
            elem.serialize_into(data);
        }
    }

    fn deserialize<R: ::std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error> {
        let items: Result<Vec<T>, Error> = (0..N)
            .map(|_| T::deserialize(reader))
            .collect();
        Ok(items?.try_into().unwrap_or_else(|_| panic!("This shouldn't be able to fail")))
    }
    #[cfg(feature = "async")]
    fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> impl Future<Output=Result<Self, ::std::io::Error>> {
        async {
            let mut r = vec!();
            for _ in 0..N {
                r.push(T::async_deserialize(reader).await?);
            }
            Ok(r.try_into().unwrap_or_else(|_| panic!("This shouldn't be able to fail")))
        }
    }
}