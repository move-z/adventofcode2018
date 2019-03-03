fn first(input: usize) -> (usize, usize) {
    let cells = build_cells(input);

    (1..=298)
        .flat_map(|x| {
            (1..=298).map(move |y| {
                (x, y)
            })
        })
        .max_by_key(|c| grid_power(&cells, c.0, c.1, 3))
        .unwrap()
}

fn second(input: usize) -> (usize, usize, usize) {
    let cells = build_cells(input);
    let c = &cells;

    (1..=298)
        .flat_map(|x| {
            (1..=298).flat_map(move |y| {
                let max_size = 300 - x.max(y) + 1;
                let mut p = 0;
                (1..=max_size).map(move |s| {
                    p = cached_grid_power(c, x, y, s, p);
                    (p, (x, y, s))
                })
            })
        })
        .max_by_key(|r| r.0)
        .unwrap().1
}

fn build_cells(serial: usize) -> Vec<Vec<isize>> {
    let mut cells = Vec::with_capacity(300);
    for y in 1..=300 {
        let mut row = Vec::with_capacity(300);
        for x in 1..=300 {
            row.push(power(x, y, serial));
        };
        cells.push(row);
    };
    cells
}

fn grid_power(cells: &Vec<Vec<isize>>, x: usize, y: usize, gridsize: usize) -> isize {
    let mut p = 0;
    for y in y..y + gridsize {
        for x in x..x + gridsize {
            p += cells[y - 1][x - 1];
        }
    }
    p
}

fn cached_grid_power(cells: &Vec<Vec<isize>>,
                     x: usize,
                     y: usize,
                     gridsize: usize,
                     prev: isize) -> isize {
    let power = if gridsize == 1 {
        cells[y - 1][x - 1].clone()
    } else {
        let mut p = prev;
        for x in x..x + gridsize {
            p += cells[y + gridsize - 2][x - 1];
        }
        for y in y..y + gridsize {
            p += cells[y - 1][x + gridsize - 2];
        }
        p
    };

    power
}

fn power(x: usize, y: usize, serial: usize) -> isize {
    let id = x + 10;
    let power = id * y;
    let power = power + serial;
    let power = power * id;
    let power = (power % 1000) / 100;
    power as isize - 5
}

fn main() {
    let start = std::time::Instant::now();

    println!("{:?}", first(5468));

    println!("{:?}", second(5468));

    println!("elapsed {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3_5_8() {
        assert_eq!(power(3, 5, 8), 4);
    }

    #[test]
    fn test_122_79_57() {
        assert_eq!(power(122, 79, 57), -5);
    }

    #[test]
    fn test_217_196_39() {
        assert_eq!(power(217, 196, 39), 0);
    }

    #[test]
    fn test() {
        let cells = build_cells(18);
        assert_eq!(grid_power(&cells, 33, 45, 3), 29);
    }

    #[test]
    fn test1_18() {
        assert_eq!(first(18), (33, 45));
    }

    #[test]
    fn test1_42() {
        assert_eq!(first(42), (21, 61));
    }

    #[test]
    fn test2_18() {
        assert_eq!(second(18), (90, 269, 16));
    }

    #[test]
    fn test2_42() {
        assert_eq!(second(42), (232, 251, 12));
    }
}
