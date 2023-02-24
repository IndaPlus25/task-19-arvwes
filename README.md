# Quicksort
Quicksort stands as one of the top 10 of algorithms (https://www.computer.org/csdl/magazine/cs/2000/01/c1022/13rRUxBJhBm). It is fascinating not only because it is one of the fastest general sorting algorithms, but also because it is so open to experimentation and variation. The basic implementation is not considered overly challenging, but the real challenge comes from tuning varients to outperform it and defend against its inherent fragility.

### 💀 Deadline
This is a two week assignment with two deadlines:

* Implementation: **Friday 3th March**
* Evaluation & Report: **Friday 10th March**

**Important: This task is mandatory and a pass is required to complete DD1338.**  

### 👩‍🏫 Instructions
For instructions on how to do and submit the assignment, please see the
[assignments section of the course instructions](https://gits-15.sys.kth.se/inda-22/course-instructions#assignments).

### 📝 Preparation
- Read and answer questions in [Module 10: Quicksort](https://kth.oli.cmu.edu/jcourse/webui/syllabus/module.do?context=8af631c5ac1f08886add429f0d1b2ec1)
  - If you have not done so, goto https://kth.oli.cmu.edu/, signup and register for the course key `dd1338-ht22`

### ✅ Learning Goals
* Implement the classic quicksort algorithm
* Develop variations that improve performance and reduce fragility of quicksort
* Test variations against unseen datasets with Kattis
* Empirically evaluate quicksort
* Summarise your findings as a short technical report

### 🚨 Troubleshooting Guide
If you have any questions or problems, follow this procedure: <br/>

1. Look at this week's [posted issues](https://gits-15.sys.kth.se/inda-22/help/issues). Are other students asking about your problem?
2. If not, post a question yourself by creating a [New Issue](https://gits-15.sys.kth.se/inda-22/help/issues/new). Add a descriptive title, beginning with "Task *x*: *summary of problem here*"
3. Ask a TA in person during the [weekly lab](https://queue.csc.kth.se/Queue/INDA). Check your schedule to see when the next lab is.

We encourage you to discuss with your course friends, but **do not share answers**! Similarily, use of AI services  🤖 are great to *help explain things*, but please **do not submit AI-generated solutions** - you must be both responsible for your own solutions and be able to explain them under examination.

### 🏛 Assignment

There's a lot of information in this repo. Here's an overview of what you need
to accomplish (and submit) for each of the weeks:

* Week 1
    - An insertion sort implementation of the `IntSorter` interface.
    - Four quicksort implementations, detailed in [Variations of Quicksort](#variations-of-quicksort)
    - Test classes for all five production classes, using inheritance to avoid
      test duplication.
    - At least one implementation must be submitted to, and pass, the Kattis
    task. At least one successful submission id must be pushed to the repo.

* Week 2
    - A class to create runtime tests for the report (this class you will have
      to design yourself).
    - A fancy report.

### Week 1 - Implementing Quicksort
In the first week, you will implement the Quicksort algorithm, as well as
several variations of the basic algorithm. Once you have implemented your set
of algorithms, you can test them by using Kattis.

#### Sort Interface
Each Quicksort algorithm variation should be a single class that implements the
[IntSorter](src/IntSorter.java) interface:

```java
public interface IntSorter {
    /**
     * Sorts the array into ascending numerical order.
     */
    void sort(int[] v);
}
```

This will make it easier to run repeated tests and measurements on different
versions of your algorithms during the empirical evaluation.

##### Warmup
Create an insertion sort implementation of `IntSorter` called `InsertionSort`
as a warmup. If you have already implemented insertion sort, you may reuse your
earlier implementation.  If you have not, this is the algorithm:

```python
Insertion Sort (A)
------------------
1   for i = 1 to A.length - 1
2       j = i
3       while j > 0 and A[j-1] > A[j]
4           temp = A[j]
5           A[j] = A[j-1]
6           A[j-1] = temp
7           j = j - 1
```

It is strongly recommended that you have a look at the
[testing section](#testing) before moving on to implementing quicksort.

#### Variations of Quicksort
Implement four variations of the Quicksort algorithm:

* `QuicksortFixedPivot` - uses a fixed pivot
* `QuicksortRandomPivot` - uses a randomly selected pivot
* `QuicksortFixedPivotInsertion` - fixed pivot with cut-off to insertion sort
  at k.  Instead of stopping the recursion when the sub array only has at most
  one element, implement a version where sub arrays that contain at most `k`
  elements are sorted with insertion sort. `k` can be decided by
  experimentation.
* `QuicksortRandomPivotInsertion` - as above, but with a random pivot

> **Assistant's requirement:** Be very careful in getting the names of the
> classes right, _including capitalization_. For example, it should be
> `QuicksortFixedPivot`, and not `quicksortFixedPivot` or `QuickSortFixedPivot`
> or `quicksortfixedPivot` or ... you get the point.

> **Assistant's requirement** You will probably find that you duplicate some
> code when writing the different Quicksort implementations. Make this
> duplication minimal. It's not okay to have four exact duplicates of the same
> partitioning algorithm.

#### Testing

For the testing this week, you are left to stand pretty much on your own legs.
Draw inspiration from the previous weeks' test suites if you find that you have
trouble getting started. The important thing here is to make a good attempt at
creating a manageable test suite, it is more important that you structure it
well than that you cover every single corner case imaginable.

* You have been given a skeleton for an abstract test class called
  [IntSorterTest](src/IntSorterTest.java).
    - For the most basic configuration, **put all your tests in IntSorterTest.**
    - Use the `getIntSorter()` method to instantiate an `IntSorter`.
    - For each of your implementations, extend the test class with an
      implementing test class (e.g. with `QuicksortFixedPivotTest`) and
      implement the `getIntSorter()` method.
* **Optionally**, feel free to make a more elaborate setup, for example with a
  more specialized abstract test class for the quicksort implementations.
* The sorting tests in week 11's `BoxProcessorTest` may prove helpful. Have a
  look at them if you are feeling lost.
* You will want to test the following types of arrays:
    - Arrays of even and odd length
    - Sorted in ascending order
    - Sorted in descending order (reversed)
    - Random
    - All elements equal
    - Very large arrays (you may want to exclude insertion sort from some of
      these)
    - Use the provided [Data](src/Data.java) class to generate these arrays!
      The main method in said class has some examples of how to use it.

> **Assistant's requirement:** Test class design is as important as production
> class design. You are _not_ allowed to have duplicated tests, use inheritance
> (draw inspiration from the previous weeks) to avoid this. **All test classes
> must inherit from IntSorterTest.**

#### Kattis Test

In addition to the unit tests you write yourself, you should also submit _at
least one_ of your implementations to Kattis. You **absolutely have to register
for the DD1338 course** and then submit your solution. Failure to do so will
result in kompleterring.

Kattis will test both that your implementations are correct, and how fast they
run. _Note that the run times on Kattis can differ between different runs on the
same code, especially if you have randomisation!_

#### Kattis Submission

**Submit your implementation [here](https://kth.kattis.com/courses/DD1338/algdat22)**. Please note that if you are testing varients that do not use a randomised pivot then some test cases will always fail.

**Once you have a successful run, please copy the submission ID and put it in the [docs/submission.txt](docs/submission.txt) file.** There are no limits on the number of submissions, so you can try out your different variations of Quicksort.

### Week 2 - Empirical Evaluation

In the second week, you will perform an empirical evaluation of your
implementation and produce a short report.

#### Timing Execution

The class [StopWatch](src/Stopwatch.java) implements a simple stopwatch for
timing execution. Its usage can be seen in the
[TimingExample](src/TimingExample.java) class.  Run `TimingExample` and pay
attention to the results.  You should notice that the result varies. Modify
`TimingExample` so that it:

* Discard results that are anomalous or affected by JVM warmup
* Finds the average for `n` experiments
* Prints the minimum, mean (average) and maximum times

> **Assistant's note:** A common error is to sort the same array over and over,
> without considering that it will actually be sorted after the first time
> around.  Make sure to utilize the `Data.get()` method properly to get a new
> copy of the array for each sort.

#### Evaluating Sorting Algorithms
The `Data` class has been provided for you to generate different datasets for
evaluating your variations of the Quicksort algorithm. Read the source code to
become familiar on how it is to be used. You should gather results for the
following tests, according to the table template below. Make sure you use the
method you developed above to ensure the results are more accurate and try to
control variables in the computing environment as much as possible.

* Test 1: Random Data
* Test 2: Sorted Data
* Test 3: Reversed Data
* Test 4: Equal Data

Example table of execution time costs for different algorithms and problem
sizes (produce one table per test):

| Test 1: Random Data |               |       |       |       |       |               |
| ------------------  | ------------- | ----- | ----- | ----- | ----- | ------------- |
| Problem Size        | InsertionSort | Qs V1 | Qs V2 | Qs V3 | Qs V4 | Arrays.sort † |
| 100                 |               |       |       |       |       |               |
| 1000                |               |       |       |       |       |               |
| 10000               |               |       |       |       |       |               |
| 100000              |               |       |       |       |       |               |
| 1000000             |               |       |       |       |       |               |

> **Assistant's note:** For some of the tests, insertion sort will take a
> _very_ long time to run to completion on large porblem sizes. It is fine to
> point out these test cases with a motivation for why they are slow, and put
> something like a few dashes in that cell instead of an execution time.

> **Assistant's other note:** That's _one column per quicksort variation_.

† [Arrays.sort(int[] a)](https://docs.oracle.com/javase/8/docs/api/java/util/Arrays.html#sort-int:A-)
is from the Java standard library, and for your interest, it also uses
Quicksort, but with a dual-pivot variation with lots of optimisations :-)

#### Report

Finally, you should prepare a short report. The report will contain the
following sections (a template can be found in `docs`):

1. Origins, characteristics and complexity of Quicksort algorithm
2. Explanation of each Quicksort variation you implemented
3. Explanation of each test you performed and how you ensured accurate results
4. Presentation of results using tables and appropriate charts for all tests
5. Discussion of your findings

### 🙏 Acknowledgment
This task was designed by:               <br>
Simon Larsén                             <br>
Anton Lyxell                             <br>
Stefan Nilsson                           <br>
Ric Glassey                              <br>