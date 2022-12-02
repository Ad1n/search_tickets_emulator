use std::collections::HashSet;
use petgraph::Graph;

lazy_static! {
    pub static ref FlightGraph: Graph<String, HashSet<String>> = Graph::new();
}