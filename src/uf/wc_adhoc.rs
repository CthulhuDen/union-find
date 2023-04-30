pub fn count_useful_edges(num_verts: usize, edges: Vec<(u32, u32)>) -> usize {
    fn get_root(mut item: u32, parents: &mut [u32]) -> u32 {
        if parents[item as usize] == 0 {
            return item;
        }

        let mut root = parents[item as usize];
        while parents[root as usize] != 0 {
            root = parents[root as usize];
        }

        while parents[item as usize] != root {
            item = std::mem::replace(&mut parents[item as usize], root);
        }

        root
    }

    fn connect(item1: u32, item2: u32, parents: &mut [u32], sizes: &mut [u32]) -> bool {
        let root1 = get_root(item1, parents);
        let root2 = get_root(item2, parents);
        if root1 == root2 {
            return false;
        }

        if sizes[root1 as usize] > sizes[root2 as usize] {
            parents[root2 as usize] = root1;
            sizes[root1 as usize] += sizes[root2 as usize] + 1;
        } else {
            parents[root1 as usize] = root2;
            sizes[root2 as usize] += sizes[root1 as usize] + 1;
        }
        true
    }

    let mut parents = vec![0; num_verts + 1];
    let mut sizes = vec![0; num_verts + 1];

    let mut counter = 0;

    for edge in edges {
        if connect(edge.0 + 1, edge.1 + 1, &mut parents, &mut sizes) {
            counter += 1;
        }
    }

    counter
}
