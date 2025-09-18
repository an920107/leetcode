use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, PartialEq, Eq)]
pub struct Task {
    pub uid: i32,
    pub pid: i32,
    pub priority: i32,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then(self.pid.cmp(&other.pid))
            .then(self.uid.cmp(&other.uid))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct TaskManager {
    tasks: HashMap<i32, Task>,
    max_heap: BinaryHeap<Task>,
}

impl TaskManager {
    pub fn new(tasks_vec: Vec<Vec<i32>>) -> Self {
        let tasks_iter = tasks_vec.iter().map(|t| Task {
            uid: t[0],
            pid: t[1],
            priority: t[2],
        });

        Self {
            tasks: HashMap::from_iter(tasks_iter.clone().map(|t| (t.pid, t))),
            max_heap: BinaryHeap::from_iter(tasks_iter),
        }
    }

    pub fn add(&mut self, uid: i32, pid: i32, priority: i32) {
        let task = Task { uid, pid, priority };
        self.tasks.insert(pid, task.clone());
        self.max_heap.push(task);
    }

    pub fn edit(&mut self, pid: i32, new_priority: i32) {
        let task = self.tasks.get_mut(&pid).unwrap();
        task.priority = new_priority;
        self.max_heap.push(task.clone());
    }

    pub fn rmv(&mut self, pid: i32) {
        self.tasks.remove(&pid);
    }

    pub fn exec_top(&mut self) -> i32 {
        let mut result_task: Option<Task> = None;

        while let Some(top_task) = self.max_heap.peek() {
            if let Some(task) = self.tasks.get(&top_task.pid)
                && task == top_task
            {
                result_task = Some(task.clone());
                self.tasks.remove(&top_task.pid);
                self.max_heap.pop();
                break;
            } else {
                self.max_heap.pop();
            }
        }

        result_task.map_or(-1, |t| t.uid)
    }
}
