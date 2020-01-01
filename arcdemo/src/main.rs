
use std::sync::Arc;
use std::thread;

fn main() {
    let var: Arc<i32> = Arc::new(5);
    let share_var = var.clone();

    let new_thread = thread::spawn(move || {
        println!("thread value {} ,addresd {:p}", share_var, &*share_var);
    });

    new_thread.join().unwrap();
    println!("thread value {} ,addresd {:p}", var, &*var);
}