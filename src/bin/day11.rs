use adventofcode2018::*;

fn first(input: usize) -> (usize, usize) {
    let r =
        (1..=298)
            .flat_map(|x| (1..=298).map(move |y| (x, y)))
            .max_by_key(|c| {
                let mut p = 0;
                for x in c.0..c.0+3 {
                    for y in c.1..c.1+3 {
                        p += power(x, y, input);
                    }
                }
                p
            })
            .unwrap();
    r
}

fn second(_input: &Vec<&str>) -> u32 {
    unimplemented!()
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
    fn test_101_153_71() {
        assert_eq!(power(101, 153, 71), 4);
    }

    #[test]
    fn test1_18() {
        assert_eq!(first(18), (33, 45));
    }

    #[test]
    fn test1_42() {
        assert_eq!(first(42), (21, 61));
    }
}
