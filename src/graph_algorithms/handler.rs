use log::info;

#[derive(Clone, Copy, Debug)]
pub struct IndexedGraphEdge {
    pub start_node: usize,
    pub end_node: usize,
    pub log_conversion_value: f64,
}

pub struct NoneIndexedGraphEdge {
    pub start_node: String,
    pub end_node: String,
    pub conversion_rate: f64,
}

pub struct Graph {
    pub edges: Vec<IndexedGraphEdge>,
    pub total_vertices: usize,
}

impl Graph {
    pub fn new(graph_edges: Vec<IndexedGraphEdge>, calculated_total_vertices: usize) -> Self {
        info!("Creating new nodes for edges");
        Self {
            edges: graph_edges,
            total_vertices: calculated_total_vertices,
        }
    }
}

pub trait SearchAllEdgesAlgorithm {
    fn search_for_arbitrage(&self, start: usize) -> ();
}
