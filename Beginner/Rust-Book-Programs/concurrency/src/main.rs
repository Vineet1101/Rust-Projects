use std::sync::{Mutex,Arc};
use std::thread;

fn main(){
    let m=Arc::new(Mutex::new(0));
    let mut handles=vec![];
    for _ in 1..11{
        let m=Arc::clone(&m);
        let handle=thread::spawn(move||{
            let mut val=m.lock().unwrap();
            *val+=1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("{}",*m.lock().unwrap());
}