# Quicksort

Quicksort stands as one of the [top 10 of algorithms](https://www.computer.org/csdl/magazine/cs/2000/01/c1022/13rRUxBJhBm). It is fascinating not only because it is one of the fastest general sorting algorithms, but also because it is so open to experimentation and variation. The basic implementation is not considered overly challenging, but the real challenge comes from tuning varients to outperform it and defend against its inherent fragility.

## 💀 Deadline

This is a two part assignment with two deadlines:

* Implementation: **Friday 27th February**
* Evaluation & Report: **Friday 13th March** (end of tenta period)

**Important: This task is mandatory and a pass on both parts is required to complete DD1338. It's time to lock in.**

## 👩‍🏫 Instructions

For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-25/course-instructions#assignments).

## 📝 Preparation

* Review the lecture slides: [Quicksort](https://docs.google.com/presentation/d/1GTP8IivW_sZ4cx-6eSFFFHpp3cJ7F1QmCSQW3vo0s04/edit#slide=id.p) and [Empirical Evaluation](https://docs.google.com/presentation/d/1r-uw6k9o5yV21UV0NFecOXFX7MY3IPHNn8sMNhcHMz0/edit#slide=id.p)
* Read and answer all questions in Module 10: [Quicksort](https://qbl.sys.kth.se/sections/dd1338_algorithms_and_data_str/container/quicksort)
* Take a deep breath!

## ✅ Learning Goals

* Implement the classic quicksort algorithm
* Develop variations that improve performance and reduce fragility of quicksort
* Test variations against unseen datasets with Kattis
* Empirically evaluate quicksort
* Summarise your findings as a short technical report

## 🚨 Troubleshooting Guide

If you have any questions or problems, follow this procedure:

1. Look at this week's [posted issues](https://gits-15.sys.kth.se/inda-25/help/issues). Are other students asking about your problem?
2. If not, post a question yourself by creating a [New Issue](https://gits-15.sys.kth.se/inda-25/help/issues/new). Add a descriptive title, beginning with "Task *x*: *summary of problem here*"
3. Ask a TA in person during the [weekly lab](https://queue.csc.kth.se/Queue/INDA). Check your schedule to see when the next lab is.

We encourage you to discuss with your course friends, but **do not share answers**! Similarily, use of AI services  🤖 are great to *help explain things*, but please **do not submit AI-generated solutions** - you must be both responsible for your own solutions and be able to explain them under examination. If in doubt, refer to the **AI Policy on Canvas**.

## 🏛 Assignment

There's a lot of information in this repo. Here's an overview of what you need to accomplish (and submit) for each of the phases:

Phase 1 (due 27th Feb):

* An insertion sort implementation of the `IntSorter` interface.
* Four quicksort implementations
* Test classes for all five production classes, using inheritance to avoid test duplication.
* At least one implementation must be submitted to, and pass, the Kattis task.

Phase 2 (due 13th Mar):
  
* A class to create runtime tests for the report (this class you will have to design yourself).
* A fancy technical report.

## Phase 1 - Implementing Quicksort

In the first phase, you will implement the Quicksort algorithm, as well as several variations of the basic algorithm. Once you have implemented your set of algorithms, you can test them by using Kattis.

### Exercise 1.1 - Implement the IntSorter Interface

Each Quicksort algorithm variation should be a single class that implements the [IntSorter](src/IntSorter.java) interface:

```java
public interface IntSorter {
    /**
     * Sorts the array into ascending numerical order.
     */
    void sort(int[] v);
}
```

This will make it easier to run repeated tests and measurements on different versions of your algorithms during the empirical evaluation.

Create an insertion sort implementation of `IntSorter` called `InsertionSort` as a warmup. If you have implemented insertion sort, you may reuse your earlier implementation.

[Open an issue for Exercise 1.1](../../issues/new?title=Implement%20IntSorter%20interface)

### Exercise 1.2 - Test Insertion Sort

For the testing, you are left to stand pretty much on your own legs. Draw inspiration from the previous weeks' test suites if you find that you have trouble getting started.

You have been given a skeleton for an abstract test class called [IntSorterTest](src/IntSorterTest.java).

* For the most basic configuration, **put all your tests in IntSorterTest.**
* Use the `getIntSorter()` method to instantiate an `IntSorter`.

You will want to test the following types of arrays:

* Arrays of even and odd length, e.g. `[1, 2, 3, 4]` and `[1, 2, 3, 4, 5]`
* Sorted in ascending order, e.g. `[1, 2, 3, 4, 5]`
* Sorted in descending order (reversed), e.g. `[5, 4, 3, 2, 1]`
* Randomised arrays, e.g. `[5, 2, 8, 1, 3]`
* All elements equal, e.g. `[1, 1, 1, 1, 1]`
* Few elements all equal, e.g. `[1, 1, 1, 2, 2, 2]`
* Larger arrays, e.g. of size 100, 1000, 10000. Find the limits of insertion sort. Get your fan going!

Tip! Use the provided [Data](src/Data.java) class to generate these arrays! The main method in said class has some examples of how to use it.

[Open an issue for Exercise 1.2](../../issues/new?title=Test%20Insertion%20Sort)

### Exercise 1.3 - Variations of Quicksort

Implement four variations of the Quicksort algorithm:

#### Exercise 1.3.1 - Implement QuicksortFixedPivot

Implement a basic version of the quicksort algorithm called `QuicksortFixedPivot` that uses a fixed pivot. The traditional choice for the pivot is the last element of the array.

> **Assistant's requirement:** Be very careful in getting the names of the
> classes right, *including capitalization*. For example, it should be
> `QuicksortFixedPivot`, and not `quicksortFixedPivot` or `QuickSortFixedPivot`
> or `quicksortfixedPivot` or ... you get the point.

[Open an issue for Exercise 1.3.1](../../issues/new?title=Implement%20QuicksortFixedPivot)

#### Exercise 1.3.2 - Implement QuicksortRandomPivot

As before, but with a randomly selected pivot. Call this `QuicksortRandomPivot`.

> **Assistant's requirement** You will probably find that you duplicate some
> code when writing the different Quicksort implementations. Make this
> duplication minimal. It's not okay to have four exact duplicates of the same
> partitioning algorithm.

[Open an issue for Exercise 1.3.2](../../issues/new?title=Implement%20QuicksortRandomPivot)

#### Exercise 1.3.3 - Implement QuicksortFixedPivotInsertion

Implement a class called `QuicksortFixedPivotInsertion` that uses a fixed pivot with cut-off to insertion sort at k.  Instead of stopping the recursion when the sub array only has at most one element, implement a version where sub arrays that contain at most `k` elements are sorted with insertion sort. `k` can be decided by experimentation.

[Open an issue for Exercise 1.3.3](../../issues/new?title=Implement%20QuicksortFixedPivotInsertion)

#### Exercise 1.3.4 - Implement QuicksortRandomPivotInsertion

As above, but with a random pivot. Call this `QuicksortRandomPivotInsertion`.

[Open an issue for Exercise 1.3.4](../../issues/new?title=Implement%20QuicksortRandomPivotInsertion)

### Exercise 1.4 - Test Quicksort Variations

Repeat the testing you did for insertion sort for each of the four variations of quicksort. You should be testing the same things for each variation, so make sure to use inheritance to avoid test duplication. The sorting tests in task-11's `BoxProcessorTest` may prove helpful. Have a look at them if you are feeling lost on using inheritance in testing.

For each of your implementations, extend the test class with an implementing test class (e.g. with `QuicksortFixedPivotTest`) and implement the `getIntSorter()` method.

One thing you should now do is test VERY LARGE arrays that insertion sort could not handle. Get that fan going!

> **Assistant's requirement:** Test class design is as important as production
> class design. You are *not* allowed to have duplicated tests, use inheritance
> (draw inspiration from the previous weeks) to avoid this. **All test classes
> must inherit from IntSorterTest.**

[Open an issue for Exercise 1.4](../../issues/new?title=Test%20Quicksort%20Variations)

### Kattis Test

In addition to the unit tests you write yourself, you should also submit *at least one* of your implementations to Kattis. You **absolutely have to register for the DD1338 course** and then submit your solution. Failure to do so will result in kompleterring.

Kattis will test both that your implementations are correct, and how fast they run. *Note that the run times on Kattis can differ between different runs on the same code, especially if you have randomisation!*

### Kattis Submission

**Submit your implementation [on Kattis](https://kth.kattis.com/courses/DD1338/algdat25)**. Please note that if you are testing varients that do not use a randomised pivot then some test cases will always fail.

**Once you have a successful run, please copy the submission ID and put it in the [docs/submission.txt](docs/submission.txt) file.** There are no limits on the number of submissions, so you can try out your different variations of Quicksort.

## Phase 2 - Empirical Evaluation

In the second phase, you will perform an empirical evaluation of your implementation and produce a short report.

### Exercise 2.1 - Modify Timing Execution Analysis

The class [StopWatch](src/Stopwatch.java) implements a simple stopwatch for timing execution. Its usage can be seen in the [TimingExample](src/TimingExample.java) class.  Run `TimingExample` and pay attention to the results.  You should notice that the result varies. Modify `TimingExample` so that it:

* Discard results that are anomalous or affected by JVM warmup
* Finds the average for `n` experiments
* Prints the minimum, mean (average) and maximum times

> **Assistant's note:** A common error is to sort the same array over and over,
> without considering that it will actually be sorted after the first time
> around.  Make sure to utilize the `Data.get()` method properly to get a new
> copy of the array for each sort.

[Open an issue for Exercise 2.1](../../issues/new?title=Modify%20Timing%20Execution%20Analysis)

### Exercise 2.2 - Evaluate Sorting Algorithms

The `Data` class has been provided for you to generate different datasets for evaluating your variations of the Quicksort algorithm. Read the source code to become familiar on how it is to be used. You should gather results for the following tests, according to the table template below. Make sure you use the method you developed above to ensure the results are more accurate and try to control variables in the computing environment as much as possible.

* Test 1: Random Data
* Test 2: Sorted Data
* Test 3: Reversed Data
* Test 4: Equal Data

Example table of execution time costs for different algorithms and problem sizes (produce one table per test):

| Test 1: Random Data |               |       |       |       |       |               |
| ------------------  | ------------- | ----- | ----- | ----- | ----- | ------------- |
| Problem Size        | InsertionSort | Qs V1 | Qs V2 | Qs V3 | Qs V4 | Arrays.sort † |
| 100                 |               |       |       |       |       |               |
| 1000                |               |       |       |       |       |               |
| 10000               |               |       |       |       |       |               |
| 100000              |               |       |       |       |       |               |
| 1000000             |               |       |       |       |       |               |

> **Assistant's note:** For some of the tests, insertion sort will take a
> *very* long time to run to completion on large problem sizes. It is fine to
> point out these test cases with a motivation for why they are slow, and put
> something like a few dashes in that cell instead of an execution time.

† [Arrays.sort(int[] a)](https://docs.oracle.com/javase/8/docs/api/java/util/Arrays.html#sort-int:A-) is from the Java standard library, and for your interest, it also uses Quicksort, but with a dual-pivot variation with lots of optimisations :)

> **Assistant's other note:** That's *one column per quicksort variation*.

[Open an issue for Exercise 2.2](../../issues/new?title=Evaluate%20Sorting%20Algorithms)

### Exercise 2.3 - Write Technical Report

Finally, you should prepare a short report. The report will contain the following sections (a template can be found in `docs`):

1. Origins, characteristics and complexity of Quicksort algorithm
2. Explanation of each Quicksort variation you implemented
3. Explanation of each test you performed and how you ensured accurate results
4. Presentation of results using tables and appropriate charts for all tests
5. Discussion of your findings

The teachers from DA1600 have prepared a helpful guide that will let you integrate the learning from their course into your report. You can find it here: [docs/da1600-guidance.md](docs/da1600-guidance.md). Read it as a reminder and as a checklist for your report. Please check the boxes once you have made sure that your report meets the criteria -- you have to edit the markdown to make the ticks appear :)

[Open an issue for Exercise 2.3](../../issues/new?title=Write%20Technical%20Report)

## 🙏 Acknowledgments

This task was designed by:

* Simon Larsén
* Anton Lyxell
* Stefan Nilsson
* Ric Glassey
