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
    source: NodeIndex,
    target: NodeIndex,
    next_edge: Option<EdgeIndex>,
}

impl Edge {
    pub fn new(source: NodeIndex, target: NodeIndex, next_edge: Option<EdgeIndex>) -> Edge {
        Edge {
            source,
            target,
            next_edge,
        }
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
        self.edges
            .push(Edge::new(source, target, node_source.first_edge));
        self.nodes[source.0].first_edge = Some(EdgeIndex(index));
    }

    pub fn direct_children(&self, from: Option<NodeIndex>) -> Vec<NodeIndex> {
        let mut childrens = vec![];
        if let Some(value) = from {
            for edge in self.edges.iter().filter(|edge| edge.source == value) {
                childrens.push(edge.target);
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

    pub fn upstream_parents(&self, from: NodeIndex) -> Vec<NodeIndex> {
        let parents = vec![];
        self.iterate_parents(from, &mut parents);
        parents
    }

    fn iterate_parents(&self, child: NodeIndex, parents: &mut Vec<NodeIndex>) {
        for edge in self.edges.iter().filter(|edge| edge.target == child) {
            if let None = parents.iter().find(|&&index| index == edge.source) {
                parents.push(edge.source);
                self.iterate_parents(edge.source, parents);
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    struct Dummy(&'static str);

    #[test]
    fn graph_new() {
        let mut graph = Graph::<Dummy>::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
        assert_eq!(graph.next(), None);
    }

    #[test]
    fn adding_nodes() {
        let mut graph = Graph::<Dummy>::new();
        let index = graph.add_node(Dummy("one"));
        assert_eq!(index, NodeIndex(0));
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[0].data.0, "one");
        let index = graph.add_node(Dummy("two"));
        assert_eq!(index, NodeIndex(1));
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.nodes[1].data.0, "two");
    }

    #[test]
    fn adding_edges() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        graph.add_edge(one, two);
    }
}
