use petgraph::{Graph, Undirected};
use std::sync::{Mutex, MutexGuard};
use petgraph::graph::NodeIndex;


// Node
#[derive(Debug, Clone)]
pub struct IataAirline {
    code: String,
}

// Edge weight
pub struct FlightWeight {
    flight_time: u32,
    pub(crate) flights: Vec<String>,
}

pub struct FlightGraph {
    pub(crate) data: Graph<IataAirline, FlightWeight, Undirected>
}

impl FlightGraph {
    pub fn new() -> FlightGraph {
        FlightGraph {
            data: Graph::new_undirected(),
        }
    }

    pub fn find_node_idx(&self, code: String) -> std::option::Option<NodeIndex> {
        self.data.node_indices().find(|i| self.data[*i].code == code)
    }

    pub fn find_node_or_create(&mut self, iata_node: IataAirline) -> std::option::Option<NodeIndex> {
        let temp_node = iata_node.clone();
        match self.find_node_idx(iata_node.code) {
            Some(node_index) => Some(node_index),
            None => Some(self.data.add_node(temp_node))
        }
    }
}

lazy_static! {
    pub static ref FLIGHT_GRAPH: Mutex<FlightGraph> = Mutex::new(FlightGraph::new());
}

pub mod graph_operations {
    use std::collections::HashMap;
    use std::sync::MutexGuard;
    use std::time::SystemTime;
    use petgraph::adj::NodeIndex;
    use crate::flight_graph::{FLIGHT_GRAPH, FlightGraph, FlightWeight, IataAirline};
    use crate::ticket::SimpleTicket;
    use petgraph::algo::{dijkstra, astar};
    use petgraph::csr::EdgeIndex;
    use crate::search_request::SearchRequest;
    use crate::ticket_solution::TicketSolution;
    use petgraph::visit::{
        IntoEdges, IntoEdgesDirected, IntoNeighbors, IntoNodeIdentifiers, NodeFiltered, Reversed, Topo,
        VisitMap, Walker,
    };
    use crate::search::compose_search;

    pub fn insert_ticket(
        ticket: &SimpleTicket,
        digest_as_string: String)
        -> petgraph::graph::EdgeIndex {

        let iata_node_0: IataAirline = IataAirline {
            code: String::from(&ticket.departure_code.clone()),
        };
        let iata_node_1: IataAirline = IataAirline {
            code: String::from(&ticket.arrival_code.clone()),
        };

        let mut locked_m: MutexGuard<FlightGraph> = FLIGHT_GRAPH.lock().unwrap();
        // Add first IataNode into graph - i.e MOW
        // let iata_node_0_idx = FLIGHT_GRAPH.lock().unwrap().data.add_node(iata_node_0);
        let iata_node_0_idx = locked_m.find_node_or_create(iata_node_0).unwrap();
        // Add second IataNode into graph - i.e LED
        // let iata_node_1_idx = FLIGHT_GRAPH.lock().unwrap().data.add_node(iata_node_1);
        let iata_node_1_idx = locked_m.find_node_or_create(iata_node_1).unwrap();
        let flight_time_0_1: u32 = ticket.arrival_time - ticket.departure_time;

        let mut new_weight: FlightWeight = FlightWeight {
            flight_time: flight_time_0_1,
            flights: vec![digest_as_string],
        };

        // Add flight segment from MOW to LED as edge where edge weight stores as
        // FlightWeight struct, which consists of flight time and vec of MD5 flights
        match locked_m.data.find_edge(iata_node_0_idx, iata_node_1_idx) {
            Some(edge_index) => {
                // let graph_mutex = FLIGHT_GRAPH.lock().unwrap();
                let old_weight = locked_m.data.edge_weight(edge_index).unwrap();
                // let mut new_weight: FlightWeight = FlightWeight {
                //     flight_time: flight_time_0_1,
                //     flights: Vec::new(),
                // };
                for i in &old_weight.flights {
                    new_weight.flights.push(i.clone());
                }
                // new_weight.flights.push(digest_as_string);
                return locked_m.data.update_edge(iata_node_0_idx, iata_node_1_idx, new_weight)
            },
            None => {
                log::info!("Added new flight: {} - {}", &ticket.departure_code.clone(), &ticket.arrival_code.clone());
            },
        }

        // let mut graph_mutex = FLIGHT_GRAPH.lock().unwrap();
        locked_m.data.add_edge(
            iata_node_0_idx,
            iata_node_1_idx,
            new_weight
        )
    }

    // AStar
    pub fn a_star_search(search_request: SearchRequest) -> Vec<TicketSolution> {
        let graph_mutex = FLIGHT_GRAPH.lock().unwrap();
        let start = match graph_mutex.find_node_idx(search_request.departure_code){
            Some(node_idx) => node_idx,
            None => panic!("There is no such node"),
        };
        let destination = match graph_mutex.find_node_idx(search_request.arrival_code){
            Some(node_idx) => node_idx,
            None => panic!("There is no such node"),
        };
        let path = astar(
            &graph_mutex.data,
            start,
            |finish| finish == destination,
            |e| e.weight().flight_time,
            |_| 0
        );
        match path {
            Some(path) => compose_search(graph_mutex, path),
            None => panic!("There is no such path")
        }
    }

    // Implementation only for example, not for real world
    #[allow(dead_code)]
    pub fn shortest_path_dijkstra(search_request: SearchRequest) -> HashMap<petgraph::graph::NodeIndex, u32> {
        let graph_mutex = FLIGHT_GRAPH.lock().unwrap();
        let start = match graph_mutex.find_node_idx(search_request.departure_code){
            Some(node_idx) => node_idx,
            None => panic!("There is no such node"),
        };
        let goal = graph_mutex.find_node_idx(search_request.arrival_code);
        dijkstra(
            &graph_mutex.data,
            start,
            goal,
            |e| e.weight().flight_time
        )
    }
}