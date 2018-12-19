use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn contains(&self, left: u32, top: u32) -> bool {
        left >= self.left
            && left < (self.left + self.width)
            && top >= self.top
            && top < (self.top + self.height)
    }

    pub fn overlaps(&self, other: &Claim) -> bool {
        self.is_overlapped_by(other) || other.is_overlapped_by(self)
    }

    fn is_overlapped_by(&self, other: &Claim) -> bool {
        let &Claim {
            left: l,
            top: t,
            width: w,
            height: h,
            ..
        } = self;
        let &Claim {
            left,
            top,
            width,
            height,
            ..
        } = other;
        let right = left + width - 1;
        let bottom = top + height - 1;
        let r = l + w - 1;
        let b = t + h - 1;

        let mid = |x: u32, l: u32, r: u32| x >= l && x <= r;

        ((mid(left, l, r) || mid(right, l, r))
            && (mid(top, t, b) || mid(bottom, t, b) || (top < t && bottom > b)))
            || ((mid(top, t, b) || mid(bottom, t, b))
                && (mid(left, l, r) || mid(right, l, r) || (left < l && right > r)))
    }
}

#[derive(Debug)]
pub struct ParseClaimError();

impl From<ParseIntError> for ParseClaimError {
    fn from(_error: ParseIntError) -> Self {
        ParseClaimError()
    }
}

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // #1 @ 1,3: 4x4
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let caps = RE.captures(s).ok_or(ParseClaimError())?;
        let id = caps[1].parse::<u32>()?;
        let left = caps[2].parse::<u32>()?;
        let top = caps[3].parse::<u32>()?;
        let width = caps[4].parse::<u32>()?;
        let height = caps[5].parse::<u32>()?;

        Ok(Claim {
            id,
            left,
            top,
            width,
            height,
        })
    }
}

pub fn double_claimed_square_inches(claims: &[Claim]) -> i32 {
    let size = 1000u32;
    (0..size)
        .map(|top| (0..size).map(move |left| (left, top)))
        .flatten()
        .filter(|&(left, top)| is_double_claimed(left, top, claims))
        .count() as i32
}

fn is_double_claimed(left: u32, top: u32, claims: &[Claim]) -> bool {
    claims
        .iter()
        .filter(|claim| claim.contains(left, top))
        .take(2)
        .count()
        == 2
}

pub fn find_first_valid_claim_id(claims: &[Claim]) -> Option<u32> {
    for (i, current_claim) in claims.iter().enumerate() {
        let other_claims_1 = &claims[..i];
        let other_claims_2 = &claims[i + 1..];

        let is_valid_claim = other_claims_2
            .iter()
            .chain(other_claims_1.iter())
            .all(|other_claim| !current_claim.overlaps(other_claim));
        if is_valid_claim {
            return Some(current_claim.id());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &[&'static str] = &["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];

    fn claims() -> Vec<Claim> {
        DATA.iter().map(|data| data.parse().unwrap()).collect()
    }

    #[test]
    fn test_parse_claim() {
        assert_eq!(
            DATA[0].parse::<Claim>().unwrap(),
            Claim {
                id: 1,
                left: 1,
                top: 3,
                width: 4,
                height: 4,
            }
        );
        assert_eq!(
            DATA[1].parse::<Claim>().unwrap(),
            Claim {
                id: 2,
                left: 3,
                top: 1,
                width: 4,
                height: 4,
            }
        );
        assert_eq!(
            DATA[2].parse::<Claim>().unwrap(),
            Claim {
                id: 3,
                left: 5,
                top: 5,
                width: 2,
                height: 2,
            }
        );
    }

    #[test]
    fn test_is_double_claimed() {
        let claims = claims();
        assert!(!is_double_claimed(0, 0, &claims));
        assert!(!is_double_claimed(3, 1, &claims));
        assert!(!is_double_claimed(1, 3, &claims));
        assert!(!is_double_claimed(6, 6, &claims));
        assert!(is_double_claimed(3, 3, &claims));
    }

    #[test]
    fn test_double_claimed_square_inches() {
        assert_eq!(4, double_claimed_square_inches(&claims()));
    }

    #[test]
    fn test_find_first_valid_claim_id() {
        assert_eq!(Some(3), find_first_valid_claim_id(&claims()));
    }
}
