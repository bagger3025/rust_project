pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub trait IteratorExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator;
}

impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
    {
        flatten(self.into_iter())
    }
}

pub struct Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    outer: I,
    front_iter: Option<<I::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Flatten<I>
where
    I: Iterator,
    <I as Iterator>::Item: IntoIterator,
{
    fn new(outer: I) -> Self {
        Self {
            outer,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<I> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut next_iter) = self.front_iter {
                if let Some(i) = next_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }

            if let Some(next_inner) = self.outer.next() {
                self.front_iter = Some(next_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }

    // fn next(&mut self) -> Option<Self::Item> {
    //     let inner = if let Some(inner) = self.inner.as_mut() {
    //         inner
    //     } else {
    //         let outer = self.outer.next()?;
    //         self.inner = Some(outer.into_iter());
    //         self.inner.as_mut().unwrap()
    //     };
    //     let next_data = {
    //         let mut data = inner.next();
    //         while data.is_none() {
    //             let outer = self.outer.next()?;
    //             *inner = outer.into_iter();
    //             data = inner.next();
    //         }
    //         data
    //     };
    //     next_data
    // }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }

            if let Some(next_back_inner) = self.outer.next_back() {
                self.back_iter = Some(next_back_inner.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[test]
fn zero() {
    let v: Vec<Vec<()>> = vec![vec![]];
    let it = Flatten::new(v.into_iter());

    assert!(it.collect::<Vec<()>>().len() == 0);
}

#[test]
fn one() {
    let v = vec![vec![()]];
    let it = Flatten::new(v.into_iter());

    assert!(it.collect::<Vec<()>>().len() == 1);
}

#[test]
fn two() {
    let v = vec![vec![()], vec![()]];
    let it = Flatten::new(v.into_iter());

    assert!(it.collect::<Vec<()>>().len() == 2);
}

#[test]
fn two_inner() {
    let v = vec![vec![(), ()]];
    let it = Flatten::new(v.into_iter());

    assert!(it.collect::<Vec<()>>().len() == 2);
}

#[test]
fn one_after_one() {
    let v = vec![vec![()], vec![], vec![()]];
    let it = Flatten::new(v.into_iter());

    assert!(it.collect::<Vec<()>>().len() == 2);
}

#[test]
fn back_iterator() {
    let v = vec![vec![1, 2], vec![], vec![3]];
    let it = flatten(v);
    let it = it.rev();

    let c = it.collect::<Vec<_>>();
    assert_eq!(c, vec![3, 2, 1]);
}

#[test]
fn next_and_back_iterator() {
    let v = vec![vec![1, 2], vec![], vec![3, 4]];
    let mut it = flatten(v);
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next_back(), Some(4));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next_back(), Some(3));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn next_and_back_iterator2() {
    let v = vec![vec![1, 2, 3], vec![], vec![4]];
    let mut it = flatten(v);
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next_back(), Some(4));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next_back(), Some(3));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
}

#[test]
fn deep() {
    assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
}

#[test]
fn ext() {
    assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
}
