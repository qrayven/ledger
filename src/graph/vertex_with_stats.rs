use crate::vertex::Vertex;
type Id = usize;

/// [`Vertex`] that is additionally equipped with the metadata and allow calculating statistics
#[derive(Debug)]
pub struct VertexWithStats {
    pub vertex: Vertex,
    pub visited: bool,
    pub inbounds: Vec<Id>,
    pub root_depth: usize,
}

impl Default for VertexWithStats {
    fn default() -> Self {
        Self {
            vertex: Default::default(),
            visited: Default::default(),
            inbounds: Default::default(),
            root_depth: usize::MAX,
        }
    }
}

impl From<Vertex> for VertexWithStats {
    fn from(vertex: Vertex) -> Self {
        VertexWithStats {
            vertex,
            visited: false,
            inbounds: Default::default(),
            root_depth: usize::MAX,
        }
    }
}
