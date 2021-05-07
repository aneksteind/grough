extern crate grough;

use grough::algo::search::{bfs, dfs};
use grough::graph::Graph;

fn graph_1() -> Graph<i32, i128> {
    let mut graph = Graph::<i32, i128>::new();
    graph.add_edge(1, 2, 1);
    graph.add_edge(2, 3, 1);
    graph.add_edge(1, 3, 1);
    graph.add_edge(1, 4, 1);
    graph.add_edge(3, 4, 1);

    graph
}

#[test]
fn test_dfs() {
    let graph = graph_1();

    let result = dfs(&1, &4, &graph);
    assert!(result.is_some());
    assert_eq!(result, Some(vec![&1, &2, &3, &4]));

    assert_eq!(dfs(&1, &5, &graph), None);
}

#[test]
fn test_bfs() {
    let graph = graph_1();
    let result = bfs(&1, &2, &graph);

    assert!(result.is_some());
    assert_eq!(result, Some(vec![&1, &2]));
}
