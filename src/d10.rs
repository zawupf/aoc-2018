use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Position(i32, i32);

#[derive(PartialEq, Debug)]
struct Velocity(i32, i32);

#[derive(PartialEq, Debug)]
struct Light {
    position: Position,
    velocity: Velocity,
}

struct BBox {
    min: Position,
    max: Position,
}

impl BBox {
    fn merge(&mut self, position: &Position) -> &mut Self {
        let &Position(x, y) = position;
        if x < self.min.0 {
            self.min.0 = x;
        }
        if x > self.max.0 {
            self.max.0 = x;
        }
        if y < self.min.1 {
            self.min.1 = y;
        }
        if y > self.max.1 {
            self.max.1 = y;
        }
        self
    }
}

impl Light {
    fn new((px, py): (i32, i32), (vx, vy): (i32, i32)) -> Light {
        Light {
            position: Position(px, py),
            velocity: Velocity(vx, vy),
        }
    }

    fn forward(&mut self) -> &mut Self {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self
    }

    fn backward(&mut self) -> &mut Self {
        self.position.0 -= self.velocity.0;
        self.position.1 -= self.velocity.1;
        self
    }
}

#[derive(PartialEq, Debug)]
pub struct ParseLightError();

impl From<ParseIntError> for ParseLightError {
    fn from(_error: ParseIntError) -> Self {
        ParseLightError()
    }
}

impl FromStr for Light {
    type Err = ParseLightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // position=< 9,  1> velocity=< 0,  2>
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"position=<\s*(-?\d+)\s*,\s*(-?\d+)\s*>\s+velocity=<\s*(-?\d+)\s*,\s*(-?\d+)\s*>"
            )
            .unwrap();
        }

        let caps = RE.captures(s).ok_or(ParseLightError())?;
        let px: i32 = caps[1].parse()?;
        let py: i32 = caps[2].parse()?;
        let vx: i32 = caps[3].parse()?;
        let vy: i32 = caps[4].parse()?;

        Ok(Light::new((px, py), (vx, vy)))
    }
}

trait LightShow {
    fn forward(&mut self) -> &mut Self;
    fn backward(&mut self) -> &mut Self;
    fn row_count(&self) -> usize;
    fn bounding_box(&self) -> BBox;
    fn to_strings(&self) -> Vec<String>;

    fn fast_forward_to_message(&mut self) -> (&mut Self, usize) {
        let mut prev_row_count = self.row_count();
        let mut seconds = 0;

        loop {
            let row_count = self.forward().row_count();
            if row_count > prev_row_count {
                self.backward();
                break;
            } else {
                prev_row_count = row_count;
                seconds += 1;
            }
        }

        (self, seconds)
    }
}

impl LightShow for [Light] {
    fn forward(&mut self) -> &mut Self {
        self.iter_mut().for_each(|light| {
            light.forward();
        });
        self
    }

    fn backward(&mut self) -> &mut Self {
        self.iter_mut().for_each(|light| {
            light.backward();
        });
        self
    }

    fn row_count(&self) -> usize {
        self.iter()
            .fold(HashSet::new(), |mut y_set, light| {
                y_set.insert(light.position.1);
                y_set
            })
            .len()
    }

    fn bounding_box(&self) -> BBox {
        use std::i32::{MAX, MIN};
        let mut bbox = BBox {
            min: Position(MAX, MAX),
            max: Position(MIN, MIN),
        };
        self.iter().for_each(|light| {
            bbox.merge(&light.position);
        });
        bbox
    }

    fn to_strings(&self) -> Vec<String> {
        let bbox = self.bounding_box();
        let Position(x_offset, y_offset) = bbox.min;
        let (x_offset, y_offset) = (x_offset as usize, y_offset as usize);
        let (width, height) = (
            bbox.max.0 as usize - x_offset + 1,
            bbox.max.1 as usize - y_offset + 1,
        );
        let mut grid = vec![vec!['.'; width as usize]; height as usize];
        self.iter().for_each(|light| {
            let &Position(x, y) = &light.position;
            let (x, y) = (x as usize, y as usize);
            grid[y - y_offset][x - x_offset] = '#';
        });
        grid.into_iter()
            .map(|line| String::from_iter(line.into_iter()))
            .collect()
    }
}

