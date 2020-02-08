use std::ops::Index;

#[derive(Debug, Copy, Clone)]
pub struct NodeIndex(pub usize);

#[derive(Debug, Copy, Clone)]
pub struct EdgeIndex(pub usize);

#[derive(Debug)]
pub struct Graph<T> {
    pub nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
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
        }
    }

    pub fn add_node(&mut self, data: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node::new(None, data));
        NodeIndex(index)
    }

    pub fn add_node_to(&mut self, to: NodeIndex, data: T) -> NodeIndex {
        let index = self.add_node(data);
        self.add_edge(to, index);
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        if self.nodes.len() < 2
            || source == target
            || source.0 >= self.nodes.len()
            || target.0 >= self.nodes.len()
        {
            panic!("invalid edge");
        }
        if let Some(_) = self
            .edges
            .iter()
            .find(|edge| edge.source == source && edge.target == target)
        {
            panic!("invalid edge");
        }
        let index = self.edges.len();
        let node_source = &self.nodes[source];
        self.edges
            .push(Edge::new(source, target, node_source.first_edge));
        self.nodes[source.0].first_edge = Some(EdgeIndex(index));
    }

    pub fn roots(&self) -> Roots {
        Roots::new(self)
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_edge;
        Successors::new(&self.edges, first_outgoing_edge)
    }

    pub fn ancestors(&self, source: NodeIndex) -> Ancestors {
        Ancestors::new(&self, source)
    }
}

pub struct Successors<'a> {
    edges: &'a Vec<Edge>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'a> Successors<'a> {
    fn new(edges: &'a Vec<Edge>, index: Option<EdgeIndex>) -> Self {
        Successors {
            edges,
            current_edge_index: index,
        }
    }
}

impl Iterator for Successors<'_> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.edges.is_empty() == true {
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

pub struct Ancestors {
    data: Vec<NodeIndex>,
    current_index: usize,
}

impl Ancestors {
    fn new<T>(graph: &Graph<T>, from: NodeIndex) -> Self {
        if from.0 >= graph.nodes.len() {
            panic!("invalid index");
        }
        let mut data = vec![];
        for edge in graph.edges.iter().filter(|edge| edge.target == from) {
            if let None = data.iter().find(|&&index| index == edge.source) {
                if edge.source != from {
                    data.push(edge.source);
                }
            }
        }
        Ancestors {
            data,
            current_index: 0,
        }
    }
}

impl Iterator for Ancestors {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() == true {
            return None;
        }
        if let Some(value) = self.data.get(self.current_index) {
            self.current_index += 1;
            Some(*value)
        } else {
            None
        }
    }
}

pub struct Roots {
    data: Vec<NodeIndex>,
    current_index: usize,
}

impl Roots {
    fn new<T>(graph: &Graph<T>) -> Roots {
        let mut data = vec![];
        for (i, _) in graph.nodes.iter().enumerate() {
            if let None = graph.edges.iter().find(|edge| edge.target == NodeIndex(i)) {
                data.push(NodeIndex(i));
            }
        }
        Roots {
            data,
            current_index: 0,
        }
    }
}

impl Iterator for Roots {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() == true {
            return None;
        }
        if let Some(value) = self.data.get(self.current_index) {
            self.current_index += 1;
            Some(*value)
        } else {
            None
        }
    }
}

impl<T> Index<NodeIndex> for Vec<Node<T>> {
    type Output = Node<T>;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self[index.0]
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
    fn node_new() {
        let node = Node::<Dummy>::new(None, Dummy("test"));
        assert_eq!(node.first_edge, None);
        assert_eq!(node.data.0, "test");
        let node = Node::<Dummy>::new(Some(EdgeIndex(42)), Dummy("test"));
        assert_eq!(node.first_edge, Some(EdgeIndex(42)));
        assert_eq!(node.data.0, "test");
    }

    #[test]
    fn edge_new() {
        let edge = Edge::new(NodeIndex(0), NodeIndex(1), None);
        assert_eq!(edge.source, NodeIndex(0));
        assert_eq!(edge.target, NodeIndex(1));
        assert_eq!(edge.next_edge, None);
        let edge = Edge::new(NodeIndex(0), NodeIndex(1), Some(EdgeIndex(42)));
        assert_eq!(edge.source, NodeIndex(0));
        assert_eq!(edge.target, NodeIndex(1));
        assert_eq!(edge.next_edge, Some(EdgeIndex(42)));
    }

