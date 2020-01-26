use std::sync::{Mutex, Arc, MutexGuard};
use std::time::Duration;
use std::any::Any;
use std::thread;


type Lock = Arc<Mutex<bool>>;
struct Deadlock(Lock, Lock);

impl Default for Deadlock {
    fn default() -> Self {
        Self(Arc::new(Mutex::new(false)), Arc::new(Mutex::new(false)))
    }
}

impl Clone for Deadlock {
    fn clone(&self) -> Self {
        Self(self.1.clone(), self.0.clone())
    }
}

impl Deadlock {
    fn lock_first(&self) -> MutexGuard<bool> {
        self.0.lock().unwrap()
    }

    fn lock_second(&self) -> MutexGuard<bool> {
        self.1.lock().unwrap()
    }

    fn do_it(&self, wait: u64) {
        let mut a = self.lock_first();
        thread::sleep(Duration::from_millis(wait));
        let b = self.lock_second();

        *a = !*b;
        println!("Done!")
    }
}


fn main() -> Result<(), Box<dyn Any + Send + 'static>> {
    let x = Deadlock::default();
    let y = x.clone(); // clone swap mutexes

    let handle = thread::spawn(move || x.do_it(5));
    y.do_it(5);

    handle.join()
}
