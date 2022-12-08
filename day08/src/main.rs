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

fn visible(row: usize, col: usize, map: &Vec<Vec<(u32, Option<bool>)>>) -> bool {
    let height = map[row][col].0;

    // left visibility
    let lvis = map[row][0..col].iter().all(|(h, _)| *h < height);
    if lvis {
        return true;
    }

    // right visibility
    let rvis = map[row][(col + 1)..].iter().all(|(h, _)| *h < height);
    if rvis {
        return true;
    }

    // up visibility
    let uvis = map[0..row].iter().map(|r| r[col]).all(|(h, _)| h < height);
    if uvis {
        return true;
    }

    // down visibility
    let dvis = map[(row + 1)..]
        .iter()
        .map(|r| r[col])
        .all(|(h, _)| h < height);
    if dvis {
        return true;
    }

    false
}

fn part1(text: &String) {
    let mut rows: Vec<Vec<(u32, Option<bool>)>> = text
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c.to_string().parse::<u32>().unwrap(), None))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let len = rows.len();
    let rowlen = rows[0].len();

    // Set the outide all visible
    for (_, vis) in rows[0].iter_mut() {
        vis.replace(true);
    }

    for (_, vis) in rows[len - 1].iter_mut() {
        vis.replace(true);
    }

    for i in 0..len {
        rows[i][0].1.replace(true);
        rows[i][rowlen - 1].1.replace(true);
    }

    for i in 1..len - 1 {
        for j in 1..rowlen - 1 {
            let visibility = visible(i, j, &rows);
            rows[i][j].1.replace(visibility);
        }
    }

    println!(
        "{}",
        rows.iter()
            .flat_map(|row| row.iter())
            .filter(|(_, vis)| match vis {
                None => false,
                Some(v) => *v,
            })
            .count()
    );
}

fn part2(text: &String) {}
