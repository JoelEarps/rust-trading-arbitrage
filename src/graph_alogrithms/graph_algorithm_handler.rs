use log::info;


#[derive(Clone, Copy, Debug)]
pub struct IndexedGraphEdge {
    pub start_node: usize,
    pub end_node: usize,
    pub conversion_rate: f64,
    pub log_conversion_value: f64
}

pub struct NoneIndexedGraphEdge {
    pub start_node: String,
    pub end_node: String,
    pub conversion_rate: f64
}

pub struct Graph {
   pub edges: Vec<IndexedGraphEdge>,
   pub total_vertexes: usize
}

impl Graph {
    pub fn new(graph_edges: Vec<IndexedGraphEdge>, calculated_total_vertexes: usize) -> Self {
        info!("Creating new nodes for edges");
        Self {
            edges: graph_edges,
            total_vertexes: calculated_total_vertexes
        }
    }
}

pub trait SearchAllEdgesAlgorithm {
    fn search_for_arbitrage(&self) -> ();
}