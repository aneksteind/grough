extern crate indexmap;
extern crate rand;
extern crate nom;

use indexmap::map::{IndexMap,Keys};
use indexmap::set::{IndexSet};
use rand::{thread_rng, Rng};
use nom::{
    IResult,
    sequence::tuple,
    character::complete::{digit1, multispace1}
};

use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct EdgeIter<'a, K, W> {
    edges: Keys<'a,K,W>,
}

impl<'a, K, W> Iterator for EdgeIter<'a,K,W> where
    K: Hash + Eq 
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.next()
    }
}

#[derive(Clone)]
pub struct Graph<W> {
    node_map : IndexMap<i32,IndexSet<i32>>,
    edge_map: IndexMap<(i32,i32),W>,
    size: u32,
    order: u32
}

impl<W: Clone> Graph<W> {
    pub fn new() -> Self {
        Graph {
            node_map: IndexMap::new(),
            edge_map: IndexMap::new(),
            order: 0,
            size: 0,
        }
    }

    // The number of edges in the graph
    pub fn size(&self) -> u32 {
        self.size
    }

    // The number of nodes in the graph
    pub fn order(&self) -> u32 {
        self.order
    }

    // adds a node to the graph, if the node
    // already exists then nothing happens
    pub fn add_node(&mut self, u: i32) -> () {
        if !self.node_map.contains_key(&u) {
            let node_map = IndexSet::new();
            self.node_map.insert(u, node_map);
            self.order += 1
        }
    }

    pub fn contains_node(&self, u: &i32) -> bool {
        return self.node_map.contains_key(u);
    }

    // adds an edge to the graph
    // if the edge is already in the graph
    // then nothing happens
    pub fn add_edge(&mut self, u: i32, v: i32, w: W) -> () {
        let mut back = false;
        let mut forth = false;

        if !self.contains_node(&u){
            self.add_node(u);
        }

        if !self.contains_node(&v){
            self.add_node(v);
        } 

        // add (u,v,w)
        let u_map = self.neighbors_mut(&u).unwrap();
        if !u_map.contains(&v) {
            u_map.insert(v);
            back = true;
        }

        // add (v,u,w)
        let v_map = self.neighbors_mut(&v).unwrap();
        if !v_map.contains(&u) {
            v_map.insert(u);
            forth = true
        }

        let pair = self.edge(u,v);

        if !self.edge_map.contains_key(&pair) {
            self.edge_map.insert(pair, w.clone());
        }

        if back && forth {
            self.size += 1
        }
    }

    fn edge(&self, u: i32, v: i32) -> (i32,i32) {
        if u < v { (u,v) } else { (v,u) }
    }

    pub fn edges(&self) -> EdgeIter<(i32,i32),W> {
        EdgeIter{
            edges: self.edge_map.keys()
        }
    }

    pub fn nodes(&self) -> Keys<i32,IndexSet<i32>> {
        self.node_map.keys()
    }

    pub fn neighbors(&self, u: &i32) -> Option<&IndexSet<i32>> {
        self.node_map.get(u)
    }

    pub fn neighbors_mut(&mut self, u: &i32) -> Option<&mut IndexSet<i32>> {
        self.node_map.get_mut(u)
    }

    pub fn remove_edge(&mut self, u: &i32, v: &i32) -> () {
        if self.contains_edge(u,v) {
            self.neighbors_mut(u).unwrap().swap_remove(v);
            self.neighbors_mut(v).unwrap().swap_remove(u);

            let e = self.edge(*u,*v);
            self.edge_map.remove(&e);

            self.size -= 1;
        }
    }

    pub fn remove_node(&mut self, u: &i32) -> () {
        // get neighbors of u after removing u
        if let Some(neighbs) = self.node_map.swap_remove(u) {
            // remove u from each neighbor
            for n in neighbs.iter() {
                self.node_map.get_mut(n).unwrap().swap_remove(u);
                let e = self.edge(*u,*n);
                self.edge_map.swap_remove(&e);
                self.size -= 1;
            }
            self.order -= 1;
        }
    }

