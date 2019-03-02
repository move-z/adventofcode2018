use adventofcode2018::*;

use lazy_static::lazy_static;
use regex::Regex;

fn first(input: &Vec<&str>) {
    let points = parse(input);

    let candidate = (0..50000).map(|t| {
        let conf = conf_at_t(&points, t);
        let minmax = min_max(&conf);
        (conf, minmax)
    }).min_by_key(|c| {
        let minmax = &c.1;
        (minmax.1.x - minmax.0.x) * (minmax.1.y - minmax.0.y)
    }).unwrap();

    let (conf, (min, max)) = candidate;
    print(&conf, min, max);
}

fn second(input: &Vec<&str>) {
    let points = parse(input);

    let candidate = (0..50000).map(|t| {
        let conf = conf_at_t(&points, t);
        let minmax = min_max(&conf);
        (conf, minmax, t)
    }).min_by_key(|c| {
        let minmax = &c.1;
        (minmax.1.x - minmax.0.x) * (minmax.1.y - minmax.0.y)
    }).unwrap();

    let (conf, (min, max), t) = candidate;
    println!(">>>> TIME={}", t);
    print(&conf, min, max);
}

struct Coord {
    x: isize,
    y: isize,
}

fn min_max(conf: &Vec<Coord>) -> (Coord, Coord) {
    conf.iter().fold(None, |cur: Option<(Coord, Coord)>, point| {
        let minmax = match cur {
            Some((min, max)) => {
                let minx = min.x.min(point.x);
                let miny = min.y.min(point.y);
                let maxx = max.x.max(point.x);
                let maxy = max.y.max(point.y);
                (minx, miny, maxx, maxy)
            }
            None => {
                (point.x, point.y, point.x, point.y)
            }
        };
        let min = Coord {
            x: minmax.0,
            y: minmax.1,
        };
        let max = Coord {
            x: minmax.2,
            y: minmax.3,
        };
        Some((min, max))
    }).unwrap()
}

fn print(conf: &Vec<Coord>, min: Coord, max: Coord) {
    let width = (max.x - min.x) as usize + 3;
    let height = (max.y - min.y) as usize + 3;
    let separator = "-".repeat(width);

    println!("{}", separator);

    let mut screen = vec![vec!['.'; width]; height];

    conf.iter().for_each(|c| {
        let rownum = (c.y - min.y) as usize + 1;
        let colnum = (c.x - min.x) as usize + 1;
        let row = screen.get_mut(rownum).unwrap();
        let char = row.get_mut(colnum).unwrap();
        *char = '#';
    });

    for row in screen {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }

    println!("{}", separator);
}

struct Point {
    start_coord: Coord,
    speedx: isize,
    speedy: isize,
}

impl Point {
    const fn pos_at_time(&self, t: usize) -> Coord {
        const fn pos_at_t(start: isize, speed: isize, t: usize) -> isize {
            start + speed * t as isize
        }

        let x = pos_at_t(self.start_coord.x, self.speedx, t);
        let y = pos_at_t(self.start_coord.y, self.speedy, t);
        Coord {
            x,
            y,
        }
    }
}

fn conf_at_t(points: &Vec<Point>, t: usize) -> Vec<Coord> {
    points.iter().map(|p| p.pos_at_time(t)).collect::<Vec<Coord>>()
}

fn parse(input: &Vec<&str>) -> Vec<Point> {
    input.iter().map(|s| {
        RE.captures(s).map(|c| {
            let startx = parse_capture::<isize>(&c, 1, "startx").unwrap();
            let starty = parse_capture::<isize>(&c, 2, "starty").unwrap();
            let speedx = parse_capture::<isize>(&c, 3, "speedx").unwrap();
            let speedy = parse_capture::<isize>(&c, 4, "speedy").unwrap();
            let start_coord = Coord {
                x: startx,
                y: starty,
            };

            Point {
                start_coord,
                speedx,
                speedy,
            }
        }).expect(format!("failed to parse {}", s).as_str())
    }).collect()
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>$").unwrap();
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("10");
    let input: Vec<&str> = input.trim().split("\n").collect();

    first(&input);

    second(&input);

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        first(&vec![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>"]);
    }
}
