use std::collections::HashMap;

type NapMap = [u32; 60];

#[derive(Clone)]
pub struct Guard {
    id: u32,
    minutes_asleep: u32,
    naps: NapMap,
}

impl Guard {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_sleeping_at(&self, minute: u8) -> bool {
        self.naps[usize::from(minute)] != 0
    }

    pub fn minutes_asleep(&self) -> u32 {
        self.minutes_asleep
    }

    pub fn most_asleep_minute(&self) -> u8 {
        self.most_asleep_minute_and_count().0
    }

    pub fn most_asleep_minute_and_count(&self) -> (u8, u32) {
        self.naps
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(minute, count)| (minute as u8, *count))
            .unwrap()
    }

    pub fn mark_asleep(&mut self, start: u8, end: u8) {
        self.naps[usize::from(start)..usize::from(end)]
            .iter_mut()
            .for_each(|n| *n += 1);
        self.minutes_asleep += u32::from(end - start);
    }
}

pub fn read_data(lines: &[&str]) -> HashMap<u32, Guard> {
    let mut lines = Vec::from(lines);
    lines.sort_unstable();

    let mut guards: HashMap<u32, Guard> = HashMap::new();

    let mut dummy_guard = Guard {
        id: 0,
        minutes_asleep: 0,
        naps: [0; 60],
    };
    let mut guard: &mut Guard = &mut dummy_guard;
    let mut start: u8 = 0;
    for line in &lines {
        if line.ends_with(" shift") {
            let id = line[26..]
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            guard = guards.entry(id).or_insert_with(|| Guard {
                id,
                minutes_asleep: 0,
                naps: [0; 60],
            });
        } else if line.ends_with(" asleep") {
            start = line[15..17].parse::<u8>().unwrap();
        } else if line.ends_with(" up") {
            let end = line[15..17].parse::<u8>().unwrap();
            guard.mark_asleep(start, end);
        }
    }

    guards
}

pub fn most_asleep_guard<'a>(guards: &[&'a Guard]) -> &'a Guard {
    guards
        .iter()
        .max_by_key(|guard| guard.minutes_asleep())
        .unwrap()
}

pub fn most_asleep_guard_on_same_minute<'a>(guards: &[&'a Guard]) -> &'a Guard {
    guards
        .iter()
        .map(|guard| (guard, guard.most_asleep_minute_and_count()))
        .max_by_key(|(_, (_, count))| *count)
        .map(|(guard, _)| guard)
        .unwrap()
}

pub fn strategy_1(guards: &[&Guard]) -> u32 {
    let guard = most_asleep_guard(guards);
    guard.id() * u32::from(guard.most_asleep_minute())
}

pub fn strategy_2(guards: &[&Guard]) -> u32 {
    let guard = most_asleep_guard_on_same_minute(guards);
    guard.id() * u32::from(guard.most_asleep_minute())
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &[&'static str] = &[
        "[1518-11-01 00:00] Guard #10 begins shift",
        "[1518-11-01 00:05] falls asleep",
        "[1518-11-01 00:25] wakes up",
        "[1518-11-01 00:30] falls asleep",
        "[1518-11-01 00:55] wakes up",
        "[1518-11-01 23:58] Guard #99 begins shift",
        "[1518-11-02 00:40] falls asleep",
        "[1518-11-02 00:50] wakes up",
        "[1518-11-03 00:05] Guard #10 begins shift",
        "[1518-11-03 00:24] falls asleep",
        "[1518-11-03 00:29] wakes up",
        "[1518-11-04 00:02] Guard #99 begins shift",
        "[1518-11-04 00:36] falls asleep",
        "[1518-11-04 00:46] wakes up",
        "[1518-11-05 00:03] Guard #99 begins shift",
        "[1518-11-05 00:45] falls asleep",
        "[1518-11-05 00:55] wakes up",
    ];

    #[test]
    fn test_read_data() {
        let guards = read_data(DATA);
        assert_eq!(2, guards.len());

        let guard = &guards[&10];
        assert_eq!(50, guard.minutes_asleep());
        assert_eq!(24, guard.most_asleep_minute());
        assert_eq!((24, 2), guard.most_asleep_minute_and_count());

        let guard = &guards[&99];
        assert_eq!(30, guard.minutes_asleep());
        assert_eq!(45, guard.most_asleep_minute());
        assert_eq!((45, 3), guard.most_asleep_minute_and_count());
    }

    #[test]
    fn test_most_asleep_guard() {
        let guards_data = read_data(DATA);
        let guards: Vec<&Guard> = guards_data.values().collect();
        let guard = most_asleep_guard(&guards);
        assert_eq!(10, guard.id());
        assert_eq!(24, guard.most_asleep_minute());
    }

    #[test]
    fn test_most_asleep_guard_on_same_minute() {
        let guards_data = read_data(DATA);
        let guards: Vec<&Guard> = guards_data.values().collect();
        let guard = most_asleep_guard_on_same_minute(&guards);
        assert_eq!(99, guard.id());
        assert_eq!(45, guard.most_asleep_minute());
    }

    #[test]
    fn test_strategy_1() {
        let guards_data = read_data(DATA);
        let guards: Vec<&Guard> = guards_data.values().collect();
        assert_eq!(240, strategy_1(&guards));
    }

    #[test]
    fn test_strategy_2() {
        let guards_data = read_data(DATA);
        let guards: Vec<&Guard> = guards_data.values().collect();
        assert_eq!(4455, strategy_2(&guards));
    }
}
