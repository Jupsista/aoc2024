use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let (antennas, field) = parse_input("./day8/input");
    let output1 = part_1(&antennas, &field);
    let output2 = part_2(&antennas, &field);
    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn get_opposite(&self, other: &Self) -> Self {
        Self {
            x: 2 * self.x - other.x,
            y: 2 * self.y - other.y,
        }
    }

    fn get_proceeding(&self, other: &Self) -> Self {
        Self {
            x: other.x + (other.x - self.x),
            y: other.y + (other.y - self.y),
        }
    }

    fn get_opposites_repeating(&self, other: &Self, field: &Field) -> Vec<Coord> {
        let mut output = Vec::new();
        output.push(*self);

        let mut prev = *self;
        let mut next = prev.get_opposite(other);

        if field.is_within_bounds(&next) {
            output.push(next.clone());
        } else {
            return output;
        }

        loop {
            let next_proceeding = prev.get_proceeding(&next);

            if field.is_within_bounds(&next_proceeding) {
                output.push(next_proceeding.clone());
                prev = next;
                next = next_proceeding;
            } else {
                break;
            }
        }

        output
    }
}

#[derive(Debug)]
struct Antenna {
    location: Coord,
}

#[derive(Debug)]
struct Field {
    width: usize,
    height: usize,
}

impl Field {
    fn is_within_bounds(&self, coord: &Coord) -> bool {
        0 <= coord.x && coord.x < self.width as i64 && 0 <= coord.y && coord.y < self.height as i64
    }
}

fn parse_input(path: &str) -> (HashMap<char, Vec<Antenna>>, Field) {
    let data = fs::read_to_string(path).expect("File exists");
    let mut hm: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                hm.entry(c).or_insert_with(Vec::new).push(Antenna {
                    location: Coord {
                        x: x as i64,
                        y: y as i64,
                    },
                });
            }
        }
    }
    let field = Field {
        width: data.lines().next().unwrap().len(),
        height: data.lines().count(),
    };
    (hm, field)
}

fn part_1(antennas: &HashMap<char, Vec<Antenna>>, field: &Field) -> usize {
    let mut antinodes = HashSet::new();

    for (_key, antenna_list) in antennas {
        for i in 0..antenna_list.len() {
            let current = &antenna_list[i];

            for j in 0..antenna_list.len() {
                if i != j {
                    let other = &antenna_list[j];
                    let opposite = current.location.get_opposite(&other.location);

                    if field.is_within_bounds(&opposite) {
                        antinodes.insert(opposite);
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn part_2(antennas: &HashMap<char, Vec<Antenna>>, field: &Field) -> usize {
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for (_key, antenna_list) in antennas {
        for i in 0..antenna_list.len() {
            let current = &antenna_list[i];

            for j in 0..antenna_list.len() {
                if i != j {
                    let other = &antenna_list[j];
                    let opposites = current
                        .location
                        .get_opposites_repeating(&other.location, field);

                    opposites.iter().for_each(|v| {
                        antinodes.insert(*v);
                    });
                }
            }
        }
    }

    antinodes.len()
}
