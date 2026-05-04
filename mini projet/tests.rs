use super::{LinkedList, list};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let ll = LinkedList::from([1, 2, 3, 4, 5]);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        assert_eq!(ll.len(), 5);
        ll.pop_front();
        assert_eq!(ll.len(), 4);
        ll.pop_front();
        assert_eq!(ll.len(), 3);
        ll.pop_front();
        assert_eq!(ll.len(), 2);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![4, 5]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        assert_eq!(ll.len(), 5);
        ll.pop_back();
        assert_eq!(ll.len(), 4);
        ll.pop_back();
        assert_eq!(ll.len(), 3);
        ll.pop_back();
        assert_eq!(ll.len(), 2);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![1, 2]);

        let mut l1 = LinkedList::from([1, 2, 3, 4, 5]);
        let mut l2 = LinkedList::from([6, 7, 8, 9, 10]);
        l1.append(&mut l2);
        let v: Vec<_> = l1.into();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let ll = LinkedList::from([1, 2, 3, 4, 5]);
        assert_eq!(ll.front(), Some(&1));
        assert_eq!(ll.back(), Some(&5));
        assert_eq!(ll.len(), 5);

        let ll: LinkedList<usize> = LinkedList::new();
        assert_eq!(ll.front(), None);
        assert_eq!(ll.back(), None);
        assert_eq!(ll.is_empty(), true);
        assert_eq!(ll.len(), 0);

        let mut ll: LinkedList<usize> = LinkedList::new();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        ll.push_front(4);
        ll.push_front(5);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![5, 4, 3, 2, 1]);

        let mut ll: LinkedList<usize> = LinkedList::new();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        ll.push_back(4);
        ll.push_back(5);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        //0  1  2  3  4  5  6  7
        let mut ll = LinkedList::from([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(ll.pop_at(0), Some(1));
        assert_eq!(ll.pop_at(3), Some(5));
        assert_eq!(ll.pop_at(1), Some(3));
        assert_eq!(ll.pop_at(4), Some(8));
        assert_eq!(ll.pop_at(2), Some(6));
        assert_eq!(ll.len(), 3);

        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![2, 4, 7]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.reverse();
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![5, 4, 3, 2, 1]);

        let mut l1 = LinkedList::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

        let mut iter = l1.iter();
        for _ in 0..5 {
            iter.next();
        }

        //let l2 = unsafe { l1.split_off_before_node(iter.head, 5) };
        //let v1: Vec<_> = l1.into();
        //let v2: Vec<_> = l2.into();
        //assert_eq!(v1, vec![6, 7, 8, 9, 10]);
        //assert_eq!(v2, vec![1, 2, 3, 4, 5]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.push_at(0, -1);
        ll.push_at(1, 0);
        ll.push_at(ll.len, 7);
        ll.push_at(ll.len - 1, 6);
        ll.push_at(ll.len / 2, 99);

        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![-1, 0, 1, 2, 99, 3, 4, 5, 6, 7]);
        //0   1  2  3  4  5  6  7  8  9

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.rotate_right(3);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![3, 4, 5, 1, 2]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);

        ll.rotate_right(1);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![5, 1, 2, 3, 4]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);

        ll.rotate_right(0);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.rotate_right(5);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.rotate_right(4);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![2, 3, 4, 5, 1]);

        //-------

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.rotate_left(3);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![4, 5, 1, 2, 3]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);

        ll.rotate_left(1);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![2, 3, 4, 5, 1]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);

        ll.rotate_left(0);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.rotate_left(5);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);

        let mut ll = LinkedList::from([1, 2, 3, 4, 5]);
        ll.rotate_left(4);
        let v: Vec<_> = ll.into();
        assert_eq!(v, vec![5, 1, 2, 3, 4]);

        // TODO: test peek_next and peek_next_back and impelement them for
        // iterMut

        //let mut l1 = LinkedList::from([1, 3, 5, 7, 9, 12]);
        //let mut l2 = LinkedList::new();
        //l1.merge_by(&mut l2, |a, b| a.cmp(b));
        //let v: Vec<_> = l1.into();
        //assert_eq!(v, [1, 3, 5, 7, 9, 12]);
        //
        //let mut l1 = LinkedList::new();
        //let mut l2 = LinkedList::from([1, 3, 5, 7, 9, 12]);
        //l1.merge_by(&mut l2, |a, b| a.cmp(b));
        //let v: Vec<_> = l1.into();
        //assert_eq!(v, [1, 3, 5, 7, 9, 12]);
        //
        //let mut l1 = LinkedList::from([1, 3, 5, 7, 9, 12]);
        //let mut l2 = LinkedList::from([2, 4, 6, 8, 10, 11]);
        //l1.merge_by(&mut l2, |a, b| a.cmp(b));
        //let v: Vec<_> = l1.into();
        //assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

        //MACRO Testing
        let ll: LinkedList<usize> = list![];
        let v: Vec<_> = ll.into();
        assert_eq!(v, []);

        let ll = list![1, 2, 3, 4, 5];
        let v: Vec<_> = ll.into();
        assert_eq!(v, [1, 2, 3, 4, 5]);

        let ll = list![10;10];
        //println!("{:?}", ll);
        let v: Vec<_> = ll.into();
        assert_eq!(v, [10; 10]);

        let mut ll = list![1, 2, 3, 4, 5];
        ll.truncate(3);
        let v: Vec<_> = ll.into();
        assert_eq!(v, [1, 2, 3]);

        let mut ll = list![1, 2, 3, 4, 5];
        ll.extend_with(3, 10);
        assert_eq!(ll, list![1, 2, 3, 4, 5, 10, 10, 10]);

        let l1 = ll.clone();

        assert_eq!(ll, l1);

        let l1 = list![1, 2, 3, 4, 5, 6];
        let l2 = list![1, 2, 3];

        assert!(l1 > l2);

        let l2 = list![1, 2, 3, 4, 5, 6];
        assert!(l1 >= l2);

        let l2 = list![1, 2, 1, 4, 5, 6, 8, 9, 10];
        assert!(l1 > l2);

        //let mut l1 = list![1, 2, 3, 4, 5];
        //let mut l2 = list![6, 7, 8, 9, 10];
        //l1.merge_by(&mut l2, |a, b| a.cmp(b));
        //println!("{:?}", l1);
        //               0  1 2 3 4 5 8 9
        let mut l1 = list![1, 2, 3, 4, 5, 6, 7, 8];
        let l2 = l1.split_off(5);

        assert_eq!(l1, list![1, 2, 3, 4, 5]);
        assert_eq!(l2, list![6, 7, 8]);

        //               0  1  2  3  4  5  8  9
        let mut l1 = list![1, 2, 3, 4, 5, 6, 7, 8];
        let l2 = l1.split_off(3);

        assert_eq!(l1, list![1, 2, 3]);
        assert_eq!(l2, list![4, 5, 6, 7, 8]);

        /*
         * sort() → use default ordering

         sort_by() → full control

         sort_by_key() → clean, simple field-based sorting

         sort_by_cached_key() → when key extraction is expensive
        */
    }
}
