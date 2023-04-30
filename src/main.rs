use std::io::{BufWriter, Write};

use union_find::{CompressedWeightedQuickUnionUF, UF};

fn main() {
    let mut reader = std::io::stdin().lines();
    let size = reader.next().unwrap().unwrap().parse().unwrap();

    let mut uf = CompressedWeightedQuickUnionUF::new(size);

    let mut out = BufWriter::new(std::io::stdout().lock());

    for line in reader.map(Result::unwrap) {
        let mut pieces = line.split_ascii_whitespace();
        let a = pieces.next().unwrap().parse().unwrap();
        let b = pieces.next().unwrap().parse().unwrap();

        if uf.connect(a, b) {
            let _ = writeln!(out, "{line}");
        }
    }
}
