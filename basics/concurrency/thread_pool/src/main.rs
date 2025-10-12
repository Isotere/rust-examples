use std::collections::VecDeque;
use std::sync::{mpsc, LazyLock, Mutex};

static WORK_QUEUE: LazyLock<Mutex<VecDeque<String>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

fn main() {
    let cpu_count = num_cpus::get();
    println!("Number of cores is {}", cpu_count);

    let mut threads = Vec::with_capacity(cpu_count);
    let mut broadcast = Vec::with_capacity(cpu_count);

    for cpu in 0..cpu_count {
        let (tx, rx) = mpsc::channel::<()>();
        broadcast.push(tx);

        let thread = std::thread::spawn(move || {
            while let Ok(()) = rx.recv() {
                let mut lock = WORK_QUEUE.lock().unwrap();
                if let Some(job) = lock.pop_front() {
                    drop(lock);
                    println!("CPU {cpu} got job {job}");
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    println!("CPU {cpu} finished!");
                } else {
                    // println!("CPU {cpu} found no job!");
                }
            }
        });

        threads.push(thread);
    }

    let mut count = 0;
    loop {
        count += 1;

        let sent = {
            let mut lock = WORK_QUEUE.lock().unwrap();
            let len = lock.len();
            println!("There are {len} jobs in the queue");
            if len < 5 {
                lock.push_back(format!("#{}", count));
                true
            } else {
                false
            }
        };

        if sent {
            broadcast.iter().for_each(|tx| tx.send(()).unwrap());
        }

        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}
