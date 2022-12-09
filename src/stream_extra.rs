use futures::Stream;

use crate::{add_pendings::AddPendings, split::Split};

/// An extension trait for the [`Stream`] trait that provides a variety of
/// convenient combinator functions.
///
/// [`Stream`]: https://docs.rs/futures-core/latest/futures_core/stream/trait.Stream.html
pub trait StreamExtra: Stream {
    /// Splits this stream's items at a separation item. The separation item
    /// is determined by provided closure. A stream of vectors of item type will be returned,
    /// which will yield elements until the closure returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main]
    /// # async fn main() {
    /// use tokio_stream::{self as stream, StreamExt};
    /// use tokio_stream_extra::StreamExtra;
    ///
    /// let stream = stream::iter(vec![1,2,0,3,4,0]);
    /// let mut stream = stream.split(|x| x == &0);
    ///
    /// assert_eq!(stream.next().await, Some(vec![1,2]));
    /// assert_eq!(stream.next().await, Some(vec![3,4]));
    /// # }
    /// ```

    fn split<F>(self, is_separator: F) -> Split<Self, F, Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> bool,
        Self::Item: Clone,
    {
        Split::new(self, is_separator)
    }

    fn add_pendings(self, pendings: usize) -> AddPendings<Self>
    where
        Self: Sized,
    {
        AddPendings::new(self, pendings)
    }
}

/// Blanket implementation of [`StreamExtra`] for all
/// types that implement [`Stream`]
impl<T> StreamExtra for T where T: Stream {}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use tokio_stream::{self as stream, StreamExt};

    #[tokio::test]
    async fn split_() {
        fn is_separator(s: &str) -> bool {
            s == "\n"
        }

        let src = vec!["1234", "4567", "\n", "1020", "\n", "\n"];
        let xs: Vec<Vec<&str>> = stream::iter(src).split(|x| is_separator(x)).collect().await;

        assert_eq!(3, xs.len());

        let src = vec!["1234", "4567", "\n", "1020", "\n", "\n"];
        let xs: Vec<Vec<&str>> = stream::iter(src)
            .split(|x| is_separator(x))
            .filter(|xs| !xs.is_empty())
            .collect()
            .await;

        assert_eq!(2, xs.len());
    }

    #[tokio::test]
    async fn add_pendings_() {
        let _src = vec![1, 2, 3];
        // let xs: Vec<i32> = stream::iter(src).add_pendings(1).collect().await;
        // assert_eq!(vec![1, 2, 3], xs)

        // let mut s = stream::iter(src).add_pendings(1);
        // let x = s.next().await;
        // assert_eq!(Some(1), x)
    }
}
