mod imos;

pub use crate::imos::Imos;

#[cfg(test)]
mod tests {
    use crate::Imos;
    #[test]
    fn test_imos() {
        let mut test_querys = vec![];
        test_querys.push((1,6,1));
        test_querys.push((4,5,1));
        test_querys.push((5,10,1));
        test_querys.push((7,10,1));
        let mut ims = Imos::new(10);
        for i in 0..test_querys.len() {
            ims.query(test_querys[i]);
        }
        println!("{:?}",ims.result());
    }
}