use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1182909)?;
    aoc::run!(part_two(input), 2832508)?;
    Ok(())
}

type Inode = usize;

const ROOT: Inode = 0;

#[derive(Debug, PartialEq)]
enum FileType {
    Directory,
    Regular,
}

#[derive(Debug)]
struct File {
    inode: Inode,
    parent: Option<Inode>,
    name: String,
    size: usize,
    type_: FileType,
}

struct Disk {
    files: Vec<File>,
}

impl Disk {
    fn find(&self, parent: Inode, name: &str) -> Option<&File> {
        self.files
            .iter()
            .find(|f| f.parent == Some(parent) && f.name == name)
    }

    fn all_directories(&self) -> impl Iterator<Item = &File> {
        self.files.iter().filter(|f| f.type_ == FileType::Directory)
    }

    fn size_recursive(&self, inode: Inode) -> Result<usize> {
        let file = self
            .files
            .iter()
            .find(|f| f.inode == inode)
            .with_context(|| format!("unknown inode {}", inode))?;
        let size = match file.type_ {
            FileType::Regular => file.size,
            FileType::Directory => self
                .files
                .iter()
                .filter(|f| f.parent == Some(inode))
                .filter_map(|f| self.size_recursive(f.inode).ok())
                .sum(),
        };
        Ok(size)
    }
}

fn parse(terminal_output: &str) -> Result<Disk> {
    let mut disk = Disk {
        files: vec![File {
            inode: ROOT,
            parent: None,
            name: "/".to_string(),
            size: 0,
            type_: FileType::Directory,
        }],
    };
    let mut cwd = vec![ROOT];
    for line in terminal_output.lines() {
        if line.starts_with("$ cd") {
            let dest = &line[5..];
            match dest {
                ".." => {
                    cwd.pop();
                }
                "/" => {
                    cwd = vec![ROOT];
                }
                _ => {
                    let dir = disk
                        .find(
                            *cwd.last().with_context(|| {
                                format!("no current working directory: {:?}", cwd)
                            })?,
                            dest,
                        )
                        .context("unknown directory")?;
                    cwd.push(dir.inode);
                }
            };
        } else if line == "$ ls" {
            // do nothing
        } else if let Some((a, b)) = line.split_once(' ') {
            if a == "dir" {
                disk.files.push(File {
                    inode: disk.files.len(),
                    parent: cwd.last().copied(),
                    name: b.to_string(),
                    size: 0,
                    type_: FileType::Directory,
                });
            } else {
                let size: usize = a
                    .parse()
                    .with_context(|| format!("failed to convert size to usize: '{}'", a))?;
                disk.files.push(File {
                    inode: disk.files.len(),
                    parent: cwd.last().copied(),
                    name: b.to_string(),
                    size,
                    type_: FileType::Regular,
                });
            }
        } else {
            bail!("failed to parse '{}'", line);
        }
    }
    Ok(disk)
}

fn part_one(input: &str) -> Result<usize> {
    let disk = parse(input)?;
    let dir_sizes: Vec<_> = disk
        .all_directories()
        .map(|dir| disk.size_recursive(dir.inode))
        .collect::<Result<_, _>>()?;
    Ok(dir_sizes.iter().filter(|&&size| size <= 100_000).sum())
}

fn part_two(input: &str) -> Result<usize> {
    let disk = parse(input)?;
    let required = 30_000_000;
    let unused = 70_000_000 - disk.size_recursive(ROOT)?;
    let dir_sizes: Vec<_> = disk
        .all_directories()
        .map(|dir| disk.size_recursive(dir.inode))
        .collect::<Result<_, _>>()?;
    dir_sizes
        .iter()
        .filter(|&size| unused + size >= required)
        .min()
        .context("no directory large enough to remove")
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_disk() {
        let disk = parse(INPUT).unwrap();
        assert_eq!(disk.files[ROOT].name, "/");
        let a = disk.find(ROOT, "a").unwrap();
        assert_eq!(a.type_, FileType::Directory);
        let f = disk.find(a.inode, "f").unwrap();
        assert_eq!(f.type_, FileType::Regular);
        assert_eq!(f.size, 29116);
        assert_eq!(
            disk.all_directories()
                .map(|f| f.name.clone())
                .collect::<Vec<_>>(),
            ["/", "a", "d", "e"]
        );
        assert_eq!(disk.size_recursive(ROOT).unwrap(), 48381165);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 95437);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 24933642);
    }
}
