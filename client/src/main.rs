mod scheduler;

use scheduler::Scheduler;

fn main() -> ! {
    let scheduler = Scheduler::new();

    loop {
        println!("render");

        for i in 0..10 {
            scheduler.spawn(async move { i * i });
        }

        std::thread::sleep(std::time::Duration::from_secs(5));

        for report in scheduler.ready() {
            let id = report.id;
            let value: &i32 = report.value.downcast_ref().unwrap();
            println!("{id:?}: {value}");
        }
    }
}
