use std::ops::{Add, Div, Mul, Sub};

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

    fn length(&self) -> i128 {
        (self.x.pow(2) + self.y.pow(2) + self.z.pow(2)).sqrt()
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

fn find_pairs(
    hailstones: &[Hailstone],
    component_extractor: Box<dyn Fn(&Vec3) -> i128>,
) -> Vec<(&Hailstone, &Hailstone)> {
    let mut result = Vec::new();

    for (i, h1) in hailstones.iter().enumerate() {
        for h2 in hailstones.iter().skip(i + 1) {
            if component_extractor(&h1.velocity) == component_extractor(&h2.velocity) {
                result.push((h1, h2));
            }
        }
    }

    result
}

fn process(input: &str) -> u128 {
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

    let mut rock_velocity = Vec::new();

    // Used https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/keq6s6s/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

    // Loop through x y and z separately
    for extractor in [|v: &Vec3| v.x, |v: &Vec3| v.y, |v: &Vec3| v.z] {
        let pairs = find_pairs(&hailstones, Box::new(extractor));
        let mut intersection = Vec::new();

        for (h1, h2) in pairs {
            // delta_dist % (rock.vel - hail.vel) = 0
            let delta_dist = extractor(&h1.position) - extractor(&h2.position);

            let possible_vels = (-1000..1000)
                .into_iter()
                .filter(|vel| {
                    vel != &extractor(&h1.velocity)
                        && delta_dist % (vel - extractor(&h1.velocity)) == 0
                })
                .collect();

            if intersection.is_empty() {
                intersection = possible_vels;
            } else {
                intersection.retain(|vel| possible_vels.contains(vel));
            }
        }

        assert!(intersection.len() == 1);
        rock_velocity.push(intersection.first().expect("No value found").clone());
    }

    let rock_velocity = Vec3::new(rock_velocity[0], rock_velocity[1], rock_velocity[2]);
    let h1 = &hailstones[0];
    let h2 = &hailstones[1];

    let v1 = h1.velocity - rock_velocity;
    let v2 = h2.velocity - rock_velocity;

    let m1 = v1.y as f64 / v1.x as f64;
    let m2 = v2.y as f64 / v2.x as f64;

    let c1 = h1.position.y as f64 - (m1 * h1.position.x as f64);
    let c2 = h2.position.y as f64 - (m2 * h2.position.x as f64);

    let x = (c2 - c1) / (m1 - m2);
    let y = m1 * x + c1;

    let t = (x - h1.position.x as f64) / (h1.velocity.x - rock_velocity.x) as f64;
    let z = h1.position.z as f64 + (h1.velocity.z - rock_velocity.z) as f64 * t;

    (x + y + z).round() as u128
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 618534564836937);
    }
}

//   4_028_848_039_494 min dist between hailstones
// 734_195_056_528_736 max dist between hailstones
// 13910 too low
// 826595352420860 too high
// 726595352420860 too high
// 528149559591284 wrong
// 624593147834250 wrong
