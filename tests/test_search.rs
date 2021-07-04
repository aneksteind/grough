extern crate grough;

use grough::algo::search::{dfs, bfs};
use grough::graph::Graph;


fn graph_1() -> Graph::<i32, i128> {
    let mut graph = Graph::<i32, i128>::new();
    graph.add_edge(1, 2, 1);
    graph.add_edge(2, 3, 1);
    graph.add_edge(1, 3, 1);
    graph.add_edge(1, 4, 1);
    graph.add_edge(3, 4, 1);

    graph
}

fn graph_2() -> Graph::<i32, i128> {
    let mut graph = Graph::<i32, i128>::new();
    graph.add_edges(vec![
        (0,1,1),
        (1,3,1),
        (1,2,1),
        (3,4,1),
        (2,4,1)
    ]);

    graph
}

#[test]
fn test_dfs() {
    
    let mut graph = graph_2();

    let result = dfs(&0, None, &graph);
    assert!(result.is_some());
    assert_eq!(result, Some(vec![&0, &1, &2, &4, &3]));

    assert_eq!(dfs(&1, Some(&5), &graph), None);

    graph = graph_1();

    let result = dfs(&1, Some(&3), &graph);
    assert!(result.is_some());
    assert_eq!(result, Some(vec![&1, &4, &3]));

    assert_eq!(dfs(&1, Some(&5), &graph), None);
}

#[test]
fn test_bfs() {
    let graph = graph_1();
    let result = bfs(&1, &2, &graph);

    assert!(result.is_some());
    assert_eq!(result, Some(vec![&1, &2]));
}