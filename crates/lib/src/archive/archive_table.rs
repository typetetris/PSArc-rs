use anyhow::anyhow;

use crate::primitive;

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
    pub fn parse(bytes: &[u8]) -> anyhow::Result<(Self, &[u8])> {
        let (md5_digest, bytes) = bytes
            .split_at_checked(16)
            .ok_or_else(|| anyhow!("too short"))?;

        let (block_offset, bytes) = primitive(u32::from_be_bytes, bytes)?;

        let (uncompressed_size, bytes) = {
            let mut raw = [0u8; 8];
            let (snippet, bytes) = bytes
                .split_at_checked(5)
                .ok_or_else(|| anyhow!("too short"))?;
            raw[3..8].copy_from_slice(snippet);
            (u64::from_be_bytes(raw), bytes)
        };

        let (file_offset, bytes) = {
            let mut raw = [0u8; 8];
            let (snippet, bytes) = bytes
                .split_at_checked(5)
                .ok_or_else(|| anyhow!("too short"))?;
            raw[3..8].copy_from_slice(snippet);
            (u64::from_be_bytes(raw), bytes)
        };

        Ok((
            Self {
                md5_digest: md5_digest.try_into()?,
                block_offset,
                uncompressed_size,
                file_offset,
            },
            bytes,
        ))
    }
}
