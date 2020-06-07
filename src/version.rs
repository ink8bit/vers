use serde::Deserialize;

#[derive(Deserialize)]
struct Package {
    version: String,
}

#[allow(dead_code)]
enum Version {
    Major,
    Minor,
    Patch,
}

// Returns patch version
#[allow(dead_code)]
fn get_patch() -> u8 {
    0
}

// Returns minor version
#[allow(dead_code)]
fn get_minor() -> u8 {
    0
}

// Returns major version
#[allow(dead_code)]
fn get_major() -> u8 {
    0
}

// Returns parsed version from package.json
pub fn get_parsed() -> String {
    let f = std::fs::read_to_string("package.json").expect("no such file");
    let package: Package = serde_json::from_str(&f).unwrap();

    package.version
}

// Updates version in package.json by a given one
#[allow(dead_code)]
fn update(version: Version) {}
