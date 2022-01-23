use std::cmp::Ordering;

/*
   why you need Eq and PartialEq?
    - you are implementing Ord and
      Eq is bound in Ord
      - Ord: Eq + PartialOrd<Self>
    - Eq is bound in Ord
    - PartialEq is bound in Eq
*/
#[derive(Debug, Eq, PartialEq)]
pub struct Interval {
  start: usize,
  end: usize,
}

impl Interval {
  pub fn length(&self) -> usize {
    &self.end - &self.start
  }
}

/*
   - Total order
     - you can compare any two intervals
       to form a total order for the
       elements in a set
     - notice that the return type is
       Ordering not Option<Ordering>
       - this means it has to have a result;
         there is no case that two elements
         are not comparable
*/
impl Ord for Interval {
  fn cmp(&self, other: &Self) -> Ordering {
    /*
       - by default BinaryHeap is a max heap
         - the biggest element is at the root
       - we want to create a min heap, and
         that's why we flip the self and other
         when comparing
       - if the lengths are the same, use the start
         as a tie-breaker
         - again we want the interval with a
           smaller start wins so we flip self
           and other again
    */
    let ord = other
      .length()
      .cmp(&self.length())
      .then_with(|| other.start.cmp(&self.start));
    println!("comparing {:?} and {:?} : {:?} ", self, other, ord);
    ord
  }
}

/*
   - PartialOrd is bound in Ord
     - so whenever you implement Ord
       you also need to implement PartialOrd
     - also the logic needs to be consistent
       with what you implemented for Ord trait
     - remember a total order is a special
       case of partial orders
     - notice that the big difference between
       PartialOrd and Ord is the return type
       - PartialOrd has an Option type meaning
         in some cases it might not be comparable
         given two elements
*/

impl PartialOrd for Interval {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

pub struct TestFixtures {}
impl TestFixtures {
  pub fn test_fixture_1() -> Vec<Interval> {
    vec![
      Interval { start: 15, end: 20 },
      Interval { start: 0, end: 30 },
      Interval { start: 5, end: 10 },
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BinaryHeap;

  #[test]
  fn test_min_heap() {
    let intervals = TestFixtures::test_fixture_1();
    let mut heap = BinaryHeap::new();
    for i in intervals {
      heap.push(i);
    }

    assert_eq!(heap.pop(), Some(Interval { start: 5, end: 10 }));
    assert_eq!(heap.pop(), Some(Interval { start: 15, end: 20 }));
    assert_eq!(heap.pop(), Some(Interval { start: 0, end: 30 }));
  }
}