    pub fn contains_edge(&self, u: &i32, v: &i32) -> bool {
        let e = self.edge(*u,*v);
        self.edge_map.contains_key(&e)
    }

    pub fn random_node(&self) -> &i32 {
        let mut rng = thread_rng();

        // get index of a node [0,|V|)
        let upper = self.order as usize;
        let index: usize = rng.gen_range(0, upper);
        let (v,_) = self.node_map.get_index(index).unwrap();

        // return the node
        v
    }

    pub fn random_edge(&self) -> (&i32,&i32) {
        let mut rng = thread_rng();

        // get a node u in G
        let upper = self.size as usize;
        let edge_index: usize = rng.gen_range(0, upper);
        let ((u,v),_w) = self.edge_map.get_index(edge_index).unwrap();

        (u,v)
    }

    pub fn get_weight(&self, u: &i32, v: &i32) -> Option<&W> {
        if self.contains_edge(u,v) {
            let ref key = self.edge(*u,*v);
            self.edge_map.get(key)
        }

        else {
            None
        }
    }

    pub fn get_weight_mut(&mut self, u: &i32, v: &i32) -> Option<&mut W> {
        if self.contains_edge(u,v) {
            let ref key = self.edge(*u,*v);
            self.edge_map.get_mut(key)
        }

        else {
            None
        }
    }

    pub fn set_weight(&mut self, u: &i32, v: &i32, w: W) -> () {
        if let Some(weight) = self.get_weight_mut(u,v) {
            *weight = w.clone();
        }

        if let Some(weight) = self.get_weight_mut(v,u) {
            *weight = w.clone();
        }
    }

    pub fn contraction_cost<F>(&self, u: &i32, v: &i32, combine: F) -> W where
        F: Clone + Copy + Fn(&W,&W) -> W
    {
        // cost of this edge
        let mut contraction_cost = self.get_weight(u,v).unwrap().clone();

        // costs of edges incident to u
        for x in self.neighbors(u).unwrap().iter() {
            if x != v {
                let wn = self.get_weight(u,x).unwrap();
                contraction_cost = combine(&contraction_cost, &wn);
            }
        }

        // costs of edges incident to v
        for x in self.neighbors(v).unwrap().iter() {
            if x != u {
                let wn = self.get_weight(v,x).unwrap();
                contraction_cost = combine(&contraction_cost, &wn);
            }
        }    

        contraction_cost
    }

    pub fn contract_edge<F>(&mut self, u: &i32, v: &i32, combine: F) -> () where
        F: Clone + Copy + Fn(&W,&W) -> W
    {
        self.remove_edge(u,v);

        // calculate and save the new weights of edges incident to v
        let mut v_incident_weights = HashMap::new();
        for x in self.neighbors(&v).unwrap() {
            
            // if u and v are both incident to x, the weights will be combined
            if self.neighbors(&u).unwrap().contains(x) {
                let wvx = self.get_weight(v, x).unwrap();
                let wux = self.get_weight(u,x).unwrap();
                let new_weight = combine(wvx,wux);
                v_incident_weights.insert(*x,new_weight);
            } 
            // otherwise the weight will simply stay the same as it was
            else {
                let wvx = self.get_weight(v,x).unwrap();
                v_incident_weights.insert(*x,wvx.clone());
            }
            
        }

        // replace all edges incident to v by by remapping them to u
        for (x,wvx) in v_incident_weights {
            // remove (x,v) because v is getting fused into u
            self.remove_edge(&x,v);

            if self.contains_edge(&x,u) {
                self.set_weight(&x,u, wvx);
            } else {
                self.add_edge(x,*u, wvx);
            }   
        }

        self.remove_node(v);
    }

