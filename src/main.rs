use clap::Clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Square {
    Empty,
    Tree,
}

#[derive(Default, Debug)]
struct Row {
    trees: Vec<Square>,
}

#[derive(Default, Debug)]
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    fn get_at(&self, x: usize, y: usize) -> &Square {
        let grid_x = x % self.rows[0].trees.len();
        &self.rows[y].trees[grid_x]
    }

    fn get_tree_count(&self, dx: usize, dy: usize) -> usize {
        let mut tree_count = 0;
        for ii in 0..(self.rows.len() / dy) {
            if self.get_at(dx * ii, dy * ii) == &Square::Tree {
                tree_count += 1;
            }
        }
        tree_count
    }
}

#[derive(Clap)]
struct Opts {
    part: i32,
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let grid = get_grid(opts.input);
    if opts.part == 1 {
        let tree_count = grid.get_tree_count(3, 1);
        println!("There were {} trees", tree_count);
    } else {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let mut product = 1;
        for slope in &slopes {
            product *= grid.get_tree_count(slope.0, slope.1);
        }

        println!("Product of counts is {}", product);
    };
}

fn get_grid(filename: String) -> Grid {
    let mut grid = Grid::default();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_as_string) = line {
                let mut row = Row::default();
                for c in line_as_string.chars() {
                    let s = match c {
                        '.' => Square::Empty,
                        '#' => Square::Tree,
                        e => panic!("Invalid character {0}", e),
                    };
                    row.trees.push(s);
                }
                grid.rows.push(row);
            }
        }
    }

    grid
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
