/// **PSArchiveTableItem** is a table of contents table for the Playstation Archive file
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchiveTableItem {
    /// **md5_digest** is the the 128-bit md5 hash of the file
    pub md5_digest: [u8; 16],
    /// **block_offset** is the offset in blocks of the item
    pub block_offset: u32,
    /// **uncompressed_size** is the uncompressed size of the item
    pub uncompressed_size: u64,
    /// **file_offset** is the file offset of the item
    pub file_offset: u64,
}

impl PSArchiveTableItem {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<Self> {
        let md5_digest: [u8; 16] = bytes[0..0x10].try_into()?;
        let block_offset = u32::from_be_bytes(bytes[0x10..0x14].try_into()?);
        let uncompressed_size = {
            let mut raw = [0u8; 8];
            raw[3..8].copy_from_slice(&bytes[0x14..0x19]);
            u64::from_be_bytes(raw)
        };
        let file_offset = {
            let mut raw = [0u8; 8];
            raw[3..8].copy_from_slice(&bytes[0x19..0x1E]);
            u64::from_be_bytes(raw)
        };
        Ok(Self {
            md5_digest,
            block_offset,
            uncompressed_size,
            file_offset,
        })
    }
}

#[cfg(test)]
#[doc(hidden)]
mod test {
    use super::PSArchiveTableItem;

    #[test]
    fn test_archive_table_item_parsing_manifest() {
        let bytes = include_bytes!("../../res/test.pak")[0x20..0x3E].to_vec();
        let result = PSArchiveTableItem::parse(&bytes[..]);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            PSArchiveTableItem {
                md5_digest: [0u8; 16],
                block_offset: 0,
                uncompressed_size: 17,
                file_offset: 96
            }
        );
    }

    #[test]
    fn test_archive_table_item_parsing_item() {
        let bytes = include_bytes!("../../res/test.pak")[0x3E..0x5C].to_vec();
        let result = PSArchiveTableItem::parse(&bytes[..]);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            PSArchiveTableItem {
                md5_digest: [0u8; 16],
                block_offset: 1,
                uncompressed_size: 115,
                file_offset: 113,
            }
        );
    }
}
