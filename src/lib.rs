
use std::sync::mpsc::Sender;

trait Sink<T> {
    fn drain<I>(&self, iter: I) -> Result<(), ()> where I: Iterator<Item = T>;
}

impl<T> Sink<T> for Sender<T> {
    fn drain<I>(&self, iter: I) -> Result<(), ()>
        where I: Iterator<Item = T>
    {
        for item in iter {
            match self.send(item) {
                Err(_) => return Err(()),
                Ok(_) => {}
            }
        }
        Ok(())
    }
}

trait SinkIter<T> {
    fn sink<S>(self, sink: S) -> Result<(), ()> where S: Sink<T>;
}

impl<T, I> SinkIter<T> for I
    where I: Iterator<Item = T>
{
    fn sink<S>(self, sink: S) -> Result<(), ()>
        where S: Sink<T>
    {
        sink.drain(self)
    }
}





#[cfg(test)]
mod tests {
    use super::SinkIter;
    use std::sync::mpsc::channel;
    #[test]
    fn it_works() {
        let (tx, rx) = channel();
        let _ = (0..3).sink(tx);


        let v: Vec<_> = rx.iter().take(3).collect();
        assert_eq!(v, vec![0, 1, 2]);

    }
}
