use crate::traits::ConvertAsBytes;

/// **ParsableContext** means that the item is parsable from raw bytes to itself with a self context
pub trait ParsableContext {
	/// **Error** should be whatever your return error type is
	type Error;
	/// **parse** converts raw bytes to itself
	fn parse(&mut self, bytes: impl ConvertAsBytes) -> Result<Self, Self::Error> where Self: Sized;
}