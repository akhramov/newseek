use std::str;
use std::io::Write;

use serde::ser::{self, Serialize, Impossible};
use byteorder::{LittleEndian, WriteBytesExt};

use errors::*;

pub struct Serializer<W> {
    output: W,
}

pub fn to_slice<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(vec![]);

    value.serialize(&mut serializer)?;

    Ok(serializer.output)
}

impl<W> Serializer<W>
where W: Write,
{
    fn new(writer: W) -> Self {
        Self { output: writer }
    }

    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.output.write_all(bytes)
            .with_context(|e| {
                ErrorKind::Io(format!("Serialization error: {:?}", e))
            })?;
        Ok(())
    }
}

macro_rules! unimplemented {
    ($method:expr) => {
        Err(ErrorKind::Unimplemented($method.to_string()).into())
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Error = Error;
    type Ok = ();

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeStruct = Self;
    type SerializeMap = Self;
    type SerializeStructVariant = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;

    fn serialize_bool(self, value: bool) -> Result<()> {
        let result = if value { 0x1 } else { 0x0 };

        Ok(self.write(&[result])?)
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        let mut vec = vec![];

        vec.write_u32::<LittleEndian>(value).unwrap();

        Ok(self.write(&vec)?)
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        let mut vec = vec![];

        vec.write_u64::<LittleEndian>(value).unwrap();

        Ok(self.write(&vec)?)
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        let len = value.len() as u32;

        self.serialize_u32(len)?;

        Ok(self.write(value.as_bytes())?)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            Some(length) => {
                self.serialize_u32(length as u32)?;
                Ok(self)
            },
            None => Err(
                ErrorKind::Format.into()
            )
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_i8(self, _value: i8) -> Result<()> {
        unimplemented!("serialize_i8")
    }

    fn serialize_i16(self, _value: i16) -> Result<()> {
        unimplemented!("serialize_i16")
    }

    fn serialize_i32(self, _value: i32) -> Result<()> {
        unimplemented!("serialize_i32")
    }

    fn serialize_i64(self, _value: i64) -> Result<()> {
        unimplemented!("serialize_i64")
    }

    fn serialize_u8(self, _value: u8) -> Result<()> {
        unimplemented!("serialize_u8")
    }

    fn serialize_u16(self, _value: u16) -> Result<()> {
        unimplemented!("serialize_u16")
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        unimplemented!("serialize_f32")
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        unimplemented!("serialize_f64")
    }

    fn serialize_char(self, _value: char) -> Result<()> {
        unimplemented!("serialize_char")
    }

    fn serialize_none(self) -> Result<()> {
        unimplemented!("serialize_none")
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("serialize_some")
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        unimplemented!("serialize_bytes")
    }

    fn serialize_unit(self) -> Result<()> {
        unimplemented!("serialize_unit")
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!("serialize_unit_variant")
    }

    fn serialize_unit_struct(self, _value: &str) -> Result<()> {
        unimplemented!("serialize_unit_struct")
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("serialize_newtype_struct")
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!("serialize_newtype_variant")
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!("serialize_tuple_struct")
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!("serialize_tuple_variant")
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!("serialize_struct_variant")
    }

}


macro_rules! sequential_serializer_impl {
    ($($type:ident )*) => {
        $(
            impl<'a, W> ser::$type for &'a mut Serializer<W>
            where
                W: Write
            {
                type Error = Error;
                type Ok = ();

                fn serialize_element<T>(&mut self, value: &T) -> Result<()>
                where
                    T: ?Sized + Serialize,
                {
                    value.serialize(&mut **self)
                }

                fn end(self) -> Result<()> {
                    Ok(())
                }
            }

        )*
    };
}

sequential_serializer_impl!(SerializeSeq SerializeTuple);

impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: Write
{
    type Error = Error;
    type Ok = ();

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: Write
{
    type Error = Error;
    type Ok = ();

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
