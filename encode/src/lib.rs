mod rlencode;

pub use crate::rlencode::RLE;

#[cfg(test)]
mod tests {
    use crate::RLE;
    #[test]
    fn test_encode() {
        let test_data = String::from("hhhaakkkk");
        let encode_result: Vec<(char,usize)> = RLE::encode(&test_data);
        println!("{:?}",encode_result);
        assert!(encode_result == vec![('h',3),('a',2),('k',4)]);
    }
    
    #[test]
    fn test_decode() {
        let test_data = vec![('h',3),('a',2),('k',4)];
        let decode_result = RLE::decode(&test_data);
        assert!(decode_result == String::from("hhhaakkkk"));
    }
}