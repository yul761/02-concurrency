use anyhow::Result;
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Consumer: {:?}", msg);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Error joining consumer thread: {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        thread::sleep(std::time::Duration::from_millis(1000));
    }
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
