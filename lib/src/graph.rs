use crate::artifact::Artifact;
use crate::tgf::TGF;
use super::tgf::Entry;

#[derive(Debug)]
pub struct DependencyGraph {
    pub root: Node
}

impl DependencyGraph {
    pub fn from(mut tgf: TGF) -> DependencyGraph {
        let root = Node::new(tgf.root.clone(), &mut tgf);
        DependencyGraph{root}
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub artifact: Artifact,
    pub dependencies: Vec<Node>,
}

impl Node {

    fn new(entry: Entry, tgf: &mut TGF) -> Node {
        let mut children = vec![];
        match tgf.edges.remove(&entry.id) {
            Some(edges) => {
                edges.iter().for_each(|child| {
                    children.push(Node::new(tgf.entries.remove(child).unwrap(), tgf));
                });
            },
            None => ()
        }
        Node{ artifact: entry.gav, dependencies: children }
    }
}

impl Iterator for Node {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dependencies.len() > 0 {
            true => Some(self.dependencies.remove(0)),
            false => None
        }
    }
}