    fn node_ref(&self, fusion: &HashMap<i32,i32>, v: i32) -> i32 {
        let mut fused = fusion.get(&v).unwrap();
        let mut last = v;

        while *fused != last {
            last = *fused;
            fused = fusion.get(&fused).unwrap();
        }

        *fused
    }

    // TODO: separate contraction from cost calculation
    pub fn contract_edges<F>(&mut self, edges: Vec<(i32,i32)>, base: W, combine: F) -> W
    where F: Clone + Copy + Fn(&W,&W) -> W,
          W: std::ops::Add<Output=W>
    {
        let mut total_cost = base;

        let mut overwrite = HashMap::<i32,i32>::new();
        for u in self.nodes() {
            overwrite.insert(*u,*u);
        }

        for (u,v) in edges {
            let u = self.node_ref(&overwrite, u);
            let v = self.node_ref(&overwrite, v);

            if u != v {
                let cost = self.contraction_cost(&u, &v, combine);
                self.contract_edge(&u,&v,combine);
                let map = overwrite.get_mut(&v).unwrap();
                *map = u;
                total_cost = total_cost + cost;
            }   
        }

        total_cost
    }

    pub fn contract_random_edge<F>(&mut self, combine: F) -> W 
    where F: Clone + Copy + Fn(&W,&W) -> W,
    {
        let (u,v) = self.random_edge();
        let (x,y) = (*u,*v);
        let cost = self.contraction_cost(&x, &y, combine);
        self.contract_edge(&x,&y,combine);
        cost
    }

    pub fn edge_idx(&self, idx: usize) -> Option<(&i32,&i32,&W)> {
        if let Some(((u,v),w)) = self.edge_map.get_index(idx) {
            Some((u,v,w))
        }

        else {
            None
        }
    }

    pub fn from_file_ew(path: &str) -> std::io::Result<Graph<i128>> where
        W: std::str::FromStr + std::fmt::Debug {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut graph = Graph::new();

        for line in reader.lines() {
            let edge_line = line.unwrap();
            let (_, (x,_,y,_,z)) = parse_edge_weight(&edge_line).unwrap();
            let (u,v,w) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap(), z.parse::<i128>().unwrap());
            graph.add_edge(u, v, w);
        }

        return Ok(graph);
    }
}

