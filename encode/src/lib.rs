mod rlencode;

pub use crate::rlencode::RLE;

#[cfg(test)]
mod tests {
    use crate::RLE;
    #[test]
    fn test_encode() {
        let test_data = String::from("hhhaakkkk");
        println!("{:?}",RLE::encode(&test_data));
        assert!(RLE::encode(&test_data) == vec![('h',3),('a',2),('k',4)]);
    }
}