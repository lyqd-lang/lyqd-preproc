use std::collections::HashMap;
use std::fs;

mod preprocs;

const PREFIX: &str = "@";

#[allow(dead_code)]
fn defines_win() -> HashMap<String, String> {
    let mut defines = HashMap::new();
    defines.insert("_WIN32".to_string(), "1".to_string());
    defines
}

#[allow(dead_code)]
fn defines_unix() -> HashMap<String, String> {
    let mut defines = HashMap::new();
    defines.insert("_UNIX".to_string(), "1".to_string());
    defines
}

#[allow(dead_code)]
fn defines_macos() -> HashMap<String, String> {
    let mut defines = HashMap::new();
    defines.insert("_MACOS".to_string(), "1".to_string());
    defines
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut defines = HashMap::new();

    #[cfg(target_os = "windows")]
    defines.extend(defines_win());

    #[cfg(target_os = "unix")]
    defines.extend(defines_unix());

    #[cfg(target_os = "macos")]
    defines.extend(defines_macos());

    let code = fs::read_to_string(&args[1]).unwrap();
    let regex = regex::Regex::new(format!("{PREFIX}.*$").as_str()).unwrap();
    for line in regex.find_iter(&code) {
        if regex.is_match(line.as_str()) {
            let line = line.as_str();
            let args = line.split(" ").collect::<Vec<_>>();
            let _args = args[1..].to_vec();
            match &args[..1][0][1..] {
                "define" => defines.extend(preprocs::define(_args)),
                _ => ()
            }
        }
    }
    // strip the code of preprocessor directives
    let mut postproc_code = String::new();
    for line in code.lines() {
        if !line.starts_with(PREFIX) {
            postproc_code.push_str(line.trim());
        }
    }

    // handle defines
    for (k, v) in defines.iter() {
        postproc_code = postproc_code.replace(k, v);
    }

    println!("{:?}", postproc_code);
}
