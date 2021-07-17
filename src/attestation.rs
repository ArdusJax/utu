use anyhow::Result;

#[derive(Debug)]
pub struct Binary {
    pub name: String,
    pub url: String,
    pub version: String,
    hash: String,
    algorithm: String,
}

#[derive(Debug)]
pub struct Build {
    pub id: String,
    pub name: String,
    pub number: i32,
    log: String,
    config: String,
    artifacts: Vec<Binary>,
}
#[derive(Debug)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub file_type: String,
    hash: String,
    algorithm: String,
}
#[derive(Debug)]
pub struct Deployment {
    pub id: String,
    pub name: String,
    pub environments: Vec<String>,
    build_id: String,
}

#[derive(Debug)]
pub enum Kind {
    Build(Build),
    Binary(Binary),
    Package(Package),
    Deployment(Deployment),
}

pub fn attestation_type(kind: Kind) -> Result<String> {
    match kind {
        Kind::Build(build) => Ok(build.name),
        Kind::Binary(binary) => Ok(binary.name),
        Kind::Deployment(deployment) => Ok(deployment.name),
        Kind::Package(package) => Ok(package.name),
    }
}
