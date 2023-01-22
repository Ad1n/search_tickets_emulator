use std::sync::MutexGuard;
use petgraph::graph::NodeIndex;
use crate::flight_graph::{FlightGraph};
use crate::search_response::SearchResponse;
use crate::ticket_solution::{count_price, TicketSolution};

pub fn compose_search(
    graph_mutex: MutexGuard<FlightGraph>,
    astar_result: (u32, Vec<NodeIndex>)
) -> SearchResponse {
    let mut result: Vec<TicketSolution> = Vec::new();

    // Iterate over vector by 2 elems
    let flights_by_edge: Vec<Vec<String>> = astar_result.1.windows(2)
        .map(|pair_of_node_idxs| {
            let edge_idx = graph_mutex.data.find_edge(pair_of_node_idxs[0], pair_of_node_idxs[1]).unwrap();
            let weight = graph_mutex.data.edge_weight(edge_idx).unwrap();
            weight.flights.clone()
        })
        .collect::<Vec<_>>();

    let _just_iter = flights_by_edge.iter()
        .map(|el| {
            if result.is_empty() {
                for i in el {
                    let s: String = String::from(i);
                    let ticket_ids: Vec<String> = vec![s];
                    let new_solution = TicketSolution { 
                        ticket_ids: ticket_ids.clone(),
                        price: count_price(&ticket_ids)
                    };
                    result.push(new_solution)
                }
            } else {
                let temp = result.clone();
                result.clear();
                for el1 in temp {
                    for el2 in el {
                        let mut temp_solution: TicketSolution = el1.clone();
                        temp_solution.ticket_ids.push(String::from(el2));
                        temp_solution.recount_price();
                        result.push(temp_solution);
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    SearchResponse { solutions: result }
}