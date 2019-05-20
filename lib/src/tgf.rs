use std::collections::HashMap;
use std::str::SplitAsciiWhitespace;
use crate::artifact::Artifact;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: u32,
    pub gav: Artifact
}
#[derive(Debug)]
pub struct Edge {
    pub parent: u32,
    pub child: u32
}

impl Entry {
    pub fn new(mut s: SplitAsciiWhitespace) -> Entry {
        Entry{ id: s.next().unwrap().parse().unwrap(), gav: Artifact::from_dep(s.next().unwrap().to_string()) }
    }
}

impl Edge {
    pub fn new(mut s: SplitAsciiWhitespace) -> Edge {
        Edge{ parent: s.next().unwrap().parse().unwrap(), child: s.next().unwrap().parse().unwrap() }
    }
}

#[derive(Debug)]
pub struct TGF {
    pub root: Entry,
    pub entries: std::collections::HashMap<u32, Entry>,
    pub edges: std::collections::HashMap<u32, Vec<u32>>,
}


impl TGF {

    pub fn from(graph: &Path) -> TGF {
        let f = std::fs::read_to_string(graph).unwrap();
        let mut lines = f.lines();
        let mut tgf = TGF::new(Entry::new(lines.next().unwrap().split_ascii_whitespace()));
        while let Some(line) = lines.next() {
            if line == "#" {
                break;
            }
            tgf.add_entry(Entry::new(line.split_ascii_whitespace()));
        }
        while let Some(line) = lines.next() {
            if line == "" {
                continue;
            }
            tgf.add_edge(Edge::new(line.split_ascii_whitespace()));
        }
        tgf
    }

    pub fn new(root: Entry) -> TGF {
        TGF{root:root, entries: HashMap::new(), edges: HashMap::new()}
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.insert(entry.id, entry);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        match self.edges.contains_key(&edge.parent) {
            true => {
                self.edges.get_mut(&edge.parent).unwrap().push(edge.child);
            },
            false => {
                self.edges.insert(edge.parent, vec![edge.child]);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn thg() {
//        let s = std::fs::read_to_string("/Users/chris.henderson/jmod/deps").unwrap();
//        let tgf = TGF::from(s);
//        eprintln!("tgf = {:#?}", tgf);
    }
}