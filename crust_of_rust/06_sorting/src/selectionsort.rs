use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..slice.len() {
            let min_idx = slice[i..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(idx, _)| idx + i)
                .expect("slice is non-empty");
            // let mut min_idx = i;
            // for j in (i + 1)..slice.len() {
            //     if slice[min_idx] > slice[j] {
            //         min_idx = j;
            //     }
            // }
            slice.swap(i, min_idx);
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
