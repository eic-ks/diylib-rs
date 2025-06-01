
#derive[(Debug)]
pub struct RLE {
    data: String,
    len: usize,
}

impl RLE {

    pub fn encode(data: &str) -> Vec<(char,usize)> {

        let mut result: Vec<(char,usize)> = vec![];

        for i in 0..data.len() {

            let c = data[i];
            let mut count = 0;

            for j in i+1..data.len() {
                if c == data[j] {
                    count += 1;
                }else {
                    i += count;
                    result.push((c,count));
                    break;
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_encode() {
        let test_data = String::from("hhhaaakkk");
        println!("{}",RLE::encode(&test_data));
    }
}

