use super::UF;

pub struct CompressedWeightedQuickUnionUF {
    parents: Vec<u32>,
    sizes: Vec<u32>,
}

impl CompressedWeightedQuickUnionUF {
    pub fn new(size: usize) -> Self {
        Self {
            parents: vec![0; size],
            sizes: vec![0; size],
        }
    }

    fn component_id(&self, mut item: u32) -> u32 {
        while self.parents[item as usize] != 0 {
            item = self.parents[item as usize];
        }

        item
    }

    fn component_id_compacting(&mut self, mut item: u32) -> u32 {
        if self.parents[item as usize] == 0 {
            return item;
        }

        let mut root = item;
        while self.parents[root as usize] != 0 {
            root = self.parents[root as usize];
        }

        while self.parents[item as usize] != root {
            item = std::mem::replace(&mut self.parents[item as usize], root);
        }

        root
    }
}

impl UF for CompressedWeightedQuickUnionUF {
    fn connect(&mut self, a: u32, b: u32) -> bool {
        let a = self.component_id_compacting(a);
        let b = self.component_id_compacting(b);
        if a != b {
            if self.sizes[a as usize] < self.sizes[b as usize] {
                self.parents[a as usize] = b;
                self.sizes[b as usize] += self.sizes[a as usize] + 1;
            } else {
                self.parents[b as usize] = a;
                self.sizes[a as usize] += self.sizes[b as usize] + 1;
            }
            true
        } else {
            false
        }
    }

    fn connected(&self, a: u32, b: u32) -> bool {
        self.component_id(a) == self.component_id(b)
    }
}
