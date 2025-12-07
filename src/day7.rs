use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Start,
    Splitter,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '^' => Tile::Splitter,
            'S' => Tile::Start,
            _ => panic!("Unknown tile: {c}"),
        }
    }
}

#[derive(Debug)]
struct Row {
    tiles: Vec<Tile>,
}

impl Row {
    fn from_string(s: &str) -> Row {
        Row {
            tiles: s.chars().map(Tile::from_char).collect(),
        }
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn from_string(s: &str) -> Grid {
        Grid {
            rows: s.lines().map(Row::from_string).collect(),
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Grid {
    Grid::from_string(input)
}

struct State {
    beams: Vec<bool>,
    split_count: usize,
}

impl State {
    fn new(row_size: usize) -> State {
        State {
            beams: vec![false; row_size],
            split_count: 0,
        }
    }

    fn traverse(&mut self, row: &Row) -> usize {
        let mut new_beams = self.beams.clone();

        for i in 0..row.tiles.len() {
            if row.tiles[i] == Tile::Start {
                new_beams[i] = true;
            }

            if self.beams[i] && row.tiles[i] == Tile::Splitter {
                self.split_count += 1;
                new_beams[i] = false;
                new_beams[i - 1] = true;
                new_beams[i + 1] = true;
            }
        }

        self.beams = new_beams;

        self.split_count
    }
}

#[aoc(day7, part1)]
fn part1(input: &Grid) -> usize {
    let row_size = input.rows[0].tiles.len();

    input
        .rows
        .iter()
        .scan(State::new(row_size), |state, row| Some(state.traverse(row)))
        .last()
        .unwrap()
}

struct Cache {
    cache: HashMap<(usize, usize), usize>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            cache: HashMap::new(),
        }
    }

    fn add_to_cache(&mut self, key: (usize, usize), value: usize) {
        self.cache.insert(key, value);
    }

    fn lookup(&self, key: (usize, usize)) -> Option<usize> {
        self.cache.get(&key).cloned()
    }
}

fn get_branch_count_from(cache: &mut Cache, input: &Grid, row: usize, col: usize) -> usize {
    if row == input.rows.len() {
        1
    } else {
        if let Some(value) = cache.lookup((row, col)) {
            return value;
        }

        let ret = match input.rows[row].tiles[col] {
            Tile::Splitter => {
                get_branch_count_from(cache, input, row + 1, col - 1)
                    + get_branch_count_from(cache, input, row + 1, col + 1)
            }
            _ => get_branch_count_from(cache, input, row + 1, col),
        };

        cache.add_to_cache((row, col), ret);
        ret
    }
}

#[aoc(day7, part2)]
fn part2(input: &Grid) -> usize {
    let start_col = input.rows[0]
        .tiles
        .iter()
        .position(|t| *t == Tile::Start)
        .unwrap();

    let mut cache = Cache::new();

    get_branch_count_from(&mut cache, input, 1, start_col)
}
