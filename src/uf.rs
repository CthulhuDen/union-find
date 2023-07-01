pub use compressed::CompressedQuickUnionUF;
pub use compressed_weighted::CompressedWeightedQuickUnionUF;
pub use cth::CthlUF;
pub use others::{QuickFindUF, QuickUnionUF, WeightedQuickUnionUF};
pub use wc_adhoc::count_useful_edges;

mod compressed;
mod compressed_weighted;
mod cth;
mod others;
mod wc_adhoc;

pub trait UF {
    fn connect(&mut self, a: u32, b: u32) -> bool;
    fn connected(&self, a: u32, b: u32) -> bool;
}
