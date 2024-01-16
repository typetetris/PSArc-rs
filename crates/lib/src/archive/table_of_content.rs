use super::PSArchiveFlags;
use crate::prelude::*;

/// **PSArchiveTOC** contains the table of content and details about each resource
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchiveTOC {
	/// **length** is the total length of the table of contents
	pub length: u32,
	/// **entry_size** contains each entries size
	pub entry_size: u32,
	/// **entry_count** is the amount of entries in the Playstation Archive file
	pub entry_count: u32,
	// **flags** is the flags of the Playstation Archive file
	pub flags: PSArchiveFlags
}

impl Parsable for PSArchiveTOC {
    type Error = anyhow::Error;
    fn parse(bytes: impl ConvertAsBytes) -> Result<Self, Self::Error> {
		let bytes = bytes.convert_as_bytes();
		let length = (((bytes[0] as u32) << 24) + ((bytes[1] as u32) << 16) + ((bytes[2] as u32) << 8) + (bytes[3] as u32)) - 32;
		let entry_size = ((bytes[4] as u32) << 24) + ((bytes[5] as u32) << 16) + ((bytes[6] as u32) << 8) + (bytes[7] as u32);
		let entry_count = (((bytes[8] as u32) << 24) + ((bytes[9] as u32) << 16) + ((bytes[10] as u32) << 8) + (bytes[11] as u32)) - 1;
		let flags = PSArchiveFlags::parse(&[bytes[16], bytes[17], bytes[18], bytes[19]] as &[u8])?;
		return Ok(Self {
			length,
			entry_size,
			entry_count,
			flags
		});
	}
}

#[cfg(test)]
#[doc(hidden)]
mod test {
	use crate::prelude::*;
	use super::{PSArchiveTOC, PSArchiveFlags};

	#[test]
	fn test_toc_parsing() {
		let bytes = include_bytes!("../../res/test.pak")[0xC..].to_vec();
		let result = PSArchiveTOC::parse(bytes);
		assert_eq!(result.is_ok(), true);
		let result = result.unwrap();
		assert_eq!(result.length, 64);
		assert_eq!(result.entry_size, 30);
		assert_eq!(result.entry_count, 1);
		assert_eq!(result.flags, PSArchiveFlags::ABSOLUTE);
	}
}