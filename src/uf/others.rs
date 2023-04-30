use super::UF;

pub struct QuickFindUF {
    comps_num: u32,
    comp_by_vert: Vec<u32>,
}

impl QuickFindUF {
    pub fn new(size: usize) -> Self {
        Self {
            comps_num: 0,
            comp_by_vert: vec![0; size],
        }
    }
}

impl UF for QuickFindUF {
    fn connect(&mut self, a: u32, b: u32) -> bool {
        match (self.comp_by_vert[a as usize], self.comp_by_vert[b as usize]) {
            (0, 0) => {
                self.comp_by_vert[a as usize] = self.comps_num + 1;
                self.comp_by_vert[b as usize] = self.comps_num + 1;
                self.comps_num += 1;
                true
            }
            (comp, 0) => {
                self.comp_by_vert[b as usize] = comp;
                true
            }
            (0, comp) => {
                self.comp_by_vert[a as usize] = comp;
                true
            }
            (comp1, comp2) => {
                if comp1 != comp2 {
                    for comp in &mut self.comp_by_vert {
                        if *comp == comp1 {
                            *comp = comp2;
                        }
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn connected(&self, a: u32, b: u32) -> bool {
        self.comp_by_vert[a as usize] != 0
            && self.comp_by_vert[a as usize] == self.comp_by_vert[b as usize]
    }
}

pub struct QuickUnionUF(Vec<u32>);

impl QuickUnionUF {
    pub fn new(size: usize) -> Self {
        Self(vec![0; size])
    }

    fn component_id(&self, mut item: u32) -> u32 {
        while self.0[item as usize] != 0 {
            item = self.0[item as usize];
        }

        item
    }
}

impl UF for QuickUnionUF {
    fn connect(&mut self, a: u32, b: u32) -> bool {
        let a = self.component_id(a);
        let b = self.component_id(b);
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

pub struct WeightedQuickUnionUF {
    parents: Vec<u32>,
    sizes: Vec<u32>,
}

impl WeightedQuickUnionUF {
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
}

impl UF for WeightedQuickUnionUF {
    fn connect(&mut self, a: u32, b: u32) -> bool {
        let a = self.component_id(a);
        let b = self.component_id(b);
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
