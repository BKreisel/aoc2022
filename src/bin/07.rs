use itertools::Itertools;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
enum PathKind {
    Directory,
    File,
}

#[derive(Clone, Debug)]
struct PathItem {
    kind: PathKind,
    size: usize,
    name: String,
    path: PathBuf,
}

impl PathItem {
    pub fn dir(name: &str, path: &str) -> Self {
        PathItem {
            kind: PathKind::Directory,
            size: 0,
            name: name.to_owned(),
            path: PathBuf::from(path),
        }
    }
    pub fn file(name: &str, path: &str, size: usize) -> Self {
        PathItem {
            kind: PathKind::File,
            size,
            name: name.to_owned(),
            path: PathBuf::from(path),
        }
    }

    pub fn depth(&self) -> usize {
        let path_str = self.path.to_str().unwrap();
        if path_str == "/" {
            return 0;
        }
        path_str.chars().filter(|x| x == &'/').count()
    }
}

enum Cmd {
    List,
    Cd(String),
}

impl Cmd {
    pub fn parse(line: &str) -> Cmd {
        if line == "$ ls" {
            return Cmd::List;
        }
        Cmd::Cd(line.split(' ').last().unwrap().to_owned())
    }
}

fn parse_ls(cwd: &str, ls_entries: Vec<&str>) -> Vec<PathItem> {
    let mut fs_items: Vec<PathItem> = Vec::new();

    for entry in ls_entries {
        if entry.starts_with("dir") {
            let dirname = entry.split(' ').last().unwrap();
            fs_items.push(PathItem::dir(dirname, cwd));
            continue;
        }

        // else file
        let name = entry.split(' ').last().unwrap();
        let size = entry.split(' ').next().unwrap().parse::<usize>().unwrap();
        fs_items.push(PathItem::file(name, cwd, size));
    }
    fs_items
}

fn build_filesystem(input: &str) -> Vec<PathItem> {
    let mut filesystem: Vec<PathItem> = Vec::from([PathItem::dir("/", "/")]);
    let mut iter = input.lines().skip(1);
    let mut cwd = PathBuf::from("/");

    while let Some(cmd_str) = iter.next() {
        match Cmd::parse(cmd_str) {
            Cmd::List => {
                let ls_entries = iter.take_while_ref(|x| !x.starts_with('$')).collect();
                filesystem.append(&mut parse_ls(cwd.to_str().unwrap(), ls_entries));
            }
            Cmd::Cd(x) => match x.as_str() {
                "/" => cwd = PathBuf::from("/"),
                ".." => cwd = cwd.parent().unwrap().to_owned(),
                _ => cwd.push(x),
            },
        }
    }

    //  Set directory sizes
    let max_depth = filesystem.iter().map(|x| x.depth()).max().unwrap();

    for depth in (0..=max_depth).rev() {
        let paths_at_depth = filesystem
            .iter()
            .filter(|x| x.depth() == depth)
            .map(|x| x.path.clone())
            .unique()
            .collect::<Vec<PathBuf>>();

        for path in paths_at_depth {
            let size: usize = filesystem
                .iter()
                .filter(|x| x.path == path)
                .map(|x| x.size)
                .sum();

            filesystem
                .iter_mut()
                .find(|x| {
                    let mut fullpath = x.path.clone();
                    fullpath.push(&x.name);
                    x.kind == PathKind::Directory && (fullpath == path)
                })
                .unwrap()
                .size = size;
        }
    }
    filesystem
}

pub fn part_one(input: &str) -> Option<u32> {
    let filesystem = build_filesystem(input);

    let total_size: usize = filesystem
        .iter()
        .filter(|x| x.kind == PathKind::Directory)
        .filter(|x| x.size < 100000)
        .map(|x| x.size)
        .sum();

    Some(total_size as _)
}

pub fn part_two(input: &str) -> Option<u32> {
    let filesystem = build_filesystem(input);
    let total_space = 70000000;
    let space_target = 30000000;

    let root_size = filesystem
        .iter()
        .find(|x| x.path.to_str().unwrap() == "/" && x.name == "/")
        .unwrap()
        .size;

    let free_space = total_space - root_size;
    let needed_space = space_target - free_space;

    let smallest_size = filesystem
        .iter()
        .filter(|x| x.kind == PathKind::Directory)
        .map(|x| x.size)
        .filter(|x| x > &needed_space)
        .min()
        .unwrap();

    Some(smallest_size as _)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
