use std::collections::BTreeMap;
use std::ops::Bound::Included;

use byteorder::{BigEndian, ByteOrder};

/// # Database
///
/// ```
/// use protohackers::means_to_an_end::DB;
/// let mut db = DB::new();
/// db.insert(12345, 101);
/// db.insert(12346, 102);
/// db.insert(12347, 100);
/// db.insert(40960, 5);
/// assert_eq!(db.query(12288, 16384), 101);
/// ```
pub struct DB {
    records: BTreeMap<i32, i32>,
}

impl DB {
    pub fn new() -> DB {
        DB {
            records: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, timestamp: i32, price: i32) {
        self.records.insert(timestamp, price);
    }

    pub fn query(&self, low: i32, high: i32) -> i32 {
        if low > high {
            return 0;
        }

        let (n, total) = self
            .records
            .range((Included(low), Included(high)))
            .fold((0, 0i128), |(n, total), (_, p)| (n + 1, total + *p as i128));
        if n == 0 {
            0
        } else {
            (total / n) as i32
        }
    }
}

/// # Request parsing
///
/// ```
/// use protohackers::means_to_an_end::Request;
/// assert_eq!(Request::parse(&[0x49, 0x00, 0x00, 0x30, 0x39, 0x00, 0x00, 0x00, 0x65]), Some(Request::Insert{timestamp: 12345, price: 101}));
/// assert_eq!(Request::parse(&[0x49, 0x00, 0x00, 0x30, 0x3a, 0x00, 0x00, 0x00, 0x66]), Some(Request::Insert{timestamp: 12346, price: 102}));
/// assert_eq!(Request::parse(&[0x49, 0x00, 0x00, 0x30, 0x3b, 0x00, 0x00, 0x00, 0x64]), Some(Request::Insert{timestamp: 12347, price: 100}));
/// assert_eq!(Request::parse(&[0x49, 0x00, 0x00, 0xa0, 0x00, 0x00, 0x00, 0x00, 0x05]), Some(Request::Insert{timestamp: 40960, price: 5}));
/// assert_eq!(Request::parse(&[0x51, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x40, 0x00]), Some(Request::Query{low: 12288, high: 16384}));
/// ```
#[derive(PartialEq, Debug)]
pub enum Request {
    Insert { timestamp: i32, price: i32 },
    Query { low: i32, high: i32 },
}

impl Request {
    pub fn parse(buffer: &[u8; 9]) -> Option<Request> {
        let m = BigEndian::read_i32(&buffer[1..5]);
        let n = BigEndian::read_i32(&buffer[5..9]);

        match buffer[0] {
            b'I' => Some(Request::Insert {
                timestamp: m,
                price: n,
            }),
            b'Q' => Some(Request::Query { low: m, high: n }),
            _ => None,
        }
    }
}

/// # Response parsing
///
/// ```
/// use protohackers::means_to_an_end::Response;
/// assert_eq!(Response{result: 101}.output(), [0x00, 0x00, 0x00, 0x65]);
/// ```
#[derive(PartialEq, Debug)]
pub struct Response {
    pub result: i32,
}

impl Response {
    pub fn output(&self) -> [u8; 4] {
        let mut buf = [0u8; 4];
        BigEndian::write_i32(&mut buf, self.result);
        buf
    }
}
