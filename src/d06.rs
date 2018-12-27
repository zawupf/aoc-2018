use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Pt2 {
    x: i32,
    y: i32,
}

impl Pt2 {
    pub fn new(x: i32, y: i32) -> Pt2 {
        Pt2 { x, y }
    }

    pub fn manhatten_distance(&self, pt: &Pt2) -> u32 {
        let dx = self.x - pt.x;
        let dy = self.y - pt.y;
        (dx.abs() + dy.abs()) as u32
    }

    pub fn min_distance(&self, points: &[Pt2]) -> u32 {
        points
            .iter()
            .map(|p| self.manhatten_distance(p))
            .min()
            .unwrap()
    }

    pub fn max_distance(&self, points: &[Pt2]) -> u32 {
        points
            .iter()
            .map(|p| self.manhatten_distance(p))
            .max()
            .unwrap()
    }
}

impl FromStr for Pt2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1, 3
        let coords: Vec<&str> = s.split(',').map(|coord| coord.trim()).collect();
        Ok(Pt2::new(coords[0].parse()?, coords[1].parse()?))
    }
}

fn max_distance(points: &[Pt2]) -> u32 {
    points[..points.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, p)| p.max_distance(&points[i + 1..]))
        .max()
        .unwrap()
}

fn bounding_box(points: &[Pt2]) -> (Pt2, Pt2) {
    points.iter().fold(
        (
            Pt2::new(std::i32::MAX, std::i32::MAX),
            Pt2::new(std::i32::MIN, std::i32::MIN),
        ),
        |bbox, p| {
            (
                Pt2::new(p.x.min(bbox.0.x), p.y.min(bbox.0.y)),
                Pt2::new(p.x.max(bbox.1.x), p.y.max(bbox.1.y)),
            )
        },
    )
}

pub fn largest_area(points: &[Pt2]) -> u32 {
    let dist = max_distance(points) as i32;
    let bbox = bounding_box(points);
    let bbox = (
        Pt2::new(bbox.0.x - dist, bbox.0.y - dist),
        Pt2::new(bbox.1.x + dist, bbox.1.y + dist),
    );

    let is_on_edge =
        |p: &Pt2| p.x == bbox.0.x || p.x == bbox.1.x || p.y == bbox.0.y || p.y == bbox.1.y;

    let mut excludes: HashSet<Pt2> = HashSet::new();
    let area_map = (bbox.0.y..=bbox.1.y).fold(HashMap::new(), |m: HashMap<Pt2, u32>, y| {
        (bbox.0.x..=bbox.1.x).fold(m, |m, x| {
            let mut m = m;
            let p = Pt2::new(x, y);
            let distances = points
                .iter()
                .map(|p2| (p2, p.manhatten_distance(p2)))
                .collect::<Vec<_>>();
            let min_distance = distances.iter().map(|(_, d)| d).min().unwrap();
            let min_points = distances
                .iter()
                .filter(|(_, d)| d == min_distance)
                .map(|&(p, _)| p)
                .collect::<Vec<_>>();

            if min_points.len() == 1 {
                let k = min_points[0];
                match m.get_mut(k) {
                    Some(counter) => *counter += 1,
                    None => {
                        m.insert(k.clone(), 1);
                    }
                }

                if is_on_edge(&p) {
                    excludes.insert(k.clone());
                }
            }

            m
        })
    });

    let (_pt, area) = area_map
        .iter()
        .filter(|&(p, _)| !excludes.contains(p))
        .max_by(|&(_, a1), &(_, a2)| a1.cmp(a2))
        .unwrap();

    *area
}

pub fn region_size(points: &[Pt2], threshold: u32) -> u32 {
    let bbox = bounding_box(points);
    let dist = max_distance(points) as i32; // TODO: use better value
    let bbox = (
        Pt2::new(bbox.0.x - dist, bbox.0.y - dist),
        Pt2::new(bbox.1.x + dist, bbox.1.y + dist),
    );

    (bbox.0.y..=bbox.1.y)
        .map(|y| (bbox.0.x..=bbox.1.x).map(move |x| Pt2::new(x, y)))
        .flatten()
        .map(|p| {
            points
                .iter()
                .map(|p2| p.manhatten_distance(p2))
                .sum::<u32>()
        })
        .filter(|&d| d < threshold)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &[&'static str] = &["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];

    fn data() -> Vec<Pt2> {
        DATA.iter().map(|d| d.parse().unwrap()).collect()
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            data(),
            vec![
                Pt2::new(1, 1),
                Pt2::new(1, 6),
                Pt2::new(8, 3),
                Pt2::new(3, 4),
                Pt2::new(5, 5),
                Pt2::new(8, 9),
            ]
        );
    }

    #[test]
    fn test_max_distance() {
        assert_eq!(15, max_distance(&data()))
    }

    #[test]
    fn test_bounding_box() {
        assert_eq!((Pt2::new(1, 1), Pt2::new(8, 9)), bounding_box(&data()));
    }

    #[test]
    fn test_largest_area() {
        assert_eq!(17, largest_area(&data()));
    }

    #[test]
    fn test_region_size() {
        assert_eq!(16, region_size(&data(), 32));
    }
}
