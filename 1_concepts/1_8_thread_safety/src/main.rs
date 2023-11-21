#![feature(negative_impls)]

#[derive(Debug)]
struct OnlySend;
unsafe impl Send for OnlySend {}
impl !Sync for OnlySend {}

#[derive(Debug)]
struct OnlySync;
impl !Send for OnlySync {}
unsafe impl Sync for OnlySync {}

#[derive(Debug)]
struct SyncAndSend;
unsafe impl Send for SyncAndSend {}
unsafe impl Sync for SyncAndSend {}

#[derive(Debug)]
struct NotSyncNotSend;
impl !Send for NotSyncNotSend {}
impl !Sync for NotSyncNotSend {}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

    #[test]
    fn test_only_send() {
        // static must be Sync
        // static a: OnlySend = OnlySend;
        let a = OnlySend;

        // `OnlySend` cannot be shared between threads safely
        // thread::spawn(|| {
        //     print!("{:?}", a);
        // });

        // All ok
        thread::spawn(move || {
            print!("{:?}", a);
        });
    }

    #[test]
    fn test_only_sync_static() {
        static a: OnlySync = OnlySync;

        // All ok
        thread::spawn(|| {
            print!("{:?}", a);
        });

        // All ok
        thread::spawn(move || {
            print!("{:?}", a);
        });
    }

    #[test]
    fn test_only_sync_let() {
        let a: OnlySync = OnlySync;

        // `OnlySync` cannot be sent between threads safely
        // thread::spawn(move || {
        //     print!("{:?}", a);
        // });

        // may outlive borrowed value `a`
        // thread::spawn(|| {
        //     print!("{:?}", a);
        // });
    }

    #[test]
    fn test_sync_and_send_static() {
        static a: SyncAndSend = SyncAndSend;

        // All ok
        thread::spawn(|| {
            print!("{:?}", a);
        });

        // All ok
        thread::spawn(move || {
            print!("{:?}", a);
        });
    }

    #[test]
    fn test_sync_and_send_let() {
        let a: SyncAndSend = SyncAndSend;

        // may outlive borrowed value `a`
        // thread::spawn(|| {
        //     print!("{:?}", a);
        // });

        // All ok
        thread::spawn(move || {
            print!("{:?}", a);
        });
    }

    #[test]
    fn test_not_sync_not_send() {
        // static must be Sync
        // static a: NotSyncNotSend = NotSyncNotSend;

        let a = NotSyncNotSend;

        // `NotSyncNotSend` cannot be sent between threads safely
        // thread::spawn(move || {
        //     print!("{:?}", a);
        // });

        // `NotSyncNotSend` cannot be shared between threads safely
        // thread::spawn(|| {
        //     print!("{:?}", a);
        // });
    }
}
