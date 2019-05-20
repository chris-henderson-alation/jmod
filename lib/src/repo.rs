use std::path::{PathBuf, Path};
use crate::artifact::Artifact;
use tempdir::TempDir;

#[derive(Debug)]
pub struct Repo {
    directory: TempDir,
    manifest: Vec<String>
}

impl Repo {
    pub fn new(directory: TempDir) -> Result<Repo, std::io::Error> {
        let contents = std::fs::read_dir(directory.path())?;
        let mut manifest = vec![];
        for f in contents {
            manifest.push(f.unwrap().file_name().to_str().unwrap().to_string());
        }
        Ok(Repo{directory: directory, manifest: manifest})
    }

    pub fn retrieve(&self, artifact: &Artifact) -> Option<PathBuf> {
        let target = artifact.to_string();
        let goal = self.manifest.iter().find(|f| f.starts_with(target.as_str()))?;
        Some(self.directory.path().join(goal))
    }

    pub fn path(&self) -> &Path {
        self.directory.path()
    }
}