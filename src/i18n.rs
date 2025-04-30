use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

static DEFAULT_LOCALE: &str = "en_US";

fn current_lang() -> String {
    let lang = env::var("LANG").unwrap_or_else(|_| DEFAULT_LOCALE.to_string());
    lang.split(".").next().unwrap_or_default().to_string()
}

fn load_locale() -> HashMap<String, HashMap<String, String>> {
    let dir = fs::read_dir(Path::new("./locale"));
    let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();
    dir.unwrap().for_each(|entry| {
        let mut locale: String = String::new();
        let mut locale_map: HashMap<String, String> = HashMap::new();
        if let Ok(e) = entry {
            if e.file_type().unwrap().is_file() {
                let name: String = e.file_name().to_string_lossy().to_string();
                if name.ends_with("ftl") {
                    locale = name.split(".").next().unwrap().to_string();
                    let file = fs::read(name).unwrap();
                    for line in file.lines().filter_map(Result::ok) {
                        let mut parts = line.trim().splitn(2, '=');
                        let key = parts.next().unwrap_or("none");
                        let value = parts.next().unwrap_or("none");
                        locale_map.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
        map.insert(locale, locale_map);
    });
    map
}

fn read_file_to_map(path: &str) -> HashMap<String, String> {
    let file: File = File::open(path).expect("Open locale file fail");
    let reader = BufReader::new(file);
    let mut map: HashMap<String, String> = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();
                map.insert(key, value);
            }
        }
    }
    map
}

// /// 全局只运行一次，缓存当前使用的语言
// pub static USE_LOCALE: Lazy<String> = Lazy::new(|| {
//     let current = current_lang();
//     let available = load_locale();
//     if available.contains(&current) {
//         current
//     } else {
//         DEFAULT_LOCALE.to_string()
//     }
// });

// #[cfg(test)]
// mod tests {
//     use super::read_file_to_map;

//     #[test]
//     fn it_works() {
//         read_file_to_map(path)
//     }
// }
