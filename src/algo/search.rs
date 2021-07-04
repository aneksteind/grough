use crate::graph::{Edge, Graph, Vertex};
use std::collections::{VecDeque, HashSet};


pub struct Bfs<'a, V: Vertex, E: Edge> {
    pub queue: VecDeque<&'a V>,
    pub seen: HashSet<&'a V>,
    pub graph: &'a Graph<V, E>
}

impl <'a, V: Vertex, E: Edge> Bfs<'a, V, E> {
    pub fn new(v: &'a V, g: &'a Graph<V, E>) -> Self {
        Bfs {
            queue:vec![v].into_iter().collect(),
            seen: HashSet::new(),
            graph: g
        }
    }

    pub fn next(&mut self) -> Option<&'a V> {
        while let Some(v) = self.queue.pop_front() {
            if !self.seen.contains(v) {
                self.seen.insert(v);
                if let Some(neighbors) = self.graph.neighbors(v) {
                    for u in neighbors {
                        if !self.seen.contains(u) {
                            self.queue.push_back(u);
                        }
                    }
                }
                return Some(v);
            }
        }
        None
    }
}


pub fn bfs<'a, V: Vertex, E: Edge>(
    start: &'a V,
    end: &'a V,
    g: &'a Graph<V, E>,
) -> Option<Vec<&'a V>> {

    let mut bfs_visitor = Bfs::new(start, g);
    let mut path = Vec::new();
    while let Some(v) = bfs_visitor.next() {
        path.push(v);
        if v == end {
            return Some(path);
        }
    }

    None
}


pub struct Dfs<'a, V: Vertex, E: Edge> {
    pub stack: Vec<&'a V>,
    pub seen: HashSet<&'a V>,
    pub graph: &'a Graph<V, E>
}

impl <'a, V: Vertex, E: Edge> Dfs<'a, V, E> {
    pub fn new(v: &'a V, g: &'a Graph<V, E>) -> Self {
        Dfs {
            stack: vec![v],
            seen: HashSet::new(),
            graph: g
        }
    }

    pub fn next(&mut self) -> Option<&'a V> {
        while let Some(v) = self.stack.pop() {
            if !self.seen.contains(v) {
                self.seen.insert(v);
                if let Some(neighbors) = self.graph.neighbors(v) {
                    for u in neighbors {
                        if !self.seen.contains(u) {
                            self.stack.push(u);
                        }
                    }
                }
                return Some(v);
            }
        }
        None
    }
}


pub fn dfs<'a, V: Vertex, E: Edge>(
    start: &'a V,
    end: Option<&'a V>,
    g: &'a Graph<V, E>,
) -> Option<Vec<&'a V>> {

    if !g.contains_node(start) {
        return None;
    }

    let mut dfs_visitor = Dfs::new(start, g);
    let mut path = Vec::new();
    while let Some(v) = dfs_visitor.next() {
        path.push(v);
        if end.is_some() && v == end.unwrap() {
            return Some(path);
        }
    }

    if end.is_some() {
        None
    } else {
        Some(path)
    }
}
