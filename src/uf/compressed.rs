use super::UF;

pub struct CompressedQuickUnionUF(Vec<u32>);

impl CompressedQuickUnionUF {
    pub fn new(size: usize) -> Self {
        Self(vec![0; size])
    }

    fn component_id(&self, mut item: u32) -> u32 {
        while self.0[item as usize] != 0 {
            item = self.0[item as usize];
        }

        item
    }

    fn component_id_compacting(&mut self, mut item: u32) -> u32 {
        if self.0[item as usize] == 0 {
            return item;
        }

        let mut root = item;
        while self.0[root as usize] != 0 {
            root = self.0[root as usize];
        }

        while self.0[item as usize] != root {
            item = std::mem::replace(&mut self.0[item as usize], root);
        }

        root
    }
}

impl UF for CompressedQuickUnionUF {
    fn connect(&mut self, a: u32, b: u32) -> bool {
        let a = self.component_id_compacting(a);
        let b = self.component_id_compacting(b);
        if a != b {
            self.0[a as usize] = b;

            true
        } else {
            false
        }
    }

    fn connected(&self, a: u32, b: u32) -> bool {
        self.component_id(a) == self.component_id(b)
    }
}
