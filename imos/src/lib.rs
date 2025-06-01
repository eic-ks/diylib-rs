mod imos;

pub use crate::imos::Imos;

#[cfg(test)]
mod tests {
    #[test]
    fn test_imos() {
        let test_query = (2,5,3);
        let mut ims = crate::Imos::new(10);
        ims.query(test_query);
        ims.result();
        println!("{:?}",ims.array());
    }
}