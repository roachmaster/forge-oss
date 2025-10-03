use super::model::CrateMap;

pub fn print_yaml(map: &CrateMap) {
    println!("{}", serde_yaml::to_string(&map).unwrap_or_else(|_| "--- {}\n".to_string()));
}
