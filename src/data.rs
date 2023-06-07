use crate::{Parse, ParserError};

use derive_new::new;
use nom::IResult;

#[derive(Debug, new, PartialEq)]
pub struct Data {
	iteration: u32,
	data: Vec<u8>,
}

impl Parse for Data {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		let mut bytes = Vec::new();
		bytes.extend(self.iteration.serialise()?);
		bytes.extend(self.data.serialise()?);
		Ok(bytes)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, iteration) = u32::deserialise(input)?;
		let (input, data) = <Vec<u8>>::deserialise(input)?;
		Ok((input, Self { iteration, data }))
	}
}
