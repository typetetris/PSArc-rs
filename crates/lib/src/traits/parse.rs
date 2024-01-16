use crate::traits::ConvertAsBytes;

/// **Parsable** means that the item is parsable from raw bytes to itself
pub trait Parsable {
	/// **Error** should be whatever your return error type is
	type Error;
	/// **parse** converts raw bytes to itself
	fn parse(bytes: impl ConvertAsBytes) -> Result<Self, Self::Error> where Self: Sized;
}