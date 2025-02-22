
#[derive(Debug)]
pub struct Value<T> {
    pub data: Option<T>,
}

impl<T: ToString> Value<T> {
    // Single generic constructor.
    pub fn new(val: T) -> Self {
        Self { data: Some(val) }
    }

    // Retrieve the contained value as a string if it exists.
    // T must implement ToString.
    pub fn data(&self) -> Option<String> {
        match &self.data {
            Some(value) => {
                {
                    let type_name = std::any::type_name::<T>();
                    // println!("Type: {}", type_name);
                    if type_name == "bool" {
                        // For boolean: 1-bit representation.
                        let b: bool = value.to_string().parse().unwrap();
                        Some(format!("{:01b}", if b { 1 } else { 0 }))
                    } else if type_name == "char" {
                        // For char: 8-bit representation.
                        let c: char = value.to_string().chars().next().unwrap();
                        Some(format!("{:08b}", c as u8))
                    } else if type_name.contains("i") || type_name.contains("u") {
                        // For all numeric types: represent in 8 bits.
                        let num: i64 = value.to_string().parse().unwrap();
                        let num8 = num as u8;
                        Some(format!("{:08b}", num8))
                    } else {
                        // Fallback: treat as string and convert each byte into its 8-bit binary representation.
                        let s = value.to_string();
                        let bytes = s.into_bytes();
                        let binary_string = bytes
                            .iter()
                            .map(|b| format!("{:08b}", b))
                            .collect::<Vec<String>>()
                            .join(" ");
                        Some(binary_string)
                    }
                }
            }
            None => None,
        }
    }
}