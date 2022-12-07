use aoc22::get_input;

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls(Vec<DirEntry>),
}

#[derive(Debug)]
pub enum DirEntry {
    File(usize, String),
    Dir(String),
}

impl DirEntry {
    fn new(item: &str) -> DirEntry {
        let (pref, name) = item.split_at(item.find(" ").unwrap());
        match pref {
            "dir" => DirEntry::Dir(name[1..].to_owned()),
            _ => DirEntry::File(pref.parse().unwrap(), name.to_owned()),
        }
    }
}

#[derive(Debug, Default)]
pub struct DirTree {
    pub name: String,
    pub files: Vec<(usize, String)>,
    pub children: Vec<DirTree>,
}

impl DirTree {
    fn new(path: String) -> Self {
        DirTree {
            name: path,
            ..Default::default()
        }
    }

    fn from_cmds(&mut self, cmds: &mut Vec<Command>) {
        while !cmds.is_empty() {
            match cmds.pop().unwrap() {
                Command::Cd(dest) if dest == ".." => break,
                Command::Cd(dest) => {
                    let mut d = DirTree::new(dest);
                    d.from_cmds(cmds);
                    self.children.push(d);
                }
                Command::Ls(entries) => {
                    self.files
                        .extend(entries.into_iter().filter_map(|e| match e {
                            DirEntry::File(size, name) => Some((size, name)),
                            DirEntry::Dir(_) => None,
                        }));
                }
            }
        }
    }

    fn size(&self) -> usize {
        self.children.iter().map(|d| d.size()).sum::<usize>()
            + self.files.iter().map(|(x, _)| *x).sum::<usize>()
    }

    fn walk<F, T>(&self, f: &F) -> Vec<T>
    where
        F: Fn(&DirTree) -> T,
        T: std::fmt::Debug,
    {
        let mut v = vec![f(self)];
        v.extend(self.children.iter().map(|d| d.walk(f)).flatten());
        v
    }
}

fn parse_cmd(s: &str) -> Command {
    let mut iter = s.trim_end().lines();
    let (cmd, arg) = iter.next().unwrap().split_at(2);
    match cmd {
        "cd" => Command::Cd(arg[1..].to_owned()),
        "ls" => Command::Ls(iter.map(|l| DirEntry::new(l)).collect()),
        _ => panic!("{}", cmd),
    }
}

fn solve_part1(d: &DirTree) -> Option<usize> {
    if d.size() < 100_000 {
        Some(d.size())
    } else {
        None
    }
}

pub struct Part2(usize);

impl Part2 {
    fn solve<'a>(&self, d: &'a DirTree) -> usize {
        let w = |x: &DirTree| {self.walker(x)};
        d.walk(&w)
            .into_iter()
            .flatten()
            .min().unwrap()
    }

    fn walker(&self, d: &DirTree) -> Option<usize> {
        if d.size() > self.0 {
            Some(d.size())
        } else {
            None
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();
    let mut cmds: Vec<_> = input_data
        .split("$ ")
        .filter(|s| !s.is_empty())
        .map(parse_cmd)
        .collect();
    cmds.reverse();
    cmds.pop().unwrap();
    let mut d = DirTree::new("/".to_owned());
    d.from_cmds(&mut cmds);
    let part1 = d.walk(&solve_part1);
    let sum1: usize = part1.into_iter().flatten().sum();
    println!("part 1: {sum1:?}");

    let total_size = 70000000;
    let required_free = 30000000;
    let starting_free = total_size - d.size();
    let must_free = required_free - starting_free;
    let solver = Part2(must_free);
    let answer2 = solver.solve(&d);
    println!("part 2: {answer2:?}");
    Ok(())
}
