use std::collections::HashMap;
use std::iter;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\20.txt").unwrap();
    let split:Vec<&str> = file.split("\n\n").collect();
    let mut tiles:HashMap<usize, Tile> = split.iter().map(|string| { 
        let tile = Tile::from_string(string);
        (tile.id, tile)
     }).collect();

    let mut edge_mappings:HashMap<TileSideId, TileSideId> = HashMap::new();

    let all_edges:Vec<TileSide> = tiles.iter().flat_map(|(_,tile)| { tile.get_sides() }).collect();
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

    // Part 1
    /*
    let total = tile_missing_edges.iter().filter(|(_, count)|{ **count > 1 }).fold(1usize, |acc, (tile, _)| { acc * tile });
    println!("{}", total);
    */

    // Part 2

    // /*
    // Create a massive map to hold all the tiles. 12 rows x 12 columns of tiles 8x8 (original 10x10 tiles, minus borders)
    let mut map = Tile::from_size(0, 12 * 8, 12 * 8);

    // Just pick a corner and rotate it until the north edge has no match. If the west edge has a match, then flip it.
    // let (corner_edge_id, _) = tile_missing_edges.iter().filter(|(_, count)|{ **count > 1 }).next().unwrap();
    // println!("{:?}", corner_edge_id);

    // In my input, 3989 was one of the corners. Just use it.
    let tile = tiles.remove(&3989).unwrap();
    let tile = tile.rotate_until_north_no_match(&all_edges);
    let tile = tile.flip_until_west_no_match(&all_edges);

    tile.remove_edges().write_to(&mut map, 0, 0);

    // Get the east edge, used for matching along the horizontal plane.
    let mut horizontal_edge = tile.get_side(1);

    // Get the south edge, used for starting the next row.
    let mut vertical_edge = tile.get_side(2);

    // Now scan top to bottom, left to right finding matches.
    for y in 0..12usize {
        for x in 1..12usize {
            let matching_tile_side = all_edges.iter().filter(|other_edge| {  
                horizontal_edge.id.tile_id != other_edge.id.tile_id
                    && (horizontal_edge.edge == other_edge.edge || horizontal_edge.edge == other_edge.edge_reversed)
            }).next().unwrap();

            // Get the next tile, rotate/flip until it matches, then write it (sans-borders) to the map.
            let tile = tiles.remove(&matching_tile_side.id.tile_id).unwrap();
            let rotated = tile.rotate_and_flip_to_match_edge(&horizontal_edge, 3, 4, false);
            rotated.remove_edges().write_to(&mut map, x * 8, y * 8);

            horizontal_edge = rotated.get_side(1);
        }

        if y < 11 {
            let matching_tile_side = all_edges.iter().filter(|other_edge| {  
                vertical_edge.id.tile_id != other_edge.id.tile_id
                    && (vertical_edge.edge == other_edge.edge || vertical_edge.edge == other_edge.edge_reversed)
            }).next().unwrap();
            let tile = tiles.remove(&matching_tile_side.id.tile_id).unwrap();
            let rotated = tile.rotate_and_flip_to_match_edge(&vertical_edge, 0, 4, false);
            rotated.remove_edges().write_to(&mut map, 0, (y + 1) * 8);

            horizontal_edge = rotated.get_side(1);
            vertical_edge = rotated.get_side(2);
        }
    }

    // Some debugging, at the end of the above, tiles.len() should be 0.
    // map.print();
    // println!("{}", tiles.len());

    // Lazy, lazy, lazy. I didn't write anything to try rotating/flipping the map until monsters are found.
    // I just messed around with it manually until I found an orientation that works. Weirdly, with corner 3989, 
    // I didn't actually need to do any flipping / rotating.
    let rotated_map = map.clone();
    let monster = Tile::monster();
    let monsters_found = rotated_map.scan_for_matches(&monster);
    monsters_found.print();
    let total_monster_tiles = monsters_found.total_set();
    let map_tiles = map.total_set();
    println!("{} {} {}", map_tiles, total_monster_tiles, map_tiles - total_monster_tiles);
    // */
}

