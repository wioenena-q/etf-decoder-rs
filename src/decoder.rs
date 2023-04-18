use std::collections::HashMap;

use crate::decoder;

use super::{constants, reader};

#[derive(Debug, PartialEq)]
pub enum RawTerm {
    SmallInt(u8),
    Int(i32),
    Float(f64),
    Port { node: Box<RawTerm>, id: u32, creation: u8 },
    NewPort { node: Box<RawTerm>, id: u32, creation: u32 },
    V4Port { node: Box<RawTerm>, id: u64, creation: u32 },
    Pid { node: Box<RawTerm>, id: u32, serial: u32, creation: u8 },
    NewPid { node: Box<RawTerm>, id: u32, serial: u32, creation: u32 },
    Tuple(Box<[RawTerm]>),
    Map(HashMap<String, RawTerm>),
    Nil,
    String(String),
    List(Vec<RawTerm>),
    Binary(Vec<u8>),
    SmallBigI(i64),
    SmallBigU(u64),
    LargeBigU(u64),
    LargeBigI(i64),
    Reference { node: Box<RawTerm>, id: u32, creation: u8 },
    NewReference { node: Box<RawTerm>, id: Vec<u32>, creation: u8 },
    NewerReference { node: Box<RawTerm>, id: Vec<u32>, creation: u32 },
    Fun,
    NewFun,
    Export { module: Box<RawTerm>, function: Box<RawTerm>, arity: u8 },
    BitBinary(Vec<u8>),
    Atom(String),
}

#[derive(Debug)]
pub struct Decoder {
    reader: reader::Reader,
}

impl From<Vec<u8>> for Decoder {
    fn from(buf: Vec<u8>) -> Decoder {
        let reader = reader::Reader::from(buf);
        Decoder { reader }
    }
}

impl Decoder {
    pub fn decode(&mut self) -> Option<RawTerm> {
        let version = self.reader.read_u8();
        assert_eq!(version, Some(constants::VERSION));
        self.decode_raw_term()
    }

