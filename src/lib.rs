#![feature(generic_const_exprs)]


pub use serializeable_derive::Serializeable;

mod primitive_serialize;
mod array_serialize;
mod vec_serialize;
mod box_serialize;
mod string_serialize;

#[cfg(feature = "nalgebra")]
mod nalgebra_arrays;

pub trait Serializeable: Sized {

    /// Converts self to raw bytes and appends them onto data
    fn serialize_into<E: Extend<u8>>(&self, data: &mut E);
    /// Consumes as many bytes from reader as needed to construct Self.\
    /// Should only be called when you are sure the underlying data is of the specified type (use an enum to enable multiple possible types)
    fn deserialize<R: ::std::io::Read>(reader: &mut R) -> Result<Self, std::io::Error>;

    /// deserialize a struct from an AsyncRead source. Useful if you want to deserialize from a tokio::TcpStream
    #[cfg(feature = "async")]
    fn async_deserialize<R: ::tokio::io::AsyncRead + Unpin>(reader: &mut R) -> impl Future<Output=Result<Self, ::std::io::Error>>;
    /// Converts the Data into a vector of the raw bytes.
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec!();
        self.serialize_into(&mut vec);
        vec
    }

    fn store_to_disk<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), std::io::Error> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        let bytes = self.serialize();
        file.write_all(&bytes)?;
        Ok(())
    }
    fn load_from_disk<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut file = std::fs::File::open(path)?;
        Self::deserialize(&mut file)
    }
}


#[derive(Serializeable)]
enum Test{
    A(Vec<u8>),
    B{a: Vec<u8>, b: Vec<u8>},
    C,
}

#[derive(Serializeable)]
struct Test2 {
    test: i32,
    str: String,
}

#[derive(Serializeable)]
struct Test3 (String, String);