// +-----+              +-------------+            +-----------+
// | New |--publish()-->| Unmoderated |--allow()-->| Published |
// +-----+              +-------------+            +-----------+
//                            |                          |
//                          deny()                    delete()
//                            |       +---------+        |
//                            +------>| Deleted |<-------+
//                                    +---------+

// States
#[derive(Debug)]
struct New;
#[derive(Debug)]
struct Unmoderated;
#[derive(Debug)]
struct Published;
#[derive(Debug)]
struct Deleted;

// Transitions
impl New {
    fn publish(self) -> Unmoderated {
        Unmoderated {}
    }
}

impl Unmoderated {
    fn allow(self) -> Published {
        Published {}
    }
    fn deny(self) -> Deleted {
        Deleted {}
    }
}

impl Published {
    fn delete(self) -> Deleted {
        Deleted {}
    }
}

fn main() {
    println!(
        "\
+-----+              +-------------+            +-----------+
| New |--publish()-->| Unmoderated |--allow()-->| Published |
+-----+              +-------------+            +-----------+
                           |                          |
                         deny()                    delete()
                           |       +---------+        |
                           +------>| Deleted |<-------+
                                   +---------+"
    );
    println!();

    println!("First path: New -> Unmoderated -> Deleted");
    let state = New {};
    println!("Initial: {:?}", state);
    let state = state.publish();
    println!("after publish: {:?}", state);
    let state = state.deny();
    println!("after deny: {:?}", state);
    println!();

    println!("Second path: New -> Unmoderated -> Published -> Deleted");
    let state = New {};
    println!("Initial: {:?}", state);
    let state = state.publish();
    println!("after publish: {:?}", state);
    let state = state.allow();
    println!("after allow: {:?}", state);
    let state = state.delete();
    println!("after delete: {:?}", state);
}
