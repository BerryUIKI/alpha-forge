// Task event sink used by headless task execution.
//
// The desktop forwards executor events to Tauri windows. The CLI forwards the
// same events into a channel so the command that started the task can print
// progress in arrival order without background writes to stdout.

use alpha_forge_lib::agent::events::TaskEventEmitter;
use domain::task::AgentTaskEvent;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// Forwards executor events to an in-process channel.
pub struct ChannelTaskEventEmitter {
    sender: UnboundedSender<AgentTaskEvent>,
}

impl ChannelTaskEventEmitter {
    /// Creates an emitter and the receiver for its events.
    pub fn new() -> (Self, UnboundedReceiver<AgentTaskEvent>) {
        let (sender, receiver) = unbounded_channel();
        (Self { sender }, receiver)
    }
}

impl TaskEventEmitter for ChannelTaskEventEmitter {
    fn emit(&self, event: &AgentTaskEvent) {
        // A closed receiver only means nobody is listening for progress; the
        // task run itself is unaffected.
        let _ = self.sender.send(event.clone());
    }
}

impl Default for ChannelTaskEventEmitter {
    fn default() -> Self {
        Self::new().0
    }
}

#[cfg(test)]
mod tests {
    use alpha_forge_lib::agent::events::{progress_event, TaskEventEmitter};
    use domain::task::TaskEventType;

    use super::ChannelTaskEventEmitter;

    #[tokio::test]
    async fn forwards_events_to_the_receiver() {
        let (emitter, mut receiver) = ChannelTaskEventEmitter::new();
        let event = progress_event("task-1", "Preparing request...");

        emitter.emit(&event);

        let received = receiver.recv().await.expect("event should be forwarded");
        assert_eq!(received.task_id, "task-1");
        assert_eq!(received.event_type, TaskEventType::TaskProgress);
        assert_eq!(received.payload.as_deref(), Some("Preparing request..."));
    }

    #[tokio::test]
    async fn emitting_without_a_receiver_does_not_panic() {
        let (emitter, receiver) = ChannelTaskEventEmitter::new();
        drop(receiver);

        emitter.emit(&progress_event("task-1", "progress"));
    }
}
