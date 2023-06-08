use crate::{
	data::Data,
	datainit::DataInit,
	parse::{Parse, ParserError},
};

use derive_new::new;
use nom::IResult;
use proc::Parse;
use std::time::Duration;

pub mod data;
pub mod datainit;
pub mod parse;

/// Top level Packet structure
///
/// Contains a timestamp as a `Duration`
/// which should be the elapsed duration since [UNIX_EPOCH][std::time::SystemTime::UNIX_EPOCH]
/// at the time of sending the packet.
///
/// Contains the actual data of the packet as a [PacketData].
#[derive(Debug, new, PartialEq)]
pub struct Packet {
	timestamp: Duration,
	data: PacketData,
}

impl Parse for Packet {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		let mut bytes = Vec::new();

		bytes.extend(self.timestamp.serialise()?);
		bytes.extend(self.data.serialise()?);

		Ok(bytes)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, timestamp) = Duration::deserialise(input)?;
		let (input, data) = PacketData::deserialise(input)?;

		Ok((input, Packet { timestamp, data }))
	}
}

/// The data contained within a [Packet]
///
/// `PacketData` should not be serialised and deserialised direction
/// instead using [Packet] is recommended
#[derive(Debug, Parse, PartialEq)]
pub enum PacketData {
	/// Indicates the existance of a new robot
	///
	/// `RobotInit` contains the unique identifier associated with a robot
	///
	/// This should only be sent once per Robot
	/// unless a `RobotRemove` has been sent on that identifier beforehand
	RobotInit(String),
	/// Indicates the existance of a data source
	///
	/// `DataInit` contains metadata associated with the data source
	/// including an identifier for data source and the associated robot
	///
	/// This should only be sent once per data source
	/// unless a `DataRemove` has been sent on that identifier beforehand
	DataInit(DataInit),
	/// Indicates the removal of a robot
	///
	/// This should only be sent after a `RobotInit` with a matching identifier
	RobotRemove(String),
	/// Indicates the removal of a data source
	///
	/// This should only be sent after a `DataInit` with a matching identifiers for both
	/// the data source and the associated robot
	DataRemove(DataRemove),
	/// Contains the data sent from a data source defined by `DataInit`
	///
	/// This should only be sent after a `DataInit` with a matching identifiers for both
	/// the data source and the associated robot
	Data(Data),
}

#[derive(Debug, PartialEq, new)]
pub struct DataRemove {
	robot_ident: String,
	data_ident: String,
}

impl Parse for DataRemove {
	fn serialise<'a>(&self) -> Result<Vec<u8>, ParserError<&'a [u8]>> {
		let mut bytes = Vec::new();

		bytes.extend(self.robot_ident.serialise()?);
		bytes.extend(self.data_ident.serialise()?);

		Ok(bytes)
	}
	fn deserialise(input: &[u8]) -> IResult<&[u8], Self, ParserError<&[u8]>> {
		let (input, robot_ident) = String::deserialise(input)?;
		let (input, data_ident) = String::deserialise(input)?;

		Ok((
			input,
			Self {
				robot_ident,
				data_ident,
			},
		))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::datainit::*;
	use std::time::SystemTime;

	#[test]
	fn round_trip() {
		let timestamp = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap();

		let packets = vec![
			Packet::new(timestamp, PacketData::RobotInit("Robot 1".to_owned())),
			Packet::new(
				timestamp,
				PacketData::DataInit(DataInit::new(
					"Robot 1".to_owned(),
					"Motor 1".to_owned(),
					DataTier::Raw,
					vec![
						(DataType::F32, DataLength::Single),
						(DataType::Vec3, DataLength::Variable),
						(DataType::I32, DataLength::Fixed(30)),
					],
					Availability::Always,
				)),
			),
			Packet::new(
				timestamp,
				PacketData::Data(Data::new(0, 0u16.to_be_bytes().to_vec())),
			),
			Packet::new(
				timestamp,
				PacketData::Data(Data::new(1, 4u16.to_be_bytes().to_vec())),
			),
			Packet::new(
				timestamp,
				PacketData::DataRemove(DataRemove::new("Robot 1".to_owned(), "IMU 1".to_owned())),
			),
			Packet::new(timestamp, PacketData::RobotRemove("Robot 1".to_owned())),
		];

		let bytes: Vec<u8> = packets
			.iter()
			.flat_map(|v| v.serialise().unwrap())
			.collect();

		let mut input = &bytes[..];

		let mut resulting_packets = Vec::new();

		while let Ok((new_input, packet)) = Packet::deserialise(input) {
			resulting_packets.push(packet);
			input = new_input;
		}

		assert_eq!(packets, resulting_packets);
	}
}
