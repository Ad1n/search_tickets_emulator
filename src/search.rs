use petgraph::graph::NodeIndex;
use crate::flight_graph::FLIGHT_GRAPH;
use crate::ticket_solution::TicketSolution;

pub fn compose_search(astar_result: (u32, Vec<NodeIndex>)) -> Vec<TicketSolution> {
    let graph_mutex = FLIGHT_GRAPH.lock().unwrap();
    let mut result: Vec<TicketSolution> = Vec::new();
    let mut flights_by_edge: Vec<Vec<String>> = Vec::new();
    let mut iter = astar_result.1.into_iter();

    while iter.size_hint() != (1, Some(1)) {
        let node0_idx = iter.next().unwrap();
        let node1_idx = iter.next().unwrap();
        let edge_idx = graph_mutex.data.find_edge(node0_idx, node1_idx).unwrap();
        let weight = graph_mutex.data.edge_weight(edge_idx).unwrap();
        flights_by_edge.push(weight.flights.clone());
    }

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