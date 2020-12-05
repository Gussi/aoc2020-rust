use std::collections::HashMap;

type Map = HashMap<Point, Tile>;

#[derive(PartialEq)]
enum Tile {
    Open,
    Tree,
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct World {
    map: Map,
    map_size: Point,
}

impl World {
    fn new() -> World {
        World {
            map: Map::new(),
            map_size: Point { x: 0, y: 0 }
        }
    }

    fn map_insert(&mut self, p: Point, t: Tile) {
        self.map_size.x = self.map_size.x.max(p.x);
        self.map_size.y = self.map_size.y.max(p.y);
        self.map.insert(p, t);
    }

    fn has_tree(&self, p: &Point) -> bool {
        let p = Point {
            x: p.x % (self.map_size.x + 1),
            y: p.y,
        };

        return match self.map.get(&p) {
            Some(Tile::Tree)    => true,
            _                   => false,
        };
    }

    fn done(&self, p: &Point) -> bool {
        p.y > self.map_size.y
    }

    fn trees_on_slope(&self, slide_x: usize, slide_y: usize) -> usize {
        let mut player = Point { x: 0, y: 0 };
        let mut trees = 0;

        loop {
            if self.done(&player) {
                break;
            }

            if self.has_tree(&player) {
                trees += 1
            }

            player.x += slide_x;
            player.y += slide_y;
        }

        trees
    }
}

pub fn part1() -> Result<usize, &'static str> {
    let world = input().unwrap();

    Ok(world.trees_on_slope(3, 1))
}

pub fn part2() -> Result<usize, &'static str> {
    let world = input().unwrap();

    Ok([
        world.trees_on_slope(1, 1),
        world.trees_on_slope(3, 1),
        world.trees_on_slope(5, 1),
        world.trees_on_slope(7, 1),
        world.trees_on_slope(1, 2),
    ].iter().product())
}

fn input() -> Result<World, &'static str> {
    let mut world = World::new();

    for y in 0.. {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Unable to read from stdin");

        if input.trim().is_empty() {
            break;
        }

        for (x, c) in input.trim().chars().enumerate() {
            let tile = match c {
                '#' => Ok(Tile::Tree),
                '.' => Ok(Tile::Open),
                _   => Err("Invalid tile character")
            }?;

            let pos = Point { x, y };
            world.map_insert(pos, tile);
        }
    }

    Ok(world)
}
