use std::collections::HashSet;
use tools::read_input_file;

#[derive(PartialEq, Debug)]
struct Node {
    x: i32,
    y: i32,
    z: i32,
}

impl Node {
    fn distance(&self, other: &Node) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
    }
}

fn build_edges(nodes: &[Node]) -> Vec<(f64, usize, usize)> {
    let mut edges = Vec::new();

    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push((nodes[i].distance(&nodes[j]), i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    edges
}

fn main() {
    let input = read_input_file();
    let nodes: Vec<Node> = input
        .lines()
        .map(|line| {
            let v: Vec<i32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
            Node {
                x: v[0],
                y: v[1],
                z: v[2],
            }
        })
        .collect();

    let edges = build_edges(&nodes);
    let mut uf = UnionFind::new(nodes.len());
    let mut components = nodes.len();
    let mut last_edge = (0, 0);

    for (_, a, b) in edges {
        let ra = uf.find(a);
        let rb = uf.find(b);

        if ra != rb {
            uf.union(ra, rb);
            components -= 1;
            last_edge = (a, b);

            if components == 1 {
                break;
            }
        }
    }
    let result = nodes[last_edge.0].x * nodes[last_edge.1].x;
    println!("Answer: {}", result);

}
