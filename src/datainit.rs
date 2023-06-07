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
	Fixed,
	Variable,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Parse)]
#[repr(u8)]
pub enum DataType {
	U8,
	U16,
	U32,
	U64,
	F32,
	F64,
	Vec3,
	Vec2,
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
	data_type: DataType,
	data_length: DataLength,
	availability: Availability,
}

impl Parse for DataInit {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		let mut bytes = Vec::new();

		bytes.extend(self.robot_ident.serialise()?);
		bytes.extend(self.data_ident.serialise()?);
		bytes.push(self.data_tier as u8);
		bytes.push(self.data_type as u8);
		bytes.push(self.data_length as u8);
		bytes.extend(self.availability.serialise()?);

		Ok(bytes)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, robot_ident) = String::deserialise(input)?;

		let (input, data_ident) = String::deserialise(input)?;

		let (input, data_tier) = DataTier::deserialise(input)?;

		let (input, data_type) = DataType::deserialise(input)?;

		let (input, data_length) = DataLength::deserialise(input)?;

		let (input, availability) = Availability::deserialise(input)?;

		Ok((
			input,
			DataInit {
				robot_ident,
				data_ident,
				data_tier,
				data_type,
				data_length,
				availability,
			},
		))
	}
}
