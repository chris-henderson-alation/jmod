extern crate lib;

use lib::cli::JMod;
use lib::mvn;
use structopt::StructOpt;


fn main() -> Result<(), std::io::Error> {
    let opts = JMod::from_args();
    let pom = opts.artifact.to_pom()?;
    let repo = mvn::copy_dependencies(pom.path())?;
    let target = opts.jboss_home.join("modules").join(opts.artifact.to_path());
    if opts.force {
        std::fs::remove_dir_all(&target)?;
    }
    std::fs::create_dir_all(&target)?;
    std::fs::rename(repo.path(), target)?;
    Ok(())
}
