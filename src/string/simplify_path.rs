// question 71

// spectial case:
//   1) end with /
//   2) pop from root
//   3) many '/' in the mid of path
//   4) deal with '.' and '..'
//
pub fn simplify_path(path: String) -> String {
    let dirs: Vec<&str> = path.split("/").collect();
    let mut path: Vec<&str> = Vec::new();

    for dir in dirs {
        if dir.len() == 0 || dir == "." {
            continue;
        } else if dir == ".." {
            path.pop();
        } else {
            path.push(dir);
        }
    }

    println!("{:?}", path);

    let s: String = path.join("/");
    format!("/{}", s)
}
