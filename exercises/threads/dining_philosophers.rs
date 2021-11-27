// https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html
use std::time::Duration;
use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        // we call lock() which means If the mutex is currently being accessed by someone else, 
        // we’ll block until it becomes available.
        // The call to lock() might fail, and if it does, we want to crash.
        // In this case, the error that could happen is that the mutex is ‘poisoned’, 
        // which is what happens when the thread panics while the lock is held. 
        // Since this shouldn’t happen, we just use unwrap().
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}
// This Table has a vector of Mutexes.A mutex is a way to control concurrency:
// only one thread can access the contents at once. This is exactly the property we need with our forks. 
// We use an empty tuple, (), inside the mutex, since we’re not actually going to use the value, just hold onto it.
struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 4, 0),
        // there’s one more detail here, and it’s very important. 
        // If you look at the pattern, it’s all consistent until the very end. 
        // Monsieur Foucault should have 4, 0 as arguments, but instead, has 0, 4. 
        // This is what prevents deadlock, actually: one of our philosophers is left handed! 
        // This is one way to solve the problem, and in my opinion, it’s the simplest.
        // Philosopher::new("Michel Foucault", 0, 4),
    ];

    // Next, in main(), we make a new Table and wrap it in an Arc<T>.
    // ‘arc’ stands for ‘atomic reference count’, and we need that to share our Table across multiple threads.
    // As we share it, the reference count will go up, and when each thread ends, it will go back down.
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let handles: Vec<_> = philosophers.into_iter()
        .map(|p| {
            // The clone() method on Arc<T> is what bumps up the reference count,
            //and when it goes out of scope, it decrements the count. 
            // This is needed so that we know how many references to table exist across our threads. 
            // If we didn’t have a count, we wouldn’t know how to deallocate it.
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}