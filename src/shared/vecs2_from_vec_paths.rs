use std::collections::HashMap;
use std::path::Path;

pub fn vecs2_from_vec_paths(input: &Vec<String>) -> Vec<Vec<String>> {
    let mut key_order: Vec<String> = Vec::new();
    let mut obj: HashMap<String, Vec<String>> = HashMap::new();

    for elem in input.iter() {
        let file_stem = Path::new(elem)
            .file_stem()
            .unwrap_or_else(|| panic!("no file stem in path: {elem}"))
            .to_string_lossy()
            .to_string();

        let after_delim: &str = file_stem.rsplit('_').next().unwrap_or(&file_stem);

        let chars2: String = after_delim
            .chars()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();

        let char1: String = after_delim.chars().last().unwrap().to_string();

        let chars2_is_integer = chars2.parse::<i32>().is_ok();
        let chars1_is_integer = char1.parse::<i32>().is_ok();

        let key = if chars2_is_integer {
            chars2
        } else if chars1_is_integer {
            char1
        } else {
            "d".to_string()
        };

        if !obj.contains_key(&key) {
            key_order.push(key.clone());
        }
        obj.entry(key).or_insert_with(Vec::new).push(elem.clone());
    }

    let mut output: Vec<Vec<String>> = Vec::new();
    for key in key_order {
        let value = obj.remove(&key).unwrap();
        if key == "d" {
            for item in value {
                output.push(vec![item]);
            }
        } else {
            output.push(value);
        }
    }

    output
}
