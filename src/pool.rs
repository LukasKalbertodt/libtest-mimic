use std::{sync, thread};

pub(crate) type Task = dyn FnOnce() + Send;
pub(crate) type BoxedTask = Box<Task>;

pub(crate) fn scoped_run_tasks(
    tasks: Vec<BoxedTask>,
    num_threads: usize,
) {
    if num_threads < 2 {
        // There is another code path for num_threads == 1 running entirely in the main thread.
        panic!("`run_on_scoped_pool` may not be called with `num_threads` less than 2");
    }

    let sync_iter = sync::Mutex::new(tasks.into_iter());
    let next_task = || sync_iter.lock().unwrap().next();

    thread::scope(|scope| {
        for _ in 0..num_threads {
            scope.spawn(|| {
                while let Some(task) = next_task() {
                    task();
                }
            });
        }
    });
}
