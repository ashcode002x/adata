#[derive(Debug)]
pub struct Value<T> {
    pub data: Option<T>,
}

pub trait Binary8 {
    fn to_binary_8(&self) -> Vec<u8>;  // Now returns Vec<u8> for binary storage
}

// Implement Binary8 for integers (i32, u32, etc.)
impl Binary8 for i32 {
    fn to_binary_8(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec() // Convert integer to big-endian bytes
    }
}

impl Binary8 for u32 {
    fn to_binary_8(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }
}

// Implement Binary8 for floats (convert using IEEE 754 representation)
impl Binary8 for f32 {
    fn to_binary_8(&self) -> Vec<u8> {
        self.to_bits().to_be_bytes().to_vec()
    }
}

impl Binary8 for f64 {
    fn to_binary_8(&self) -> Vec<u8> {
        self.to_bits().to_be_bytes().to_vec()
    }
}

// Implement Binary8 for &str (convert to ASCII binary bytes)
impl Binary8 for &str {
    fn to_binary_8(&self) -> Vec<u8> {
        let mut arr =self.as_bytes().to_vec();
        arr.resize(16, 0);
        arr
    }
}

// Implement Binary8 for String
impl Binary8 for String {
    fn to_binary_8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<T: Binary8> Value<T> {
    pub fn new(val: T) -> Self {
        Self { data: Some(val) }
    }
    
    pub fn data(&self) -> Option<Vec<u8>> {
        self.data.as_ref().map(|v| v.to_binary_8())
    }
}