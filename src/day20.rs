use std::collections::HashMap;
use std::iter;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\20.txt").unwrap();
    let split:Vec<&str> = file.split("\n\n").collect();
    let tiles:Vec<Tile> = split.iter().map(|string| { Tile::from_string(string) }).collect();

    let mut edge_mappings:HashMap<TileSideId, TileSideId> = HashMap::new();

    let all_edges:Vec<TileSide> = tiles.iter().flat_map(|tile| { tile.get_sides() }).collect();
    let mut tile_missing_edges:HashMap<usize, usize> = HashMap::new();

    for edge in all_edges.iter() {
        if !edge_mappings.contains_key(&edge.id) {
            let matching_edges:Vec<&TileSide> = all_edges.iter().filter(|other_edge| {  
                edge.id != other_edge.id
                    && (edge.edge == other_edge.edge || edge.edge == other_edge.edge_reversed)
            }).collect();
            if matching_edges.len() == 0 {
                // No matching edge
                let new_count = match tile_missing_edges.remove(&edge.id.tile_id) {
                    None => 1,
                    Some (t) => t + 1
                };
                tile_missing_edges.insert(edge.id.tile_id, new_count);
            } else if matching_edges.len() == 1 {
                edge_mappings.insert(edge.id, matching_edges[0].id);
                edge_mappings.insert(matching_edges[0].id, edge.id);
            } else if matching_edges.len() > 1 {
                println!("Too many matches!");
            }
        }
    }

    /*
    for (tile, count) in tile_missing_edges.iter() {
        if *count > 1 {
            println!("{} missing {} sides", tile, count);
        }
    }
    */

    let total = tile_missing_edges.iter().filter(|(_, count)|{ **count > 1 }).fold(1usize, |acc, (tile, _)| { acc * tile });
    println!("{}", total);
}

#[derive(Hash,PartialEq,Eq,Clone,Copy,Debug)]
struct TileSideId {
    tile_id: usize,
    face: usize
}

struct TileSide {
    id:TileSideId,
    edge: usize,
    edge_reversed: usize
}

struct Tile {
    id: usize,
    data:[bool;100]
}

impl Tile {
    fn from_string(string:&str) -> Tile {
        let lines:Vec<&str> = string.lines().collect();
        let id:usize = lines[0][5..9].parse().unwrap();
        let mut arr:[bool;100] = [false;100];
        let mut pos = 0;

        for line in lines.into_iter().skip(1) {
            for c in line.chars() {
                arr[pos] = c == '#';
                pos += 1;
            }
        }

        Tile { id: id, data: arr }
    }

    fn is_set(&self, x:usize, y:usize) -> bool {
        let pos = y * 10 + x;
        self.data[pos]
    }

    fn print(&self) {
        println!("Tile {}:", self.id);

        for y in 0..10 {
            for x in 0..10 {
                let c = if self.is_set(x, y) { '#' } else { '.' };
                print!("{}", c);
            }
            println!();
        }
    }

    fn get_sides(&self) -> [TileSide; 4] {
        [
            TileSide { 
                id: TileSideId { tile_id: self.id, face: 0 }, // Top
                edge: self.get_line(&mut (0usize..10), &mut iter::repeat(0usize).take(10)),
                edge_reversed: self.get_line(&mut (0usize..10).rev(), &mut iter::repeat(0usize).take(10))
            },
            TileSide { 
                id: TileSideId { tile_id: self.id, face: 1 }, // Right
                edge: self.get_line(&mut iter::repeat(9usize).take(10), &mut (0usize..10)),
                edge_reversed: self.get_line(&mut iter::repeat(9usize).take(10), &mut (0usize..10).rev())
            },
            TileSide { 
                id: TileSideId { tile_id: self.id, face: 2 }, // Bottom
                edge: self.get_line(&mut (0usize..10), &mut iter::repeat(9usize).take(10)),
                edge_reversed: self.get_line(&mut (0usize..10).rev(), &mut iter::repeat(9usize).take(10))
            },
            TileSide { 
                id: TileSideId { tile_id: self.id, face: 3 }, // Left
                edge: self.get_line(&mut iter::repeat(0usize).take(10), &mut (0usize..10)),
                edge_reversed: self.get_line(&mut iter::repeat(0usize).take(10), &mut (0usize..10).rev())
            }
        ]
    }

    // fn get_line<'a>(&self, x_iter:impl Iterator<Item = &'a usize>, y_iter:impl Iterator<Item = &'a usize>) -> usize {
    fn get_line<I1, I2>(&self, x_iter:&mut I1, y_iter:&mut I2) -> usize 
    where
        I1: Iterator<Item = usize>,
        I2: Iterator<Item = usize> {
        //let mut x_iter = (0..10).into_iter();
        //let mut y_iter = ;

        let mut value = 0;
        let mut mask = 1 << 9;

        while let Some(x) = x_iter.next() {
            let y = y_iter.next().unwrap();

            if self.is_set(x, y) {
                value |= mask;
            }
            mask >>= 1;
        }

        value
    }
}
