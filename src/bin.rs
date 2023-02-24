use clap::Parser;

use crate::{cli::CliOptions, graph::Graph};
mod cli;
mod database;
mod graph;
mod vertex;

fn main() {
    let cfg = CliOptions::parse();

    let vertices = database::load_vertices_from_database(&cfg.database_file_path)
        .expect("loading vertices for graph failed");
    let mut graph = Graph::new(vertices);
    graph.walk_and_analyze().expect("invalid graph");

    let avg_inbound_ref_per_node = graph.calc_avg_inbound_ref_per_node();
    let avg_nodes_per_depth = graph.calc_avg_nodes_per_root_depth();
    let avg_depth_per_node = graph.calc_avg_root_depth_per_node();

    println!("AVG DAG DEPTH: {:.2}", avg_depth_per_node);
    println!("AVG NODES PER DEPTH:  {:.2}", avg_nodes_per_depth);
    println!("AVG REF:  {:.2}", avg_inbound_ref_per_node);
}
