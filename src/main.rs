extern crate carmen;
extern crate tempdir;
extern crate structopt;
extern crate lib;

use structopt::StructOpt;
use lib::cli;

type ID = u32;
type GAV = String;

type Entry = (ID, GAV);
type Edge = (ID, ID);

struct TGF {
    root: Entry,
    entries: std::collections::HashMap<ID, GAV>,
    edges: std::collections::HashMap<ID, Vec<ID>>,
}

impl TGF {
    pub fn new(root: Entry) -> TGF {
        TGF{root:root, entries: std::collections::HashMap::new(), edges: std::collections::HashMap::new()}
    }
    pub fn addEntry(&mut self, entry: Entry) {
        self.entries.insert(entry.0, entry.1);
    }
    pub fn addEdge(&mut self, edge: Edge) {
        match self.edges.contains_key(&edge.0) {
            true => {
                self.edges.get_mut(&edge.0).unwrap().push(edge.1);
            },
            false => {
                self.edges.insert(edge.0, vec![edge.1]);
            }
        }
    }
}

fn read_tgf() -> TGF {
    let file = std::fs::read_to_string("/private/tmp/jmod/deps").unwrap();
    let tgff: Vec<&str> = file.split('#').collect();
    if tgff.len() != 2 {
        panic!("asdasd");
    }
    let coordinates = *tgff.get(0).unwrap();
    let edges = *tgff.get(1).unwrap();
    let mut lines = coordinates.lines();
    let r: Vec<&str> = lines.next().unwrap().split_ascii_whitespace().collect();
    let root: Entry = (r.get(0).unwrap().clone().parse().unwrap(), r.get(1).unwrap().to_string());
    let mut tgf = TGF::new(root);
    for coordinate in lines {
        let c: Vec<&str> = coordinate.split_ascii_whitespace().collect();
        if c.len() < 2 {
            panic!("asdd");
        }
        tgf.addEntry((c.get(0).unwrap().clone().parse().unwrap(), c.get(1).unwrap().to_string()));
    }
    let mut lines = edges.lines();
    lines.next();
    for edge in lines {
        let e: Vec<&str> = edge.split_ascii_whitespace().collect();
        if e.len() < 2 {
            panic!("srdfsadf");
        }
        tgf.addEdge((e.get(0).unwrap().parse().unwrap(), e.get(1).unwrap().parse().unwrap()));
    }
    tgf
}

fn resolve(entry: &Entry, tgf: TGF) {
    let edges = tgf.edges.get(&entry.0).unwrap();

}

fn main() {
    println!("Hello, world!");
    let tgf = read_tgf();
    cli::JMod::from_args();
}
