use core::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

pub struct TokenStream<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> TokenStream<'a> {
    pub fn peek<T: Token>(&self) -> Result<T, &'static str> {
        self.bytes
            .get(self.cursor..self.cursor + T::SIZE)
            .ok_or("failed to get bytes")?
            .try_into()
    }

    pub fn take<T: Token>(&mut self) -> Result<T, &'static str> {
        self.bytes
            .get(self.cursor..self.cursor + T::SIZE)
            .ok_or("failed to get bytes")?
            .try_into()
            .and_then(|token| {
                self.seek(self.cursor + T::SIZE);
                Ok(token)
            })
    }

    pub fn seek(&mut self, offset: usize) {
        self.cursor = offset;
    }
}

impl<'a> From<&'a [u8]> for TokenStream<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        TokenStream { bytes, cursor: 0 }
    }
}

pub trait Token: for<'a> TryFrom<&'a [u8], Error = &'static str> + Debug {
    const SIZE: usize;
}

#[derive(Debug, PartialEq)]
pub struct U32(pub u32);

impl Token for U32 {
    const SIZE: usize = 4;
}

impl TryFrom<&[u8]> for U32 {
    type Error = &'static str;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_| "U32 parsing failed")
            .and_then(|bytes| Ok(Self(u32::from_be_bytes(bytes))))
    }
}

#[derive(Debug, PartialEq)]
pub struct U16(pub u16);

impl Token for U16 {
    const SIZE: usize = 2;
}

impl TryFrom<&[u8]> for U16 {
    type Error = &'static str;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_| "U16 parsing failed")
            .and_then(|bytes| Ok(Self(u16::from_be_bytes(bytes))))
    }
}

#[test]
fn parse_u32() {
    let bytes = 0xDEADBEEF_u32.to_be_bytes();
    let mut test: TokenStream = bytes.as_slice().into();

    let res: U32 = test.take().expect("parse u32 from token stream");
    assert_eq!(U32(0xDEADBEEF), res);
    assert_eq!(test.cursor, 4);
}

#[test]
fn parse_u16() {
    let bytes = 0xBEEF_u16.to_be_bytes();
    let mut test: TokenStream = bytes.as_slice().into();

    let res: U16 = test.take().expect("parse u32 from token stream");
    assert_eq!(U16(0xBEEF), res);
    assert_eq!(test.cursor, 2);
}

#[test]
fn sequence() {
    let bytes = [0u8; 10];
    let mut test: TokenStream = bytes.as_slice().into();

    let _res: U32 = test.take().expect("parse u32 from token stream");
    assert_eq!(test.cursor, 4);

    let _res: U16 = test.take().expect("parse u16 from token stream");
    assert_eq!(test.cursor, 6);

    let _res: U32 = test.take().expect("parse u32 from token stream");
    assert_eq!(test.cursor, 10);
}
