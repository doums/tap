use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct NodeIndex(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct EdgeIndex(pub usize);

#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
    current_edge_index: Option<EdgeIndex>,
}

#[derive(Debug)]
pub struct Node<T> {
    first_edge: Option<EdgeIndex>,
    pub data: T,
}

impl<T> Node<T> {
    pub fn new(first_edge: Option<EdgeIndex>, data: T) -> Node<T> {
        Node { first_edge, data }
    }
}

#[derive(Debug)]
pub struct Edge {
    target: NodeIndex,
    next_edge: Option<EdgeIndex>,
}

impl Edge {
    pub fn new(target: NodeIndex, next_edge: Option<EdgeIndex>) -> Edge {
        Edge { target, next_edge }
    }
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            nodes: vec![],
            edges: vec![],
            current_edge_index: Some(EdgeIndex(0)),
        }
    }

    pub fn add_node(&mut self, data: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node::new(None, data));
        NodeIndex(index)
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let index = self.edges.len();
        let node_source = &self.nodes[source.0];
        self.edges.push(Edge::new(target, node_source.first_edge));
        self.nodes[source.0].first_edge = Some(EdgeIndex(index));
    }

    // pub fn find_node<P>(&self, predicate: P, from: Option<NodeIndex>) -> Option<NodeIndex>
    // where
    // P: FnMut(&&Node<T>) -> bool,
    // {
    // if let Some(value) = from {}
    // }

    pub fn direct_children(&self, from: Option<NodeIndex>) -> Vec<NodeIndex> {
        let mut childrens = vec![];
        if let Some(value) = from {
            let from_node = &self.nodes[value.0];
            if let Some(edge) = from_node.first_edge {
                self.iterate_edges(edge, &mut childrens);
            }
        } else {
            for (i, _) in self.nodes.iter().enumerate() {
                if let None = self.edges.iter().find(|edge| edge.target == NodeIndex(i)) {
                    childrens.push(NodeIndex(i));
                }
            }
        }
        childrens
    }

    fn iterate_edges(&self, index: EdgeIndex, childrens: &mut Vec<NodeIndex>) {
        let from_edge = &self.edges[index];
        childrens.push(from_edge.target);
        if let Some(edge) = from_edge.next_edge {
            self.iterate_edges(edge, childrens);
        }
    }
}

impl<T> Iterator for Graph<T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.edges.len() == 0 {
            return None;
        }
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges[edge_index];
                self.current_edge_index = edge.next_edge;
                Some(edge.target)
            }
        }
    }
}

impl Index<EdgeIndex> for Vec<Edge> {
    type Output = Edge;

    fn index(&self, index: EdgeIndex) -> &Self::Output {
        &self[index.0]
    }
}

impl PartialEq for NodeIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for EdgeIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
