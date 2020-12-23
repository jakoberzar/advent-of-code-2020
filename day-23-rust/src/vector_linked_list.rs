// This structure is basically a list in a vector;
// The value on given position gives us the value
// that follows this value in the list.
// TODO: Make this also work for generic data?
#[derive(Debug, Clone)]
pub struct VectorLinkedList {
    data: Vec<usize>,
}

impl VectorLinkedList {
    pub fn new(data: &[u32]) -> VectorLinkedList {
        let mut vll_data: Vec<usize> = vec![0; data.len() + 1];
        for idx in 0..data.len() {
            let next_idx = (idx + 1) % data.len();
            let value = data[idx] as usize;
            let next_value = data[next_idx] as usize;
            vll_data[value] = next_value;
        }
        VectorLinkedList { data: vll_data }
    }

    pub fn follows(&self, value: u32) -> u32 {
        self.data[value as usize] as u32
    }

    pub fn rewire(&mut self, value: u32, new_follower: u32) {
        self.data[value as usize] = new_follower as usize;
    }

    pub fn len(&self) -> usize {
        self.data.len() - 1
    }
}
