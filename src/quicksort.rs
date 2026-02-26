use crate::insertion::InsertionSort;
use crate::sorter::IntSorter;
use rand::Rng;

const INS_THRESHOLD: usize = 50;
pub struct QuicksortFixedPivot;
pub struct QuicksortRandomPivot;

pub struct QuicksortFixedPivotInsertion;

pub struct QuicksortRandomPivotInsertion;

impl IntSorter for QuicksortFixedPivot {
    fn sort(&self, slice: &mut [i32]) {
        fn fixed_pivot(slice: &[i32]) -> usize {
            return slice.len() - 1;
        }
        quicksort_core(slice, &fixed_pivot, 1);
    }
}

impl IntSorter for QuicksortRandomPivot  {
    fn sort(&self, slice: &mut [i32]) {
               fn random_pivot(slice: &[i32]) -> usize {
    rand::thread_rng().gen_range(0..slice.len())
}
        quicksort_core(slice, &random_pivot, 1);
    }
}

impl IntSorter for QuicksortFixedPivotInsertion {
    fn sort(&self, slice: &mut [i32]) {
        fn fixed_pivot(slice: &[i32]) -> usize {
            return slice.len() - 1;
        }
        quicksort_core(slice, &fixed_pivot, INS_THRESHOLD);
    }
}

impl IntSorter for QuicksortRandomPivotInsertion  {
    fn sort(&self, slice: &mut [i32]) {
               fn random_pivot(slice: &[i32]) -> usize {
    rand::thread_rng().gen_range(0..slice.len())
}
        quicksort_core(slice, &random_pivot, INS_THRESHOLD);
    }
}
/**
 * sort an array to assending order using hoare partition scheme
 */
fn quicksort_core<F>(slice: &mut [i32], choose_pivot: &F, insertion_threshold: usize)
where
    F: Fn(&[i32]) -> usize,
{
    if slice.len() <= insertion_threshold {
        InsertionSort.sort(slice);
        return;
    }
    //Hoares quicksort based on psudocode from https://en.wikipedia.org/wiki/Quicksort
    let pivot_index = choose_pivot(slice);
    slice.swap(pivot_index, 0);
    let pivot = slice[0];

    let mut i = 0;
    let mut j =slice.len() - 1;

    let split_index = loop {
        while slice[i] < pivot {
            i += 1;
        }
        while slice[j] > pivot {
            j -= 1;
        }
        if i >= j  {
            break j;
        }
        slice.swap(j, i);
        // move inward 
        i +=1;
        j -= 1;
    };
    let (left, right) = slice.split_at_mut(split_index+1);
    quicksort_core(left, choose_pivot, INS_THRESHOLD);
    quicksort_core(right, choose_pivot, INS_THRESHOLD);
}


