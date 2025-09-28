/// **ConvertAsBytes** convert different types of data into a `Vec<u8>`, made just for convinence <br>
/// It is auto implemented (if imported) for the following types: <br>
/// `Vec<u8>`, `&[u8]`, `&str`, `String`, `[u8; N]` (1 thru 16), and `u8`
pub trait ConvertAsBytes {
    /// **convert_as_bytes** converts a previous type to a `Vec<u8>`
    fn convert_as_bytes(&self) -> Vec<u8>;
}

impl ConvertAsBytes for Vec<u8> {
    fn convert_as_bytes(&self) -> Vec<u8> {
        self.to_owned()
    }
}

impl ConvertAsBytes for &[u8] {
    fn convert_as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ConvertAsBytes for &str {
    fn convert_as_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ConvertAsBytes for String {
    fn convert_as_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ConvertAsBytes for u8 {
    fn convert_as_bytes(&self) -> Vec<u8> {
        vec![self.to_owned()]
    }
}

seq_macro::seq!(N in 1..=16 {
    impl ConvertAsBytes for [u8; N] {
		fn convert_as_bytes(&self) -> Vec<u8> {
			self.to_vec()
		}
	}
});
