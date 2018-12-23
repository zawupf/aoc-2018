pub fn remaining_units(units: &str) -> String {
    let mut units = units.as_bytes().to_owned();

    loop {
        let units2 = reduce(&units);
        let reduced = units.len() != units2.len();
        units = units2;
        if !reduced {
            break;
        }
    }

    String::from_utf8(units).unwrap()
}

fn reduce(units: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(units.len());

    let mut cursor = units.iter().peekable();
    while let Some(c) = cursor.next() {
        if let Some(&next_c) = cursor.peek() {
            if is_reducable(*c, *next_c) {
                cursor.next();
            } else {
                result.push(*c);
            }
        } else {
            result.push(*c);
        }
    }

    result
}

fn is_reducable(a: u8, b: u8) -> bool {
    if a == b {
        return false;
    }

    let a = a.to_ascii_lowercase();
    let b = b.to_ascii_lowercase();

    a == b
}

fn remove_unit(unit: char, units: &str) -> String {
    let a = unit.to_ascii_lowercase();
    let b = unit.to_ascii_uppercase();
    units.replace(|c| c == a || c == b, "")
}

pub fn shortest_polymer_length(units: &str) -> u32 {
    (0..26)
        .map(|i| {
            let c = char::from(65 + i);
            remaining_units(&remove_unit(c, units)).len()
        })
        .min()
        .unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = "dabAcCaCBAcCcaDA";

    #[test]
    fn test_remaining_units() {
        assert_eq!("dabCBAcaDA", remaining_units(DATA));
    }

    #[test]
    fn test_remove_unit() {
        assert_eq!("dbcCCBcCcD", remove_unit('a', DATA));
        assert_eq!("daAcCaCAcCcaDA", remove_unit('b', DATA));
        assert_eq!("dabAaBAaDA", remove_unit('C', DATA));
        assert_eq!("abAcCaCBAcCcaA", remove_unit('D', DATA));
    }

    #[test]
    fn test_remaining_remove_unit() {
        assert_eq!("dbCBcD", remaining_units(&remove_unit('a', DATA)));
        assert_eq!("daCAcaDA", remaining_units(&remove_unit('b', DATA)));
        assert_eq!("daDA", remaining_units(&remove_unit('C', DATA)));
        assert_eq!("abCBAc", remaining_units(&remove_unit('D', DATA)));
    }

    #[test]
    fn test_shortest_polymer_length() {
        assert_eq!(4, shortest_polymer_length(DATA));
    }
}
