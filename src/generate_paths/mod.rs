use std::{collections::HashMap, fs};

pub fn generate_static_paths() -> HashMap<String, String> {
    // Path name, Path to file
    let mut static_paths: HashMap<String, String> = HashMap::new();

    let paths = fs::read_dir("./paths").unwrap();

    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();

            let file_name = entry_path.file_name().unwrap();

            let file_name_as_str = file_name.to_str().unwrap();

            let file_name_as_string = String::from(file_name_as_str);

            file_name_as_string
        })
        .collect::<Vec<String>>();

    for name in names {
        // make sure file is html file
        let len = name.len();
        if &name[(len - 5)..(len)] == ".html" {
            let slug = &name[0..(len - 5)];
            let file_path = "./paths/".to_owned() + &name;

            static_paths.insert(slug.to_owned(), file_path);
        }
    }

    return static_paths;
}
