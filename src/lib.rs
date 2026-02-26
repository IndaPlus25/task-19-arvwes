pub mod insertion;
pub mod quicksort;
pub mod sorter;

#[cfg(test)]
mod test {

    use super::insertion::InsertionSort;
    use super::quicksort::{
        QuicksortFixedPivot, QuicksortFixedPivotInsertion, QuicksortRandomPivot,
        QuicksortRandomPivotInsertion,
    };
    use super::sorter::IntSorter;

    fn run_all_tests<T: IntSorter>(sorter: &T) {
        let mut even = vec![4, 2, 1, 3];
        sorter.sort(&mut even);
        assert_eq!(even, vec![1, 2, 3, 4], "Failed on even length array");

        let mut odd = vec![5, 2, 4, 1, 3];
        sorter.sort(&mut odd);
        assert_eq!(odd, vec![1, 2, 3, 4, 5], "Failed on odd length array");

        let mut ascending = vec![1, 2, 3, 4, 5];
        sorter.sort(&mut ascending);
        assert_eq!(
            ascending,
            vec![1, 2, 3, 4, 5],
            "Failed on already sorted array"
        );

        let mut descending = vec![5, 4, 3, 2, 1];
        sorter.sort(&mut descending);
        assert_eq!(
            descending,
            vec![1, 2, 3, 4, 5],
            "Failed on descending array"
        );

        let mut random = vec![5, 2, 8, 1, 3];
        sorter.sort(&mut random);
        assert_eq!(random, vec![1, 2, 3, 5, 8], "Failed on randomised array");

        let mut all_equal = vec![1, 1, 1, 1, 1];
        sorter.sort(&mut all_equal);
        assert_eq!(
            all_equal,
            vec![1, 1, 1, 1, 1],
            "Failed on all equal elements"
        );

        let mut few_equal = vec![2, 1, 2, 1, 1, 2];
        sorter.sort(&mut few_equal);
        assert_eq!(
            few_equal,
            vec![1, 1, 1, 2, 2, 2],
            "Failed on few elements all equal"
        );
        let size = 1000;
        let mut large_reversed: Vec<i32> = (0..size).rev().collect();
        let expected: Vec<i32> = (0..size).collect();

        sorter.sort(&mut large_reversed);

        assert!(
            large_reversed == expected,
            "Failed on large array of size {}",
            size
        );
    }

     #[test]
    fn test_insertion_sort() {
        run_all_tests(&InsertionSort);
    }
    
     #[test]
    fn test_quicksort_fixed_pivot() {
        run_all_tests(&QuicksortFixedPivot);
    }
    #[test]
    fn test_quicksort_random_pivot() {
        run_all_tests(&QuicksortRandomPivot);
    }
     #[test]
    fn test_quicksort_random_pivot_insertion() {
        run_all_tests(&QuicksortRandomPivotInsertion);
    }
     #[test]
    fn test_quicksort_fixed_pivot_insertion() {
        run_all_tests(&QuicksortFixedPivotInsertion);
    }
    
}
