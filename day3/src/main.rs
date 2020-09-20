#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

/*
    We assume that lines can not overlap over large parts so that there is always exactly
    one intersection point if there is one at all.
*/
fn intersects(line1: &Line, line2: &Line) -> Option<Point> {
    //Both lines have same orientation so we won't have an intersection
    if line1.start.x == line1.end.x && line2.start.x == line2.end.x {
        return None;
    } else if line1.start.y == line1.end.y && line2.start.y == line2.end.y {
        return None;
    }

    //Find out Horizontal and vertical line
    let (vertical, horizontal) = if line1.start.x != line1.end.x {
        (line2, line1)
    } else {
        (line1, line2)
    };

    //Sort by direction so that horizontal goes left to right and vertical goes top to bottom
    let horizontal = if horizontal.start.x > horizontal.end.x {
        Line {
            start: horizontal.end,
            end: horizontal.start,
        }
    } else {
        horizontal.clone()
    };
    let vertical = if vertical.start.y < vertical.end.y {
        Line {
            start: vertical.end,
            end: vertical.start,
        }
    } else {
        vertical.clone()
    };

    //Finally check if they are intersecting
    if horizontal.start.y < vertical.start.y
        && horizontal.start.y > vertical.end.y
        && vertical.start.x > horizontal.start.x
        && vertical.start.x < horizontal.end.x
    {
        return Some(Point {
            x: vertical.start.x,
            y: horizontal.start.y,
        });
    }
    return None;
}

fn manhatten_distance(p1: &Point, p2: &Point) -> usize {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as usize
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn move_point(start: &Point, modification: &str) -> Point {
    let mut stream = modification.chars();
    let direction = match stream.next().unwrap() {
        'R' => Direction::Right,
        'L' => Direction::Left,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => unreachable!(),
    };

    let rest: String = stream.collect();
    let step_count = rest.parse::<isize>().unwrap();

    match direction {
        Direction::Right => {
            return Point {
                x: start.x + step_count,
                y: start.y,
            }
        }
        Direction::Left => {
            return Point {
                x: start.x - step_count,
                y: start.y,
            }
        }
        Direction::Up => {
            return Point {
                x: start.x,
                y: start.y + step_count,
            }
        }
        Direction::Down => {
            return Point {
                x: start.x,
                y: start.y - step_count,
            }
        }
    }
}

fn parse(input: &str) -> (Vec<Line>, Vec<Line>) {
    let preprocess: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split(',').collect())
        .collect();
    assert!(preprocess.len() == 2);
    let mut start = Point { x: 0, y: 0 };
    let mut first_line = vec![];
    for command in &preprocess[0] {
        let end = move_point(&start, command);
        first_line.push(Line { start, end });
        start = end;
    }

    start = Point { x: 0, y: 0 };
    let mut second_line = vec![];
    for command in &preprocess[1] {
        let end = move_point(&start, command);
        second_line.push(Line { start, end });
        start = end;
    }

    (first_line, second_line)
}

fn closest_distance(center: &Point, others: &Vec<Point>) -> Option<usize> {
    others
        .iter()
        .map(|p| manhatten_distance(center, p))
        .min()
}


fn intersect_distance(line: &Line, point: &Point) -> Option<usize> {
    //Point on line in vertical direction.
    if line.start.x == line.end.x && point.x == line.start.x {
        return Some((line.start.y - point.y).abs() as usize)
    } else if line.start.y == line.end.y && point.y == line.start.y {
        return Some((line.start.x - point.x).abs() as usize)
    }
    None
}

fn calculate_distance_to_intersection(line: &Vec<Line>, intersection: &Point) -> Option<usize> {
    let mut sum_distance = 0;
    for part in line {
        if let Some(dist) = intersect_distance(part, intersection) {
            return Some(sum_distance + dist)
        }
        sum_distance += manhatten_distance(&part.start, &part.end); //length of the line is the same as the manhatten distance of its two points.
    }
    None
}

fn main() {
    let input = std::fs::read_to_string("input1.txt").expect("Input file not found");
    let parse_input = parse(&input);
    let mut all_intersections = vec![];
    for line in &parse_input.0 {
        for other_line in &parse_input.1 {
            if let Some(p) = intersects(line, other_line) {
                all_intersections.push(p);
            }
        }
    }

    //Part1
    println!("{:?}", closest_distance(&Point{x: 0, y: 0}, &all_intersections));

    //Part2
    let mut min_distance = usize::MAX;
    for intersection in all_intersections {
        let first_line_dist = calculate_distance_to_intersection(&parse_input.0, &intersection).unwrap();
        let second_line_dist = calculate_distance_to_intersection(&parse_input.1, &intersection).unwrap();
        let sum = first_line_dist + second_line_dist;
        if sum < min_distance {
            min_distance = sum;
        }
    }
    println!("{:?}", min_distance);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersection_test() {
        let line1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 8, y: 0 },
        };
        let line2 = Line {
            start: Point { x: 5, y: 5 },
            end: Point { x: 5, y: -5 },
        };
        assert_eq!(intersects(&line1, &line2), Some(Point { x: 5, y: 0 }));
        let line1 = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 8, y: 0 },
        };
        let line2 = Line {
            start: Point { x: 5, y: 5 },
            end: Point { x: 5, y: 1 },
        };
        assert_eq!(intersects(&line1, &line2), None);
    }
}
