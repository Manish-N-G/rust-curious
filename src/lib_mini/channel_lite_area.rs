// Exercise: ChannelMonitor Lite
//
// Fix the borrowing error three different ways:
//
// 2. Change the *area* of data borrowed
//      Revert the change from the step above. Instead:
//      Change `send` from a method to a function,
//      and give it references to only the data it needs.
type Sender = std::sync::mpsc::Sender<String>;

pub struct ChannelMonitor {
    connection: Option<Sender>,
    sent_ok: u64,
    sent_err: u64,
}

impl ChannelMonitor {
    pub fn new() -> Self {
        ChannelMonitor {
            connection: None,
            sent_ok: 0,
            sent_err: 0,
        }
    }

    pub fn connect(&mut self, sender: Sender) {
        self.connection = Some(sender);
    }

    // we make send to associated function of type rather than a method.
    // This way we can split the types to get the once we are looking for.
    pub fn handle_event(&mut self, event: Event) {
        let Some(sender) = &self.connection else {
            return;
        };
        match event {
            Event::NewSubscriber(name) => {
                let msg = format!("New sub: {}", name);
                Self::send(sender, msg, &mut self.sent_ok, &mut self.sent_err);
            }
            Event::NewComment(comment) => {
                let msg = format!("Comment: {}", comment);
                Self::send(sender, msg, &mut self.sent_ok, &mut self.sent_err);
            }
        }
    }

    fn send(sender: &Sender, msg: String, sent_ok: &mut u64, sent_err: &mut u64) {
        match sender.send(msg) {
            Ok(()) => *sent_ok += 1,
            Err(_) => *sent_err += 1,
        }
    }

    pub fn sent_ok(&self) -> u64 {
        self.sent_ok
    }

    pub fn sent_err(&self) -> u64 {
        self.sent_err
    }
}

// -------------------------------------------------------
// No need to change anything below this line.

pub enum Event {
    NewSubscriber(String),
    NewComment(String),
}

#[test]
fn test() {
    use std::sync::mpsc;

    let mut cm = ChannelMonitor::new();
    cm.handle_event(Event::NewComment("first post".into()));
    let (sender, receiver) = mpsc::channel();
    cm.connect(sender);
    cm.handle_event(Event::NewSubscriber("hugebrain".into()));
    cm.handle_event(Event::NewComment("c++ 4eva".into()));
    assert_eq!(cm.sent_ok(), 2);
    assert_eq!(receiver.recv().unwrap(), "New sub: hugebrain");
    assert_eq!(receiver.recv().unwrap(), "Comment: c++ 4eva");

    // Sending after the receiver is dropped causes an error
    std::mem::drop(receiver);
    cm.handle_event(Event::NewSubscriber("chad".into()));
    assert_eq!(cm.sent_err(), 1);
}
