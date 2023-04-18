pub mod decoder;
pub mod reader;
pub mod constants;

#[cfg(test)]
mod tests {
    use super::*;
    use super::decoder::RawTerm;

    #[test]
    fn small_int_ext() {
        let mut d = decoder::Decoder::from(vec![131, 97, 42]);
        if let RawTerm::SmallInt(i) = d.decode().unwrap() {
            assert_eq!(i, 42);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn integer_ext() {
        let mut d = decoder::Decoder::from(vec![131, 98, 0, 0, 0, 42]);
        if let RawTerm::Int(i) = d.decode().unwrap() {
            assert_eq!(i, 42);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn float_ext() {
        todo!("FLOAT_EXT")
    }

    #[test]
    fn port_ext() {
        let mut d = decoder::Decoder::from(vec![131, 102, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0, 0, 6, 0]);
        if let RawTerm::Port { node, id, creation } = d.decode().unwrap() {
            assert_eq!(node, Box::new(RawTerm::Atom("nonode@nohost".to_string())));
            assert_eq!(id, 6);
            assert_eq!(creation, 0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn new_port_ext() {
        let mut d = decoder::Decoder::from(vec![131, 89, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0, 0, 6, 0, 0, 0, 0]);
        if let RawTerm::NewPort { node, id, creation } = d.decode().unwrap() {
            assert_eq!(node, Box::new(RawTerm::Atom("nonode@nohost".to_string())));
            assert_eq!(id, 6);
            assert_eq!(creation, 0);
        }
    }

    #[test]
    fn v4_port_ext() {
        let mut d = decoder::Decoder::from(vec![131, 120, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0]);
        if let RawTerm::V4Port { node, id, creation } = d.decode().unwrap() {
            assert_eq!(node, Box::new(RawTerm::Atom("nonode@nohost".to_string())));
            assert_eq!(id, 6);
            assert_eq!(creation, 0);
        }
    }

    #[test]
    fn pid_ext() {
        let mut d = decoder::Decoder::from(vec![131, 103, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0, 0, 135, 0, 0, 0, 0, 0]);
        if let RawTerm::Pid { node, id, serial, creation } = d.decode().unwrap() {
            assert_eq!(node, Box::new(RawTerm::Atom("nonode@nohost".to_string())));
            assert_eq!(id, 135);
            assert_eq!(serial, 0);
            assert_eq!(creation, 0);
        }
    }

    #[test]
    fn new_pid_ext() {
        let mut d = decoder::Decoder::from(vec![131, 88, 100, 0, 13, 110, 111, 110, 111, 100, 101, 64, 110, 111, 104, 111, 115, 116, 0, 0, 0, 135, 0, 0, 0, 0, 0, 0, 0, 0]);
        if let RawTerm::NewPid { node, id, serial, creation } = d.decode().unwrap() {
            assert_eq!(node, Box::new(RawTerm::Atom("nonode@nohost".to_string())));
            assert_eq!(id, 135);
            assert_eq!(serial, 0);
            assert_eq!(creation, 0);
        }
    }

    #[test]
    fn small_tuple_ext() {
        let mut d = decoder::Decoder::from(vec![131, 104, 2, 97, 10, 97, 20]);
        if let RawTerm::Tuple(t) = d.decode().unwrap() {
            assert_eq!(t.len(), 2);
            assert_eq!(t[0], RawTerm::SmallInt(10));
            assert_eq!(t[1], RawTerm::SmallInt(20));
        }
    }

    #[test]
    fn large_tuple_ext() {
        let mut d = decoder::Decoder::from(vec![131, 105, 0, 0, 0, 2, 97, 10, 97, 20]);
        if let RawTerm::Tuple(t) = d.decode().unwrap() {
            assert_eq!(t.len(), 2);
            assert_eq!(t[0], RawTerm::SmallInt(10));
            assert_eq!(t[1], RawTerm::SmallInt(20));
        }
    }

    #[test]
    fn map_ext() {
        let mut d = decoder::Decoder::from(vec![131, 116, 0, 0, 0, 5, 100, 0, 1, 97, 97, 10,
                                                100, 0, 1, 98, 97, 20, 100, 0, 1, 101, 106, 100,
                                                0, 1, 102, 109, 0, 0, 0, 11, 72, 101, 108, 108,
                                                111, 32, 87, 111, 114, 108, 100, 100, 0, 1, 103, 100,
                                                0, 13, 119, 105, 111, 101, 110, 101, 110, 97, 95, 97,
                                                116, 111, 109]);
        if let RawTerm::Map(m) = d.decode().unwrap() {
            println!("{:?}", m);
        }
    }

    #[test]
    fn nil_ext() {
        let mut d = decoder::Decoder::from(vec![131, 106]);
        assert_eq!(d.decode().unwrap(), RawTerm::Nil);
    }

    #[test]
    fn string_ext() {
        let mut d = decoder::Decoder::from(vec![131, 107, 0, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);
        if let RawTerm::String(s) = d.decode().unwrap() {
            assert_eq!(s, "Hello World");
        }
    }

    #[test]
    fn list_ext() {
        let mut d = decoder::Decoder::from(vec![131, 108, 0, 0, 0, 3, 97, 10, 97, 20, 97, 30, 106]);
        if let RawTerm::List(l) = d.decode().unwrap() {
            assert_eq!(l.len(), 3);
            assert_eq!(l[0], RawTerm::SmallInt(10));
            assert_eq!(l[1], RawTerm::SmallInt(20));
            assert_eq!(l[2], RawTerm::SmallInt(30));
        }
    }

    #[test]
    fn binary_ext() {
        let mut d = decoder::Decoder::from(vec![131, 109, 0, 0, 0, 12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
        if let RawTerm::String(s) = d.decode().unwrap() {
            assert_eq!(s, "Hello World!");
        }
    }
}
