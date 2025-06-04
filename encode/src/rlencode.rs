
#[derive(Debug)]
pub struct RLE {
    data: String,
    len: usize,
}

impl RLE {

    pub fn encode(data: &str) -> Vec<(char,usize)> {

        let mut result: Vec<(char,usize)> = vec![];
        let data: Vec<char> = data.chars().collect();
        let mut i = 0;

        while i < data.len() {
            let c = data[i];
            let mut count = 1;

            while i + count < data.len() && data[i + count] == c {
                count += 1;
            }

            result.push((c, count));
            i += count;
        }
        result
    }
}


