use anyhow::bail;
use log::trace;
use std::collections::VecDeque;

use crate::vertex::Vertex;
mod vertex_with_stats;
use vertex_with_stats::VertexWithStats;

type Id = usize;
type PathLength = usize;

// Graph is a loosely coupled abstraction over the functions that returns statistical data about the graph
pub struct Graph {
    pub graph: Vec<VertexWithStats>,
}

impl Graph {
    /// Creates a new graph from vertices
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Graph {
            graph: vertices.into_iter().map(VertexWithStats::from).collect(),
        }
    }

    /// Performs statistical analysis on the graph
    pub fn walk_and_analyze(&mut self) -> Result<(), anyhow::Error> {
        find_inward_references(&mut self.graph)?;
        find_root_depth(&mut self.graph)?;
        Ok(())
    }

    /// Calculates the avg number of inbound references per node
    pub fn calc_avg_inbound_ref_per_node(&self) -> f64 {
        calc_avg_inbound_ref_per_node(&self.graph)
    }

    /// Calculates the avg number of nodes per root depth
    pub fn calc_avg_nodes_per_root_depth(&self) -> f64 {
        calc_avg_nodes_per_root_depth(&self.graph)
    }

    /// Calculates the avg root depth per node
    pub fn calc_avg_root_depth_per_node(&self) -> f64 {
        calc_avg_root_depth_per_node(&self.graph)
    }
}

fn calc_avg_inbound_ref_per_node(graph: &[VertexWithStats]) -> f64 {
    graph
        .iter()
        .map(|vertex| vertex.inbounds.len())
        .sum::<usize>() as f64
        / graph.len() as f64
}

fn calc_avg_root_depth_per_node(graph: &[VertexWithStats]) -> f64 {
    graph.iter().map(|vertex| vertex.root_depth).sum::<usize>() as f64 / graph.len() as f64
}

fn calc_avg_nodes_per_root_depth(graph: &[VertexWithStats]) -> f64 {
    let mut depths_cnt = vec![0; graph.len()];

    for vertex in graph {
        depths_cnt[vertex.root_depth] += 1
    }

    let mut sum: usize = 0;
    let mut cnt: usize = 0;
    // the depth of size 0 is skipped according to the requirements
    for d in depths_cnt.into_iter().skip(1).filter(|depth| *depth > 0) {
        sum += d;
        cnt += 1;
    }

    sum as f64 / cnt as f64
}

/// finds the inward references for all vertices in graph
pub fn find_inward_references(graph: &mut [VertexWithStats]) -> Result<(), anyhow::Error> {
    if graph.is_empty() {
        bail!("the graph cannot be empty")
    }
    let max_id = graph.len();

    for i in 0..graph.len() {
        let current_id = i + 1;

        if let Some(ref id) = graph[i].vertex.left {
            check_valid_id(*id, max_id)?;

            let left_idx = id - 1;
            graph[left_idx].inbounds.push(current_id)
        }

        if let Some(ref id) = graph[i].vertex.right {
            check_valid_id(*id, max_id)?;

            let right_idx = id - 1;
            graph[right_idx].inbounds.push(current_id)
        }
    }

    Ok(())
}

/// finds the depth (the shortest path to the root) for each vertex. **Before finding
/// the root depth you must find find the inward references**.
pub fn find_root_depth(graph: &mut [VertexWithStats]) -> Result<(), anyhow::Error> {
    if graph.is_empty() {
        bail!("the graph cannot be empty")
    }
    let max_id = graph.len();
    let mut queue: VecDeque<(PathLength, Id)> = VecDeque::new();
    queue.push_front((0, 1));

    while let Some((path_len, vertex_id)) = queue.pop_front() {
        check_valid_id(vertex_id, max_id)?;

        let vertex_idx = vertex_id - 1;
        if graph[vertex_idx].visited {
            continue;
        }

        trace!("visiting vertex id: {vertex_id}");
        graph[vertex_idx].visited = true;
        if path_len < graph[vertex_idx].root_depth {
            graph[vertex_idx].root_depth = path_len
        }

        queue.extend(
            graph[vertex_idx]
                .inbounds
                .iter()
                .map(|id| (path_len + 1, *id)),
        );
        trace!("left to visit: {:?}", queue);
    }
    Ok(())
}

