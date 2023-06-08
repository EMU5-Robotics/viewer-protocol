use crate::{parse::Parse, ParserError};

use derive_new::new;
use nom::IResult;
use proc::Parse;
use std::time::Duration;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Parse)]
#[repr(u8)]
pub enum DataTier {
	Raw,
	Processed,
	Calculated,
	State,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Parse)]
#[repr(u8)]
pub enum DataLength {
	Single,
	Fixed(u16),
	Variable,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Parse)]
#[repr(u8)]
pub enum DataType {
	Boolean,
	F32,
	F64,
	I128,
	I16,
	I32,
	I64,
	I8,
	U128,
	U16,
	U32,
	U64,
	U8,
	Vec2,
	Vec3,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Parse)]
pub enum Availability {
	Once,
	Whenever,
	Within(Duration),
	Always,
}

#[derive(Debug, PartialEq, new)]
pub struct DataInit {
	robot_ident: String,
	data_ident: String,
	data_tier: DataTier,
	data_info: Vec<(DataType, DataLength)>,
	availability: Availability,
}

impl Parse for (DataType, DataLength) {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		let mut bytes = Vec::new();
		bytes.extend(self.0.serialise()?);
		bytes.extend(self.1.serialise()?);
		Ok(bytes)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, data_type) = DataType::deserialise(input)?;
		let (input, data_length) = DataLength::deserialise(input)?;
		Ok((input, (data_type, data_length)))
	}
}

impl Parse for DataInit {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		let mut bytes = Vec::new();

		bytes.extend(self.robot_ident.serialise()?);
		bytes.extend(self.data_ident.serialise()?);
		bytes.extend(self.data_tier.serialise()?);
		bytes.extend(self.data_info.serialise()?);
		bytes.extend(self.availability.serialise()?);

		Ok(bytes)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, robot_ident) = String::deserialise(input)?;

		let (input, data_ident) = String::deserialise(input)?;

		let (input, data_tier) = DataTier::deserialise(input)?;

		let (input, data_info) = <Vec<(DataType, DataLength)>>::deserialise(input)?;

		let (input, availability) = Availability::deserialise(input)?;

		Ok((
			input,
			DataInit {
				robot_ident,
				data_ident,
				data_tier,
				data_info,
				availability,
			},
		))
	}
}
