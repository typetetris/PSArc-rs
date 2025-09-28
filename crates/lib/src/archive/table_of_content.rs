use crate::primitive;

use super::{PSArchiveFlags, PSArchiveTableItem};

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
    /// **block_size** is the block size
    pub block_size: u32,
    // **flags** is the flags of the Playstation Archive file
    pub flags: PSArchiveFlags,
    // **entries** is the list of toc entries
    pub entries: Vec<PSArchiveTableItem>,
    // **block_count** number of blocks in archive
    pub block_count: usize,
    // **block_sizes** are the block sizes
    pub block_sizes: Vec<u16>,
}

impl PSArchiveTOC {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<(Self, &[u8])> {
        let (length, bytes) = primitive(u32::from_be_bytes, bytes)?;
        let (entry_size, bytes) = primitive(u32::from_be_bytes, bytes)?;
        let (entry_count, bytes) = primitive(u32::from_be_bytes, bytes)?;
        let (block_size, bytes) = primitive(u32::from_be_bytes, bytes)?;
        let (flags, bytes) = PSArchiveFlags::parse(bytes)?;

        let mut entries: Vec<PSArchiveTableItem> = Vec::with_capacity(entry_count as usize);
        let bytes = (0..entry_count).try_fold(bytes, |bytes, _| -> anyhow::Result<_> {
            let (entry, bytes) = PSArchiveTableItem::parse(bytes)?;
            entries.push(entry);
            Ok(bytes)
        })?;

        let block_count = ((length - 32 - entry_count * entry_size) / 2) as usize;

        let mut block_sizes: Vec<u16> = Vec::with_capacity(block_count);
        let bytes = (0..block_count).try_fold(bytes, |bytes, _| -> anyhow::Result<_> {
            let (block_size, bytes) = primitive(u16::from_be_bytes, bytes)?;
            block_sizes.push(block_size);
            Ok(bytes)
        })?;

        Ok((
            Self {
                length,
                entry_size,
                entry_count,
                block_size,
                flags,
                entries,
                block_count,
                block_sizes,
            },
            bytes,
        ))
    }
}
