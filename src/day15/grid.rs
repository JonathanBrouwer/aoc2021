use std::collections::HashSet;
use std::vec::IntoIter;
use petgraph::visit::{Data, GraphBase, IntoEdgeReferences, IntoEdges, IntoNeighbors, Visitable};

#[derive(Copy, Clone)]
pub struct Grid {
    size: usize,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Grid{ size }
    }
}

impl Data for Grid {
    type NodeWeight = (usize, usize);
    type EdgeWeight = ();
}

impl GraphBase for Grid {
    type NodeId = (usize, usize);
    type EdgeId = (Self::NodeId, Self::NodeId);
}

impl<'a> IntoNeighbors for &'a Grid {
    type Neighbors = IntoIter<(usize, usize)>;

    fn neighbors(self, (x, y): (usize, usize)) -> Self::Neighbors {
        let mut vec = vec![];
        if x != 0 { vec.push((x - 1, y)) }
        if y != 0 { vec.push((x, y - 1)) }
        if x != self.size - 1 { vec.push((x + 1, y)) }
        if y != self.size - 1 { vec.push((x, y + 1)) }
        vec.into_iter()
    }
}

impl<'a> IntoEdgeReferences for &'a Grid {
    type EdgeRef = (Self::NodeId, Self::NodeId, &'a Self::EdgeWeight);
    type EdgeReferences = IntoIter<Self::EdgeRef>;

    fn edge_references(self) -> Self::EdgeReferences {
        todo!()
    }
}

impl<'a> IntoEdges for &'a Grid {
    type Edges = IntoIter<Self::EdgeRef>;

    fn edges(self, s @ (x, y): (usize, usize)) -> Self::Edges {
        let mut vec = vec![];
        if x != 0 { vec.push((s, (x-1, y), &())) }
        if y != 0 { vec.push((s, (x, y-1), &())) }
        if x != self.size - 1 { vec.push((s, (x+1, y), &())) }
        if y != self.size - 1 { vec.push((s, (x, y+1), &())) }
        vec.into_iter()
    }
}

impl Visitable for Grid {
    type Map = HashSet<Self::NodeId>;

    fn visit_map(self: &Self) -> Self::Map {
        HashSet::with_capacity(self.size * self.size)
    }

    fn reset_map(self: &Self, map: &mut Self::Map) {
        map.clear()
    }
}