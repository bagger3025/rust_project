use super::{linked_list_node::LinkedListNode, LinkedList};

pub struct LinkedListIterator<'a, T> {
    node: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = LinkedListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: Double Linked List is borrowed, so ptr is also allowed to bor
        //         rowed.
        Self::IntoIter {
            node: self.head.map(|ref ptr| unsafe { ptr.as_ref() }),
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T>
where
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            Some(node) => {
                let ret = node.get_data();
                self.node = node.next().map(|ptr| {
                    // SAFETY: Self has shared reference, hence it cannot be cha
                    //         nged while it is borrowing, and it is safe to cal
                    //         l as_ref()
                    unsafe { ptr.as_ref() }
                });
                Some(ret)
            }
            None => None,
        }
    }
}
