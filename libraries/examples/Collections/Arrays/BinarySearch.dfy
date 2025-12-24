// RUN: %run "%s" > "%t"

// OutputCheck is broken, see https://github.com/dafny-lang/libraries/issues/164
// not working: %OutputCheck --file-to-check "%t" "%s"
// CHECK-L: [-6, 0, 1, 3, 7, 7, 9]
// CHECK-NEXT-L: 3

/*******************************************************************************
 *  Copyright by the contributors to the Dafny Project
 *  SPDX-License-Identifier: MIT 
 *******************************************************************************/

include "../../../src/Relations.dfy"
include "../../../src/Collections/Arrays/BinarySearch.dfy"
include "../../../src/Collections/Sequences/Seq.dfy"
include "../../../src/Collections/Sequences/MergeSort.dfy"

module BinarySearchExamples {

  import BinarySearch
  import opened Seq
  import opened Seq.MergeSort
  import opened Relations

  lemma SortedByLessThanOrEqualTo(s: seq<int>)
    requires SortedBy(s, (x, y) => x <= y)
    ensures SortedBy(s, (x, y) => x < y || x == y)
  {}

  method {:vcs_split_on_every_assert} SortAndSearch() {
    var input := [1, 7, 7, 3, 9, 0, -6];

    var sortedInput := MergeSortBy(input, (x, y) => x <= y);
    print sortedInput, "\n";

    assert SortedBy(sortedInput, (x, y) => x <= y);
    var sortedArray := ToArray(sortedInput);
    SortedByLessThanOrEqualTo(sortedArray[..]);
    var indexOfThree := BinarySearch.BinarySearch(sortedArray, 3, (x, y) => x < y);
    if indexOfThree.Some? {
      print indexOfThree.value, "\n";
    } else {
      print "Not found\n";
    }
  }

  method Main() {
    SortAndSearch();
  }
}
