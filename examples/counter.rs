use async_observable::Observable;
use async_std::task::spawn;

#[async_std::main]
async fn main() {
    let mut observable = Observable::new(0);
    let mut tasks = vec![];

    for i in 0..10 {
        let mut fork = observable.clone();

        tasks.push(spawn(async move {
            let update = fork.next().await;

            println!(
                "Task {} was notified about updated observable {}",
                i, update
            );
        }));
    }

    observable.publish(1);

    for t in tasks {
        t.await
    }
}
