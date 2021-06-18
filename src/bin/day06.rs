use adventofcode2018::*;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn first(input: &[&str]) -> u32 {
    let points = parse(input);
    let limits = limits(&points);

    let regions = build_map(&points, limits);
    let regions = regions.values();

    let inner = regions.filter(|r| inner_region(r, limits));
    inner.map(|r| r.len()).max().unwrap() as u32
}

fn second(input: &[&str], max_distance: Option<i32>) -> u32 {
    const DISTANCE: i32 = 10000;
    let max_distance = max_distance.unwrap_or(DISTANCE);

    let points = parse(input);
    let limits = limits(&points);
    let ((minx, miny), (maxx, maxy)) = limits;

    let mut safe_points = 0;

    for y in miny..=maxy {
        for x in minx..=maxx {
            let distances = points.iter().map(|p| distance(*p, (x, y)));
            let safe = distances.sum::<i32>() < max_distance;

            if safe {
                safe_points += 1;
            }
        }
    }

    safe_points
}

type Point = (i32, i32);

fn inner_region(region: &[Point], limits: (Point, Point)) -> bool {
    let ((minx, miny), (maxx, maxy)) = limits;
    !region
        .iter()
        .any(|p| p.0 == minx || p.0 == maxx || p.1 == miny || p.1 == maxy)
}

fn build_map(points: &[Point], limits: (Point, Point)) -> HashMap<Point, Vec<Point>> {
    let mut map: HashMap<Point, Vec<Point>> = HashMap::new();
    let ((minx, miny), (maxx, maxy)) = limits;

    for y in miny..=maxy {
        for x in minx..=maxx {
            let p = (x, y);
            if let Some(closest) = find_min_distance(p, points) {
                if map.contains_key(closest) {
                    let v = map.get_mut(closest).unwrap();
                    v.push(p);
                } else {
                    let v = vec![p];
                    map.insert(*closest, v);
                }
            }
        }
    }

    map
}

fn find_min_distance(from: Point, to: &[Point]) -> Option<&Point> {
    let mut closest = to
        .iter()
        .map(|p| (p, distance(from, *p)))
        .collect::<Vec<(&Point, i32)>>();
    closest.sort_by_key(|p| p.1);
    let mut closest = closest.iter();
    let p = closest.next().unwrap();

    if p.1 == closest.next().unwrap().1 {
        None
    } else {
        Some(p.0)
    }
}

fn distance(a: Point, b: Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn limits(points: &[Point]) -> (Point, Point) {
    let minx = points.iter().map(|p| p.0).min().unwrap();
    let miny = points.iter().map(|p| p.1).min().unwrap();
    let maxx = points.iter().map(|p| p.0).max().unwrap();
    let maxy = points.iter().map(|p| p.1).max().unwrap();
    ((minx, miny), (maxx, maxy))
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+), (\d+)$").unwrap();
}

fn parse(input: &[&str]) -> Vec<Point> {
    input
        .iter()
        .map(|input| {
            let cap = RE.captures(input).unwrap();
            (
                parse_capture(&cap, 1, "line").unwrap(),
                parse_capture(&cap, 2, "line").unwrap(),
            )
        })
        .collect()
}

fn main() {
    let start = std::time::Instant::now();

    let input = read_file("06");
    let input: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", first(&input));

    println!("{}", second(&input, None));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            17,
            first(&vec!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"])
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            16,
            second(
                &vec!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"],
                Some(32)
            )
        );
    }
}
