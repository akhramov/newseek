use serde::de::{self, Deserialize, Visitor};
use byteorder::{ LittleEndian, ByteOrder };

use std::io::Read;
use std::str;
use errors::*;

macro_rules! not_implemented {
    ($($method:ident )*) => {
        $(fn $method<V>(self, _: V) -> Result<V::Value>
            where V: Visitor<'de>,
          {
              Err(ErrorKind::Unimplemented(stringify!($method).to_string()).into())
          }
        )*
    };
    ($($method:ident )* ; 2) => {
        $(fn $method<V>(self, _: &'static str, _: V) -> Result<V::Value>
            where V: Visitor<'de>,
          {
              Err(ErrorKind::Unimplemented(stringify!($method).to_string()).into())
          }
        )*
    };
    ($($method:ident )* ; 3) => {
        $(fn $method<V>(self, _: &'static str, _:  &'static [&'static str],  _: V) -> Result<V::Value>
            where V: Visitor<'de>,
          {
              Err(ErrorKind::Unimplemented(stringify!($method).to_string()).into())
          }
        )*
    }
}

pub struct Deserializer<'de> {
    input: &'de [u8]
}

impl<'de> Deserializer<'de> {
    pub fn from_slice(bytes: &'de [u8]) -> Deserializer<'de> {
        Deserializer { input: bytes }
    }
}

pub fn from_slice<'de, T>(bytes: &'de mut [u8]) -> Result<T>
where
    T: Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_slice(bytes);
    T::deserialize(&mut deserializer)
}


impl<'de> Deserializer<'de> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<usize> {
        if bytes.len() > self.input.len() {
            return Err(ErrorKind::Io(format!("Deserialization error: {:?}", "io")).into());
        }

        Ok(
            self.input.read(bytes)
                .with_context(|e| {
                    ErrorKind::Io(format!("Deserialization error: {:?}", e))
                })?
        )
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read(&mut buf)?;

        Ok(buf[0])
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read(&mut buf)?;

        Ok(LittleEndian::read_u32(&buf))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        self.read(&mut buf)?;

        Ok(LittleEndian::read_u64(&buf))
    }

    fn read_str(&mut self) -> Result<String> {
        let len = self.read_u32()? as usize;
        let mut buffer = vec![0; len];

        self.read(&mut buffer)?;

        Ok(
            String::from_utf8(buffer)
                .with_context(|e| {
                    ErrorKind::FromUtf8Error(format!("Deserialization error: {:?}", e))
                })?

        )
    }
}

impl<'a, 'de: 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value: u8 = self.read_u8()?;
        match value {
            0 => visitor.visit_bool(false),
            _ => visitor.visit_bool(true)
        }

    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.read_u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.read_u64()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.read_str()?)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.read_u32()? as usize;

        visitor.visit_seq(Access::new(&mut self, len))
    }

    fn deserialize_struct<V>(
        self,
        _name: &str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V>(
        mut self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(Access::new(&mut self, len))
    }

    not_implemented!(deserialize_i8 deserialize_any deserialize_i16 deserialize_i32 deserialize_i64 deserialize_u8 deserialize_u16 deserialize_f32 deserialize_f64 deserialize_char deserialize_str deserialize_bytes deserialize_byte_buf deserialize_option deserialize_unit deserialize_map deserialize_identifier deserialize_ignored_any);
    not_implemented!(deserialize_unit_struct deserialize_newtype_struct ; 2);
    not_implemented!(deserialize_enum ; 3);
}

struct Access<'a, 'de: 'a> {
    deserializer: &'a mut Deserializer<'de>,
    len: usize
}

impl<'a, 'de: 'a> Access<'a, 'de> {
    fn new(deserializer: &'a mut Deserializer<'de>, len: usize) -> Self {
        Access {
            deserializer,
            len,
        }
    }
}

impl<'a, 'de: 'a> de::SeqAccess<'de> for Access<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.len == 0 {
            return Ok(None);
        }

        self.len = self.len - 1;

        let value = seed.deserialize(&mut *self.deserializer)?;

        Ok(Some(value))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}
