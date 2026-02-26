use std::{cell::Cell, io::{self, BufWriter, Read, Write}};

pub trait IntSorter {
    fn sort(&self, slice: &mut [i32]);
}

pub struct InsertionSort;

impl IntSorter for InsertionSort {
    fn sort(&self, slice: &mut [i32]) {
        let mut i = 1;
        while i < slice.len() {
            let mut j = i;
            while j > 0 && slice[j - 1] > slice[j] {
                slice.swap(j - 1, j);
                j -= 1;
            }
            i += 1;
        }
    }
}

const INS_THRESHOLD: usize = 50;

pub struct QuicksortRandomPivotInsertion;

impl IntSorter for QuicksortRandomPivotInsertion {
    fn sort(&self, slice: &mut [i32]) {
        // 1. Initialize a seed for our custom random number generator
        let rng_state = Cell::new(123456789u64);

        let random_pivot = |s: &[i32]| -> usize {
            // 2. Perform standard Linear Congruential Generator (LCG) math
            let mut state = rng_state.get();
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            
            // 3. Save the new state for the next time the closure is called
            rng_state.set(state);

            // 4. Shift the bits and modulo by the slice length to get a valid index
            (state >> 32) as usize % s.len()
        };

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
    let mut j = slice.len() - 1;

    let split_index = loop {
        while slice[i] < pivot {
            i += 1;
        }
        while slice[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break j;
        }
        slice.swap(j, i);
        // move inward
        i += 1;
        j -= 1;
    };
    let (left, right) = slice.split_at_mut(split_index + 1);
    quicksort_core(left, choose_pivot, INS_THRESHOLD);
    quicksort_core(right, choose_pivot, INS_THRESHOLD);
}

fn main() -> io::Result<()> {
    // 1. FAST I/O: Read all input at once
    let mut input_string = String::new();
    io::stdin().read_to_string(&mut input_string)?;
    
    // Create an iterator over all whitespace-separated tokens
    let mut tokens = input_string.split_whitespace();

    // 2. Parse the first token as 'n' (the number of elements)
    if let Some(n_str) = tokens.next() {
        let n: usize = n_str.parse().expect("First token should be integer 'n'");
        
        // 3. Parse exactly 'n' elements into our Vector
        let mut data: Vec<i32> = tokens
            .take(n)
            .map(|s| s.parse().expect("Failed to parse array element"))
            .collect();

        // 4. Sort the array
        QuicksortRandomPivotInsertion.sort(&mut data);

        // 5. FAST I/O: Write to stdout using BufWriter
        let stdout = io::stdout();
        let mut out = BufWriter::new(stdout.lock());

        // Use split_first to cleanly handle the space formatting
        if let Some((first, rest)) = data.split_first() {
            write!(out, "{}", first)?;
            for val in rest {
                write!(out, " {}", val)?;
            }
        }
        writeln!(out)?; // Trailing newline
        
        // Flush ensures everything sitting in the buffer is actually printed
        out.flush()?;
    }
    
    Ok(())
}
