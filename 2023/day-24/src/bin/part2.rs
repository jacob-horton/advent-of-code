use std::ops::{Add, Div, Mul, Sub};

use itertools::Itertools;
use num::integer::Roots;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let result = process(input);
    println!("{result}");
}

#[derive(Debug, Clone, PartialEq, Copy)]
struct Vec3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<i128> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i128) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<i128> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i128) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Vec3 {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hailstone {
    position: Vec3,
    velocity: Vec3,
}

impl Hailstone {
    fn new(position: Vec3, velocity: Vec3) -> Self {
        Self { position, velocity }
    }
}

fn are_colinear(p1: Vec3, p2: Vec3, p3: Vec3) -> bool {
    let dp1 = p2 - p1;
    let dp2 = p3 - p2;
    let dp1x = dp1.x;
    let dp2x = dp2.x;

    // println!("{dp2x} {dp1x}");
    // Times instead of divide because integers
    let dp1 = dp1 / dp1x;
    let dp2 = dp2 / dp2x;

    dp1 == dp2
}

fn when_colinear(h1: &Hailstone, h2: &Hailstone, h3: &Hailstone) -> (u128, u128, u128) {
    for (t1, (t2, t3)) in (0..10000).cartesian_product((0..10000).cartesian_product(0..10000)) {
        let p1 = h1.position + h1.velocity * t1 as i128;
        let p2 = h2.position + h2.velocity * t2 as i128;
        let p3 = h3.position + h3.velocity * t3 as i128;

        if are_colinear(p1, p2, p3) {
            return (t1, t2, t3);
        }
    }

    panic!("Out of numbers");
}

fn dot(p1: &Vec3, p2: &Vec3) -> i128 {
    p1.x * p2.x + p1.y * p2.y + p1.z * p2.z
}

// fn cross(v1: &Vec3, v2: &Vec3) ->

fn parallel_and_opposite(p1: &Vec3, p2: &Vec3) -> bool {
    // Negative when opposite
    let dot = dot(p1, p2);

    dot < 0 && p1.y * p2.x == p2.y * p1.x && p1.z * p2.x == p2.z * p1.x
}

fn process(input: &str) -> u32 {
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
            Vec3::new(
                position_components[0],
                position_components[1],
                position_components[2],
            ),
            Vec3::new(
                velocity_components[0],
                velocity_components[1],
                velocity_components[2],
            ),
        ));
    }

    let mut distances = Vec::new();
    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i + 1) {
            let dist_sq = (h2.position.x - h1.position.x).pow(2)
                + (h2.position.y - h1.position.y).pow(2)
                + (h2.position.z - h1.position.z).pow(2);
            distances.push(dist_sq.sqrt());
        }
    }

    println!("{distances:?}");
    println!("{:?}", distances.iter().max());
    println!("{:?}", distances.iter().min());

    todo!()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_part2.txt");
        let result = process(input);
        assert_eq!(result, 0);
    }
}

//   4_028_848_039_494 min dist between hailstones
// 734_195_056_528_736 max dist between hailstones
