extern crate grough;

use grough::graph::{Graph};
use grough::algo::traversal::{dfs_edges};

#[test]
fn test_dfs() {
    let mut graph = Graph::<i128>::new();

    graph.add_edge(1,2, 1);
    graph.add_edge(2,3, 1);
    graph.add_edge(2,4,2);

    dfs_edges(&graph);

    assert_eq!(graph.order(), 4);
}