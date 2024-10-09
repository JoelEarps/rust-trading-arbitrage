// Given size of float values used 64 here, could use 128 if you wanted double precision
#[derive(Debug, PartialEq)]
pub struct GraphEdge {
    pub start_node: String,
    pub end_node: String,
    pub conversion_rate: f64
}

pub struct BellmanFord {
   pub edges: Vec<GraphEdge>
}

impl BellmanFord {
    pub fn new(graph_edges: Vec<GraphEdge>) -> Self {
        println!("Creating new nodes for edges");
        Self {
            edges: graph_edges
        }
    }
}


// Turn this into generic graph algorithm with general traits for future application

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_creation_of_graph() {
        // Convert this vector to a common input - as it is used elsewhere
        let test_graph_edges = vec![
        GraphEdge { start_node: "BORG".to_string(), end_node: "DAI".to_string(), conversion_rate: 6.10395983 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "BTC".to_string(), conversion_rate: 57157.97132034 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "BORG".to_string(), conversion_rate: 0.14846482 },
        GraphEdge { start_node: "BORG".to_string(), end_node: "BTC".to_string(), conversion_rate: 370331.49347896 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "BORG".to_string(), conversion_rate: 0.16276449 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "BTC".to_string(), conversion_rate: 61194.73626107 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "DAI".to_string(), conversion_rate: 1.586e-5 },
        GraphEdge { start_node: "DAI".to_string(), end_node: "EUR".to_string(), conversion_rate: 1.10578631 },
        GraphEdge { start_node: "BORG".to_string(), end_node: "EUR".to_string(), conversion_rate: 6.5350502 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "BORG".to_string(), conversion_rate: 2.69e-6 },
        GraphEdge { start_node: "EUR".to_string(), end_node: "DAI".to_string(), conversion_rate: 0.89846283 },
        GraphEdge { start_node: "BTC".to_string(), end_node: "EUR".to_string(), conversion_rate: 1.739e-5 },
        ];

        let test_bellman = BellmanFord::new(test_graph_edges);
    }
}