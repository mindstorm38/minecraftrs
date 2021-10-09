use std::io::{Read, IoSliceMut};


#[macro_export]
macro_rules! build_flags {
    ($($flag:expr),+) => {{
        let mut flag = 0;
        let mut shift = 0; // Should be opt-out
        $(
        if $flag { flag |= 1 << shift; }
        shift += 1;
        )+
        let _ = shift; // To avoid unused warning
        flag
    }};
}


/// A wrapper for a `Read` implementation that can be used to count how many bytes are
/// being read from the inner reader.
pub struct ReadCounter<R> {
    inner: R,
    count: usize
}

impl<R> ReadCounter<R> {

    pub fn new(inner: R) -> Self {
        Self {
            inner,
            count: 0
        }
    }

    pub fn count_with<F, E>(&mut self, op: F) -> std::io::Result<(usize, E)>
    where
        F: FnOnce(&mut Self) -> std::io::Result<E>
    {
        let start_count = self.count;
        let ret = op(self)?;
        Ok((self.count - start_count, ret))
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }

    #[inline]
    pub fn into_inner(self) -> R {
        self.inner
    }

    #[inline]
    fn increment_count(&mut self, res: std::io::Result<usize>) -> std::io::Result<usize> {
        if let Ok(size) = res {
            self.count += size;
        }
        res
    }

}


impl<R: Read> Read for ReadCounter<R> {

    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let res = self.inner.read(buf);
        self.increment_count(res)
    }

    #[inline]
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
        let res = self.inner.read_vectored(bufs);
        self.increment_count(res)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> std::io::Result<usize> {
        let res = self.inner.read_to_end(buf);
        self.increment_count(res)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let res = self.inner.read_to_string(buf);
        self.increment_count(res)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        let res = self.inner.read_exact(buf);
        if let Ok(()) = res {
            self.count += buf.len();
        }
        res
    }

}
