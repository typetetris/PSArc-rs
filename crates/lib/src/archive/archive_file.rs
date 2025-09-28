use std::io::Read;

use super::{PSArchiveCompression, PSArchiveTOC, PSArchiveVersion};
use anyhow::anyhow;

const PSARC_HEADER: &[u8; 4] = b"PSAR";

/// **PSArchive** contains all the information about a complete singular Playstation Archive file
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PSArchive<'a> {
    /// **version** is the Playstation Archive file version
    pub version: PSArchiveVersion,
    /// **compression** is the Playstation Archive file compression
    pub compression: PSArchiveCompression,
    /// **table_of_contents** is the table of contents for the Playstation Archive file
    pub table_of_contents: PSArchiveTOC,
    pub files: Vec<String>,
    pub file_contents: &'a [u8],
}

impl<'a> PSArchive<'a> {
    pub fn parse_manifest(&self) -> anyhow::Result<Vec<String>> {
        let strings = String::from_utf8(self.parse_file(0)?)?
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        Ok(strings)
    }

    pub fn parse_file(&self, item_index: usize) -> anyhow::Result<Vec<u8>> {
        let item = &self.table_of_contents.entries[item_index];
        let bytes = &self.file_contents[item.file_offset as usize..];
        let uncompressed_size = item.uncompressed_size as usize;
        Ok(if uncompressed_size > 100 {
            let block_offset = item.block_offset;
            // That's an assumption, but hopefully the items are listed in ascending block_offsets
            // Otherwise we would have to search for the file with the smallest block_offset larger
            // than this files block_offest, phew ...
            let block_end_offset = if item_index + 1 >= self.table_of_contents.entries.len() {
                self.table_of_contents.block_count as u32
            } else {
                self.table_of_contents.entries[item_index + 1].block_offset
            };
            let mut s = Vec::with_capacity(item.uncompressed_size as usize);

            (block_offset..block_end_offset).try_fold(
                bytes,
                |bytes, block_index| -> anyhow::Result<_> {
                    let block_size = self.table_of_contents.block_sizes[block_index as usize];
                    let (block, bytes) = bytes
                        .split_at_checked(block_size as usize)
                        .ok_or_else(|| anyhow!("too short while decompressing"))?;
                    let mut z = flate2::read::ZlibDecoder::new(block);
                    z.read_to_end(&mut s)?;
                    Ok(bytes)
                },
            )?;

            s
        } else {
            bytes[..uncompressed_size].into()
        })
    }

    pub fn parse(bytes: &'a [u8]) -> anyhow::Result<Self> {
        let file_contents = bytes;
        let (header, bytes) = bytes
            .split_at_checked(4)
            .ok_or_else(|| anyhow!("too short"))?;

        if header == PSARC_HEADER {
            let (version, bytes) = PSArchiveVersion::parse(bytes)?;
            let (compression, bytes) = PSArchiveCompression::parse(bytes)?;

            // Read the toc header
            #[allow(unused)]
            let (table_of_contents, bytes) = PSArchiveTOC::parse(bytes)?;

            Ok(Self {
                version,
                compression,
                table_of_contents,
                files: Vec::new(),
                file_contents,
            })
        } else {
            Err(anyhow!("Invalid header for PSArc format"))
        }
    }
}
