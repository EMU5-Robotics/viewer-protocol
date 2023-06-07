use nom::{
	error::{ErrorKind, ParseError},
	IResult,
};

pub mod common;
pub mod primitives;

/// Byte serialisation and deserialisation trait
pub trait Parse {
	/// Serialises the object into a byte vector.
	///
	/// # Errors
	///
	/// Serialisation will error when either the type is unable to be serialised
	/// or the type does not meet predefined standards.
	///
	/// # Returns
	///
	/// Returns an `Result` containing the serialised bytes or a `ParserError` on failure.
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>>;
	/// Deserialises bytes into an object.
	///
	/// # Errors
	///
	/// Deserialisation will error when either the data is invalid
	/// or the input is incomplete
	///
	/// # Returns
	///
	/// Returns an `IResult` containing the remaining bytes and the deserialised object or a `ParserError` on failure.
	fn deserialise(_bytes: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>>
	where
		Self: std::marker::Sized;
}

#[derive(Debug, PartialEq)]
pub enum ParserError<I> {
	DeserialiseOnBorrowedType,
	Nom(I, ErrorKind),
	TooLarge(usize),
	InvalidVariant,
	InvalidUtf8,
}

impl<I> ParseError<I> for ParserError<I> {
	fn from_error_kind(input: I, kind: ErrorKind) -> Self {
		ParserError::Nom(input, kind)
	}

	fn append(_: I, _: ErrorKind, other: Self) -> Self {
		other
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn nom_parse_error() {
		let ref_err: ParserError<&[u8]> = ParserError::Nom(&[], ErrorKind::Satisfy);
		let err: ParserError<&[u8]> = ParserError::from_error_kind(&[], ErrorKind::Satisfy);
		assert_eq!(err, ref_err);

		let err: ParserError<&[u8]> = ParserError::append(&[], ErrorKind::Fail, err);
		assert_eq!(err, ref_err);
	}
}
