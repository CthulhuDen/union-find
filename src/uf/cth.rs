use super::UF;

pub struct CthlUF {
    comps: Vec<Vec<u32>>,
    comp_by_vert: Vec<u32>,
}

impl CthlUF {
    pub fn new(size: usize) -> Self {
        Self {
            comps: vec![],
            comp_by_vert: vec![0; size],
        }
    }
}

impl UF for CthlUF {
    fn connect(&mut self, a: u32, b: u32) -> bool {
        match (self.comp_by_vert[a as usize], self.comp_by_vert[b as usize]) {
            (0, 0) => {
                self.comp_by_vert[a as usize] = self.comps.len() as u32 + 1;
                self.comp_by_vert[b as usize] = self.comps.len() as u32 + 1;
                self.comps.push(vec![a, b]);
                true
            }
            (comp, 0) => {
                self.comp_by_vert[b as usize] = comp;
                self.comps[comp as usize - 1].push(b);
                true
            }
            (0, comp) => {
                self.comp_by_vert[a as usize] = comp;
                self.comps[comp as usize - 1].push(a);
                true
            }
            (comp1, comp2) => {
                if comp1 != comp2 {
                    let (from, to) = if self.comps[comp1 as usize - 1].len()
                        < self.comps[comp2 as usize - 1].len()
                    {
                        (comp1, comp2)
                    } else {
                        (comp2, comp1)
                    };

                    for vert in &self.comps[from as usize - 1] {
                        self.comp_by_vert[*vert as usize] = to;
                    }
                    let mut from_verts = std::mem::take(&mut self.comps[from as usize - 1]);
                    self.comps[to as usize - 1].append(&mut from_verts);

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
