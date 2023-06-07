use super::{Parse, ParserError};

use nom::{bytes::streaming::take, IResult};
use std::time::Duration;

#[allow(clippy::cast_possible_truncation)]
impl Parse for String {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		if self.len() > u16::MAX as usize {
			return Err(ParserError::TooLarge(self.len()));
		}
		let mut data = Vec::new();
		data.extend((self.len() as u16).serialise()?);
		data.extend(self.as_bytes());
		Ok(data)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, len) = u16::deserialise(input)?;

		let (input, data) = take(len as usize)(input)?;
		Ok((input, String::from_utf8(data.to_owned()).unwrap()))
	}
}

#[allow(clippy::cast_possible_truncation)]
impl Parse for &str {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		if self.len() > u16::MAX as usize {
			return Err(ParserError::TooLarge(self.len()));
		}
		let mut data = Vec::new();
		data.extend((self.len() as u16).serialise()?);
		data.extend(self.as_bytes());
		Ok(data)
	}
	fn deserialise(_: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		Err(nom::Err::Failure(ParserError::DeserialiseOnBorrowedType))
	}
}

#[allow(clippy::cast_possible_truncation)]
impl<T: Parse> Parse for Vec<T> {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		if self.len() > u16::MAX as usize {
			return Err(ParserError::TooLarge(self.len()));
		}
		let mut data = Vec::new();
		data.extend((self.len() as u16).serialise()?);
		for item in self.iter() {
			data.extend(item.serialise()?);
		}
		Ok(data)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (mut input, len) = u16::deserialise(input)?;

		let mut data = Vec::new();
		for _ in 0..len {
			let (new_input, new_data) = T::deserialise(input)?;
			input = new_input;
			data.push(new_data);
		}

		Ok((input, data))
	}
}

#[allow(clippy::cast_possible_truncation)]
impl<T: Parse> Parse for &[T] {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		if self.len() > u16::MAX as usize {
			return Err(ParserError::TooLarge(self.len()));
		}
		let mut data = Vec::new();
		data.extend((self.len() as u16).serialise()?);
		for item in self.iter() {
			data.extend(item.serialise()?);
		}
		Ok(data)
	}
	fn deserialise(_: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		Err(nom::Err::Failure(ParserError::DeserialiseOnBorrowedType))
	}
}

impl Parse for Duration {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		let mut data = Vec::new();
		data.extend(self.as_secs().serialise()?);
		data.extend(self.subsec_nanos().serialise()?);
		Ok(data)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, secs) = u64::deserialise(input)?;
		let (input, nanos) = u32::deserialise(input)?;

		Ok((input, Duration::new(secs, nanos)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn large_data() {
		let test_string = String::from_utf8(vec![b'a'; u16::MAX as usize + 1]).unwrap();
		let test_vec = vec![0i32; u16::MAX as usize + 1];
		let serialised = [
			test_string.serialise(),
			(&test_string[..]).serialise(),
			test_vec.serialise(),
			(&test_vec[..]).serialise(),
		];

		assert_eq!(serialised.into_iter().filter(|v| v.is_err()).count(), 4)
	}

	#[test]
	fn serialise_data() {
		let test_string = String::from_utf8(vec![b'a'; u16::MAX as usize]).unwrap();
		let test_vec = vec![0i32; u16::MAX as usize];
		let serialised = [
			test_string.serialise(),
			(&test_string[..]).serialise(),
			test_vec.serialise(),
			(&test_vec[..]).serialise(),
		];

		assert_eq!(serialised.into_iter().filter(|v| v.is_ok()).count(), 4)
	}
	#[test]
	fn fail_deserialise() {
		let test_string = String::from_utf8(vec![b'a'; u16::MAX as usize]).unwrap();
		let test_vec = vec![0i32; u16::MAX as usize];
		let serialised = [
			test_string.serialise().unwrap(),
			test_vec.serialise().unwrap(),
		];

		let deserialised = (
			<&str>::deserialise(&serialised[0]),
			<String>::deserialise(&serialised[0]),
			<&[i32]>::deserialise(&serialised[1]),
			<Vec<i32>>::deserialise(&serialised[1]),
		);
		assert!(
			deserialised.0.is_err()
				&& deserialised.1.unwrap() == (&[], test_string)
				&& deserialised.2.is_err()
				&& deserialised.3.unwrap() == (&[], test_vec)
		);
	}
}
