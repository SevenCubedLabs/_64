use crate::{
    data::List,
    parse::{Token, TokenStream, U16, U32},
};
use core::convert::{TryFrom, TryInto};

pub struct Font {
    sfnt_version: u32,
    n_tables: u16,
    tables: List<TableRecord>,
}

impl core::fmt::Debug for Font {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "sfntVersion: {}\nnumTables: {}\n{:?}",
            self.sfnt_version, self.n_tables, self.tables
        )
    }
}

impl TryFrom<&[u8]> for Font {
    type Error = &'static str;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        let mut stream: TokenStream = from.into();
        let U32(sfnt_version) = stream.take().and_then(|sfnt_version| match sfnt_version {
            U32(0x00010000) | U32(0x4F54544F) => Ok(sfnt_version),
            _ => Err("Invalid sfntVersion"),
        })?;

        let U16(n_tables) = stream.take()?;

        let _search_range: U16 = stream.take()?;
        let _entry_selector: U16 = stream.take()?;
        let _range_shift: U16 = stream.take()?;

        let mut tables = List::new(n_tables as _);
        for _ in 0..n_tables as _ {
            tables.push(stream.take()?);
        }

        Ok(Font {
            sfnt_version,
            n_tables,
            tables,
        })
    }
}

struct TableRecord {
    tag: Tag,
    offset: u32,
}

impl Token for TableRecord {
    const SIZE: usize = 16;
}

impl core::fmt::Debug for TableRecord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "'{}': {:#010x}",
            unsafe { core::str::from_utf8_unchecked(&self.tag.0) },
            self.offset
        )
    }
}

impl TryFrom<&[u8]> for TableRecord {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut stream: TokenStream = bytes.into();
        let tag: Tag = stream.take()?;
        let _checksum: U32 = stream.take()?;
        let U32(offset) = stream.take()?;

        Ok(TableRecord { tag, offset })
    }
}

#[derive(Debug)]
struct Tag([u8; 4]);

impl Token for Tag {
    const SIZE: usize = 4;
}

impl TryFrom<&[u8]> for Tag {
    type Error = &'static str;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_| "Tag parsing failed")
            .and_then(|bytes| Ok(Self(bytes)))
    }
}

#[test]
fn parse_font() {
    let hack = Font::try_from(include_bytes!("../assets/Hack-Regular.ttf").as_slice())
        .expect("Hack-Regular.ttf parsed");

    assert_eq!(hack.n_tables as usize, hack.tables.len());
}
