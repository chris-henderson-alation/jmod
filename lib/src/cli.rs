extern crate structopt;
extern crate tempdir;

use std::path::PathBuf;
use structopt::StructOpt;

use super::artifact::Artifact;

/// A command line tool for converting a single Maven dependency, and all of its transitive
/// dependencies, into a single JBoss Module.
///
/// Please https://jboss-modules.github.io/jboss-modules/manual/ for an in-depth discussion
/// on JBoss modules, their implementation, and their use cases.
#[derive(StructOpt, Debug)]
#[structopt(name = "", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
pub struct JMod {
    /// A groupID:artifactID:version string following the standard Maven naming convention (see https://maven.apache.org/guides/mini/guide-naming-conventions.html)
    #[structopt(short = "a", long = "artifact")]
    pub artifact: Artifact,

    /// The path to the target JBoss installation root directory. This directory must have
    /// the `modules` subdirectory within it.
    #[structopt(short = "j", long = "jboss", env = "JBOSS_HOME")]
    pub jboss_home: PathBuf,

    /// If that target installation already exists, then force will delete the existing target
    /// before installation.
    #[structopt(short = "f", long = "force")]
    pub force: bool
}