fn parse_edge_weight(line: &str) -> IResult<&str,(&str,&str,&str,&str,&str)> {
    tuple((digit1,multispace1,digit1,multispace1,digit1))(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_init() {
        let graph = Graph::<i128>::new();
        assert_eq!(graph.order(), 0);
        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::<i128>::new();
        graph.add_node(1);
        assert_eq!(graph.order(), 1);
        assert!(graph.contains_node(&1));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();

        graph.add_edge(1,2,3);
        graph.add_edge(2,3,4);
        graph.add_edge(3,1,0);

        graph.add_node(7);
        graph.add_node(6);
        graph.add_edge(6,7,0);

        assert!(graph.contains_edge(&3,&2));
        assert!(graph.contains_edge(&1,&2));
        assert!(graph.contains_edge(&1,&3));

        assert_eq!(graph.size(), 4);
    }

    #[test]
    fn test_random_node() {
        let mut graph = Graph::<i128>::new();

        graph.add_node(104);
        graph.add_node(201);

        let rnode = graph.random_node();
        assert!(graph.contains_node(rnode));
    }

    #[test]
    fn test_random_edge() {
        let mut graph = Graph::new();

        graph.add_edge(104,201,1);
        graph.add_edge(201,103,1);
        graph.add_edge(104,104,1);

        let (u,v) = graph.random_edge();
        assert!(graph.contains_edge(u,v));
    }

    #[test]
    fn test_remove_node() {
        let mut graph = Graph::new();

        graph.add_edge(1,2,0);
        graph.add_edge(2,3,0);

        graph.remove_node(&2);

        assert!(!graph.contains_edge(&2,&3));
        assert!(!graph.contains_node(&2));

        assert_eq!(graph.order(), 2);
        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = Graph::new();

        graph.add_edge(1,2,0);
        graph.add_edge(2,3,0);

        graph.remove_edge(&2, &1);

        assert!(graph.contains_edge(&2,&3));
        assert!(!graph.contains_edge(&1,&2));

        assert_eq!(graph.order(), 3);
        assert_eq!(graph.size(), 1);
    }

    #[test]
    fn test_nodes() {
        let mut graph = Graph::<i128>::new();

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

        graph.add_edge(1,2,0);
        graph.add_edge(1,3,0);
        graph.add_edge(2,3,0);

        let mut es = graph.edges();

        assert_eq!(es.next(), Some(&(1,2)));
        assert_eq!(es.next(), Some(&(1,3)));
        assert_eq!(es.next(), Some(&(2,3)));
        assert_eq!(es.next(), None);
    }

    #[test]
    fn test_get_weight() {
        let mut graph = Graph::new();

        graph.add_edge(1,2,3);
        let w = graph.get_weight(&2,&1).unwrap();
        assert_eq!(*w, 3);
    }

    #[test]
    fn test_set_weight() {
        let mut graph = Graph::new();

        graph.add_edge(1,2,3);
        graph.set_weight(&1,&2,4);
        let w = graph.get_weight(&1,&2).unwrap();
        assert_eq!(*w, 4);
    }

    #[test]
    fn test_contract_edge() {
        // create simple graph
        let mut graph = Graph::new();

        // add edges
        graph.add_edge(1,2,5);
        graph.add_edge(2,3,4);
        graph.add_edge(3,1,3);
        graph.add_edge(4,2,7);

        // contract an edge
        let contraction_cost = graph.contraction_cost(&1,&2, |x,y| x+y);
        graph.contract_edge(&1, &2, |x,y| x + y);

        // assert cost is correct
        assert_eq!(contraction_cost, 19);

        // assert edge weights of edges incident to absorbed node are correct
        let w13 = graph.get_weight(&1,&3).unwrap();
        assert_eq!(*w13, 7);

        // assert new edges have been created
        assert!(graph.contains_edge(&1,&3));
        assert!(graph.contains_edge(&1,&4));

        // assert node number is correct
        assert_eq!(graph.order(), 3);        

        // assert edge number is correct
        assert_eq!(graph.size(), 2);

        let mut graph2 = Graph::new();
        graph2.add_edge(1,2,0);
        graph2.contract_edge(&1, &2, |x,y| x + y);
        assert_eq!(graph2.order(), 1);
        assert_eq!(graph2.size(), 0);
    }

    #[test]
    fn test_contract_edges(){
        let mut graph = Graph::new();

        // inspired MERA graph from netcon
        graph.add_edge(1,2,2);
        graph.add_edge(1,3,2);
        graph.add_edge(1,4,2);
        graph.add_edge(2,3,2);
        graph.add_edge(2,4,2);
        graph.add_edge(2,5,2);
        graph.add_edge(3,5,2);
        graph.add_edge(4,5,2);
        graph.add_edge(4,6,2);
        graph.add_edge(5,7,2);
        graph.add_edge(6,7,2);

        let edge_list : Vec<(i32,i32)> = vec![
            (1,3),
            (1,2),
            (2,3),
            (1,4),
            (2,4),
            (2,5),
            (3,5),
            (4,6),
            (4,5),
            (5,7),
            (6,7)
        ];

        let mut graph_clone = graph.clone();
        let total_cost = graph_clone.contract_edges(edge_list, 0, |x,y| x*y);

        assert_eq!(total_cost,204);
    }

    #[test]
    fn test_from_file_ew() {
        let filename = "src/test_graph.ew";
        let graph = Graph::<i128>::from_file_ew(filename).unwrap();
        assert_eq!(graph.order(), 64);
    }
}