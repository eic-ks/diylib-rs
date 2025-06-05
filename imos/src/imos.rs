// mutableでインスタンス作成する必要あり
#[derive(Debug)]
pub struct Ims {
    array: Vec<isize>,
    size: usize,
}

impl Ims {

    pub fn new(size: usize) -> Self {
        Self {
            array: vec![0; size],
            size: size,
        }
    }

    pub fn query(&mut self, (start,end,val): (usize,usize,isize)) {
        if 0 < start && start < self.size+1 {
            self.array[start.saturating_sub(1)] += val;
        }
        if end < self.size {
            self.array[end] -= val;
        }
    }

    pub fn result(&mut self) -> &[isize] {
        for i in 1..self.size {
            self.array[i] += self.array[i-1];
        }
        &self.array
    }

}