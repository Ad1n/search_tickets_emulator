use petgraph::{Graph, Undirected};
use std::sync::Mutex;

// Node
pub struct IataAirline {
    code: String,
}

// Edge weight
pub struct FlightWeight {
    flight_time: u32,
    flights: Vec<String>,
}

pub struct FlightGraph {
    data: Graph<IataAirline, FlightWeight, Undirected>
}

impl FlightGraph {
    pub fn new() -> FlightGraph {
        FlightGraph {
            data: Graph::new_undirected(),
        }
    }
}

lazy_static! {
    pub static ref FLIGHT_GRAPH: Mutex<FlightGraph> = Mutex::new(FlightGraph::new());
}

pub mod graph_operations {
    use crate::flight_graph::{FLIGHT_GRAPH, FlightGraph, FlightWeight, IataAirline};
    use crate::ticket::SimpleTicket;
    use petgraph::algo::dijkstra;
    use petgraph::csr::EdgeIndex;

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
        // Add first IataNode into graph - i.e MOW
        let iata_node_0_idx = FLIGHT_GRAPH.lock().unwrap().data.add_node(iata_node_0);
        // Add second IataNode into graph - i.e LED
        let iata_node_1_idx = FLIGHT_GRAPH.lock().unwrap().data.add_node(iata_node_1);

        let flight_time_0_1: u32 = ticket.arrival_time - ticket.departure_time;

        let mut new_weight: FlightWeight = FlightWeight {
            flight_time: flight_time_0_1,
            flights: vec![digest_as_string],
        };

        // Add flight segment from MOW to LED as edge where edge weight stores as
        // FlightWeight struct, which consists of flight time and vec of MD5 flights
        match FLIGHT_GRAPH.lock().unwrap().data.find_edge(iata_node_0_idx, iata_node_1_idx) {
            Some(edge_index) => {
                let graph_mutex = FLIGHT_GRAPH.lock().unwrap();
                let old_weight = graph_mutex.data.edge_weight(edge_index).unwrap();
                // let mut new_weight: FlightWeight = FlightWeight {
                //     flight_time: flight_time_0_1,
                //     flights: Vec::new(),
                // };
                for i in &old_weight.flights {
                    new_weight.flights.push(i.clone());
                }
                // new_weight.flights.push(digest_as_string);
                return FLIGHT_GRAPH.lock().unwrap().data.update_edge(iata_node_0_idx, iata_node_1_idx, new_weight)
            },
            None => {
                log::info!("Added new flight: {} - {}", &ticket.departure_code.clone(), &ticket.arrival_code.clone());
            },
        }

        let mut graph_mutex = FLIGHT_GRAPH.lock().unwrap();
        graph_mutex.data.add_edge(
            iata_node_0_idx,
            iata_node_1_idx,
            new_weight
        )
    }
}