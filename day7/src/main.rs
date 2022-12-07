use petgraph::visit::IntoNodeIdentifiers;
use petgraph::{graph::NodeIndex, Graph};
use std::collections::HashMap;
use std::iter::Peekable;

static INPUT: &str = include_str!("../input");

#[derive(Debug)]
struct Du {
    fs: Graph<String, usize>,
    stack: Vec<NodeIndex>,
}

impl Du {
    fn new() -> Self {
        let fs = Graph::new();
        let stack = vec![];
        Self { fs, stack }
    }

    fn cd(&mut self, dir: String) {
        if self.stack.is_empty() {
            self.stack.push(self.fs.add_node(dir));
            return;
        }

        if dir.as_str() == ".." {
            self.stack.pop();
            return;
        }

        let current = self.stack.last().unwrap();
        let next = self
            .fs
            .neighbors(*current)
            .find(|&idx| self.fs[idx].as_str() == dir.as_str())
            .unwrap_or_else(|| self.add_dir(dir));

        self.stack.push(next);
    }

    fn add_dir(&mut self, dir: String) -> NodeIndex {
        let current = self.stack.last().unwrap();
        let new_dir = self.fs.add_node(dir);
        self.fs.add_edge(*current, new_dir, 0);
        new_dir
    }

    fn add_file(&mut self, file: String, fsize: usize) -> NodeIndex {
        let current = self.stack.last().unwrap();
        let new_file = self.fs.add_node(file);
        self.fs.add_edge(*current, new_file, fsize);
        new_file
    }

    fn summarize(mut self) -> HashMap<String, usize> {
        let leafs: Vec<_> = self
            .fs
            .node_identifiers()
            .filter(|&idx| self.fs.neighbors(idx).count() == 0)
            .collect();

        // traverse from each leaf to root summing up results for each entry
        self.fs.reverse();
        let mut map = HashMap::new();

        for idx in leafs {
            let parent = self.fs.neighbors(idx).next().unwrap();
            let size = self
                .fs
                .edges_connecting(idx, parent)
                .next()
                .unwrap()
                .weight();

            let mut idx = idx;
            let mut dirs = vec![];
            while let Some(parent) = self.fs.neighbors(idx).next() {
                dirs.push(self.fs[parent].clone());
                idx = parent;
            }

            dirs.reverse();
            for n in 0..dirs.len() {
                let name = dirs[..n + 1].join("/");
                *map.entry(name).or_insert(0) += size;
            }
        }
        map
    }
}

enum FsNode {
    File(String, usize),
    Dir(String),
}

enum Cmd {
    Cd(String),
    Ls(Vec<FsNode>),
}

impl Cmd {
    fn read_next<'a, I: Iterator<Item = &'a str>>(input: &mut Peekable<I>) -> Option<Self> {
        let cmd = input.next()?;
        let (marker, cmd) = cmd.split_once(' ')?;
        assert_eq!(marker, "$");
        if cmd.starts_with("cd") {
            Some(Cmd::parse_cd(cmd))
        } else if cmd.starts_with("ls") {
            Some(Cmd::parse_ls(input))
        } else {
            panic!("Unsupported command")
        }
    }

    fn parse_cd(cmd: &str) -> Self {
        let (_, dir) = cmd.split_once(' ').expect("Invalid cd command: {cmd}");
        Cmd::Cd(dir.to_owned())
    }

    fn parse_ls<'a, I: Iterator<Item = &'a str>>(input: &mut Peekable<I>) -> Self {
        let mut ls_out = vec![];
        while let Some(next) = input.peek() {
            if next.starts_with('$') {
                break;
            }
            let (fst, snd) = input
                .next()
                .unwrap()
                .split_once(' ')
                .expect("invalid entry");
            if fst == "dir" {
                ls_out.push(FsNode::Dir(snd.to_owned()));
            } else {
                ls_out.push(FsNode::File(snd.to_owned(), fst.parse().unwrap()))
            }
        }
        Cmd::Ls(ls_out)
    }
}

fn main() {
    let mut du = Du::new();
    let mut input = INPUT.lines().peekable();

    while let Some(cmd) = Cmd::read_next(&mut input) {
        match cmd {
            Cmd::Cd(dir) => du.cd(dir),
            Cmd::Ls(ls_out) => {
                for node in ls_out {
                    match node {
                        FsNode::File(name, size) => du.add_file(name, size),
                        FsNode::Dir(name) => du.add_dir(name),
                    };
                }
            }
        }
    }

    let summary = du.summarize();

    // part 1
    println!(
        "{}",
        summary.values().filter(|&&v| v <= 100000).sum::<usize>()
    );

    let total_space = 70_000_000usize;
    let required_space = 30_000_000usize;
    let used_space = summary["/"];
    let missing = required_space - (total_space - used_space);

    // part 2
    println!(
        "{}",
        summary.values().filter(|&&v| v >= missing).min().unwrap()
    );
}
