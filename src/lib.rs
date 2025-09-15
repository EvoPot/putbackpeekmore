#![no_std]

///A smart iterator that lets you peek at more than x value of it and put back.
/// Make sure to use a large enough `BUFSIZE` , otherwise you will read garbage data.
/// The minimum amount value of `BUFSIZE` should be *how much you are going to peek more* + 1
pub struct PutBackPeekMore<Iter, const BUFSIZE: usize>
where
    Iter: Iterator,
{
    /// The iterator to consume.
    pub(crate) iter: Iter,
    /// A buffer containing "peek" data. Note that reading this blindly will give you garbage data, as its allocated efficiently according to different calls.
    pub(crate) peek: [Option<Iter::Item>; BUFSIZE],
    /// A smart counter thats used to decide when allocations should be made in the peek field.
    pub(crate) fizz: usize,
}

impl<Iter, const BUFSIZE: usize> PutBackPeekMore<Iter, BUFSIZE>
where
    Iter: Iterator,
{
    ///Create a new iterator.
    pub fn new(mut iter: Iter) -> Self {
        let peek: [Option<Iter::Item>; BUFSIZE] = [(); BUFSIZE].map(|_| iter.next());
        Self {
            iter,
            peek,
            fizz: 0,
        }
    }

    ///Look at the next value of the iterator without consuming it.
    pub fn peek(&mut self) -> &Option<Iter::Item> {
        self.demand(1);
        &self.peek[self.fizz]
    }

    ///Look at the next `amount` values of the iterator without consuming it.
    pub fn peek_value(&mut self, amount: usize) -> &[Option<Iter::Item>] {
        self.demand(amount);
        &self.peek[self.fizz..self.fizz + amount]
    }

    ///Tells the struct to allocate data in the peek field according to the ``val`` parameter.
    pub(crate) fn demand(&mut self, val: usize) {
        if self.fizz + val > self.peek.len() {
            self.write_over_start();
        }
    }

    ///Replaces every value at the structs `peek` field with the consumed values of the structs `iter` field.
    pub(crate) fn write_over_start(&mut self) {
        self.peek = [(); BUFSIZE].map(|_| self.iter.next());
        self.fizz = 0;
    }

    ///Replaces every value after the structs `peek` field after the `val` parameter with the consumed values of the structs `iter` field.
    pub(crate) fn write_over_val(&mut self, val: usize) {
        for v in self.peek[val..].iter_mut() {
            *v = self.iter.next();
        }
        self.fizz = val;
    }

    ///Change the next consumed value of the iterator.
    pub fn put_back(&mut self, val: Option<Iter::Item>) {
        if self.fizz > 0 {
            self.peek[self.fizz - 1] = val;
            self.fizz -= 1;
        } else {
            self.write_over_val(1);
            self.put_back(val);
            self.fizz -= 1;
        }
    }
}

impl<Iter, const PEEK: usize> core::fmt::Debug for PutBackPeekMore<Iter, PEEK>
where
    Iter: Iterator,
    Iter::Item: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PutBackPeekMore")
            .field("iter", &"...")
            .field("peek", &self.peek)
            .field("fizz", &self.fizz)
            .finish()
    }
}

impl<Iter, const PEEK: usize> Iterator for PutBackPeekMore<Iter, PEEK>
where
    Iter: Iterator,
{
    type Item = Iter::Item;
    ///Consume the iterator.
    fn next(&mut self) -> Option<Self::Item> {
        self.demand(1);
        let out = self.peek[self.fizz].take();
        self.fizz += 1;
        out
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::PutBackPeekMore;

    #[test]
    fn test_peek_value() {
        let mut iter: PutBackPeekMore<core::ops::Range<i32>, 3> = PutBackPeekMore::new(0..10);
        assert_eq!(iter.peek_value(3), &[Some(0), Some(1), Some(2)]);
    }

    #[test]
    fn test_peek() {
        let mut iter: PutBackPeekMore<core::ops::Range<i32>, 3> = PutBackPeekMore::new(0..10);
        assert_eq!(iter.peek(), &Some(0));
    }

    #[test]
    fn test_next() {
        let mut iter: PutBackPeekMore<core::ops::Range<i32>, 3> = PutBackPeekMore::new(0..10);
        assert_eq!(iter.next(), Some(0));
    }

    #[test]
    fn test_peek_multiple_calls() {
        let mut iter: PutBackPeekMore<core::ops::Range<i32>, 3> = PutBackPeekMore::new(0..10);
        std::println!("{:#?}", iter);
        assert_eq!(iter.peek(), &Some(0));
        iter.next();
        assert_eq!(iter.peek(), &Some(1));
    }

    #[test]
    fn test_put_back() {
        let mut iter: PutBackPeekMore<core::ops::Range<i32>, 3> = PutBackPeekMore::new(0..10);
        iter.next();
        iter.put_back(Some(0));
        assert_eq!(iter.peek(), &Some(0));
    }
}
