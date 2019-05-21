use std::str::FromStr;
use std::path::{PathBuf, Path};

use tempfile::NamedTempFile;


#[derive(Debug, Clone)]
pub struct Artifact {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String
}

impl Artifact {

    pub fn from_dep(s: String) -> Artifact {
        let mut split = s.split(':');
        let gid = split.next().unwrap();
        let aid = split.next().unwrap();
        split.next();
        let version = split.next().unwrap();
        Artifact{
            group_id: gid.to_string(),
            artifact_id: aid.to_string(),
            version: version.to_string()
        }
    }

    pub fn to_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        self.group_id.split(".").for_each(|f| path.push(f));
        path.push(&self.artifact_id);
        path.push(&self.version);
        path
    }

    pub fn to_package(&self) -> String {
        let mut p = String::new();
        p.push_str(&self.group_id);
        p.push('.');
        p.push_str(&self.artifact_id);
        p
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}", self. artifact_id, self.version)
    }

    pub fn to_pom(&self) -> Result<NamedTempFile, std::io::Error> {
        let pom = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://maven.apache.org/POM/4.0.0 http://maven.apache.org/xsd/maven-4.0.0.xsd">
    <modelVersion>4.0.0</modelVersion>

    <groupId>com.alation.jboss</groupId>
    <artifactId>module-builder</artifactId>
    <version>1.0</version>

    <dependencies>
        <dependency>
            <groupId>{}</groupId>
            <artifactId>{}</artifactId>
            <version>{}</version>
        </dependency>
    </dependencies>

</project>"#, self.group_id, self.artifact_id, self.version);
        let pom_file = tempfile::NamedTempFile::new()?;
        std::fs::write(pom_file.path(), pom)?;
        Ok(pom_file)
    }

    pub fn install(&self, installation: &Path) -> Result<(), std::io::Error> {
        let dir = installation.read_dir()?;
        let mut files = vec![];
        for f in dir {
            let file = f?;
            let fname = file.file_name().clone();
            let fname_str = fname.to_str().unwrap().to_string();
            files.push(fname_str);
        }
        files.sort();
        let mut resources = String::new();
        for file in files {
            resources.push('\t');
            resources.push('\t');
            resources.push_str(format!(r#"<resource-root path="{}" />"#, file).as_ref());
            resources.push('\n');
        }
        let module = format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<module xmlns="urn:jboss:module:1.1" name="{}.{}" slot="{}">

	<resources>

{}
	</resources>

</module>
"#, self.group_id, self.artifact_id, self.version, resources);
        std::fs::write(installation.join("module.xml"), module)?;
        Ok(())
    }
}

impl FromStr for Artifact {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut gav = s.split(":").collect::<Vec<&str>>();
        if gav.len() != 3 {
            return Err(String::from("The --artifact value must follow the Maven `GAV` naming convention of `groupID:artifactID:version"));
        }
        if unsafe{gav.get_unchecked(0)} == &"" {
            return Err(String::from("The groupID component of --artifact may not be empty"));
        }
        if unsafe{gav.get_unchecked(1)} == &"" {
            return Err(String::from("The artifactID component of --artifact may not be empty"));
        }
        if unsafe{gav.get_unchecked(2)} == &"" {
            return Err(String::from("The version component of --artifact may not be empty"));
        }
        Ok(Artifact{
            group_id: String::from(gav.remove(0)),
            artifact_id: String::from(gav.remove(0)),
            version: String::from(gav.remove(0))
        })
    }
}