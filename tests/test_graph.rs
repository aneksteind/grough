extern crate grough;
use grough::graph::Graph;
use grough::io::from_file_ew;

#[test]
fn test_graph_init() {
    let graph = Graph::<i32, i128>::new();
    assert_eq!(graph.order(), 0);
    assert_eq!(graph.size(), 0);
}

#[test]
fn test_add_node() {
    let mut graph = Graph::<i32, i128>::new();
    graph.add_node(1);
    assert_eq!(graph.order(), 1);
    assert!(graph.contains_node(&1));
}

#[test]
fn test_add_edge() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2, 3);
    graph.add_edge(2, 3, 4);
    graph.add_edge(3, 1, 0);

    graph.add_node(7);
    graph.add_node(6);
    graph.add_edge(6, 7, 0);

    assert!(graph.contains_edge(&3, &2));
    assert!(graph.contains_edge(&1, &2));
    assert!(graph.contains_edge(&1, &3));

    assert_eq!(graph.size(), 4);
}

#[test]
fn test_random_node() {
    let mut graph = Graph::<i32, i128>::new();

    graph.add_node(104);
    graph.add_node(201);

    let rnode = graph.random_node();
    assert!(graph.contains_node(rnode));
}

#[test]
fn test_random_edge() {
    let mut graph = Graph::new();

    graph.add_edge(104, 201, 1);
    graph.add_edge(201, 103, 1);
    graph.add_edge(104, 104, 1);

    let (u, v) = graph.random_edge();
    assert!(graph.contains_edge(u, v));
}

#[test]
fn test_remove_node() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2, 0);
    graph.add_edge(2, 3, 0);

    graph.remove_node(&2);

    assert!(!graph.contains_edge(&2, &3));
    assert!(!graph.contains_node(&2));

    assert_eq!(graph.order(), 2);
    assert_eq!(graph.size(), 0);
}

#[test]
fn test_remove_edge() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2, 0);
    graph.add_edge(2, 3, 0);

    graph.remove_edge(&2, &1);

    assert!(graph.contains_edge(&2, &3));
    assert!(!graph.contains_edge(&1, &2));

    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 1);
}

#[test]
fn test_nodes() {
    let mut graph = Graph::<i32, i128>::new();

    graph.add_node(1);
    graph.add_node(2);

    let mut vs = graph.nodes();

    assert_eq!(vs.next(), Some(&1));
    assert_eq!(vs.next(), Some(&2));
    assert_eq!(vs.next(), None);
}

#[test]
fn test_edges() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2, 0);
    graph.add_edge(1, 3, 0);
    graph.add_edge(2, 3, 0);

    let mut es = graph.edges();

    assert_eq!(es.next(), Some(&(1, 2)));
    assert_eq!(es.next(), Some(&(1, 3)));
    assert_eq!(es.next(), Some(&(2, 3)));
    assert_eq!(es.next(), None);
}

#[test]
fn test_get_weight() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2, 3);
    let w = graph.get_weight(&2, &1).unwrap();
    assert_eq!(*w, 3);
}

#[test]
fn test_set_weight() {
    let mut graph = Graph::new();

    graph.add_edge(1, 2, 3);
    graph.set_weight(&1, &2, 4);
    let w = graph.get_weight(&1, &2).unwrap();
    assert_eq!(*w, 4);
}

#[test]
fn test_contract_edge() {
    // create simple graph
    let mut graph = Graph::new();

    // add edges
    graph.add_edge(1, 2, 5);
    graph.add_edge(2, 3, 4);
    graph.add_edge(3, 1, 3);
    graph.add_edge(4, 2, 7);

    // contract an edge
    let contraction_cost = graph.contraction_cost(&1, &2, &|x, y| x + y);
    graph.contract_edge(&1, &2, |x, y| x + y);

    // assert cost is correct
    assert_eq!(contraction_cost, 19);

    // assert edge weights of edges incident to absorbed node are correct
    let w13 = graph.get_weight(&1, &3).unwrap();
    assert_eq!(*w13, 7);

    // assert new edges have been created
    assert!(graph.contains_edge(&1, &3));
    assert!(graph.contains_edge(&1, &4));

    // assert node number is correct
    assert_eq!(graph.order(), 3);

    // assert edge number is correct
    assert_eq!(graph.size(), 2);

    let mut graph2 = Graph::new();
    graph2.add_edge(1, 2, 0);
    graph2.contract_edge(&1, &2, |x, y| x + y);
    assert_eq!(graph2.order(), 1);
    assert_eq!(graph2.size(), 0);
}

#[test]
fn test_contract_edges() {
    let mut graph = Graph::new();

    // inspired MERA graph from netcon
    graph.add_edge(1, 2, 2);
    graph.add_edge(1, 3, 2);
    graph.add_edge(1, 4, 2);
    graph.add_edge(2, 3, 2);
    graph.add_edge(2, 4, 2);
    graph.add_edge(2, 5, 2);
    graph.add_edge(3, 5, 2);
    graph.add_edge(4, 5, 2);
    graph.add_edge(4, 6, 2);
    graph.add_edge(5, 7, 2);
    graph.add_edge(6, 7, 2);

    let edge_list: Vec<(i32, i32)> = vec![
        (1, 3),
        (1, 2),
        (2, 3),
        (1, 4),
        (2, 4),
        (2, 5),
        (3, 5),
        (4, 6),
        (4, 5),
        (5, 7),
        (6, 7),
    ];

    let mut graph_clone = graph.clone();
    let total_cost = graph_clone.contract_edges(edge_list, 0, &|x, y| x * y);

    assert_eq!(total_cost, 204);
}

#[test]
fn test_from_file_ew() {
    let filename = "src/test_graph.ew";
    let graph = from_file_ew::<i32, i32>(filename).unwrap();
    assert_eq!(graph.order(), 64);
}
