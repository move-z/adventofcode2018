use adventofcode2018::*;
use std::collections::HashMap;

fn first(input: &[&str]) -> usize {
    let mut area = CollectionArea::parse(input).unwrap();
    for _ in 0..10 {
        area = area.next();
    }
    area.resource_value()
}

fn second(input: &[&str]) -> usize {
    const ITERATIONS: usize = 1_000_000_000;
    let mut area = CollectionArea::parse(input).unwrap();

    let mut cache = HashMap::new();
    let mut res_cache = Vec::new();
    let mut idx = 0;
    loop {
        if idx == ITERATIONS {
            return area.resource_value();
        }

        if cache.contains_key(&area) {
            break;
        }

        let val = area.resource_value();
        res_cache.push(val);
        cache.insert(area.clone(), idx);

        idx += 1;
        area = area.next();
    }

    let loop_start = cache.get(&area).unwrap();
    let loop_size = idx - loop_start;
    let remainder = (ITERATIONS - loop_start) % loop_size;

    res_cache[loop_start + remainder]
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}

impl Acre {
    fn parse(input: &char) -> Result<Acre, String> {
        match input {
            '.' => Ok(Acre::OpenGround),
            '|' => Ok(Acre::Trees),
            '#' => Ok(Acre::Lumberyard),
            c => Err(format!("can't parse {} as Acre", c)),
        }
    }

    fn next_state(&self, adjacent: &[&Acre]) -> Acre {
        let count = |t| adjacent.iter().filter(|&&&a| a == t).count();
        match self {
            Acre::OpenGround => {
                if count(Acre::Trees) >= 3 {
                    Acre::Trees
                } else {
                    Acre::OpenGround
                }
            }
            Acre::Trees => {
                if count(Acre::Lumberyard) >= 3 {
                    Acre::Lumberyard
                } else {
                    Acre::Trees
                }
            }
            Acre::Lumberyard => {
                if count(Acre::Lumberyard) >= 1 && count(Acre::Trees) >= 1 {
                    Acre::Lumberyard
                } else {
                    Acre::OpenGround
                }
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct CollectionArea {
    inner: Vec<Vec<Acre>>,
}

impl CollectionArea {
    fn parse(input: &[&str]) -> Result<CollectionArea, String> {
        let inner = input
            .iter()
            .map(|line| line.chars().map(|c| Acre::parse(&c)).collect())
            .collect::<Result<Vec<Vec<Acre>>, String>>()?;
        Ok(CollectionArea { inner })
    }

    fn next(&self) -> CollectionArea {
        let new = self
            .inner
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(|(col, a)| {
                        let adj = self.adjacent(row, col);
                        a.next_state(&adj)
                    })
                    .collect()
            })
            .collect();
        CollectionArea { inner: new }
    }

    fn adjacent(&self, row: usize, col: usize) -> Vec<&Acre> {
        let mut res = Vec::new();

        let minr = if row > 0 { row - 1 } else { row };
        let maxr = if row < self.inner.len() - 1 {
            row + 1
        } else {
            row
        };
        for curr_row in minr..=maxr {
            let line = &self.inner[curr_row];
            let minc = if col > 0 { col - 1 } else { col };
            let maxc = if col < line.len() - 1 { col + 1 } else { col };
            for (curr_col, acre) in line.iter().enumerate().take(maxc + 1).skip(minc) {
                if curr_row != row || curr_col != col {
                    res.push(acre);
                }
            }
        }
        res
    }

    fn resource_value(&self) -> usize {
        let mut trees = 0;
        let mut lumberyards = 0;
        for acre in self.iter() {
            match acre {
                Acre::OpenGround => {}
                Acre::Trees => trees += 1,
                Acre::Lumberyard => lumberyards += 1,
            }
        }
        trees * lumberyards
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Acre> + '_> {
        Box::new(self.inner.iter().flatten().copied())
    }
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("18");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));
    println!("{}", second(&input));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref INITIAL_STATE: Vec<&'static str> = vec![
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ];
    }

    #[test]
    fn test() {
        let area = CollectionArea::parse(&INITIAL_STATE).unwrap();

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = ".......##.......|###.|..|...#...|#||...#..##||.|#|...#||||..||...|||..|||||.||.|||||||||||....||..|."
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 1 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = ".......#........|#...|.|||......##|||..#..###|||#|...#|||||.|||||||||.||||||||||||||||||||.|||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 2 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = ".......#......|||#...|.||||.....###|||.#...##|||#|.||##|||||||||||||||||||||||||||||||||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 3 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = ".....|.#.....||||#...|.#||||....###||||#...###||#||||##|||||||||||||||||||||||||||||||||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 4 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = "....|||#.....||||#...|.##||||...####|||#.|.###||#||||###||||||||||||||||||||||||||||||||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 5 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = "...||||#.....||||#...|.###|||...#.##|||#|||#.##|#||||###||||||||#|||||||||||||||||||||||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 6 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = "...||||#....||#|##...|.####||.||#..##||#||##.##|#||||####||||||###||||||||||||||||||||||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 7 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = "..||||##....|#####..|||#####|.||#...##|#||##..###|||##.###|||||####|||||||#|||||||||||||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 8 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = "..||###....||#####..||##...##.||#....###|##....##|||##..###|||######|||||###||||||||||||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 9 with {:?}", state);

        let area = area.next();
        let state: Vec<Acre> = area.iter().collect();
        let expected: Vec<Acre> = ".||##.....||###.....||##......|##.....##|##.....##|##....##|||##.####|||#####|||||||#|||||||||||||||"
            .chars().map(|c| Acre::parse(&c).unwrap()).collect();
        assert_eq!(state, expected, "failed on step 10 with {:?}", state);

        assert_eq!(area.resource_value(), 1147);
    }
}
