static INPUT: &str = include_str!("../input");

use petgraph::graph::DiGraph;

fn create_map(input: &str) -> DiGraph<char, usize> {
    let mut map = DiGraph::new();
    let lines: Vec<_> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let nodes: Vec<Vec<_>> = (0..rows)
        .map(|row| {
            (0..cols)
                .map(|col| map.add_node(lines[row].chars().nth(col).unwrap()))
                .collect()
        })
        .collect();
    map
}

fn main() {
    println!("Hello, world!");
}