#[derive(Hash,PartialEq,Eq,Clone,Copy,Debug)]
struct TileSideId {
    tile_id: usize,
    face: usize
}

#[derive(Hash,PartialEq,Eq,Clone,Copy,Debug)]
struct TileSide {
    id:TileSideId,
    edge: usize,
    edge_reversed: usize
}

struct Tile {
    id: usize,
    data:Vec<bool>,
    width: usize,
    height: usize
}

impl Tile {
    fn from_string(string:&str) -> Tile {
        let lines:Vec<&str> = string.lines().collect();
        let id:usize = lines[0][5..9].parse().unwrap();
        let mut arr:Vec<bool> = Vec::new();

        for line in lines.into_iter().skip(1) {
            for c in line.chars() {
                arr.push(c == '#');
            }
        }

        Tile { id: id, data: arr, width: 10, height: 10 }
    }

    fn from_size(id:usize, width:usize, height:usize) -> Tile {
        Tile {
            id: id,
            width: width,
            height: height,
            data: vec![false; width * height]
        }
    }

    fn monster() -> Tile {
        let mut output = Tile::from_size(0, 20, 3);

        // Too lazy to rewrite parse function.
        //                   # 
        // #    ##    ##    ###
        //  #  #  #  #  #  #   

        output.set(18, 0, true);
        output.set(0, 1, true);
        output.set(5, 1, true);
        output.set(6, 1, true);
        output.set(11, 1, true);
        output.set(12, 1, true);
        output.set(17, 1, true);
        output.set(18, 1, true);
        output.set(19, 1, true);
        output.set(1, 2, true);
        output.set(4, 2, true);
        output.set(7, 2, true);
        output.set(10, 2, true);
        output.set(13, 2, true);
        output.set(16, 2, true);

        output
    }

    fn is_set(&self, x:usize, y:usize) -> bool {
        let pos = (y * self.width) + x;
        self.data[pos]
    }

    fn set(&mut self, x:usize, y:usize, value:bool) {
        let pos = (y * self.width) + x;
        self.data[pos] = value;
    }

    fn total_set(&self) -> usize { 
        self.data.iter().map(|v| { if *v { 1 } else { 0 }  }).sum() 
    }

