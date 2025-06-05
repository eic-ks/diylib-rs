mod bound;

pub use crate::bound::{upperbound,lowerbound};

#[cfg(test)]
mod tests {
    use crate::{upperbound,lowerbound};
    #[test]
    fn test_upper() {
        let test_data = vec![1,2,3,4,5,6,7,8,9,10];
        let result = upperbound(&test_data,0,9,4);
        assert!(result == 4);
    }
    
    #[test]
    fn test_lower() {
        let test_data = vec![1,2,3,4,5,6,7,8,9,10];
        let result = lowerbound(&test_data,0,9,8);
        assert!(result == 8);
    }
}