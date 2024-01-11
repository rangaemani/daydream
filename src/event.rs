use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
#[derive(Clone, Copy, Debug)]
pub enum Event {
    // terminal tick
    Tick,
    // keypress
    Key(KeyEvent),
    // mouseclick/scroll
    Mouse(MouseEvent),
    // terminal resize/redraw
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    // event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    // event receiever channel
    receiver: mpsc::Receiver<Event>,
    // event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

// initiates generic event polling. basic sifting for event type
impl EventHandler {
    /// constructs a new EventHandler instance
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => sender.send(Event::Tick),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }
    // receive next event from handler thread
    // will always block current thread if there is no data available but datastream is still functioning
    pub fn next(&self) -> Result<Event, Box<dyn std::error::Error>> {
        Ok(self.receiver.recv()?)
    }
}
