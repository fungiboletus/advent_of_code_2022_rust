use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug)]
pub struct FileSystem {
    folders: HashMap<String, Rc<RefCell<Folder>>>,
    files: HashMap<String, Rc<RefCell<File>>>,
}

impl Default for FileSystem {
    fn default() -> Self {
        FileSystem {
            folders: HashMap::new(),
            files: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Folder {
    name: String,
    subfolders: HashSet<String>,
    files: HashSet<String>,
}

impl Default for Folder {
    fn default() -> Self {
        Folder {
            name: "/".to_string(),
            subfolders: HashSet::new(),
            files: HashSet::new(),
        }
    }
}

#[derive(Debug)]
pub struct File {
    name: String,
    size: usize,
}

fn build_full_path(stack: &Vec<String>, name: &str) -> String {
    let mut full_path = stack.join("/");
    if full_path != "/" {
        full_path.push('/');
    }
    full_path.push_str(name);
    // It's ugly but I'm done with this
    if full_path.starts_with("//") {
        full_path = full_path[1..].to_string();
    }
    return full_path;
}

fn build_current_full_path(stack: &Vec<String>) -> String {
    let mut full_path = stack.join("/");
    if full_path.starts_with("//") {
        full_path = full_path[1..].to_string();
    }
    return full_path;
}

pub fn parse_input_data(input: &str) -> FileSystem {
    let mut lines = input.lines();
    let first_line = lines.next().expect("Unable to get first line");
    if first_line != "$ cd /" {
        panic!("First line is not cd /");
    }

    let mut filesystem = FileSystem::default();
    let mut stack: Vec<String> = Vec::new();
    let root = Folder::default();
    filesystem
        .folders
        .insert("/".to_string(), Rc::new(RefCell::new(root)));
    stack.push("/".to_string());

    for line in lines {
        if line.starts_with("$ cd ") {
            let folder_name = line.split(" ").nth(2).expect("Unable to get folder name");
            if folder_name == ".." {
                stack.pop();
            } else {
                stack.push(folder_name.to_string());
            }
        } else if line.starts_with("$ ls") {
            // ignore
        } else if line.starts_with("dir ") {
            let scan = sscanf::sscanf!(line, "dir {String}").expect("Unable to parse dir line");
            let full_path = build_full_path(&stack, &scan);
            let folder = Folder {
                name: full_path.clone(),
                ..Default::default()
            };
            filesystem
                .folders
                .insert(full_path.clone(), Rc::new(RefCell::new(folder)));
            let current_full_path = build_current_full_path(&stack);
            filesystem
                .folders
                .get(&current_full_path)
                .expect("Unable to get current folder")
                .borrow_mut()
                .subfolders
                .insert(full_path.clone());
        } else {
            let scan =
                sscanf::sscanf!(line, "{usize} {String}").expect("Unable to parse file line");
            let full_path = build_full_path(&stack, &scan.1);
            let file = File {
                name: full_path.clone(),
                size: scan.0,
            };
            filesystem
                .files
                .insert(full_path.clone(), Rc::new(RefCell::new(file)));
            let current_full_path = build_current_full_path(&stack);
            filesystem
                .folders
                .get(&current_full_path)
                .expect("Unable to get current folder")
                .borrow_mut()
                .files
                .insert(full_path.clone());
        }
    }

    return filesystem;
}

pub fn compute_folder_size(filesystem: &FileSystem, folder: String) -> usize {
    let folder = filesystem
        .folders
        .get(&folder)
        .expect("Unable to get folder")
        .borrow();
    let mut size = 0;
    for file in folder.files.iter() {
        size += filesystem
            .files
            .get(file)
            .expect("Unable to get file")
            .borrow()
            .size;
    }
    for subfolder in folder.subfolders.iter() {
        size += compute_folder_size(filesystem, subfolder.clone());
    }
    return size;
}

pub fn day_7_part_1(data: &str) -> i64 {
    let data = parse_input_data(data);

    let size: usize = data
        .folders
        .iter()
        .map(|(name, _)| compute_folder_size(&data, name.clone()))
        .filter(|size| *size <= 100000)
        .sum();

    return size as i64;
}

pub fn day_7_part_2(data: &str) -> i64 {
    let data = parse_input_data(data);

    return 42;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k1000";

    const EXAMPLE_2: &str = "$ cd /
$ ls
dir a
42 zzz
$ cd a
$ ls
1000 zzz";

    #[test]
    fn test_day_7_part_1() {
        assert_eq!(day_7_part_1(EXAMPLE), 95437);
        assert_eq!(day_7_part_1(EXAMPLE_2), 2042);
    }

    #[test]
    fn test_day_7_part_2() {
        assert_eq!(day_7_part_2(EXAMPLE), 45000);
    }
}
