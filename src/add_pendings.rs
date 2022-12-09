use futures::Stream;
use pin_project::pin_project;
use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

#[pin_project]
pub struct AddPendings<St> {
    #[pin]
    stream: St,
    remaining: usize,
    pendings: usize,
}

impl<St> AddPendings<St> {
    pub(super) fn new(stream: St, pendings: usize) -> Self {
        Self {
            stream,
            pendings,
            remaining: pendings,
        }
    }
}

impl<St> fmt::Debug for AddPendings<St>
where
    St: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddPendings")
            .field("stream", &self.stream)
            .field("pendings", &self.pendings)
            .field("remaining", &self.remaining)
            .finish()
    }
}

impl<St> Stream for AddPendings<St>
where
    St: Stream,
{
    type Item = St::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let me = self.project();
        println!("> remaining: {:?}", me.remaining);

        if *me.remaining == 0 {
            let res = me.stream.poll_next(cx);
            match res {
                Poll::Pending => Poll::Pending,
                Poll::Ready(x) => {
                    *me.remaining = *me.pendings;
                    Poll::Ready(x)
                }
            }
        } else {
            *me.remaining -= 1;
            println!("< remaining: {:?}", me.remaining);
            Poll::Pending
        }
    }
}