#[inline]
fn check_valid_id(id: usize, max_id: usize) -> Result<(), anyhow::Error> {
    if id > max_id {
        bail!("vertex with ID {id} doesn't exist. Max number is {max_id}",)
    }
    if id == 0 {
        bail!("the graph cannot have the vertex with ID 0. The minimum is 1")
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::vertex::Vertex;

    use super::{find_inward_references, find_root_depth, vertex_with_stats::VertexWithStats};

    #[test]
    fn test_find_inward_references() {
        let mut graph = vec![
            VertexWithStats {
                ..Default::default()
            },
            VertexWithStats {
                vertex: Vertex {
                    left: Some(1),
                    right: None,
                    ..Default::default()
                },
                ..Default::default()
            },
        ];
        find_inward_references(&mut graph).expect("shouldn't return error");
        assert_eq!(vec![2], graph[0].inbounds);
    }

    #[test]
    fn test_find_inward_references_invalid_id() {
        let mut graph = vec![
            VertexWithStats {
                ..Default::default()
            },
            VertexWithStats {
                vertex: Vertex {
                    left: Some(3),
                    right: None,
                    ..Default::default()
                },
                ..Default::default()
            },
        ];
        let err = find_inward_references(&mut graph).expect_err("should return error");
        assert!(
            err.to_string().contains("vertex with ID 3 doesn't exist"),
            "{err}"
        )
    }

    #[test]
    fn test_find_inward_references_empty_graph() {
        let mut graph: Vec<VertexWithStats> = vec![];
        let err = find_inward_references(&mut graph).expect_err("should return error");
        assert!(
            err.to_string().contains("the graph cannot be empty"),
            "{err}"
        )
    }

    #[test]
    fn test_find_root_depth() {
        let mut graph = vec![
            VertexWithStats {
                inbounds: vec![2, 3],
                ..Default::default()
            },
            VertexWithStats {
                ..Default::default()
            },
            VertexWithStats {
                inbounds: vec![4],
                ..Default::default()
            },
            VertexWithStats {
                ..Default::default()
            },
        ];
        find_root_depth(&mut graph).expect("shouldn't return error");
        assert!(graph[0].visited);
        assert!(graph[1].visited);
        assert!(graph[2].visited);
        assert!(graph[3].visited);

        assert_eq!(graph[0].root_depth, 0);
        assert_eq!(graph[1].root_depth, 1);
        assert_eq!(graph[2].root_depth, 1);
        assert_eq!(graph[3].root_depth, 2);
    }

    #[test]
    fn test_find_root_depth_invalid_vertex_id() {
        let mut graph = vec![
            VertexWithStats {
                inbounds: vec![3],
                ..Default::default()
            },
            VertexWithStats {
                ..Default::default()
            },
        ];
        let err = find_root_depth(&mut graph).expect_err("should no return error");
        assert!(
            err.to_string().contains("vertex with ID 3 doesn't exist"),
            "{err}"
        )
    }

    #[test]
    fn test_find_root_depth_invalid_vertex_id_is_zero() {
        let mut graph = vec![
            VertexWithStats {
                inbounds: vec![0],
                ..Default::default()
            },
            VertexWithStats {
                ..Default::default()
            },
        ];
        let err = find_root_depth(&mut graph).expect_err("should no return error");
        assert!(
            err.to_string()
                .contains("the graph cannot have the vertex with ID 0"),
            "{err}"
        )
    }

    #[test]
    fn test_find_root_depth_empty_graph() {
        let mut graph = vec![];
        let err = find_root_depth(&mut graph).expect_err("should no return error");
        assert!(
            err.to_string().contains("the graph cannot be empty"),
            "{err}"
        )
    }
}
