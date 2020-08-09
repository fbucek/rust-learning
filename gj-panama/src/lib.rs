use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        { 
            let mut queue = self.inner.queue.lock().unwrap();
            queue.push_back(t);
            // drop(queue);
        }
        self.inner.available.notify_one();
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>, 
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        loop {
            let mut queue = self.inner.queue.lock().unwrap();
            match queue.pop_front() {
                Some(t) => return t,
                None => { 
                    self.inner.available.wait(queue).unwrap();
                }
            }
        }
    }
    pub fn try_recv(&mut self) -> Option<T> {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.pop_front()
    }
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {  
        queue: Mutex::default(),
        available: Condvar::default(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
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
        assert_eq!(receiver.recv(), 41);
    }

    #[test]
    fn closed() {
        let (mut sender, mut receiver) = channel();
        sender.send(41);
        assert_eq!(receiver.recv(), 41);
    }

}
