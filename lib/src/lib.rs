extern crate tempfile;

pub mod cli;
pub mod mvn;
pub mod artifact;
pub mod repo;
pub mod graph;
pub mod tgf;
pub mod jboss;


#[cfg(test)]
mod tests {

    extern crate carmen;

    use super::*;
    use std::path::{Path, PathBuf};
    use std::io::Read;
    use crate::jboss::JBoss;

    #[test]
    fn integration() {
        let pom = Path::new("/Users/chris.henderson/wildFlyEval/hive_3_1_1");
        let repo = mvn::copy_dependencies(pom).unwrap();
        let tgf =  tgf::TGF::from(mvn::dependency_tree(pom).unwrap().path());
        let graph = graph::DependencyGraph::from(tgf);
        eprintln!("graph = {:#?}", graph);
    }

    #[test]
    fn walkit() {
        let pom = Path::new("/Users/chris.henderson/wildFlyEval/hive_1_2_1/pom.xml");
        let repo = mvn::copy_dependencies(pom).unwrap();
        let tgf =  tgf::TGF::from(mvn::dependency_tree(pom).unwrap().path());
        let graph = graph::DependencyGraph::from(tgf);
        let jboss = JBoss::new(Path::new("/Users/chris.henderson/wildFlyEval/wildfly-16.0.0.Final").to_path_buf());
        carmen::sequential::search(graph.root,
                                   |_, candidate| {
                                       match repo.retrieve(&candidate.artifact) {
                                           None => {
                                               eprintln!("WARNING: {} was not found", candidate.artifact.to_string());
                                               true
                                           },
                                           Some(jar) => {
                                               jboss.install(&candidate.artifact, &jar, candidate.dependencies.iter().map(|f| &f.artifact).collect());
                                               false
                                           }
                                       }
                                   },
                                   |solution| {
                                        false
                                   })
    }
}