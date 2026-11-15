#[derive(Debug, PartialEq)]
enum JobState {
    Pending,
    Running {
        started_at: u64,
    },
    Succeeded {
        result: String,
    },
    Failed{
        error: String,
    },
}

struct Job {
    id: u64,
    state: JobState,
}

impl Job {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: JobState::Pending,
        }
    }

    fn start(&mut self, started_at: u64) -> bool {
        if self.state != JobState::Pending {
            return false;
        }
        self.state = JobState::Running{started_at};
        true
    }

    fn succeed(&mut self, result: String) -> bool {
        if !matches!(self.state, JobState::Running{..}) {
            return false;
        }
        self.state = JobState::Succeeded{result};
        true
    }

    fn fail(&mut self, error: String) -> bool {
        if !matches!(self.state, JobState::Running{..}) {
            return false;
        }
        self.state = JobState::Failed{error};
        true
    }

    fn reset(&mut self) -> bool {
        if !matches!(self.state, JobState::Succeeded{..} | JobState::Failed{..}) {
            return false;
        }
        self.state = JobState::Pending;
        true
    }

    fn status(&self) -> &str {
        match self.state {
            JobState::Pending => return "pending",
            JobState::Running {started_at: _}=> return "running",
            JobState::Succeeded {result: _} => return "succeeded",
            JobState::Failed {error: _} => return "failed",
        }
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
        let job = Job::new(1);
        assert_eq!(job.id, 1);
        assert_eq!(job.state, JobState::Pending);
    }

    #[test]
    fn test_start() {
        let mut job = Job::new(1);
        assert_eq!(job.state, JobState::Pending);
        assert!(job.start(1));
        assert_eq!(job.state, JobState::Running{started_at:1});
        assert!(!job.start(1));

        let mut job_succeeded = Job::new(1);
        job_succeeded.state = JobState::Succeeded{result: "ok".to_string()};
        assert!(!job_succeeded.start(1));

        let mut job_failed = Job::new(1);
        job_failed.state = JobState::Failed{error: "bad".to_string()};
        assert!(!job_failed.start(1));
    }

    #[test]
    fn test_suceeded() {
        let mut job = Job::new(1);
        assert_eq!(job.state, JobState::Pending);
        assert!(job.start(1));
        assert_eq!(job.state, JobState::Running{started_at:1});
        assert!(job.succeed("ok".to_string()));
        assert_eq!(job.state, JobState::Succeeded{result:"ok".to_string()});
        assert!(!job.succeed("ok".to_string()));
        job.state = JobState::Failed{error:"bad".to_string()};
        assert!(!job.succeed("ok".to_string()));
    }

    #[test]
    fn test_failed() {
        let mut job = Job::new(1);
        assert_eq!(job.state, JobState::Pending);
        assert!(job.start(1));
        assert_eq!(job.state, JobState::Running{started_at:1});
        assert!(job.fail("bad".to_string()));
        assert_eq!(job.state, JobState::Failed{error:"bad".to_string()});
        assert!(!job.fail("bad".to_string()));
        job.state = JobState::Succeeded{result:"bad".to_string()};
        assert!(!job.fail("fail".to_string()));
    }

    #[test]
    fn test_reset() {
        let mut job = Job::new(1);
        assert_eq!(job.state, JobState::Pending);
        assert!(!job.reset());
        job.state = JobState::Running{started_at:1};
        assert!(!job.reset());
        job.state = JobState::Succeeded{result:"ok".to_string()};
        assert!(job.reset());
        assert_eq!(job.state, JobState::Pending);
        job.state = JobState::Failed{error:"bad".to_string()};
        assert!(job.reset());
        assert_eq!(job.state, JobState::Pending);
    }

    #[test]
    fn test_status() {
        let mut job = Job::new(1);
        assert_eq!(job.state, JobState::Pending);
        assert_eq!(job.status(), "pending");
        assert!(job.start(1));
        assert_eq!(job.state, JobState::Running{started_at:1});
        assert_eq!(job.status(), "running");
        assert!(job.succeed("ok".to_string()));
        assert_eq!(job.state, JobState::Succeeded{result:"ok".to_string()});
        assert_eq!(job.status(), "succeeded");
        job.state = JobState::Running{started_at:1};
        assert!(job.fail("bad".to_string()));
        assert_eq!(job.state, JobState::Failed{error:"bad".to_string()});
        assert_eq!(job.status(), "failed");
    }
}
