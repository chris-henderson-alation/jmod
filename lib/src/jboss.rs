use std::path::{PathBuf, Path};
use crate::artifact::Artifact;

pub struct JBoss {
    home: PathBuf
}

impl JBoss {

    pub fn new(home: PathBuf) -> JBoss {
        JBoss{home}
    }

    pub fn install(&self, artifact: &Artifact, jar: &Path, dependencies: Vec<&Artifact>) {
        let dir = self.home.join("modules").join(artifact.to_path());
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::rename(jar, dir.join(jar.file_name().unwrap()));
        let mut deps = String::new();
        for dep in dependencies {
            deps.push_str(format!(r#"<module name="{}.{}" slot="{}" />"#, dep.group_id, dep.artifact_id, dep.version).as_str());
            deps.push('\n');
        }
        let module = format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<module xmlns="urn:jboss:module:1.1" name="{}.{}" slot="{}">

	<resources>
	    <resource-root path="{}"/>
	</resources>

	<dependencies>
	   {}
	</dependencies>

</module>
"#, artifact.group_id, artifact.artifact_id, artifact.version, jar.file_name().unwrap().to_str().unwrap(), deps);
        std::fs::write(dir.join("module.xml"), module).unwrap();
    }
}

//fn make_module(gav: &GAV, resources: &Path) -> String {
//    let mut r = String::new();
//    let mut jars = std::fs::read_dir(resources).unwrap().map(Result::unwrap).map(|f| String::from(f.path().file_name().unwrap().to_str().unwrap()))
//        .collect::<Vec<String>>();
//    jars.sort();
//    jars.iter().for_each(|f| {
//        r.push('\t');
//        r.push('\t');
//        r.push_str(format!(r#"<resource-root path="{}"/>"#, f).as_ref());
//        r.push('\n');
//    });
//    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
//<module xmlns="urn:jboss:module:1.1" name="{}" slot="{}">
//	<resources>{}</resources>
//</module>"#, gav.to_package(), gav.version, r)
//}
