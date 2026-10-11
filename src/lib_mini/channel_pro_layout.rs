// Exercise: ChannelMonitor Pro
//
// Fix the borrowing error three different ways:
//
// 3. Change the *data layout* to match the access
type Sender = std::sync::mpsc::Sender<String>;

pub struct ChannelMonitor {
    connections: Vec<Sender>,
    sent: SendChannelMonitor,
}

pub struct SendChannelMonitor {
    sent_ok: u64,
    sent_err: u64,
}
impl ChannelMonitor {
    pub fn new() -> Self {
        ChannelMonitor {
            connections: Vec::new(),
            sent: SendChannelMonitor { sent_ok: 0, sent_err: 0 }
        }
    }

    pub fn connect(&mut self, sender: Sender) {
        self.connections.push(sender);
    }

    pub fn handle_event(&mut self, event: Event) {
        for sender in &self.connections {
            match &event {
                Event::NewSubscriber(name) => {
                    self.sent.send(sender, format!("New sub: {}", name) );
                }
                Event::NewComment(comment) => {
                    self.sent.send(sender, format!("Comment: {}", comment) );
                }
            }
        }
    }

    pub fn sent_ok(&self) -> u64 {
        self.sent.sent_ok
    }

    pub fn sent_err(&self) -> u64 {
        self.sent.sent_err
    }
}

impl SendChannelMonitor {
    fn send(&mut self, sender: &Sender, msg: String) {
        match sender.send(msg) {
            Ok(()) => self.sent_ok += 1,
            Err(_) => self.sent_err += 1,
        }
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
    let (sender0, receiver0) = mpsc::channel();
    let (sender1, receiver1) = mpsc::channel();
    cm.connect(sender0);
    cm.handle_event(Event::NewSubscriber("hugebrain".into()));
    cm.connect(sender1);
    cm.handle_event(Event::NewComment("c++ 4eva".into()));
    assert_eq!(cm.sent_ok(), 3);
    assert_eq!(receiver0.recv().unwrap(), "New sub: hugebrain");
    assert_eq!(receiver0.recv().unwrap(), "Comment: c++ 4eva");
    assert_eq!(receiver1.recv().unwrap(), "Comment: c++ 4eva");

    cm.handle_event(Event::NewComment("c++ 4eva again".into()));
    assert_eq!(cm.sent_ok(), 5);
    assert_eq!(receiver0.recv().unwrap(), "Comment: c++ 4eva again");
    assert_eq!(receiver1.recv().unwrap(), "Comment: c++ 4eva again");

    // Sending after the receiver is dropped causes an error
    std::mem::drop(receiver1);
    cm.handle_event(Event::NewSubscriber("chad".into()));
    assert_eq!(cm.sent_ok(), 6);
    assert_eq!(cm.sent_err(), 1);
}