pub fn message_and_seconds(input: &[&str]) -> (String, usize) {
    let mut lights: Vec<Light> = input.iter().map(|light| light.parse().unwrap()).collect();
    let (lights, seconds) = lights.fast_forward_to_message();
    (lights.to_strings().join("\n"), seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &[&'static str] = &[
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
        "position=<-3,  6> velocity=< 2, -1>",
    ];

    fn lights() -> Vec<Light> {
        DATA.iter().map(|light| light.parse().unwrap()).collect()
    }

    #[test]
    fn parse_light() {
        assert_eq!(
            Ok(Light::new((9, 1), (0, 2))),
            "position=< 9,  1> velocity=< 0,  2>".parse()
        );
        assert_eq!(
            Ok(Light::new((3, -2), (-1, 1))),
            "position=< 3, -2> velocity=<-1,  1>".parse()
        );
        assert_eq!(
            Err(ParseLightError()),
            "position=< 3, -2> velocity= <-1,  1>".parse::<Light>()
        );
        assert_eq!(
            lights(),
            vec![
                Light::new((9, 1), (0, 2)),
                Light::new((7, 0), (-1, 0)),
                Light::new((3, -2), (-1, 1)),
                Light::new((6, 10), (-2, -1)),
                Light::new((2, -4), (2, 2)),
                Light::new((-6, 10), (2, -2)),
                Light::new((1, 8), (1, -1)),
                Light::new((1, 7), (1, 0)),
                Light::new((-3, 11), (1, -2)),
                Light::new((7, 6), (-1, -1)),
                Light::new((-2, 3), (1, 0)),
                Light::new((-4, 3), (2, 0)),
                Light::new((10, -3), (-1, 1)),
                Light::new((5, 11), (1, -2)),
                Light::new((4, 7), (0, -1)),
                Light::new((8, -2), (0, 1)),
                Light::new((15, 0), (-2, 0)),
                Light::new((1, 6), (1, 0)),
                Light::new((8, 9), (0, -1)),
                Light::new((3, 3), (-1, 1)),
                Light::new((0, 5), (0, -1)),
                Light::new((-2, 2), (2, 0)),
                Light::new((5, -2), (1, 2)),
                Light::new((1, 4), (2, 1)),
                Light::new((-2, 7), (2, -2)),
                Light::new((3, 6), (-1, -1)),
                Light::new((5, 0), (1, 0)),
                Light::new((-6, 0), (2, 0)),
                Light::new((5, 9), (1, -2)),
                Light::new((14, 7), (-2, 0)),
                Light::new((-3, 6), (2, -1)),
            ]
        );
    }

    #[test]
    fn lightshow_forward() {
        let mut lights = lights();
        assert_eq!(15, lights.row_count());
        lights.forward();
        assert_eq!(11, lights.row_count());
        lights.forward();
        assert_eq!(9, lights.row_count());
        lights.forward();
        assert_eq!(8, lights.row_count());
        lights.forward();
        assert_eq!(11, lights.row_count());
    }

    #[test]
    fn fast_forward_to_message() {
        assert_eq!(3, lights().fast_forward_to_message().1);
        assert_eq!(8, lights().fast_forward_to_message().0.row_count());
        assert_eq!(
            lights().fast_forward_to_message().0.to_strings(),
            vec![
                "#...#..###".to_owned(),
                "#...#...#.".to_owned(),
                "#...#...#.".to_owned(),
                "#####...#.".to_owned(),
                "#...#...#.".to_owned(),
                "#...#...#.".to_owned(),
                "#...#...#.".to_owned(),
                "#...#..###".to_owned(),
            ]
        );
    }

    #[test]
    fn message_and_seconds() {
        assert_eq!(3, super::message_and_seconds(DATA).1);
        assert_eq!(
            super::message_and_seconds(DATA).0,
            [
                "#...#..###",
                "#...#...#.",
                "#...#...#.",
                "#####...#.",
                "#...#...#.",
                "#...#...#.",
                "#...#...#.",
                "#...#..###",
            ]
            .join("\n")
        );
    }
}
