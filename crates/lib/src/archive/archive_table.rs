use super::PSArchiveCompression;
use crate::prelude::*;

/// **PSArchiveTableItem** is a table of contents table for the Playstation Archive file
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchiveTableItem {
	/// **compression_type** is the compression type of the item
    pub compression_type: PSArchiveCompression,
	/// **block_offset** is the offset in blocks of the item
    pub block_offset: u32,
	/// **uncompressed_size** is the uncompressed size of the item
    pub uncompressed_size: u64,
	/// **file_offset** is the file offset of the item
    pub file_offset: u64,
}

impl PSArchiveTableItem {
    pub fn new(compression_type: PSArchiveCompression) -> Self {
        return Self {
            compression_type,
            block_offset: 0,
            uncompressed_size: 0,
            file_offset: 0,
        };
    }
}

impl ParsableContext for PSArchiveTableItem {
    type Error = anyhow::Error;
    fn parse(&mut self, bytes: impl ConvertAsBytes) -> Result<Self, Self::Error> {
        let bytes = bytes.convert_as_bytes();
        let block_offset = ((bytes[0x10] as u32) << 24)
            + ((bytes[0x11] as u32) << 16)
            + ((bytes[0x12] as u32) << 8)
            + (bytes[0x13] as u32);
        let uncompressed_size = ((bytes[0x14] as u64) << 32)
            + ((bytes[0x15] as u64) << 24)
            + ((bytes[0x16] as u64) << 16)
            + ((bytes[0x17] as u64) << 8)
            + (bytes[0x18] as u64);
        let file_offset = ((bytes[0x19] as u64) << 32)
            + ((bytes[0x1A] as u64) << 24)
            + ((bytes[0x1B] as u64) << 16)
            + ((bytes[0x1C] as u64) << 8)
            + (bytes[0x1D] as u64);
        return Ok(Self {
            compression_type: self.compression_type.clone(),
            block_offset,
            uncompressed_size,
            file_offset,
        });
    }
}

#[cfg(test)]
#[doc(hidden)]
mod test {
    use super::{PSArchiveCompression, PSArchiveTableItem};
    use crate::prelude::*;

    #[test]
    fn test_archive_table_item_parsing_manifest() {
        let bytes = include_bytes!("../../res/test.pak")[0x20..0x3E].to_vec();
        let mut table_item = PSArchiveTableItem::new(PSArchiveCompression::ZLIB);
        let result = table_item.parse(bytes);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(
            result,
            PSArchiveTableItem {
                compression_type: PSArchiveCompression::ZLIB,
                block_offset: 0,
                uncompressed_size: 17,
                file_offset: 96
            }
        );
    }

    #[test]
    fn test_archive_table_item_parsing_item() {
        let bytes = include_bytes!("../../res/test.pak")[0x3E..0x5C].to_vec();
        let mut table_item = PSArchiveTableItem::new(PSArchiveCompression::ZLIB);
        let result = table_item.parse(bytes);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(
            result,
            PSArchiveTableItem {
                compression_type: PSArchiveCompression::ZLIB,
                block_offset: 1,
                uncompressed_size: 115,
                file_offset: 113,
            }
        );
    }
}