    #[test]
    fn graph_new() {
        let graph = Graph::<Dummy>::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn successors_new() {
        let graph = Graph::<Dummy>::new();
        let successors = Successors::new(&graph.edges, None);
        assert_eq!(successors.current_edge_index, None);
        let successors = Successors::new(&graph.edges, Some(EdgeIndex(42)));
        assert_eq!(successors.current_edge_index, Some(EdgeIndex(42)));
    }

    #[test]
    fn adding_nodes() {
        let mut graph = Graph::<Dummy>::new();
        let index = graph.add_node(Dummy("one"));
        assert_eq!(index, NodeIndex(0));
        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.nodes[index].data.0, "one");
        assert_eq!(graph.nodes[index].first_edge, None);
        let index = graph.add_node(Dummy("two"));
        assert_eq!(index, NodeIndex(1));
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.nodes[index].data.0, "two");
        assert_eq!(graph.nodes[index].first_edge, None);
    }

    #[test]
    #[should_panic]
    fn add_node_to_empty_graph() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_node_to(NodeIndex(0), Dummy("test"));
    }

    #[test]
    #[should_panic]
    fn add_node_to_invalid_index() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_node(Dummy("one"));
        graph.add_node_to(NodeIndex(1), Dummy("two"));
    }

    #[test]
    #[should_panic]
    fn add_node_to_invalid_index_2() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_node(Dummy("one"));
        graph.add_node_to(NodeIndex(42), Dummy("two"));
    }

    #[test]
    fn adding_nodes_to() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node_to(one, Dummy("two"));
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.nodes[one].data.0, "one");
        assert_eq!(graph.nodes[two].data.0, "two");
        assert_eq!(graph.nodes[one].first_edge, Some(EdgeIndex(0)));
        assert_eq!(graph.nodes[two].first_edge, None);
        assert_eq!(graph.edges[0].source, one);
        assert_eq!(graph.edges[0].target, two);
        assert_eq!(graph.edges[0].next_edge, None);
        let three = graph.add_node_to(one, Dummy("three"));
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 2);
        assert_eq!(graph.nodes[one].data.0, "one");
        assert_eq!(graph.nodes[two].data.0, "two");
        assert_eq!(graph.nodes[three].data.0, "three");
        assert_eq!(graph.nodes[one].first_edge, Some(EdgeIndex(1)));
        assert_eq!(graph.nodes[two].first_edge, None);
        assert_eq!(graph.nodes[three].first_edge, None);
        assert_eq!(graph.edges[0].source, one);
        assert_eq!(graph.edges[0].target, two);
        assert_eq!(graph.edges[0].next_edge, None);
        assert_eq!(graph.edges[1].source, one);
        assert_eq!(graph.edges[1].target, three);
        assert_eq!(graph.edges[1].next_edge, Some(EdgeIndex(0)));
    }

    #[test]
    #[should_panic]
    fn add_edge_on_empty_graph() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_edge(NodeIndex(0), NodeIndex(1));
    }

    #[test]
    #[should_panic]
    fn add_edge_on_graph_with_one_node() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        graph.add_edge(one, NodeIndex(1));
    }

    #[test]
    #[should_panic]
    fn add_edge_with_two_equal_indexes() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_node(Dummy("one"));
        graph.add_node(Dummy("two"));
        graph.add_edge(NodeIndex(0), NodeIndex(0));
    }

    #[test]
    #[should_panic]
    fn add_edge_with_invalid_index() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_node(Dummy("one"));
        graph.add_node(Dummy("two"));
        graph.add_edge(NodeIndex(0), NodeIndex(2));
    }

    #[test]
    #[should_panic]
    fn add_same_edge_twice() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        graph.add_edge(one, two);
        assert_eq!(graph.edges.len(), 1);
        graph.add_edge(one, two);
    }

    #[test]
    fn adding_edges() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        graph.add_edge(one, two);
        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.edges[0].source, one);
        assert_eq!(graph.edges[0].target, two);
        assert_eq!(graph.edges[0].next_edge, None);
        assert_eq!(graph.nodes[one].first_edge, Some(EdgeIndex(0)));
        let three = graph.add_node(Dummy("three"));
        graph.add_edge(one, three);
        assert_eq!(graph.edges.len(), 2);
        assert_eq!(graph.edges[0].source, one);
        assert_eq!(graph.edges[0].target, two);
        assert_eq!(graph.edges[0].next_edge, None);
        assert_eq!(graph.nodes[one].first_edge, Some(EdgeIndex(1)));
        assert_eq!(graph.edges[1].source, one);
        assert_eq!(graph.edges[1].target, three);
        assert_eq!(graph.edges[1].next_edge, Some(EdgeIndex(0)));
    }

    #[test]
    fn successors() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        let three = graph.add_node(Dummy("three"));
        let four = graph.add_node_to(one, Dummy("four"));
        let five = graph.add_node_to(four, Dummy("five"));
        assert_eq!(graph.nodes.len(), 5);
        assert_eq!(graph.edges.len(), 2);
        let mut successors = graph.successors(one);
        assert_eq!(successors.next(), Some(four));
        assert_eq!(successors.next(), None);
        let mut successors = graph.successors(four);
        assert_eq!(successors.next(), Some(five));
        assert_eq!(successors.next(), None);
        let successors = graph.successors(two);
        assert_eq!(successors.count(), 0);
        let successors = graph.successors(three);
        assert_eq!(successors.count(), 0);
    }

    #[test]
    fn successors_on_circular_graph() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let child = graph.add_node_to(one, Dummy("child"));
        graph.add_edge(child, one);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);
        let mut successors = graph.successors(one);
        assert_eq!(successors.next(), Some(child));
        assert_eq!(successors.next(), None);
        let mut successors = graph.successors(child);
        assert_eq!(successors.next(), Some(one));
        assert_eq!(successors.next(), None);
    }

    #[test]
    fn roots() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        let three = graph.add_node(Dummy("three"));
        let four = graph.add_node_to(one, Dummy("four"));
        graph.add_node_to(four, Dummy("five"));
        assert_eq!(graph.nodes.len(), 5);
        assert_eq!(graph.edges.len(), 2);
        let mut roots = graph.roots();
        assert_eq!(roots.next(), Some(one));
        assert_eq!(roots.next(), Some(two));
        assert_eq!(roots.next(), Some(three));
        assert_eq!(roots.next(), None);
    }

    #[test]
    fn no_roots_on_circular_graph() {
        let mut graph = Graph::<Dummy>::new();
        let first = graph.add_node(Dummy("first"));
        let second = graph.add_node_to(first, Dummy("second"));
        graph.add_edge(second, first);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);
        let roots = graph.roots();
        assert_eq!(roots.count(), 0);
    }

    #[test]
    fn no_roots_on_empty_graph() {
        let graph = Graph::<Dummy>::new();
        let roots = graph.roots();
        assert_eq!(roots.count(), 0);
    }

    #[test]
    #[should_panic]
    fn ancestors_on_empty_graph() {
        let graph = Graph::<Dummy>::new();
        graph.ancestors(NodeIndex(0));
    }

    #[test]
    #[should_panic]
    fn ancestors_invalid_index() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_node(Dummy("test"));
        graph.ancestors(NodeIndex(1));
    }

    #[test]
    fn ancestors() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        let three = graph.add_node(Dummy("three"));
        let four = graph.add_node_to(one, Dummy("four"));
        graph.add_edge(two, four);
        graph.add_edge(three, four);
        assert_eq!(graph.nodes.len(), 4);
        assert_eq!(graph.edges.len(), 3);
        let mut ancestors = graph.ancestors(four);
        assert_eq!(ancestors.next(), Some(one));
        assert_eq!(ancestors.next(), Some(two));
        assert_eq!(ancestors.next(), Some(three));
        assert_eq!(ancestors.next(), None);
        let ancestors = graph.ancestors(one);
        assert_eq!(ancestors.count(), 0);
        let ancestors = graph.ancestors(two);
        assert_eq!(ancestors.count(), 0);
        let ancestors = graph.ancestors(three);
        assert_eq!(ancestors.count(), 0);
    }

    #[test]
    fn ancestors_on_circular_graph() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let child = graph.add_node_to(one, Dummy("child"));
        graph.add_edge(child, one);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);
        let mut ancestors = graph.ancestors(child);
        assert_eq!(ancestors.next(), Some(one));
        assert_eq!(ancestors.next(), None);
        let mut ancestors = graph.ancestors(one);
        assert_eq!(ancestors.next(), Some(child));
        assert_eq!(ancestors.next(), None);
    }
}
