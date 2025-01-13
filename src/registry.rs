use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::Read,
    sync::{LazyLock, RwLock},
};

use anyhow::anyhow;
use pubgrub::{
    solver::{Dependencies, DependencyProvider},
    version::SemanticVersion,
};
use semver::Version;

use crate::{module::Module, to_pubgrub_range, to_pubgrub_ver, to_semver, ARG};

pub trait Reg {
    fn query(&self, mod_id: &str) -> Option<BTreeMap<Version, Module>>;
}

pub enum Registry {
    File(FileRegistry),
}

impl Reg for Registry {
    fn query(&self, mod_id: &str) -> Option<BTreeMap<Version, Module>> {
        match self {
            Registry::File(file_registry) => file_registry.query(mod_id),
        }
    }
}

impl DependencyProvider<String, SemanticVersion> for Registry {
    fn choose_package_version<
        T: std::borrow::Borrow<String>,
        U: std::borrow::Borrow<pubgrub::range::Range<SemanticVersion>>,
    >(
        &self,
        potential_packages: impl Iterator<Item = (T, U)>,
    ) -> Result<(T, Option<SemanticVersion>), Box<dyn std::error::Error>> {
        let mut it = potential_packages;
        let (package, range) = it.next().unwrap();
        if let Some(available) = self.query(package.borrow()) {
            let version = available
                .iter()
                .rev()
                .filter(|(version, _)| range.borrow().contains(&to_pubgrub_ver(&version)))
                .filter(|(_, info)| info.mc_dep.matches(&OP_MC_VERSION.read().unwrap()))
                .map(|(v, _)| v)
                .next();
            Ok((package, version.map(|v| to_pubgrub_ver(v))))
        } else {
            Err(anyhow!("invalid mod {}", package.borrow()).into())
        }
    }

    fn get_dependencies(
        &self,
        package: &String,
        version: &SemanticVersion,
    ) -> Result<pubgrub::solver::Dependencies<String, SemanticVersion>, Box<dyn std::error::Error>>
    {
        if let Some(id) = self.query(package) {
            if let Some(info) = id.get(&to_semver(version)) {
                let dep = info
                    .dep
                    .iter()
                    .map(|(k, v)| (k.clone(), to_pubgrub_range(v)));
                Ok(Dependencies::Known(dep.collect()))
            } else {
                Err(anyhow!("invalid version {version} for mod {package}").into())
            }
        } else {
            Err(anyhow!("invalid mod {package}").into())
        }
    }
}

pub struct FileRegistry {
    data: HashMap<String, BTreeMap<Version, Module>>,
}

impl FileRegistry {
    pub fn new(path: &str) -> Self {
        let mut buf = String::new();
        File::open(path)
            .expect(&format!("unable to open {}", ARG.registry.path()))
            .read_to_string(&mut buf)
            .expect(&format!("error reading {}", ARG.registry.path()));
        let v = toml::from_str(&buf).expect("invalid TOML");
        Self { data: v }
    }
}

impl Reg for FileRegistry {
    fn query(&self, mod_id: &str) -> Option<BTreeMap<Version, Module>> {
        self.data.get(mod_id).cloned()
    }
}

/// The registry used according to argument or configuration passed.
pub static REGISTRY: LazyLock<Registry> = LazyLock::new(|| match ARG.registry.scheme() {
    "file" => Registry::File(FileRegistry::new(ARG.registry.path())),
    _ => panic!("URL scheme not supported"),
});

/// The operating minecraft version,
/// assign to this to make PubGrub take minecraft version requirement into consideration.
pub static OP_MC_VERSION: LazyLock<RwLock<Version>> =
    LazyLock::new(|| RwLock::new(Version::new(0, 0, 0)));
