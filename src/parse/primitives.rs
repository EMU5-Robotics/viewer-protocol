use super::{Parse, ParserError};

use nom::{bytes::streaming::take, IResult};

macro_rules! integer_parse {
	($type:ty) => {
		impl Parse for $type {
			fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
				Ok(self.to_be_bytes().to_vec())
			}
			fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
				let (input, val) = take(<$type>::BITS as usize / 8)(input)?;
				let val: [u8; <$type>::BITS as usize / 8] = val.try_into().unwrap();
				Ok((input, <$type>::from_be_bytes(val)))
			}
		}
	};
}

integer_parse!(u8);
integer_parse!(u16);
integer_parse!(u32);
integer_parse!(u64);
integer_parse!(u128);
integer_parse!(i8);
integer_parse!(i16);
integer_parse!(i32);
integer_parse!(i64);
integer_parse!(i128);