    fn decode_raw_term(&mut self) -> Option<RawTerm> {
        let tag = self.reader.read_u8().unwrap();

        match tag {
            constants::SMALL_INTEGER_EXT => Some(RawTerm::SmallInt(self.reader.read_u8().unwrap())),
            constants::INTEGER_EXT => Some(RawTerm::Int(self.reader.read_i32(true).unwrap())),
            constants::FLOAT_EXT => todo!("FLOAT_EXT"),
            constants::PORT_EXT | constants::NEW_PORT_EXT | constants::V4_PORT_EXT => {
                let node = Box::new(self.decode_raw_term().unwrap());
                match tag {
                    constants::PORT_EXT | constants::NEW_PORT_EXT => {
                        let id = self.reader.read_u32(true).unwrap();
                        match tag {
                            constants::PORT_EXT => Some(RawTerm::Port { node, id, creation: self.reader.read_u8().unwrap() }),
                            constants::NEW_PORT_EXT => Some(RawTerm::NewPort { node, id, creation: self.reader.read_u32(true).unwrap() }),
                            _ => None
                        }
                    }
                    constants::V4_PORT_EXT => {
                        let id = self.reader.read_u64(true).unwrap();
                        Some(RawTerm::V4Port { node, id, creation: self.reader.read_u32(true).unwrap() })
                    }
                    _ => None
                }
            }
            constants::PID_EXT | constants::NEW_PID_EXT => {
                let node = Box::new(self.decode_raw_term().unwrap());
                let id = self.reader.read_u32(true).unwrap();
                let serial = self.reader.read_u32(true).unwrap();
                match tag {
                    constants::PID_EXT => Some(RawTerm::Pid { node, id, serial, creation: self.reader.read_u8().unwrap() }),
                    constants::NEW_PID_EXT => Some(RawTerm::NewPid { node, id, serial, creation: self.reader.read_u32(true).unwrap() }),
                    _ => None
                }
            }
            constants::SMALL_TUPLE_EXT | constants::LARGE_TUPLE_EXT => {
                let len: usize = match tag {
                    constants::SMALL_TUPLE_EXT => self.reader.read_u8().unwrap() as usize,
                    constants::LARGE_TUPLE_EXT => self.reader.read_u32(true).unwrap() as usize,
                    _ => 0
                };

                let mut tuple = Vec::with_capacity(len);
                for _ in 0..len {
                    tuple.push(self.decode_raw_term().unwrap());
                }

                Some(RawTerm::Tuple(tuple.into_boxed_slice()))
            }
            constants::MAP_EXT => {
                let len = self.reader.read_u32(true).unwrap() as usize;
                let mut map = HashMap::with_capacity(len);
                for _ in 0..len {
                    if let Some(RawTerm::Atom(key)) = self.decode_raw_term() {
                        let value = self.decode_raw_term().unwrap();
                        map.insert(key, value);
                    } else {
                        panic!("Invalid map key");
                    }
                }
                Some(RawTerm::Map(map))
            }
            constants::NIL_EXT => Some(RawTerm::Nil),
            constants::STRING_EXT => {
                let len = self.reader.read_u16(true).unwrap();
                let buf = self.reader.buf[self.reader.pos..self.reader.pos + len as usize].to_vec();
                Some(RawTerm::String(String::from_utf8(buf).unwrap()))
            }
            constants::LIST_EXT => {
                let len = self.reader.read_u32(true).unwrap() as usize;
                let mut list = Vec::with_capacity(len);
                for _ in 0..len {
                    match self.decode_raw_term() {
                        Some(decoded) => list.push(decoded),
                        None => ()
                    }
                }

                Some(decoder::RawTerm::List(list))
            }
            constants::BINARY_EXT => {
                let len = self.reader.read_u32(true).unwrap() as usize;
                let buf = self.reader.buf[self.reader.pos..self.reader.pos + len].to_vec();
                self.reader.pos += len;
                Some(RawTerm::String(String::from_utf8(buf).unwrap()))
            }
            constants::SMALL_BIG_EXT | constants::LARGE_BIG_EXT => todo!("SMALL_BIG_EXT | LARGE_BIG_EXT"),
            constants::DEPRECATED_REFERENCE_EXT => todo!("DEPRECATED_REFERENCE_EXT"),
            constants::NEW_REFERENCE_EXT | constants::NEWER_REFERENCE_EXT => todo!("NEW_REFERENCE_EXT | NEWER_REFERENCE_EXT"),
            constants::REMOVED_FUN_EXT => todo!("REMOVED_FUN_EXT"),
            constants::NEW_FUN_EXT => todo!("FUN_EXT"),
            constants::EXPORT_EXT => todo!("EXPORT_EXT"),
            constants::BIT_BINARY_EXT => todo!("BIT_BINARY_EXT"),
            constants::NEW_FLOAT_EXT => todo!("NEW_FLOAT_EXT"),
            constants::ATOM_UTF_8_EXT | constants::SMALL_ATOM_UTF_8_EXT | constants::DEPRECATED_ATOM_EXT | constants::DEPRECATED_SMALL_ATOM_EXT => {
                let len: usize = match tag {
                    constants::ATOM_UTF_8_EXT | constants::DEPRECATED_ATOM_EXT => self.reader.read_u16(true).unwrap() as usize,
                    constants::SMALL_ATOM_UTF_8_EXT | constants::DEPRECATED_SMALL_ATOM_EXT => self.reader.read_u8().unwrap() as usize,
                    _ => 0
                };
                let name_bytes = self.reader.buf[self.reader.pos..self.reader.pos + len].to_vec();
                self.reader.pos += len;
                Some(RawTerm::Atom(String::from_utf8(name_bytes).unwrap()))
            }
            _ => panic!("Unsupported tag: {}", tag)
        }
    }
}