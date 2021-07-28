use adventofcode2018::*;

fn first(depth: u32, target: (usize, usize)) -> u32 {
    let cave = Cave::new(depth, target);
    cave.risk_level()
}

fn second(depth: u32, target: (usize, usize)) -> u32 {
    todo!()
}

struct Cave {
    map: Box<Vec<Vec<u32>>>,
}

impl Cave {
    fn new(depth: u32, target: (usize, usize)) -> Cave {
        let mut map: Box<Vec<Vec<u32>>> = Box::new(Vec::with_capacity(target.0 + 1));

        for x in 0..=target.0 {
            let mut row = Vec::with_capacity(target.1 + 1);

            for y in 0..=target.1 {
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

    fn risk_level(&self) -> u32 {
        self.map.iter().flat_map(|r| r.iter().map(|l| {
            match erosion_to_type(*l) {
                RegionType::Wet => 1,
                RegionType::Narrow => 2,
                _ => 0,
            }
        })).sum()
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

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test00() {
    //     let cave = Cave { depth: 510, target: (10, 10) };
    //     let coord = (0, 0);
    //     assert_eq!(0, cave.geologic_index(coord));
    //     assert_eq!(510, cave.erosion_level(coord));
    //     assert_eq!(RegionType::Rocky, cave.region_type(coord));
    // }
    //
    // #[test]
    // fn test10() {
    //     let cave = Cave { depth: 510, target: (10, 10) };
    //     let coord = (1, 0);
    //     assert_eq!(16807, cave.geologic_index(coord));
    //     assert_eq!(17317, cave.erosion_level(coord));
    //     assert_eq!(RegionType::Wet, cave.region_type(coord));
    // }
    //
    // #[test]
    // fn test01() {
    //     let cave = Cave { depth: 510, target: (10, 10) };
    //     let coord = (0, 1);
    //     assert_eq!(48271, cave.geologic_index(coord));
    //     assert_eq!(8415, cave.erosion_level(coord));
    //     assert_eq!(RegionType::Rocky, cave.region_type(coord));
    // }
    //
    // #[test]
    // fn test11() {
    //     let cave = Cave { depth: 510, target: (10, 10) };
    //     let coord = (1, 1);
    //     assert_eq!(145722555, cave.geologic_index(coord));
    //     assert_eq!(1805, cave.erosion_level(coord));
    //     assert_eq!(RegionType::Narrow, cave.region_type(coord));
    // }
    //
    // #[test]
    // fn test1010() {
    //     let cave = Cave { depth: 510, target: (10, 10) };
    //     let coord = (10, 10);
    //     assert_eq!(0, cave.geologic_index(coord));
    //     assert_eq!(510, cave.erosion_level(coord));
    //     assert_eq!(RegionType::Rocky, cave.region_type(coord));
    // }
    //
    #[test]
    fn test1() {
        let cave = Cave::new(510, (10, 10));
        assert_eq!(114, cave.risk_level());
    }
}
