use adventofcode2018::*;
use std::collections::HashMap;

fn first(depth: u32, target: (usize, usize)) -> u32 {
    let cave = Cave::new(depth, target);
    cave.risk_level()
}

fn second(depth: u32, target: (usize, usize)) -> u32 {
    let limits = (target.0 + 1000, target.1 + 1000);
    let cave = Cave::bounded(depth, target, limits);

    let start = Location {
        pos: (0, 0),
        tool: Tool::Torch,
        time: 0,
    };
    let mut visited = HashMap::new();
    visited.insert(start.pos, (start.tool, start.time));
    let mut current_loc = vec![start];
    let mut new_loc = Vec::new();

    loop {
        for loc in current_loc {
            let pos = loc.pos;
            let mut next_pos = Vec::new();

            if pos.0 > 0 {
                next_pos.push((pos.0 - 1, pos.1));
            }
            if pos.1 > 0 {
                next_pos.push((pos.0, pos.1 - 1))
            }
            if pos.0 < limits.0 {
                next_pos.push((pos.0 + 1, pos.1));
            }
            if pos.1 < limits.1 {
                next_pos.push((pos.0, pos.1 + 1));
            }

            let other_tool = valid_tools(cave.region_type(loc.pos));
            let other_tool = if other_tool[0] == loc.tool {
                other_tool[1]
            } else {
                other_tool[0]
            };

            for n in next_pos {
                let valid_tools = valid_tools(cave.region_type(n));
                let n = if valid_tools.contains(&loc.tool) {
                    Some(Location {
                        pos: n,
                        tool: loc.tool,
                        time: loc.time + 1,
                    })
                } else if valid_tools.contains(&other_tool) {
                    Some(Location {
                        pos: n,
                        tool: other_tool,
                        time: loc.time + 8,
                    })
                } else {
                    None
                };

                if let Some(n) = n {
                    let mut n = n;
                    if n.pos == target && n.tool != Tool::Torch {
                        n.time += 7;
                    }

                    let previous = |(tool, time): &&(Tool, u32)| {
                        let same_tool = *tool == n.tool && *time <= n.time;
                        let different_tool = *time + 7 <= n.time;
                        same_tool || different_tool
                    };

                    if visited.get(&n.pos).filter(previous).is_some() {
                        continue;
                    }
                    if visited
                        .get(&target)
                        .filter(|(_, time)| *time < n.time)
                        .is_some()
                    {
                        continue;
                    }

                    let mut insert = true;
                    if let Some((_, t)) = visited.get(&n.pos) {
                        if *t <= n.time {
                            insert = false;
                        }
                    }
                    if insert {
                        visited.insert(n.pos, (n.tool, n.time));
                    }
                    new_loc.retain(|l: &Location| {
                        l.pos != n.pos || (l.tool != n.tool && l.time - 7 < n.time)
                    });
                    new_loc.push(n);
                }
            }
        }

        if new_loc.is_empty() {
            return visited.get(&target).unwrap().1;
        }

        current_loc = new_loc;
        new_loc = Vec::new();
    }
}

#[derive(Copy, Clone)]
struct Location {
    pos: (usize, usize),
    tool: Tool,
    time: u32,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Tool {
    Torch,
    ClimbingGear,
    None,
}

fn valid_tools(t: RegionType) -> [Tool; 2] {
    match t {
        RegionType::Rocky => [Tool::Torch, Tool::ClimbingGear],
        RegionType::Wet => [Tool::ClimbingGear, Tool::None],
        RegionType::Narrow => [Tool::Torch, Tool::None],
    }
}

struct Cave {
    map: Vec<Vec<u32>>,
}

impl Cave {
    fn new(depth: u32, target: (usize, usize)) -> Cave {
        Cave::bounded(depth, target, target)
    }

    fn bounded(depth: u32, target: (usize, usize), bounds: (usize, usize)) -> Cave {
        let mut map: Vec<Vec<u32>> = Vec::with_capacity(bounds.0 + 1);

        for x in 0..=bounds.0 {
            let mut row = Vec::with_capacity(bounds.1 + 1);

            for y in 0..=bounds.1 {
                let index = match (x, y) {
                    (0, 0) => 0,
                    t if t == target => 0,
                    (x, 0) => x as u32 * 16807,
                    (0, y) => y as u32 * 48271,
                    (x, y) => map[x - 1][y] * row[y - 1],
                };

                let erosion = (index + depth) % 20183;
                row.push(erosion);
            }

            map.push(row);
        }

        Cave { map }
    }

    fn region_type(&self, coord: (usize, usize)) -> RegionType {
        erosion_to_type(self.map[coord.0][coord.1])
    }

    fn risk_level(&self) -> u32 {
        self.map
            .iter()
            .flat_map(|r| {
                r.iter().map(|l| match erosion_to_type(*l) {
                    RegionType::Wet => 1,
                    RegionType::Narrow => 2,
                    _ => 0,
                })
            })
            .sum()
    }
}

#[derive(PartialEq, Debug)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

fn erosion_to_type(erosion_level: u32) -> RegionType {
    match erosion_level % 3 {
        0 => RegionType::Rocky,
        1 => RegionType::Wet,
        _ => RegionType::Narrow,
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("22");
    let mut input = input.trim().split('\n');

    let depth = input.next().unwrap();
    let depth = depth.split(' ').last().unwrap();
    let depth: u32 = depth.parse().unwrap();

    let target = input.next().unwrap();
    let target = target.split(' ').last().unwrap();
    let mut target = target.split(',');
    let target = (target.next().unwrap(), target.next().unwrap());
    let target = (target.0.parse().unwrap(), target.1.parse().unwrap());

    println!("{}", first(depth, target));
    println!("{}", second(depth, target));
    // second(510, (10, 10));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let res = first(510, (10, 10));
        assert_eq!(114, res);
    }

    #[test]
    fn test2() {
        let res = second(510, (10, 10));
        assert_eq!(45, res);
    }
}
