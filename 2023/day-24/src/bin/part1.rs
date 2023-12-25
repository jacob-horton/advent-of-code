fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input, TestArea::new(200000000000000.0, 400000000000000.0));
    println!("{result}");
}

#[derive(Debug, Clone, PartialEq)]
struct TestArea {
    start: f64,
    end: f64,
}

impl TestArea {
    fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hailstone {
    position: Vec2,
    velocity: Vec2,
}

impl Hailstone {
    fn new(position: Vec2, velocity: Vec2) -> Self {
        Self { position, velocity }
    }

    fn do_paths_intersect(&self, other: &Hailstone, test_area: &TestArea) -> bool {
        // Find time of each hailstone when paths cross
        let top = self.velocity.x * (other.position.y - self.position.y)
            - self.velocity.y * (other.position.x - self.position.x);
        let bottom = self.velocity.y * other.velocity.x - self.velocity.x * other.velocity.y;

        // Time when other reaches intersection point
        let t2 = top / bottom;

        // Time when self reaches intersection point
        let t1 = (other.position.x - self.position.x + other.velocity.x * t2) / self.velocity.x;

        if t1 < 0.0 || t2 < 0.0 {
            return false;
        }

        let position_x = self.position.x + self.velocity.x * t1;
        let position_y = self.position.y + self.velocity.y * t1;

        if position_x < test_area.start
            || position_x > test_area.end
            || position_y < test_area.start
            || position_y > test_area.end
        {
            return false;
        }

        true
    }
}

fn process(input: &str, test_area: TestArea) -> u32 {
    let mut hailstones = Vec::new();
    for line in input.trim().split('\n') {
        let (position, velocity) = line.split_once(" @ ").unwrap();
        let position_components: Vec<_> = position
            .split(',')
            .map(|p| p.trim().parse().unwrap())
            .collect();
        let velocity_components: Vec<_> = velocity
            .split(',')
            .map(|v| v.trim().parse().unwrap())
            .collect();

        hailstones.push(Hailstone::new(
            Vec2::new(position_components[0], position_components[1]),
            Vec2::new(velocity_components[0], velocity_components[1]),
        ));
    }

    let mut count = 0;
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i + 1) {
            if h1.do_paths_intersect(&h2, &test_area) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part1.txt");
        let result = process(input, TestArea::new(7.0, 27.0));
        assert_eq!(result, 2);
    }
}
