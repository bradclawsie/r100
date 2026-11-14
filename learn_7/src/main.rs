struct Job {
    id: u64,
    description: String,
    payload: Vec<u8>,
}

struct WorkQueue {
    jobs: Vec<Job>,
}

impl WorkQueue {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
        }
    }

    pub fn submit(&mut self, job: Job) {
        self.jobs.push(job)
    }

    pub fn next(&mut self) -> Option<Job> {
        if self.jobs.len() != 0 {
            return Some(self.jobs.remove(0));
        }
        None
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn drain_descriptions(&mut self) -> Vec<String> {
        self.jobs.drain(..).map(|job| job.description).collect()
    }
}

fn main() {
    ()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn test_new() {
        assert_eq!(WorkQueue::new().jobs.len(), 0);
    }

    #[test]
    pub fn test_submit() {
        let mut wq = WorkQueue::new();
        assert_eq!(wq.jobs.len(), 0);
        wq.submit(Job{id:1, description:String::from("1"), payload: Vec::new()});
        assert_eq!(wq.jobs.len(), 1);
    }

    #[test]
    pub fn test_next() {
        let mut wq = WorkQueue::new();
        assert_eq!(wq.jobs.len(), 0);
        wq.submit(Job{id:1, description:String::from("1"), payload: Vec::new()});
        assert_eq!(wq.jobs.len(), 1);
        wq.submit(Job{id:2, description:String::from("2"), payload: Vec::new()});
        assert_eq!(wq.jobs.len(), 2);
        assert!(wq.next().is_some_and(|x| x.id == 1));
        assert!(wq.next().is_some_and(|x| x.id == 2));
        assert!(wq.next().is_none());
    }

    #[test]
    pub fn test_len() {
        let mut wq = WorkQueue::new();
        assert_eq!(wq.len(), 0);
        wq.submit(Job{id:1, description:String::from("1"), payload: Vec::new()});
        assert_eq!(wq.len(), 1);
        wq.submit(Job{id:2, description:String::from("2"), payload: Vec::new()});
        assert_eq!(wq.len(), 2);
        assert!(wq.next().is_some_and(|x| x.id == 1));
        assert_eq!(wq.len(), 1);
        assert!(wq.next().is_some_and(|x| x.id == 2));
        assert_eq!(wq.len(), 0);
        assert!(wq.next().is_none());
    }

    #[test]
    pub fn test_drain_descriptions() {
        let mut wq = WorkQueue::new();
        assert_eq!(wq.drain_descriptions().len(), 0);
        wq.submit(Job{id:1, description:String::from("1"), payload: Vec::new()});
        wq.submit(Job{id:2, description:String::from("2"), payload: Vec::new()});
        let descriptions = wq.drain_descriptions();
        assert_eq!(descriptions.as_slice(), &["1", "2"]);
    }
}
