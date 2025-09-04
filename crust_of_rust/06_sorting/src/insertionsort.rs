use super::Sorter;

pub struct InsertionSort {
    pub smart: bool,
}

impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        if !self.smart {
            for i in 1..slice.len() {
                for j in (1..=i).rev() {
                    if slice[j - 1] > slice[j] {
                        slice.swap(j - 1, j);
                    } else {
                        break;
                    }
                }
            }
        } else {
            for unsorted in 1..slice.len() {
                // Use binary search to find index
                // Then use .insert to splice in i
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    // [a, c, e].binary_search(c) => Ok(1);
                    Ok(i) => i,
                    // [a, c, e].binary_search(b) => Err(1);
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn it_works_not_smart() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort { smart: false }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

#[test]
fn it_works_smart() {
    let mut things = vec![4, 2, 5, 3, 1];
    InsertionSort { smart: true }.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
