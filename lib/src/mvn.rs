use super::repo::Repo;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn copy_dependencies(pom: &Path) -> Result<Repo, std::io::Error> {
    let deps = tempdir::TempDir::new("jmod")?;
    std::process::Command::new("mvn")
        .arg("-f")
        .arg(pom.as_os_str())
        .arg("dependency:copy-dependencies")
        .arg("-DexcludeTypes=pom")
        .arg(format!("-DoutputDirectory={}", deps.path().to_str().unwrap()))
        .status()?;
    Repo::new(deps)
}

pub fn dependency_tree(pom: &Path) -> Result<NamedTempFile, std::io::Error> {
    let tgf_file = NamedTempFile::new()?;
    std::process::Command::new("mvn")
        .arg("-f")
        .arg(pom.as_os_str())
        .arg("dependency:tree")
        .arg(format!("-Doutput={}", tgf_file.path().to_str().unwrap()))
        .arg("-DoutputType=tgf")
        .arg("-Dexcludes=::pom:")
        .status()?;
    Ok(tgf_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use std::ffi::OsString;

    #[test]
    fn test_coopy() {
//        let a = Artifact::from_str("org.apache.hive:hive-jdbc:1.2.1").unwrap();

//        let repo = copy_dependencies(a).unwrap();
    }
    
    #[test]
    fn getit() {
        let deps = dependency_tree(Path::new("/Users/chris.henderson/wildFlyEval/hive_3_1_1/pom.xml")).unwrap();
        std::fs::copy(deps, Path::new("/tmp/deps"));
    }
}