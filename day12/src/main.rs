use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file: File = File::open("input").unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

fn part1(text: &String) {
    let mut graph: Graph<char, i32, Directed> = Graph::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = text
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.char_indices()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = (i, j);
                        ('a'.into(), graph.add_node('S'))
                    } else if c == 'E' {
                        end = (i, j);
                        ('z'.into(), graph.add_node('E'))
                    } else {
                        (c.into(), graph.add_node(c))
                    }
                })
                .collect::<Vec<(u32, NodeIndex)>>()
        })
        .collect::<Vec<_>>();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let curspot = grid[i][j];
            for (imod, jmod) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let adji = ((i as isize) + imod) as usize;
                let adjj = ((j as isize) + jmod) as usize;
                match grid.get(adji) {
                    Some(r) => match r.get(adjj) {
                        Some(end) => {
                            if curspot.0 + 1 >= end.0 {
                                graph.add_edge(curspot.1, end.1, 1);
                            };
                        }
                        None => continue,
                    },
                    None => continue,
                };
            }
        }
    }

    println!(
        "{}",
        dijkstra(
            &graph,
            grid[start.0][start.1].1,
            Some(grid[end.0][end.1].1),
            |_| 1
        )
        .get(&grid[end.0][end.1].1)
        .unwrap()
    )
}

fn part2(text: &String) {}
