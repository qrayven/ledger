#[cfg(test)]
mod test {

    #[test]
    fn test_integration() {
        let vertices = ledger::database::load_vertices_from_database("database.txt")
            .expect("loading vertices for graph failed");
        let mut graph = ledger::graph::Graph::new(vertices);
        graph.walk_and_analyze().expect("invalid graph");

        let avg_inbound_ref_per_node = graph.calc_avg_inbound_ref_per_node();
        let avg_nodes_per_depth = graph.calc_avg_nodes_per_root_depth();
        let avg_depth_per_node = graph.calc_avg_root_depth_per_node();

        assert_eq!(avg_inbound_ref_per_node, 1.6666666666666667);
        assert_eq!(avg_nodes_per_depth, 2.5);
        assert_eq!(avg_depth_per_node, 1.3333333333333333);
    }
}
