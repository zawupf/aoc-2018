pub fn checksum(values: &[&str]) -> i32 {
    let (a, b) = values
        .iter()
        .map(|value| chksum(value))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    a * b
}

pub fn chksum(value: &str) -> (i32, i32) {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let counts: HashSet<i32> = value
        .bytes()
        .fold(HashMap::new(), |mut acc: HashMap<u8, i32>, b| {
            acc.entry(b).and_modify(|e| *e += 1).or_insert(1);
            acc
        })
        .values()
        .cloned()
        .collect();
    (
        if counts.contains(&2) { 1 } else { 0 },
        if counts.contains(&3) { 1 } else { 0 },
    )
}

pub fn common_letters(values: &[&str]) -> Option<String> {
    for (i, a) in values.iter().enumerate() {
        let rest = &values[i + 1..];
        for b in rest.iter() {
            let indexes: Vec<i32> =
                a.bytes()
                    .zip(b.bytes())
                    .enumerate()
                    .fold(Vec::new(), |mut acc, (i, (a, b))| {
                        if a != b {
                            acc.push(i as i32);
                        }
                        acc
                    });
            if indexes.len() == 1 {
                let j = indexes[0];
                let s = String::from_utf8(
                    a.bytes()
                        .enumerate()
                        .filter(|(i, _b)| *i as i32 != j)
                        .map(|(_i, b)| b)
                        .collect::<Vec<_>>(),
                )
                .expect("Invalid string");
                return Some(s);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(
            12,
            checksum(&["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",])
        );
    }

    #[test]
    fn test_common_letters() {
        assert_eq!(
            Some("fgij".to_owned()),
            common_letters(&["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",])
        );
    }
}
