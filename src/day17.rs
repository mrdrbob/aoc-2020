
use std::collections::HashMap;
use std::collections::HashSet;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\17.txt").unwrap();
    let mut world = Space4d::new();
    
    for (y, line) in file.lines().enumerate() {
        for (x, cha) in line.chars().enumerate() {
            if cha == '#' {
                let point = Point4d::new(x, y);
                world.add_point(&point);
            }
        }
    }

    let test_point = Point4d::new(0, 0).surrounding_points();
    println!("{}", test_point.len());

    let active_cubes = world.cube_count();
    println!("Starting cubes: {}", active_cubes);
    
    for _ in 0..6 {
        world = world.iterate();
    }

    let active_cubes = world.cube_count();
    println!("Ending cubes: {}", active_cubes);
}

#[derive(PartialEq, Debug, Clone)]
struct Point4d {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

impl Point4d {
    fn new(x: usize, y:usize) -> Point4d {
        Point4d {
            x: x as i32,
            y: y as i32,
            z: 0,
            w: 0
        }
    }

    fn new_from_points(x:i32, y:i32, z:i32, w:i32) -> Point4d {
        Point4d {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    fn new_from_offset(&self, x: i32, y: i32, z: i32, w:i32) -> Point4d {
        Point4d {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
            w: self.w + w
        }
    }

    fn surrounding_points(&self) -> Vec<Point4d> {
        // LAZY! LOW ENERGY!
        let mut output:Vec<Point4d> = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x != 0 || y != 0 || z != 0 || w != 0 {
                            output.push(self.new_from_offset(x, y, z, w));
                        }
                    }
                }
            }
        }

        output
    }

    fn remain_active(&self, world:&Space4d) -> bool {
        let active_neighbors_count = self.surrounding_points().iter().filter(|pt| { world.space_is_active(pt) }).count();
        let i_am_active = world.space_is_active(self);
        match i_am_active {
            true => active_neighbors_count == 2 || active_neighbors_count == 3,
            false => active_neighbors_count == 3
        }
    }
}

struct Space4d {
    // HashSet<i32> = w
    // HashMap<i32, w> = z
    // HashMap<i32, z> = y
    // HashMap<i32, y> = x
    space: HashMap<i32,HashMap<i32,HashMap<i32,HashSet<i32>>>>
}

impl Space4d {
    fn new() -> Space4d {
        Space4d {
            space: HashMap::new()
        }
    }

    fn add_point(&mut self, point:&Point4d) {
        let mut y_space = match self.space.remove(&point.x) {
            Some(y) => { y }
            None => { HashMap::new() }
        };

        let mut z_space = match y_space.remove(&point.y) {
            Some(z) => { z },
            None => { HashMap::new() }
        };

        let mut w_space = match z_space.remove(&point.z) {
            Some (w) => { w },
            None => { HashSet::new() }
        };

        w_space.insert(point.w);

        z_space.insert(point.z, w_space);

        y_space.insert(point.y, z_space);

        self.space.insert(point.x, y_space);
    }

    fn space_is_active(&self, point:&Point4d) -> bool {
        match self.space.get(&point.x) {
            None => false,
            Some (x_space) => match x_space.get(&point.y) {
                None => false,
                Some (y_space) => match y_space.get(&point.z) {
                    None => false,
                    Some (z_space) => z_space.contains(&point.w)
                }
            }
        }
    }

    fn cube_count(&self) -> usize {
        self.space.values().fold(0, |yacc, x| {
            yacc + x.values().fold(0, |xacc, z| {
                xacc + z.values().fold(0, |zacc, w| {
                    zacc + w.len()
                })
            })
        })
    }

    fn iterate(&self) -> Space4d {
        let mut new_world = Space4d::new();

        for (x, x_space) in self.space.iter() {
            for (y, y_space) in x_space.iter() {
                for (z, z_space) in y_space.iter() {
                    for w in z_space {
                        let point = Point4d::new_from_points(*x, *y, *z, *w);
                        let points_to_check = point.surrounding_points();
                        for point in points_to_check {
                            if point.remain_active(self) {
                                new_world.add_point(&point);
                            }
                        }
                    }
                }
            }
        }

        new_world
    }
}