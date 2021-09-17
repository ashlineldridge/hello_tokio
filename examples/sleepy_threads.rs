use rand::Rng;
use std::{thread, time};

const TOTAL_TASKS: u32 = 20;
const MIN_SLEEP_MILLIS: u64 = 1000;
const MAX_SLEEP_MILLIS: u64 = 2000;

#[tokio::main]
async fn main() {
    let mut rng = rand::thread_rng();
    let mut tasks = vec![];

    for i in 1..TOTAL_TASKS + 1 {
        println!("Spawning task {}/{}...", i, TOTAL_TASKS);
        let sleep_millis: u64 = rng.gen_range(MIN_SLEEP_MILLIS..MAX_SLEEP_MILLIS + 1);

        let handle = tokio::spawn(async move {
            run_task(i, sleep_millis).await;
        });

        tasks.push(handle);
    }

    for handle in tasks {
        handle.await.unwrap();
    }
}

async fn run_task(task_id: u32, sleep_millis: u64) {
    let thread_id = thread::current().id();
    println!(
        "Task (ID: {}, {:?}) is getting sleepy...",
        task_id, thread_id
    );
    thread::sleep(time::Duration::from_millis(sleep_millis));
    println!("Thread {:?} woke up.", thread_id);
}
