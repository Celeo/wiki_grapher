use anyhow::Result;
use csv::Reader;
use log::{debug, error, info};
use petgraph::{
    algo::astar,
    graph::{DiGraph, Graph, NodeIndex},
};
use std::{collections::HashMap, env, fs::File};

fn load_to_graph() -> DiGraph<String, ()> {
    debug!("Loading data from CSV file and into graph");
    let mut graph = DiGraph::default();

    let file_ref = File::open("links.csv").expect("Could not open 'links.csv'");
    let mut reader = Reader::from_reader(file_ref);
    let mut added: HashMap<String, NodeIndex> = HashMap::new();

    for line in reader.records() {
        let line = line.unwrap();
        let from_str = line.get(0).unwrap().to_owned();
        let to_str = line.get(1).unwrap().to_owned();
        let from_index = match added.get(&from_str) {
            Some(i) => i.to_owned(),
            None => {
                let i = graph.add_node(from_str.clone());
                added.insert(from_str, i);
                i
            }
        };
        let to_index = match added.get(&to_str) {
            Some(i) => i.to_owned(),
            None => {
                let i = graph.add_node(to_str.clone());
                added.insert(to_str, i);
                i
            }
        };
        graph.add_edge(from_index, to_index, ());
    }

    debug!("Graph loaded");
    graph
}

fn lookup_index(graph: &Graph<String, ()>, value: &str) -> Option<NodeIndex> {
    for id in graph.node_indices() {
        if let Some(weight) = graph.node_weight(id) {
            if weight == value {
                return Some(id);
            }
        }
    }
    None
}

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    let graph = load_to_graph();

    info!(
        "There are {} nodes and {} edges",
        graph.node_count(),
        graph.edge_count()
    );

    let start = lookup_index(&graph, "A").unwrap();
    let end = lookup_index(&graph, "Unicode").unwrap();

    let path = astar(&graph, start, |finish| finish == end, |_| 1, |_| 0);
    match path {
        Some((cost, steps)) => {
            info!("Path took {} steps", cost);
            for index in steps {
                match graph.node_weight(index) {
                    Some(w) => {
                        info!("Step: {}", w);
                    }
                    None => {
                        error!("Could not find weight for index: {:?}", index);
                    }
                };
            }
        }
        None => {
            error!("No path could be found");
        }
    }

    Ok(())
}
