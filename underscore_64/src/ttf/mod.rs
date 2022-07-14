use core::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct Font {
    sfnt_version: u32,
    n_tables: u16,
}

impl TryFrom<&[u8]> for Font {
    type Error = &'static str;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        let U32(sfnt_version) =
            from[0..4]
                .try_into()
                .and_then(|sfnt_version| match sfnt_version {
                    U32(0x00010000) | U32(0x4F54544F) => Ok(sfnt_version),
                    _ => Err("Invalid sfntVersion"),
                })?;

        let U16(n_tables) = from[4..6].try_into()?;

        Ok(Font {
            sfnt_version,
            n_tables,
        })
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct U32(u32);

impl TryFrom<&[u8]> for U32 {
    type Error = &'static str;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_| "U32 parsing failed")
            .and_then(|bytes| Ok(Self(u32::from_be_bytes(bytes))))
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct U16(u16);

impl TryFrom<&[u8]> for U16 {
    type Error = &'static str;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_| "U16 parsing failed")
            .and_then(|bytes| Ok(Self(u16::from_be_bytes(bytes))))
    }
}

#[test]
fn parse_font() {
    let hack = Font::try_from(include_bytes!("../assets/Hack-Regular.ttf").as_slice());

    assert!(hack.is_ok());
}

#[test]
fn parse_u32() {
    let test = 0xDEADBEEF;

    assert_eq!(Ok(U32(test)), test.to_be_bytes().as_slice().try_into());
}
