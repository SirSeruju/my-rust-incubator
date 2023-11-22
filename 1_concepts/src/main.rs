use std::thread;
use std::time::Duration;

use step_1::DoublyLinkedList as DLL;

fn main() {
    let dll = vec![1, 2, 3, 4].into_iter().collect::<DLL<i64>>();
    dll.push_left(0);
    dll.push_left(1);
    dll.push_right(5);
    println!("{:?}", dll);
    println!("Count {:?}", dll.count());

    let dll: DLL<i32> = DLL::new();
    let thread1;
    let thread2;

    {
        let dll = dll.clone();
        thread1 = thread::spawn(move || {
            for i in 1..10 {
                dll.push_right(i);
                thread::sleep(Duration::from_millis(200));
            }
        });
    }
    {
        let dll = dll.clone();
        thread2 = thread::spawn(move || {
            for _ in 1..30 {
                println!("Poped value: {:?}", dll.pop_right());
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    thread1.join().unwrap();
    thread2.join().unwrap();
}
