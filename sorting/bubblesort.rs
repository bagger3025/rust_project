use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut len = slice.len();

        loop {
            if len <= 1 {
                return;
            }

            let mut swapped = false;
            for i in 0..(len - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
            len -= 1;
            if !swapped {
                return;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    BubbleSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
