/// ### Archive
/// The archive module contains structs and methods related to handling the root archive file
pub mod archive;

fn primitive<const N: usize, T, F: Fn([u8; N]) -> T>(
    f: F,
    bytes: &[u8],
) -> anyhow::Result<(T, &[u8])> {
    let (snippet, rest) = bytes
        .split_at_checked(N)
        .ok_or_else(|| anyhow::anyhow!("too short"))?;
    Ok((f(snippet.try_into()?), rest))
}
