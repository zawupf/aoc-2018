use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Task {
    id: u8,
    deps: HashSet<u8>,
    eta: u32,
}

impl Task {
    fn new(id: u8) -> Task {
        Task {
            id,
            deps: HashSet::new(),
            eta: 0,
        }
    }

    fn with_dep(id: u8, dep: u8) -> Task {
        let mut deps = HashSet::new();
        deps.insert(dep);
        Task { id, deps, eta: 0 }
    }
}

fn make_task_map(rules: &[&str]) -> HashMap<u8, Task> {
    let mut task_map: HashMap<u8, Task> = HashMap::new();

    rules
        .iter()
        .map(|rule| {
            let mut r = rule.bytes();
            let prereq = r.nth(5).unwrap();
            let task = r.nth(30).unwrap();
            (task, prereq)
        })
        .fold(&mut task_map, |task_map, (task, prereq)| {
            task_map
                .entry(task)
                .and_modify(|task| {
                    task.deps.insert(prereq);
                })
                .or_insert_with(|| Task::with_dep(task, prereq));
            task_map.entry(prereq).or_insert_with(|| Task::new(prereq));
            task_map
        });

    task_map
}

pub fn task_execution_order(rules: &[&str]) -> String {
    let mut remaining_tasks: Vec<Task> =
        make_task_map(rules).drain().map(|(_, task)| task).collect();
    let mut pending_tasks: Vec<Task> = Vec::new();
    let mut ordered_tasks = String::new();

    while find_pending_tasks(&mut pending_tasks, &mut remaining_tasks) {
        ordered_tasks.push(char::from(run_first_pending_task(
            &mut pending_tasks,
            &mut remaining_tasks,
        )));
    }

    ordered_tasks
}

pub fn task_duration(rules: &[&str], worker_count: usize, extra_task_duration: u32) -> u32 {
    let mut remaining_tasks: Vec<Task> =
        make_task_map(rules).drain().map(|(_, task)| task).collect();
    let mut pending_tasks: Vec<Task> = Vec::new();
    let mut running_tasks: Vec<Task> = Vec::new();
    let mut seconds = 0u32;

    while !(running_tasks.is_empty() && pending_tasks.is_empty() && remaining_tasks.is_empty()) {
        find_pending_tasks(&mut pending_tasks, &mut remaining_tasks);
        while running_tasks.len() != worker_count && !pending_tasks.is_empty() {
            start_first_pending_task(&mut pending_tasks, &mut running_tasks, extra_task_duration);
        }

        seconds += 1;
        running_tasks.iter_mut().for_each(|task| {
            task.eta -= 1;
        });
        let task_ids: Vec<u8> = running_tasks
            .iter()
            .filter(|task| task.eta == 0)
            .map(|task| task.id)
            .collect();
        task_ids.iter().for_each(|&task_id| {
            finish_task(task_id, &mut running_tasks, &mut remaining_tasks);
        });
    }

    while find_pending_tasks(&mut pending_tasks, &mut remaining_tasks) {
        run_first_pending_task(&mut pending_tasks, &mut remaining_tasks);
    }

    seconds
}

fn find_pending_tasks(pending_tasks: &mut Vec<Task>, remaining_tasks: &mut Vec<Task>) -> bool {
    let (mut pending, mut remaining): (Vec<Task>, Vec<Task>) = remaining_tasks
        .clone()
        .into_iter()
        .partition(|task| task.deps.is_empty());
    remaining_tasks.clear();
    remaining_tasks.append(&mut remaining);
    pending_tasks.append(&mut pending);
    pending_tasks.sort_unstable_by_key(|task| task.id);
    !(pending_tasks.is_empty() && remaining_tasks.is_empty())
}

fn run_first_pending_task(pending_tasks: &mut Vec<Task>, remaining_tasks: &mut Vec<Task>) -> u8 {
    let task_id = pending_tasks.remove(0).id;
    remaining_tasks.iter_mut().for_each(|task| {
        task.deps.remove(&task_id);
    });
    task_id
}

fn start_first_pending_task(
    pending_tasks: &mut Vec<Task>,
    running_tasks: &mut Vec<Task>,
    extra_task_duration: u32,
) {
    let mut task = pending_tasks.remove(0);
    task.eta = extra_task_duration + u32::from(task.id - 64);
    running_tasks.push(task);
}

fn finish_task(task_id: u8, running_tasks: &mut Vec<Task>, remaining_tasks: &mut Vec<Task>) {
    let index = running_tasks
        .iter()
        .enumerate()
        .find(|&(_, task)| task.id == task_id)
        .unwrap()
        .0;
    running_tasks.remove(index);
    remaining_tasks.iter_mut().for_each(|task| {
        task.deps.remove(&task_id);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    static DATA: &[&'static str] = &[
        "Step C must be finished before step A can begin.",
        "Step C must be finished before step F can begin.",
        "Step A must be finished before step B can begin.",
        "Step A must be finished before step D can begin.",
        "Step B must be finished before step E can begin.",
        "Step D must be finished before step E can begin.",
        "Step F must be finished before step E can begin.",
    ];

    fn make_set(chars: &str) -> HashSet<u8> {
        HashSet::from_iter(chars.bytes())
    }

    #[test]
    fn test_make_task_map() {
        let t = make_task_map(DATA);
        assert_eq!(
            t.keys().cloned().collect::<HashSet<_>>(),
            make_set("ABCDEF")
        );
        assert_eq!(t[&b'A'].deps, make_set("C"));
        assert_eq!(t[&b'B'].deps, make_set("A"));
        assert_eq!(t[&b'C'].deps, make_set(""));
        assert_eq!(t[&b'D'].deps, make_set("A"));
        assert_eq!(t[&b'E'].deps, make_set("BDF"));
        assert_eq!(t[&b'F'].deps, make_set("C"));
    }

    #[test]
    fn test_task_execution_order() {
        assert_eq!("CABDFE", task_execution_order(DATA));
    }

    #[test]
    fn test_task_duration() {
        assert_eq!(15, task_duration(DATA, 2, 0));
    }
}
