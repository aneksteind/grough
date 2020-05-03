extern crate indexmap;
extern crate rand;
extern crate nom;

use indexmap::map::{IndexMap,Keys};
use rand::{thread_rng, Rng};
use nom::{
    IResult,
    sequence::tuple,
    character::complete::{digit1, multispace1}
};

use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct NodeIter<'a,K> {
    nodes: Keys<'a,K,IndexMap<K,i64>>
}

impl<'a,K> Iterator for NodeIter<'a,K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next()
    }
}

pub struct EdgeIter<'a, K> {
    collection: &'a IndexMap<K,IndexMap<K,i64>>,
    u: usize,
    v: usize,
    seen: HashSet<(&'a K,&'a K)>
}

impl<'a, K> Iterator for EdgeIter<'a, K> where
    K: Hash + Eq 
{

    type Item = (&'a K, &'a K, &'a i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.u < self.collection.len() {

            let (u,vs) = self.collection.get_index(self.u).unwrap();

            if self.v >= vs.len() {
                self.v = 0
            }

            let (v,w) = vs.get_index(self.v).unwrap();

            self.v = (self.v + 1) % vs.len();

            if self.v == 0 {
                self.u += 1
            }

            if !self.seen.contains(&(u,v)){
                self.seen.insert((u,v));
                self.seen.insert((v,u));

                Some((u,v,w))
            }

            else {
                self.next()
            }    
        }

        else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Graph {
    node_map : IndexMap<i32,IndexMap<i32,i64>>,
    edge_map: IndexMap<(i32,i32),i64>,
    size: u32,
    order: u32
}

impl Graph {
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
            let node_map = IndexMap::new();
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
    pub fn add_edge(&mut self, u: i32, v: i32, w: i64) -> () {
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
        if !u_map.contains_key(&v) {
            u_map.insert(v, w);
            back = true;
        }

        // add (v,u,w)
        let v_map = self.neighbors_mut(&v).unwrap();
        if !v_map.contains_key(&u) {
            v_map.insert(u, w);
            forth = true
        }

        let pair = if u < v {
            (u,v)
        } else {
            (v,u)
        };

        if !self.edge_map.contains_key(&pair) {
            self.edge_map.insert(pair, w);
        }

        if back && forth {
            self.size += 1
        }
    }

    // TODO: adjust to edge_map field
    pub fn edges(&self) -> EdgeIter<i32> {
        EdgeIter {
            collection: &self.node_map,
            u: 0,
            v: 0,
            seen: HashSet::new()
        }
    }

    pub fn nodes(&self) -> NodeIter<i32> {
        NodeIter {
            nodes: self.node_map.keys()
        }
    }

    pub fn neighbors(&self, u: &i32) -> Option<&IndexMap<i32,i64>> {
        self.node_map.get(u)
    }

    pub fn neighbors_mut(&mut self, u: &i32) -> Option<&mut IndexMap<i32,i64>> {
        self.node_map.get_mut(u)
    }

    pub fn remove_edge(&mut self, u: &i32, v: &i32) -> () {
        if self.contains_edge(u,v) {
            self.neighbors_mut(u).unwrap().swap_remove(v);
            self.neighbors_mut(v).unwrap().swap_remove(u);
            self.size -= 1;
        }
    }

    pub fn remove_node(&mut self, u: &i32) -> () {
        // get neighbors of u after removing u
        if let Some(neighbs) = self.node_map.swap_remove(u) {
            // remove u from each neighbor
            for n in neighbs.keys() {
                self.node_map.get_mut(n).unwrap().swap_remove(u);
                self.size -= 1;
            }
            self.order -= 1;
        }
    }

    // TODO: adjust to edge_map field
    pub fn contains_edge(&self, u: &i32, v: &i32) -> bool {
        if !self.contains_node(u) {
            return false
        }

        if !self.contains_node(v) {
            return false
        }

        let u_map = self.neighbors(u).unwrap();
        if !u_map.contains_key(v) {
            return false
        }

        let v_map = self.neighbors(v).unwrap();
        if !v_map.contains_key(u) {
            return false
        }

        true
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

    // TODO: adjust to edge_map field
    pub fn random_edge(&self) -> (&i32,&i32,&i64) {
        let mut rng = thread_rng();

        // get a node u in G
        let upper = self.order as usize;
        let u_index: usize = rng.gen_range(0, upper);
        let (u,u_map) = self.node_map.get_index(u_index).unwrap();

        // get a random neighbor v of u
        let u_map_size = u_map.len();
        let v_index = rng.gen_range(0,u_map_size);
        let (v,w) = u_map.get_index(v_index).unwrap();

        (u,v,w)
    }

    pub fn get_weight(&self, u: &i32, v: &i32) -> Option<&i64> {
        if self.contains_edge(u,v) {
            self.node_map.get(u).unwrap().get(v)
        }

        else {
            None
        }
    }

    pub fn get_weight_mut(&mut self, u: &i32, v: &i32) -> Option<&mut i64> {
        if self.contains_edge(u,v) {
            self.node_map.get_mut(u).unwrap().get_mut(v)
        }

        else {
            None
        }
    }

    pub fn set_weight(&mut self, u: &i32, v: &i32, w: i64) -> () {
        if let Some(weight) = self.get_weight_mut(u,v) {
            *weight = w;
        }

        if let Some(weight) = self.get_weight_mut(v,u) {
            *weight = w;
        }
    }

    pub fn contract_edge<F>(&mut self, u: &i32, v: &i32, combine: F) -> i64 where
        F: Clone + Copy + Fn(i64,i64) -> i64
    {

        /* STEP 1: calculate the cost */

        // cost of this edge
        let mut contraction_cost = *self.get_weight(u,v).unwrap();

        self.remove_edge(u,v);

        // costs of edges incident to u
        for x in self.neighbors(u).unwrap().keys() {
            let wn = self.get_weight(u,x).unwrap();
            contraction_cost = combine(contraction_cost, *wn);
        }

        // costs of edges incident to v
        for x in self.neighbors(v).unwrap().keys() {
            let wn = self.get_weight(v,x).unwrap();
            contraction_cost = combine(contraction_cost, *wn);
        }    

        /* STEP 2: contract the edge */

        // calculate and save the new weights of edges incident to v
        let mut v_incident_weights = HashMap::new();
        for (x,wvx) in self.neighbors(&v).unwrap() {
            
            // if u and v are both incident to x, the weights will be combined
            if self.neighbors(&u).unwrap().contains_key(x) {
                let wux = self.get_weight(u,x).unwrap();
                let new_weight = combine(*wvx,*wux);
                v_incident_weights.insert(*x,new_weight);
            } 
            // otherwise the weight will simply stay the same as it was
            else {
                let wvx = self.get_weight(v,x).unwrap();
                v_incident_weights.insert(*x,*wvx);
            }
            
        }

        // replace all edges incident to v by by remapping them to u
        for (x,wvx) in v_incident_weights {
            // remove (x,v) because v is getting fused into u
            self.remove_edge(&x,v);

            if self.contains_edge(&x,u) {
                self.set_weight(&x,u,wvx);
            } else {
                self.add_edge(x,*u,wvx);
            }
            
            
        }

        self.remove_node(v);

        contraction_cost
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

    pub fn contract_edges<F>(&mut self, edges: Vec<(i32,i32)>, combine: F) -> i64
    where F: Clone + Copy + Fn(i64,i64) -> i64 {
        let mut total_cost = 0;

        let mut overwrite = HashMap::<i32,i32>::new();
        for u in self.nodes() {
            overwrite.insert(*u,*u);
        }

        for (u,v) in edges {
            let u = self.node_ref(&overwrite, u);
            let v = self.node_ref(&overwrite, v);

            if u != v {
                let cost = self.contract_edge(&u,&v,combine);
                let map = overwrite.get_mut(&v).unwrap();
                *map = u;
                total_cost += cost;
            }   
        }

        total_cost
    }

    pub fn contract_random_edge<F>(&mut self, combine: F) -> i64 
    where F: Clone + Copy + Fn(i64,i64) -> i64 {
        let (u,v,_w) = self.random_edge();
        let (x,y) = (*u,*v);
        self.contract_edge(&x,&y,combine)
    }

    pub fn edge_idx(&self, idx: usize) -> Option<(&i32,&i32,&i64)> {
        if let Some(((u,v),w)) = self.edge_map.get_index(idx) {
            Some((u,v,w))
        }

        else {
            None
        }
    }

    pub fn from_file_ew(path: &str) -> std::io::Result<Graph>{
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut graph = Graph::new();

        for line in reader.lines() {
            let edge_line = line.unwrap();
            let (_, (x,_,y,_,z)) = parse_edge_weight(&edge_line).unwrap();
            let (u,v,w) = (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap(), z.parse::<i64>().unwrap());
            graph.add_edge(u, v, w);
        }

        return Ok(graph);
    }
}

fn parse_edge_weight(line: &str) -> IResult<&str,(&str,&str,&str,&str,&str)> {
    tuple((digit1,multispace1,digit1,multispace1,digit1))(line)
}

// returns a complete graph on n nodes
pub fn k(n: i32) -> Graph {
    let mut graph = Graph::new();

    // add nodes
    for u in 0..n {
        graph.add_node(u);
    }

    // connect edges
    for u in 0..n {
        for v in 0..n {
            if u < v{
                let mut rng = thread_rng();
                let w = rng.gen_range(0,100);
                graph.add_edge(u,v,w);
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_init() {
        let graph = Graph::new();
        assert_eq!(graph.order(), 0);
        assert_eq!(graph.size(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
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
        let mut graph = Graph::new();

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

        let (u,v,_w) = graph.random_edge();
        assert!(graph.contains_edge(u,v));
    }

    #[test]
    fn test_remove_node() {
        let mut graph = Graph::new();

        graph.add_edge(1,2,0);
        graph.add_edge(2,3,0);

        graph.remove_node(&2);

        assert!(!graph.contains_edge(&2,&3));
        assert!(!graph.contains_edge(&1,&2));
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
        let mut graph = Graph::new();

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

        assert_eq!(es.next(), Some((&1,&2,&0)));
        assert_eq!(es.next(), Some((&1,&3,&0)));
        assert_eq!(es.next(), Some((&2,&3,&0)));
        assert_eq!(es.next(), None);
    }

    #[test]
    fn test_get_weight() {
        let mut graph = Graph::new();

        graph.add_edge(1,2,3);
        let w = graph.get_weight(&1,&2).unwrap();
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
        let contraction_cost = graph.contract_edge(&1, &2, |x,y| x + y);

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
        let total_cost = graph_clone.contract_edges(edge_list, |x,y| x*y);

        assert_eq!(total_cost,204);
    }

    #[test]
    fn test_from_file_ew() {
        let filename = "src/test_graph.ew";
        let graph = Graph::from_file_ew(filename).unwrap();
        assert_eq!(graph.order(), 64);
    }
}