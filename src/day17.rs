
use std::collections::HashMap;
use std::collections::HashSet;

pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\17.txt").unwrap();
    let mut world = Space3d::new();
    
    for (y, line) in file.lines().enumerate() {
        for (x, cha) in line.chars().enumerate() {
            if cha == '#' {
                let point = Point3d::new(x, y);
                world.add_point(&point);
            }
        }
    }

    let test_point = Point3d::new(0, 0).surrounding_points();
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
struct Point3d {
    x: i32,
    y: i32,
    z: i32
}

impl Point3d {
    fn new(x: usize, y:usize) -> Point3d {
        Point3d {
            x: x as i32,
            y: y as i32,
            z: 0
        }
    }

    fn new_from_points(x:i32, y:i32, z:i32) -> Point3d {
        Point3d {
            x: x,
            y: y,
            z: z
        }
    }

    fn new_from_offset(&self, x: i32, y: i32, z: i32) -> Point3d {
        Point3d {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z
        }
    }

    fn surrounding_points(&self) -> Vec<Point3d> {
        // LAZY! LOW ENERGY!
        let mut output:Vec<Point3d> = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x != 0 || y != 0 || z != 0 {
                        output.push(self.new_from_offset(x, y, z));
                    }
                }
            }
        }

        output
    }

    fn remain_active(&self, world:&Space3d) -> bool {
        let active_neighbors_count = self.surrounding_points().iter().filter(|pt| { world.space_is_active(pt) }).count();
        let i_am_active = world.space_is_active(self);
        match i_am_active {
            true => active_neighbors_count == 2 || active_neighbors_count == 3,
            false => active_neighbors_count == 3
        }
    }
}

struct Space3d {
    // HashSet<i32> = z
    // HashMap<i32, z> = y
    // HashMap<i32, y> = x
    space: HashMap<i32,HashMap<i32,HashSet<i32>>>
}

impl Space3d {
    fn new() -> Space3d {
        Space3d {
            space: HashMap::new()
        }
    }

    fn add_point(&mut self, point:&Point3d) {
        let mut y_space = match self.space.remove(&point.x) {
            Some(y) => { y }
            None => { HashMap::new() }
        };

        let mut z_space = match y_space.remove(&point.y) {
            Some(z) => { z },
            None => { HashSet::new() }
        };

        z_space.insert(point.z);

        y_space.insert(point.y, z_space);

        self.space.insert(point.x, y_space);
    }

    fn space_is_active(&self, point:&Point3d) -> bool {
        match self.space.get(&point.x) {
            None => false,
            Some (x_space) => match x_space.get(&point.y) {
                None => false,
                Some (y_space) => y_space.contains(&point.z)
            }
        }
    }

    fn cube_count(&self) -> usize {
        self.space.values().fold(0, |yacc, x| {
            yacc + x.values().fold(0, |xacc, z| {
                xacc + z.len()
            })
        })
    }

    fn iterate(&self) -> Space3d {
        let mut new_world = Space3d::new();

        for (x, x_space) in self.space.iter() {
            for (y, y_space) in x_space.iter() {
                for z in y_space {
                    let point = Point3d::new_from_points(*x, *y, *z);
                    let points_to_check = point.surrounding_points();
                    for point in points_to_check {
                        if point.remain_active(self) {
                            new_world.add_point(&point);
                        }
                    }
                }
            }
        }

        new_world
    }
}