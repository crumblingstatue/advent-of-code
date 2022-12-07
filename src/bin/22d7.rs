#[derive(Debug, PartialEq)]
enum CmdLine<'a> {
    Cd { dest: &'a str },
    Ls,
    OutDir { name: &'a str },
    OutFile { size: u64, name: &'a str },
}

impl<'a> CmdLine<'a> {
    fn from_line(line: &'a str) -> Self {
        if let Some(cmd) = line.strip_prefix("$ ") {
            if let Some(dir) = cmd.strip_prefix("cd ") {
                Self::Cd { dest: dir }
            } else if cmd == "ls" {
                return Self::Ls;
            } else {
                panic!("Invalid command: `{cmd}`");
            }
        } else if let Some(dir) = line.strip_prefix("dir ") {
            return Self::OutDir { name: dir };
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            return Self::OutFile {
                size: size.parse().unwrap(),
                name,
            };
        }
    }
}

#[derive(Debug)]
struct Fs<'a> {
    root_node: FsNode<'a>,
}

impl<'a> Default for Fs<'a> {
    fn default() -> Self {
        Self {
            root_node: FsNode::new_dir("/"),
        }
    }
}

struct FsDiscoverer<'a, 'fs> {
    fs: &'fs mut Fs<'a>,
    pwd: Vec<&'a str>,
}

impl<'a, 'fs> FsDiscoverer<'a, 'fs> {
    fn new(fs: &'fs mut Fs<'a>) -> Self {
        Self { fs, pwd: vec![] }
    }
    fn cd(&mut self, dst: &'a str) {
        if dst == ".." {
            self.pwd.pop();
        } else if dst == "/" {
            self.pwd.clear();
        } else {
            self.pwd.push(dst);
        }
    }
    fn add_out_dir(&mut self, name: &'a str) {
        let cur_node = self.fs.resolve(&self.pwd);
        cur_node.mkdir(name);
    }
    fn add_out_file(&mut self, size: u64, name: &'a str) {
        let cur_node = self.fs.resolve(&self.pwd);
        cur_node.mkfile(name, size);
    }
}

impl<'a> Fs<'a> {
    fn from_cmdline_discovery(mut discovery: impl Iterator<Item = CmdLine<'a>>) -> Self {
        assert_eq!(discovery.next(), Some(CmdLine::Cd { dest: "/" }));
        let mut fs = Fs::default();
        let mut discoverer = FsDiscoverer::new(&mut fs);
        for cmdline in discovery {
            match cmdline {
                CmdLine::Cd { dest } => discoverer.cd(dest),
                CmdLine::Ls => {}
                CmdLine::OutDir { name } => discoverer.add_out_dir(name),
                CmdLine::OutFile { size, name } => discoverer.add_out_file(size, name),
            }
        }
        fs
    }
    fn resolve<'root>(&'root mut self, path: &[&'a str]) -> &mut FsNode<'a> {
        let mut node = &mut self.root_node;
        for segment in path {
            node = node.get_child(segment);
        }
        node
    }
}

#[derive(Debug)]
struct FsNode<'a> {
    name: &'a str,
    body: FsNodeBody<'a>,
}

impl<'a> FsNode<'a> {
    fn get_child(&mut self, path_seg: &'a str) -> &mut FsNode<'a> {
        let FsNodeBody::Dir { children } = &mut self.body else {
            panic!("Trying to get child of a file");
        };
        children.iter_mut().find(|c| c.name == path_seg).unwrap()
    }
    fn mkdir(&mut self, name: &'a str) {
        let FsNodeBody::Dir { children } = &mut self.body else {
            panic!("Trying to get child of a file");
        };
        children.push(FsNode::new_dir(name));
    }
    fn mkfile(&mut self, name: &'a str, size: u64) {
        let FsNodeBody::Dir { children } = &mut self.body else {
            panic!("Trying to get child of a file");
        };
        children.push(FsNode::new_file(name, size));
    }
    fn new_dir(name: &'a str) -> Self {
        Self {
            name,
            body: FsNodeBody::Dir { children: vec![] },
        }
    }
    fn new_file(name: &'a str, size: u64) -> Self {
        Self {
            name,
            body: FsNodeBody::File { size },
        }
    }
    fn total_size(&mut self) -> u64 {
        match &mut self.body {
            FsNodeBody::Dir { children } => children.iter_mut().map(|node| node.total_size()).sum(),
            FsNodeBody::File { size } => *size,
        }
    }
    fn traverse(&mut self, f: &mut impl FnMut(&mut FsNode)) {
        f(self);
        if let FsNodeBody::Dir { children } = &mut self.body {
            for ch in children {
                ch.traverse(f);
            }
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self.body, FsNodeBody::Dir { .. })
    }
}

type DirContents<'a> = Vec<FsNode<'a>>;

#[derive(Debug)]
enum FsNodeBody<'a> {
    Dir { children: DirContents<'a> },
    File { size: u64 },
}

#[cfg(test)]
const TEST_INPUT: &str = "\
$ cd /
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
7214296 k";

#[test]
fn test_cmdline_from_line() {
    for l in TEST_INPUT.lines() {
        dbg!(CmdLine::from_line(l));
    }
}

#[test]
fn test_fs_from_discovery() {
    dbg!(Fs::from_cmdline_discovery(
        TEST_INPUT.lines().map(CmdLine::from_line)
    ));
}

#[test]
fn test_sizes() {
    let mut fs = Fs::from_cmdline_discovery(TEST_INPUT.lines().map(CmdLine::from_line));
    assert_eq!(fs.resolve(&["a", "e"]).total_size(), 584);
    assert_eq!(fs.resolve(&["a"]).total_size(), 94853);
    assert_eq!(fs.resolve(&["d"]).total_size(), 24933642);
    assert_eq!(fs.resolve(&[]).total_size(), 48381165);
}

fn part1(input: &str) -> u64 {
    let mut fs = Fs::from_cmdline_discovery(input.lines().map(CmdLine::from_line));
    let mut sum = 0;
    fs.root_node.traverse(&mut |node| {
        let total_size = node.total_size();
        if node.is_dir() && total_size <= 100000 {
            sum += total_size;
        }
    });
    sum
}

aoc::tests! {
fn part1:
    TEST_INPUT => 95437;
    in => 1297159;
}

aoc::main!(part1);
