#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    UserCreated { id: u64, name: String },
    UserDeleted { id: u64 },
    MessageReceived { from: u64, text: String },
}

pub struct EventBus {
    // Your choice of fields
}

impl EventBus {
    // pub fn new() -> Self;

    // pub fn subscribe<F>(&mut self, callback: F) -> SubscriptionId
    // where
    //     F: FnMut(&Event) + 'static;

    // pub fn unsubscribe(&mut self, id: SubscriptionId) -> bool;

    // pub fn publish(&mut self, event: Event);

    // pub fn subscriber_count(&self) -> usize;
}
