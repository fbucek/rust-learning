use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};


pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last= inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        { 
            let mut inner = self.shared.inner.lock().unwrap();
            inner.queue.push_back(t);
            // drop(inner);
        }
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>, 
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        loop {
            let mut inner = self.shared.inner.lock().unwrap();
            match inner.queue.pop_front() {
                Some(t) => return Some(t),
                None if inner.senders == 0  => return None,
                None => {
                    self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}


impl<T> Iterator for Receiver<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner { 
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {  
        inner: Mutex::new(inner),
        available: Condvar::default(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        }
    )
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong_test() {
        let (mut sender, mut receiver) = channel();
        sender.send(41);
        assert_eq!(receiver.recv(), Some(41));
    }

    #[test]
    fn closed() {
        let (sender, mut receiver) = channel::<()>();
        drop(sender);
        assert_eq!(receiver.recv(), None);
    }
}
