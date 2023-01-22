use std::sync::MutexGuard;
use petgraph::graph::NodeIndex;
use crate::flight_graph::{FLIGHT_GRAPH, FlightGraph};
use crate::ticket_solution::TicketSolution;

pub fn compose_search(
    graph_mutex: MutexGuard<FlightGraph>,
    astar_result: (u32, Vec<NodeIndex>)
) -> Vec<TicketSolution> {
    // let graph_mutex = FLIGHT_GRAPH.lock().unwrap();
    let mut result: Vec<TicketSolution> = Vec::new();
    // let mut flights_by_edge: Vec<Vec<String>> = Vec::new();
    // let mut iter = astar_result.1.into_iter();

    // while iter.size_hint() != (1, Some(1)) {
    //     let node0_idx = iter.next().unwrap();
    //     let node1_idx = iter.next().unwrap();
    //     let edge_idx = graph_mutex.data.find_edge(node0_idx, node1_idx).unwrap();
    //     let weight = graph_mutex.data.edge_weight(edge_idx).unwrap();
    //     flights_by_edge.push(weight.flights.clone());
    // }

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
                    println!("empty");
                    let s: String = String::from(i);
                    let new_solution = TicketSolution { 
                        ticket_ids: vec![s], 
                        price: 1 
                    };
                    result.push(new_solution)
                }
            } else {
                let temp = result.clone();
                result.clear();
                println!("any");
                for el1 in temp {
                    for el2 in el {
                        let mut temp_solution: TicketSolution = el1.clone();
                        temp_solution.ticket_ids.push(String::from(el2));
                        result.push(temp_solution);
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    result
}