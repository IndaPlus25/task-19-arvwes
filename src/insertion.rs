use crate::sorter::IntSorter;

pub struct InsertionSort;

impl IntSorter for InsertionSort {
    fn sort(&self, slice: &mut [i32]) {
        let mut i = 1;
        while i < slice.len() {
            let mut j = i;
            while j > 0 && slice[j - 1] > slice[j] {
                slice.swap(j-1, j);
                j -= 1;
            }
            i +=1;
        }
    }
}
