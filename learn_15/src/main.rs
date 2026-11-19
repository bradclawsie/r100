#[derive(Debug, PartialEq)]
struct LogEntry {
    timestamp: u64,
    message: String,
}

struct LogBuffer {
    entries: Vec<LogEntry>,
    capacity: usize,
}

impl LogBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::new(),
            capacity,
        }
    }

    fn push(&mut self, entry: LogEntry) -> bool {
        if self.entries.len() == self.capacity {
            return false;
        }
        self.entries.push(entry);
        true
    }

    fn entries(&self) -> &[LogEntry] {
        &self.entries[..]
    }

    fn drain(&mut self) -> Vec<LogEntry> {
        let entries = std::mem::take(&mut self.entries);
        entries
    }

    fn reset(&mut self) {
        std::mem::take(self);
    }
}

impl Default for LogBuffer {
    fn default() -> Self {
        const CAPACITY: usize = 100;
        Self {
            entries: Vec::new(),
            capacity: CAPACITY,
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
        const CAPACITY: usize = 100;
        let lb: LogBuffer = LogBuffer::new(CAPACITY);
        assert_eq!(lb.capacity, CAPACITY);
        assert!(lb.entries.is_empty());
    }

    #[test]
    fn test_default() {
        const CAPACITY: usize = 100;
        let lb: LogBuffer = Default::default();
        assert_eq!(lb.capacity, CAPACITY);
        let lb0: LogBuffer = LogBuffer::default();
        assert_eq!(lb0.capacity, CAPACITY);
    }

    #[test]
    fn test_push() {
        let mut lb: LogBuffer = LogBuffer::new(1);
        assert!(lb.push(LogEntry{timestamp:0, message:"a".to_string()}));
        assert_eq!(lb.entries.len(), 1);
        assert!(!lb.push(LogEntry{timestamp:0, message:"a".to_string()}));
        assert_eq!(lb.entries.len(), 1);
    }

    #[test]
    fn test_entries() {
        const CAPACITY: usize = 2;
        let mut lb: LogBuffer = LogBuffer::new(CAPACITY);
        assert!(lb.push(LogEntry{timestamp:0, message: "a".to_string()}));
        assert!(lb.push(LogEntry{timestamp:1, message: "b".to_string()}));
        assert_eq!(lb.entries.len(), 2);
        assert_eq!(lb.entries(), [
            LogEntry{timestamp:0, message: "a".to_string()},
            LogEntry{timestamp:1, message: "b".to_string()}
        ]);
    }

    #[test]
    fn test_drain() {
        const CAPACITY: usize = 2;
        let mut lb: LogBuffer = LogBuffer::new(CAPACITY);
        assert!(lb.push(LogEntry{timestamp:0, message:"a".to_string()}));
        assert_eq!(lb.entries.len(), 1);
        assert!(lb.push(LogEntry{timestamp:1, message:"b".to_string()}));
        assert_eq!(lb.entries.len(), 2);
        assert_eq!(lb.drain(), vec![
            LogEntry{timestamp:0, message: "a".to_string()},
            LogEntry{timestamp:1, message: "b".to_string()}
        ]);
        assert!(lb.entries.is_empty());
    }

    #[test]
    fn test_reset() {
        const CAPACITY: usize = 2;
        let mut lb: LogBuffer = LogBuffer::new(CAPACITY);
        assert!(lb.push(LogEntry{timestamp:0, message: "a".to_string()}));
        assert!(lb.push(LogEntry{timestamp:1, message: "b".to_string()}));
        assert_eq!(lb.entries.len(), 2);
        assert_eq!(lb.entries(), [
            LogEntry{timestamp:0, message: "a".to_string()},
            LogEntry{timestamp:1, message: "b".to_string()}
        ]);
        lb.reset();
        assert_eq!(lb.entries.len(), 0);
        assert_eq!(lb.entries(), []);
    }
}
