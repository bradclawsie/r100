struct Task {
    id: u64,
    name: String,
    completed: bool,
}

struct TaskRegistry {
    pending: Vec<Task>,
    completed: Vec<Task>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            completed: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Task) -> bool {
        if task.completed {
            return false;
        }
        if self.pending.iter().any(|x| x.id == task.id) {
            return false
        }
        if self.completed.iter().any(|x| x.id == task.id) {
            return false
        }
        self.pending.push(task);
        true
    }

    pub fn get(&self, id: u64) -> Option<&Task> {
        if let Some(pending_task) = self.pending.iter().find(|x| x.id == id) {
            return Some(pending_task)
        }
        if let Some(completed_task) = self.completed.iter().find(|x| x.id == id) {
            return Some(completed_task)
        }
        None
    }

    pub fn rename(&mut self, id: u64, name: String) -> bool {
        if let Some(pending_task) = self.pending.iter_mut().find(|x| x.id == id) {
            pending_task.name = name;
            return true;
        }
        if let Some(completed_task) = self.completed.iter_mut().find(|x| x.id == id) {
            completed_task.name = name;
            return true;
        }
        false
    }

    pub fn complete(&mut self, id: u64) -> bool {
        if let Some(i) = self.pending.iter().position(|x| x.id == id) {
            let mut task = self.pending.remove(i);
            task.completed = true;
            self.completed.push(task);
            return true;
        }
        false
    }

    pub fn take_completed(&mut self) -> Vec<Task> {
        let mut v: Vec<Task> = Vec::new();
        v.append(&mut self.completed);
        v
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new() {
        let tr = TaskRegistry::new();
        assert_eq!(tr.pending.len(), 0);
        assert_eq!(tr.completed.len(), 0);
    }

    #[test]
    fn test_add() {
        let mut tr = TaskRegistry::new();
        assert_eq!(tr.pending.len(), 0);
        assert_eq!(tr.completed.len(), 0);

        // completed prior to add
        assert!(!tr.add(Task{
            id: 1,
            name: "name".to_string(),
            completed: true,
        }));
        assert_eq!(tr.pending.len(), 0);
        assert_eq!(tr.completed.len(), 0);

        // ok
        assert!(tr.add(Task{
            id: 1,
            name: "name".to_string(),
            completed: false,
        }));
        assert_eq!(tr.pending.len(), 1);
        assert_eq!(tr.completed.len(), 0);
        
        // duplicate id - already pending
        assert!(!tr.add(Task{
            id: 1,
            name: "name".to_string(),
            completed: false,
        }));
        assert_eq!(tr.pending.len(), 1);
        assert_eq!(tr.completed.len(), 0);

        // duplicate id - already completed
        tr.completed.push(Task {
            id: 2,
            name: "name".to_string(),
            completed: true,
        });
        assert_eq!(tr.completed.len(), 1);
        assert!(!tr.add(Task{
            id: 2,
            name: "name".to_string(),
            completed: true,
        }));
        assert_eq!(tr.pending.len(), 1);
        assert_eq!(tr.completed.len(), 1);        
    }

    #[test]
    fn test_get() {
        let mut tr = TaskRegistry::new();
        assert!(tr.add(Task{
            id: 1,
            name: "name1".to_string(),
            completed: false,
        }));
        tr.completed.push(Task {
            id: 2,
            name: "name2".to_string(),
            completed: true,
        });

        assert!(tr.get(1).is_some_and(|x| x.id == 1));
        assert!(tr.get(2).is_some_and(|x| x.id == 2));
        assert!(tr.get(3).is_none());
    }

    #[test]
    fn test_rename() {
        let mut tr = TaskRegistry::new();
        assert!(tr.add(Task{
            id: 1,
            name: "name1".to_string(),
            completed: false,
        }));
        tr.completed.push(Task {
            id: 2,
            name: "name2".to_string(),
            completed: true,
        });

        assert!(tr.rename(1, "name1_new".to_string()));
        assert!(tr.get(1).is_some_and(|x| x.name == "name1_new"));
        assert!(tr.rename(2, "name2_new".to_string()));
        assert!(tr.get(2).is_some_and(|x| x.name == "name2_new"));
        assert!(!tr.rename(3, "name3_new".to_string()));
    }

    #[test]
    fn test_completed() {
        let mut tr = TaskRegistry::new();
        assert!(!tr.complete(1));
        assert!(tr.add(Task{
            id: 1,
            name: "name1".to_string(),
            completed: false,
        }));
        assert_eq!(tr.pending.len(), 1);
        assert_eq!(tr.completed.len(), 0);
        assert!(tr.complete(1));
        assert_eq!(tr.pending.len(), 0);
        assert_eq!(tr.completed.len(), 1);
    }

    #[test]
    fn test_take_completed() {
        let mut tr = TaskRegistry::new();
        assert!(tr.add(Task{
            id: 1,
            name: "name1".to_string(),
            completed: false,
        }));
        assert!(tr.complete(1));
        assert!(tr.add(Task{
            id: 2,
            name: "name2".to_string(),
            completed: false,
        }));
        assert!(tr.complete(2));
        assert_eq!(tr.completed.len(), 2);
        let completed = tr.take_completed();
        assert_eq!(tr.completed.len(), 0);
        assert_eq!(completed.len(), 2);
        assert_eq!(completed[0].id, 1);
        assert_eq!(completed[1].id, 2);
    }
}
