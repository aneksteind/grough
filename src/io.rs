use nom::{
    character::complete::{digit1, multispace1},
    sequence::tuple,
    IResult,
};
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

use crate::graph::{Edge, Graph, Vertex};

/// Reads a graph from a file with integer node IDs and integer edge weights.
///
/// Example:
/// ```ignore
/// // test.ew
/// 0 1 1
/// 1 2 1
/// 2 0 3
/// ```
///
/// ```rust,ignore
/// let graph = from_file_ew::<i32, i32>("test.ew").unwrap();
/// assert_eq!(graph.order(), 3);
/// ```
pub fn from_file_ew<V: Vertex, E: Edge>(path: &str) -> std::io::Result<Graph<V, E>>
where
    V: Debug + PartialOrd + Hash + Eq + Copy + FromStr,
    E: FromStr + Debug + Clone,
    <E as FromStr>::Err: Debug,
    <V as FromStr>::Err: Debug,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut graph = Graph::new();

    for line in reader.lines() {
        let edge_line = line.unwrap();
        let (_, (x, _, y, _, z)) = parse_edge_weight(&edge_line).unwrap();
        let (u, v, w) = (
            x.parse::<V>().unwrap(),
            y.parse::<V>().unwrap(),
            z.parse::<E>().unwrap(),
        );
        graph.add_edge(u, v, w);
    }

    return Ok(graph);
}

/// Parses a line of an edge-weight (.ew) file.
fn parse_edge_weight(line: &str) -> IResult<&str, (&str, &str, &str, &str, &str)> {
    tuple((digit1, multispace1, digit1, multispace1, digit1))(line)
}
