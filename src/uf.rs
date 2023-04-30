pub use cth::CthlUF;
pub use compressed_weighted::CompressedWeightedQuickUnionUF;
pub use others::{QuickFindUF, QuickUnionUF, WeightedQuickUnionUF};
pub use wc_adhoc::count_useful_edges;

mod cth;
mod compressed_weighted;
mod others;
mod wc_adhoc;

pub trait UF {
    fn connect(&mut self, a: u32, b: u32) -> bool;
    fn connected(&self, a: u32, b: u32) -> bool;
}
