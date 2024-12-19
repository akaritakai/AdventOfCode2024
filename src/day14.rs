use crate::puzzle::Puzzle;
use lazy_regex::regex_captures;

pub struct Day {
    robots: Vec<Robot>,
}

impl Puzzle for Day {
    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_1(&self) -> String {
        calculate_safety_factor(&self.robots, 101, 103, 100).to_string()
    }

    /// TODO
    ///
    /// Time complexity: TODO
    /// Auxiliary space complexity: TODO
    fn solve_part_2(&self) -> String {
        find_xmas_tree(&self.robots, 101, 103).to_string()
    }
}

struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

fn perform_move(robots: &[Robot], width: u64, height: u64, time: u64) -> Vec<(u64, u64)> {
    robots
        .iter()
        .map(|robot| {
            (
                (robot.p.0 + time as i64 * robot.v.0).rem_euclid(width as i64) as u64,
                (robot.p.1 + time as i64 * robot.v.1).rem_euclid(height as i64) as u64,
            )
        })
        .collect()
}

fn calculate_safety_factor(robots: &[Robot], width: u64, height: u64, time: u64) -> u64 {
    perform_move(robots, width, height, time)
        .iter()
        .map(|(p_x, p_y)| {
            let x_mid = width / 2;
            let y_mid = height / 2;
            if *p_x < x_mid && *p_y < y_mid {
                (1, 0, 0, 0)
            } else if *p_x < x_mid && *p_y > y_mid {
                (0, 1, 0, 0)
            } else if *p_x > x_mid && *p_y < y_mid {
                (0, 0, 1, 0)
            } else if *p_x > x_mid && *p_y > y_mid {
                (0, 0, 0, 1)
            } else {
                (0, 0, 0, 0)
            }
        })
        .reduce(|(a1, b1, c1, d1), (a2, b2, c2, d2)| (a1 + a2, b1 + b2, c1 + c2, d1 + d2))
        .map(|(a, b, c, d)| a * b * c * d)
        .unwrap()
}

fn find_xmas_tree(robots: &[Robot], width: u64, height: u64) -> u64 {
    let mut x_offset = 0;
    let mut y_offset = 0;
    for time in 0.. {
        let positions = perform_move(robots, width, height, time);
        let (dev_x, dev_y) = mean_absolute_deviation(&positions);
        if dev_x < 0.35 {
            x_offset = time;
        }
        if dev_y < 0.35 {
            y_offset = time;
        }
        if x_offset != 0 && y_offset != 0 {
            break;
        }
    }
    for i in 0.. {
        let time = x_offset + i * width;
        if (time - y_offset).rem_euclid(height) == 0 {
            return time;
        }
    }
    unreachable!()
}

fn mean_absolute_deviation(positions: &[(u64, u64)]) -> (f64, f64) {
    let mut mean_x = 0f64;
    let mut mean_y = 0f64;
    for (p_x, p_y) in positions {
        mean_x += *p_x as f64;
        mean_y += *p_y as f64;
    }
    mean_x /= positions.len() as f64;
    mean_y /= positions.len() as f64;
    let mut dev_x = 0f64;
    let mut dev_y = 0f64;
    for (p_x, p_y) in positions {
        dev_x += (*p_x as f64 - mean_x).abs();
        dev_y += (*p_y as f64 - mean_y).abs();
    }
    dev_x /= mean_x * positions.len() as f64;
    dev_y /= mean_y * positions.len() as f64;
    (dev_x, dev_y)
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Puzzle> {
        let robots = input
            .lines()
            .map(|line| {
                let (_, p_x, p_y, v_x, v_y) =
                    regex_captures!(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)", line).unwrap();
                Robot {
                    p: (p_x.parse().unwrap(), p_y.parse().unwrap()),
                    v: (v_x.parse().unwrap(), v_y.parse().unwrap()),
                }
            })
            .collect();
        Box::new(Day { robots })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_part_1_example_1() {
        let robots = vec![
            Robot {
                p: (0, 4),
                v: (3, -3),
            },
            Robot {
                p: (6, 3),
                v: (-1, -3),
            },
            Robot {
                p: (10, 3),
                v: (-1, 2),
            },
            Robot {
                p: (2, 0),
                v: (2, -1),
            },
            Robot {
                p: (0, 0),
                v: (1, 3),
            },
            Robot {
                p: (3, 0),
                v: (-2, -2),
            },
            Robot {
                p: (7, 6),
                v: (-1, -3),
            },
            Robot {
                p: (3, 0),
                v: (-1, -2),
            },
            Robot {
                p: (9, 3),
                v: (2, 3),
            },
            Robot {
                p: (7, 3),
                v: (-1, 2),
            },
            Robot {
                p: (2, 4),
                v: (2, -3),
            },
            Robot {
                p: (9, 5),
                v: (-3, -3),
            },
        ];
        let safety_factor = calculate_safety_factor(&robots, 11, 7, 100);
        assert_eq!(safety_factor, 12);
    }

    #[test]
    fn test_solve_part_1() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/14")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_1(), "232589280");
    }

    #[test]
    fn test_solve_part_2() {
        let input = std::fs::read_to_string(PathBuf::from("resources/tests/14")).unwrap();
        let puzzle = Day::create(&input);
        assert_eq!(puzzle.solve_part_2(), "7569");
    }
}