    fn print(&self) {
        println!("Tile {}:", self.id);

        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.is_set(x, y) { '#' } else { '.' };
                print!("{}", c);
            }
            println!();
        }
    }

    fn get_side(&self, side:usize) -> TileSide {
        let id = TileSideId { tile_id: self.id, face: side };
        match side {
            0 => {
                TileSide { 
                    id: id, // North
                    edge: self.get_line(&mut (0usize..self.width), &mut iter::repeat(0usize).take(self.height)),
                    edge_reversed: self.get_line(&mut (0usize..self.width).rev(), &mut iter::repeat(0usize).take(self.height))
                }
            },
            1 => {
                TileSide { 
                    id: id, // East
                    edge: self.get_line(&mut iter::repeat(self.width - 1).take(self.width), &mut (0usize..self.height)),
                    edge_reversed: self.get_line(&mut iter::repeat(self.width - 1).take(self.width), &mut (0usize..self.height).rev())
                }    
            },
            2 => {
                TileSide { 
                    id: id, // South
                    edge: self.get_line(&mut (0usize..self.width), &mut iter::repeat(self.height - 1).take(self.height)),
                    edge_reversed: self.get_line(&mut (0usize..self.width).rev(), &mut iter::repeat(self.height - 1).take(self.height))
                }    
            },
            3 => {
                TileSide { 
                    id: id, // West
                    edge: self.get_line(&mut iter::repeat(0usize).take(self.width), &mut (0usize..self.height)),
                    edge_reversed: self.get_line(&mut iter::repeat(0usize).take(self.width), &mut (0usize..self.height).rev())
                }
            },
            _ => unimplemented!()
        }
    }

    fn get_sides(&self) -> [TileSide; 4] {
        [
            self.get_side(0),
            self.get_side(1),
            self.get_side(2),
            self.get_side(3)
        ]
    }

    fn rotate(&self) -> Tile {
        self.mutate(|(x, y)| {  (y, self.width - x - 1) })
    }

    fn flip_horizontal(&self) -> Tile {
        self.mutate(|(x, y)| {  (self.width - x - 1, y) })
    }

    fn flip_vertical(&self) -> Tile {
        self.mutate(|(x, y)| {  (x, self.height - y - 1) })
    }

    fn clone(&self) -> Tile {
        self.mutate(|(x, y)| {  (x, y) })
    }

    fn remove_edges(&self) -> Tile {
        let mut output = Tile::from_size(self.id, self.width - 2, self.height - 2);
        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                let value = self.is_set(x, y);
                output.set(x - 1, y - 1, value);
            }
        }
        output
    }


    fn mutate<F>(&self, change_coords:F) -> Tile where
        F: Fn((usize, usize)) -> (usize, usize) {
        let mut output = Tile::from_size(self.id, self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let (dest_x, dest_y) = change_coords((x, y));
                let value = self.is_set(x, y);
                output.set(dest_x, dest_y, value);
            }
        }
        output
    }

    fn write_to(&self, tile:&mut Tile, x_offset:usize, y_offset:usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.is_set(x, y);
                tile.set(x + x_offset, y + y_offset, value);
            }
        }
    }

    // fn get_line<'a>(&self, x_iter:impl Iterator<Item = &'a usize>, y_iter:impl Iterator<Item = &'a usize>) -> usize {
    fn get_line<I1, I2>(&self, x_iter:&mut I1, y_iter:&mut I2) -> usize 
    where
        I1: Iterator<Item = usize>,
        I2: Iterator<Item = usize> {

        let mut value = 0;
        let mut mask = 1 << self.width;

        while let Some(x) = x_iter.next() {
            let y = y_iter.next().unwrap();

            if self.is_set(x, y) {
                value |= mask;
            }
            mask >>= 1;
        }

        value
    }

    fn rotate_until_north_no_match(&self, edges:&Vec<TileSide>) -> Tile {
        let north_edge = self.get_side(0);
        let any_matches = edges.iter().any(|other_edge| {  
            north_edge.id.tile_id != other_edge.id.tile_id
                && (north_edge.edge == other_edge.edge || north_edge.edge == other_edge.edge_reversed)
        });
        if !any_matches {
            self.clone()
        } else {
            self.rotate().rotate_until_north_no_match(edges)
        }
    }

    fn flip_until_west_no_match(&self, edges:&Vec<TileSide>) -> Tile {
        let west_edge = self.get_side(3);
        let any_matches = edges.iter().any(|other_edge| {
            west_edge.id.tile_id != other_edge.id.tile_id
                && (west_edge.edge == other_edge.edge || west_edge.edge == other_edge.edge_reversed)
        });
        if !any_matches {
            self.clone()
        } else {
            self.flip_horizontal()
        }
    }

    fn rotate_and_flip_to_match_edge(&self, edge:&TileSide, edge_to_match:usize, rotations_remaining:i32, flipped:bool) -> Tile {
        match rotations_remaining {
            0 => match flipped {
                false => {
                    self.flip_vertical().rotate_and_flip_to_match_edge(edge, edge_to_match, 4, true)
                },
                true => panic!("Could not match")
            },
            _ => {
                let side = self.get_side(edge_to_match);
                if side.edge == edge.edge {
                    self.clone()
                } else {
                    self.rotate().rotate_and_flip_to_match_edge(edge, edge_to_match, rotations_remaining - 1, flipped)
                }
            }
        }
    }

    fn overlay(&self, tile:&Tile, x_offset:usize, y_offset:usize) -> bool {
        for y in 0..tile.height {
            for x in 0..tile.width {
                if tile.is_set(x, y) {
                    if !self.is_set(x + x_offset, y + y_offset) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn scan_for_matches(&self, tile:&Tile) -> Tile {
        let mut output = Tile::from_size(0, self.width, self.height);
        for y_offset in 0..=(self.height - tile.height) {
            for x_offset in 0..=(self.width - tile.width) {
                if self.overlay(tile, x_offset, y_offset) {
                    tile.write_to(&mut output, x_offset, y_offset);
                }
            }
        }
        output
    }
}
