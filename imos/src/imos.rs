// mutableでインスタンス作成する必要あり
#[derive(Debug)]
pub struct Imos {
    array: Vec<isize>,
    size: usize,
}

impl Imos {

    pub fn new(size: usize) -> Self {
        Self {
            array: vec![0; size],
            size: size,
        }
    }

    pub fn array(&self) -> &[isize] {
        &self.array
    }

    pub fn query(&mut self, (start,end,val): (usize,usize,isize)) {
        if 1 < start && start < self.size {
            self.array[start.saturating_sub(1)] += val;
        }
        if end+1 < self.size {
            self.array[end] -= val;
        }
    }

    pub fn result(&mut self) {
        for i in 1..self.size {
            self.array[i] += self.array[i-1];
        }
    }

}