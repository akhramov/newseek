use binary_format::de;
use binary_format::ser;
use domain::*;
use errors::*;

use byteorder::{ LittleEndian, ByteOrder };
use bytes::{ BufMut, BytesMut };
use tokio_codec::{ Encoder, Decoder };


pub struct Codec;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Challenge(Challenge),
    Login(LoginRequest),
    Transfers(Transfer),
    Dummy
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Challenge(Challenge),
    Login(LoginResponse),
    PrivateMessage(PrivateMessageResponse),
    Ping
}

impl Codec {
    fn deserialize(&self, code: u32, buf: &mut BytesMut) -> Result<Request> {
        Ok(
            match code {
                1 => Request::Challenge(de::from_slice::<Challenge>(buf)?),
                2 => Request::Login(de::from_slice::<LoginRequest>(buf)?),
                1281 => Request::Transfers(de::from_slice::<Transfer>(buf)?),
                _ => Request::Dummy
            }
        )
    }
}

impl Decoder for Codec {
    type Item = Request;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if src.len() < 8 {
            return Ok(None);
        }

        let size = LittleEndian::read_u32(src) as usize;

        if src.len() >= size + 4 {
            let ref mut bytes = src.clone();

            bytes.split_to(4);
            let code = LittleEndian::read_u32(bytes);
            bytes.split_to(4);

            match self.deserialize(code, bytes) {
                Ok(result) => {
                    src.split_to(size + 4);
                    Ok(Some(result))
                }
                Err(_) => {
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for Codec {
    type Item = Response;
    type Error = Error;

    fn encode(&mut self, msg: Self::Item, dst: &mut BytesMut) -> Result<()> {
        // I miss HKT so much! :(
        let result =
            match msg {
                Response::Login(ref response) => {
                    ser::to_slice(response)?
                },
                Response::PrivateMessage(ref response) => {
                    println!("{:?}", response);
                    ser::to_slice(response)?
                },
                Response::Ping => ser::to_slice(&Ping::default())?,
                _ => panic!("IMPOSIBIRU")
            };

        // let message = ser::to_slice(&msg)?;
        let length = result.len();

        dst.reserve(length + 4);
        dst.put_u32_le(length as u32);
        dst.put(result);
        Ok(())
    }
}
