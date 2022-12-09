use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

use futures::Stream;
use pin_project::pin_project;

#[pin_project]
pub struct Split<S, F, T> {
    #[pin]
    stream: S,
    is_separator: F,
    items: Vec<T>,
}

impl<S, F, T> Split<S, F, T> {
    pub(super) fn new(stream: S, is_separator: F) -> Self {
        Self {
            stream,
            is_separator,
            items: Vec::new(),
        }
    }
}


impl<S, F, T> fmt::Debug for Split<S, F, T>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Split")
            .field("stream", &self.stream)
            .finish()
    }
}

impl<St, F, T> Stream for Split<St, F, T>
where
    St: Stream,
    F: FnMut(&St::Item) -> bool,
    T: From<St::Item> + Clone,
{
    type Item = Vec<T>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let res = self.as_mut().project().stream.poll_next(cx);
            match res {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => {
                    if self.items.is_empty() {
                        return Poll::Ready(None);
                    } else {
                        let xs = self.items.to_vec();
                        self.as_mut().project().items.clear();
                        return Poll::Ready(Some(xs));
                    }
                }
                Poll::Ready(Some(item)) => {
                    if (self.as_mut().project().is_separator)(&item) {
                        // a separator value
                        let xs = self.items.to_vec();
                        self.as_mut().project().items.clear();
                        return Poll::Ready(Some(xs));
                    } else {
                        // a data value
                        self.as_mut().project().items.push(item.into());
                    }
                }
            }
        }
    }
}