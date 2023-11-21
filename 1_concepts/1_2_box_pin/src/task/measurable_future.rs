use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

struct MeasurableFuture<Fut> {
    inner_future: Fut,
    started_at: Option<std::time::Instant>,
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = <Fut as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let p;
        unsafe {
            let r = self.get_unchecked_mut();
            r.started_at.get_or_insert(Instant::now());
            p = Pin::new_unchecked(&mut r.inner_future).poll(cx);

            if p.is_ready() {
                println!(
                    "Future was executed {}ns",
                    r.started_at.unwrap().elapsed().as_nanos()
                );
            }
        }
        p
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;
    use tokio;

    #[tokio::test]
    async fn test_all() {
        MeasurableFuture {
            inner_future: tokio::time::sleep(Duration::from_secs(1)),
            started_at: None,
        }
        .await
    }
}
