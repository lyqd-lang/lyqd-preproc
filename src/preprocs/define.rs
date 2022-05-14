use std::collections::HashMap;

pub fn define(args: Vec<&str>) -> HashMap<String, String> {
    let k = args[0].to_string();
    let v = args[1].to_string();
    let mut defines = HashMap::new();
    defines.insert(k, v);
    defines
}