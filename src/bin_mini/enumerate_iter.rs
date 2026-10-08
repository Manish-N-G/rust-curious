// See TODO at the very bottom.
//
// Implement Iterator for the Enumerate iterator adapter
// so the unit tests pass. See commented-out unit tests
// for bonus challenges.

/// An iterator adapter that tracks the iteration count.
pub struct Enumerate<I> {
    // The underlying iterator
    iter: I,

    // The iteration count
    n: usize,
}

// Old implementation that take only Iterartor
// Wraps the iterator `iter` in the Enumerate adapter.
// pub fn enumerate<I: Iterator>(iter: I) -> Enumerate<I> {
//     Enumerate {
//         iter,
//         n: 0,
//     }
// }

// New implementation that takes also reference to values of
// Iterator. We do this because we can convert the value to 
// and Interator, becase refernce to Iterator as of type
// IntoIterator
pub fn enumerate<I: IntoIterator>(iter: I) -> Enumerate<I::IntoIter> {
    Enumerate {
        iter: iter.into_iter(),
        n: 0,
    }
}

#[test]
fn test_for() {
    for (i, x) in enumerate(0..10) {
        assert_eq!(i, x);
    }
}

#[test]
fn test_next() {
    let mut iter = enumerate('a'..='e');
    assert_eq!(iter.next(), Some((0, 'a')));
    assert_eq!(iter.next(), Some((1, 'b')));
    assert_eq!(iter.next(), Some((2, 'c')));
    assert_eq!(iter.next(), Some((3, 'd')));
    assert_eq!(iter.next(), Some((4, 'e')));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_for_array() {
    let arr = [0, 1, 2, 3];
    for (i, x) in enumerate(arr.iter()) {
        assert_eq!(i, *x);
    }
}

// BONUS:
// Uncomment this test and override size_hint so it calls
// the size_hint method on the underlying iterator.
#[test]
fn test_size_hint() {
    let mut iter = enumerate('a'..='e');
    assert_eq!(iter.size_hint(), (5, Some(5)));
    assert_eq!(Iterator::size_hint(&iter), (5, Some(5)));
    // Adapted to have next call to reduce the start size
    iter.next();
    assert_eq!(iter.size_hint(), (4, Some(4)));
    assert_eq!(Iterator::size_hint(&iter), (4, Some(4)));
}

// BONUS:
// Uncomment this test and implement ExactSizeIterator
// if the underlying iterator I implements it.
#[test]
fn test_exact_size() {
    let mut iter = enumerate(10..15);
    assert_eq!(iter.len(), 5);
    assert_eq!(ExactSizeIterator::len(&iter), 5);
    iter.next();
    assert_eq!(iter.len(), 4);
    assert_eq!(ExactSizeIterator::len(&iter), 4);
}

// BONUS that I thought of after recording the video:
// Uncomment this test and change the enumerate function
// so it accepts any type that implements IntoIterator.
// (Requires making changes above, only to `enumerate`.)
#[test]
fn test_intoiter() {
    let arr = [0, 1, 2, 3];
    // &arr is not Iterator, but it is IntoIterator
    for (i, x) in enumerate(&arr) {
        assert_eq!(i, *x);
    }
}

// No need to change anything above this line.
// -------------------------------------------------------

// TODO: impl Iterator
// 
// The Item type should be a tuple containing the iteration
// count and the item from the underlying iterator.
impl<I: Iterator> Iterator for Enumerate<I> {
    type Item = (usize, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(val) => {
                self.n +=1;
                Some((self.n-1, val))
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        //NOTE: for size hint, there is a default implementation
        //that is used which gives us (0, None) as the output.
        //However, for certain types, we get the implementation
        //that overrides the default implementation.
        //For our situation, with range, size_range is implemented
        //for it and hence we can call just size_hint() for our
        //type. Internally it uses a nightly method called step_between

        self.iter.size_hint()
    }
}

// 2 already provided methods. len and is_empty
impl<I:Iterator> ExactSizeIterator for Enumerate<I> {}



