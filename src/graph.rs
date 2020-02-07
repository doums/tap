use std::cmp::Ordering;
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
        if from.0 >= self.nodes.len() {
            panic!("invalid index");
        }
        let mut parents = vec![];
        self.iterate_parents(from, &mut parents, from);
        parents
    }

    fn iterate_parents(&self, child: NodeIndex, parents: &mut Vec<NodeIndex>, from: NodeIndex) {
        for edge in self.edges.iter().filter(|edge| edge.target == child) {
            if let None = parents.iter().find(|&&index| index == edge.source) {
                if edge.source != from {
                    parents.push(edge.source);
                    self.iterate_parents(edge.source, parents, from);
                }
            }
        }
    }
}

pub struct Children<'a, T> {
    graph: &'a Graph<T>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'a, T> Children<'a, T> {
    fn new(graph: &'a Graph<T>, from: Option<EdgeIndex>) -> Self {
        Children {
            graph,
            current_edge_index: from,
        }
    }
}

impl<T> Iterator for Children<'_, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.graph.edges.len() == 0 {
            return None;
        }
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                self.current_edge_index = edge.next_edge;
                Some(edge.target)
            }
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
        let mut graph = Graph::<Dummy>::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.edges.len(), 0);
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
    fn direct_children() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        let three = graph.add_node(Dummy("three"));
        let four = graph.add_node_to(one, Dummy("four"));
        let five = graph.add_node_to(four, Dummy("five"));
        assert_eq!(graph.nodes.len(), 5);
        assert_eq!(graph.edges.len(), 2);
        let children = graph.direct_children(None);
        assert_eq!(children.len(), 3);
        assert_eq!(children.iter().eq([one, two, three].iter()), true);
        let children = graph.direct_children(Some(one));
        assert_eq!(children.len(), 1);
        assert_eq!(children.iter().eq([four].iter()), true);
        let children = graph.direct_children(Some(four));
        assert_eq!(children.len(), 1);
        assert_eq!(children.iter().eq([five].iter()), true);
        let children = graph.direct_children(Some(two));
        assert_eq!(children.is_empty(), true);
        let children = graph.direct_children(Some(three));
        assert_eq!(children.is_empty(), true);
    }

    #[test]
    fn direct_children_on_circular_graph() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let child = graph.add_node_to(one, Dummy("child"));
        graph.add_edge(child, one);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);
        let children = graph.direct_children(None);
        assert_eq!(children.is_empty(), true);
        let children = graph.direct_children(Some(one));
        assert_eq!(children.len(), 1);
        assert_eq!(children.iter().eq([child].iter()), true);
        let children = graph.direct_children(Some(child));
        assert_eq!(children.len(), 1);
        assert_eq!(children.iter().eq([one].iter()), true);
    }

    #[test]
    #[should_panic]
    fn upstream_parents_on_empty_graph() {
        let graph = Graph::<Dummy>::new();
        graph.upstream_parents(NodeIndex(0));
    }

    #[test]
    #[should_panic]
    fn upstream_parents_invalid_index() {
        let mut graph = Graph::<Dummy>::new();
        graph.add_node(Dummy("test"));
        graph.upstream_parents(NodeIndex(1));
    }

    #[test]
    fn upstream_parents() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        let three = graph.add_node(Dummy("three"));
        let four = graph.add_node_to(one, Dummy("four"));
        let five = graph.add_node_to(four, Dummy("five"));
        assert_eq!(graph.nodes.len(), 5);
        assert_eq!(graph.edges.len(), 2);
        let parents = graph.upstream_parents(five);
        assert_eq!(parents.iter().eq([four, one].iter()), true);
        let parents = graph.upstream_parents(four);
        assert_eq!(parents.iter().eq([one].iter()), true);
        let parents = graph.upstream_parents(one);
        assert_eq!(parents.is_empty(), true);
        let parents = graph.upstream_parents(two);
        assert_eq!(parents.is_empty(), true);
        let parents = graph.upstream_parents(three);
        assert_eq!(parents.is_empty(), true);
    }

    #[test]
    fn upstream_several_parents_for_one_child() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        let three = graph.add_node(Dummy("three"));
        let child = graph.add_node(Dummy("child"));
        graph.add_edge(one, child);
        graph.add_edge(two, child);
        graph.add_edge(three, child);
        assert_eq!(graph.nodes.len(), 4);
        assert_eq!(graph.edges.len(), 3);
        let parents = graph.upstream_parents(child);
        assert_eq!(parents.iter().eq([one, two, three].iter()), true);
    }

    #[test]
    fn upstream_complex_case() {
        let mut graph = Graph::<Dummy>::new();
        let root_one = graph.add_node(Dummy("root_one"));
        let root_two = graph.add_node(Dummy("root_two"));
        let one = graph.add_node(Dummy("one"));
        let two = graph.add_node(Dummy("two"));
        let three = graph.add_node(Dummy("three"));
        let child = graph.add_node(Dummy("child"));
        graph.add_edge(root_one, one);
        graph.add_edge(root_one, two);
        graph.add_edge(root_two, three);
        graph.add_edge(one, child);
        graph.add_edge(two, child);
        graph.add_edge(three, child);
        assert_eq!(graph.nodes.len(), 6);
        assert_eq!(graph.edges.len(), 6);
        let parents = graph.upstream_parents(child);
        assert_eq!(parents.len(), 5);
        assert_eq!(
            parents
                .iter()
                .eq([one, root_one, two, three, root_two].iter()),
            true
        );
    }

    #[test]
    fn upstream_parents_on_circular_graph() {
        let mut graph = Graph::<Dummy>::new();
        let one = graph.add_node(Dummy("one"));
        let child = graph.add_node_to(one, Dummy("child"));
        graph.add_edge(child, one);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2);
        let parents = graph.upstream_parents(child);
        assert!(parents.iter().eq([one].iter()), true);
        let parents = graph.upstream_parents(one);
        assert!(parents.iter().eq([child].iter()), true);
    }
}
