mod imos;

pub use crate::imos::Ims;

#[cfg(test)]
mod tests {
    use crate::Ims;
    #[test]
    fn test_imos() {
        let mut test_querys = vec![];
        test_querys.push((1,6,1));
        test_querys.push((4,5,1));
        test_querys.push((5,10,1));
        test_querys.push((7,10,1));
        let mut ims = Ims::new(10);
        for i in 0..test_querys.len() {
            ims.query(test_querys[i]);
        }
        assert!(ims.result() == &[1,1,1,2,3,2,2,2,2,2]);
    }
}