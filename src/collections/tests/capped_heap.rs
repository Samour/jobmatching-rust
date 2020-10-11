use crate::collections::CappedHeap;

fn expect_heap<T>(size: usize, heap: Vec<(f64, T)>) -> String
where
  T: std::fmt::Debug,
{
  let mut result = String::from("CappedHeap { size: ");
  result.push_str(&format!("{:?}", size));
  result.push_str(", push_head: ");
  result.push_str(&format!("{:?}", heap.len()));
  result.push_str(", data: [");

  let mut first_el = true;
  for node in &heap {
    if !first_el {
      result.push_str(", ");
    }
    first_el = false;
    result.push_str("Some(HeapNode { value: ");
    result.push_str(&format!("{:?}", node.0));
    result.push_str(", item: ");
    result.push_str(&format!("{:?}", node.1));
    result.push_str(" })");
  }
  for _ in heap.len()..size + 1 {
    if !first_el {
      result.push_str(", ");
    }

    result.push_str("None");
  }
  result.push_str("] }");

  result
}

#[test]
fn test_push_1() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(1.0, 1);
  heap.push(2.0, 2);
  heap.push(3.0, 3);
  heap.push(4.0, 4);

  assert_eq!(
    format!("{:?}", heap),
    expect_heap(5, vec![(1.0, 1), (2.0, 2), (3.0, 3), (4.0, 4)])
  );
}

#[test]
fn test_push_2() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(4.0, 4);
  heap.push(3.0, 3);
  heap.push(2.0, 2);
  heap.push(1.0, 1);

  assert_eq!(
    format!("{:?}", heap),
    expect_heap(5, vec![(1.0, 1), (2.0, 2), (3.0, 3), (4.0, 4)])
  );
}

#[test]
fn test_push_3() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(2.0, 2);
  heap.push(3.0, 3);
  heap.push(1.0, 1);
  heap.push(4.0, 4);

  assert_eq!(
    format!("{:?}", heap),
    expect_heap(5, vec![(1.0, 1), (3.0, 3), (2.0, 2), (4.0, 4)])
  );
}

#[test]
fn test_push_capped1() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(1.0, 1);
  heap.push(2.0, 2);
  heap.push(3.0, 3);
  heap.push(4.0, 4);
  heap.push(5.0, 5);
  heap.push(6.0, 6);
  heap.push(7.0, 7);
  heap.push(8.0, 8);

  assert_eq!(
    format!("{:?}", heap),
    expect_heap(5, vec![(4.0, 4), (5.0, 5), (7.0, 7), (6.0, 6), (8.0, 8)])
  );
}

#[test]
fn test_push_capped2() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(8.0, 8);
  heap.push(7.0, 7);
  heap.push(6.0, 6);
  heap.push(5.0, 5);
  heap.push(4.0, 4);
  heap.push(3.0, 3);
  heap.push(2.0, 2);
  heap.push(1.0, 1);

  assert_eq!(
    format!("{:?}", heap),
    expect_heap(5, vec![(4.0, 4), (5.0, 5), (7.0, 7), (8.0, 8), (6.0, 6)])
  );
}

#[test]
fn test_push_capped3() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(3.0, 3);
  heap.push(2.0, 2);
  heap.push(6.0, 6);
  heap.push(1.0, 1);
  heap.push(8.0, 8);
  heap.push(7.0, 7);
  heap.push(4.0, 4);
  heap.push(5.0, 5);

  assert_eq!(
    format!("{:?}", heap),
    expect_heap(5, vec![(4.0, 4), (6.0, 6), (5.0, 5), (7.0, 7), (8.0, 8)])
  );
}

#[test]
fn test_pop_1() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(1.0, 1);
  heap.push(2.0, 2);
  heap.push(3.0, 3);
  heap.push(4.0, 4);

  assert_eq!(heap.pop(), Some(1));
  assert_eq!(heap.pop(), Some(2));
  assert_eq!(heap.pop(), Some(3));
  assert_eq!(heap.pop(), Some(4));
  assert_eq!(heap.pop(), None);
}

#[test]
fn test_pop_2() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(4.0, 4);
  heap.push(3.0, 3);
  heap.push(2.0, 2);
  heap.push(1.0, 1);

  assert_eq!(heap.pop(), Some(1));
  assert_eq!(heap.pop(), Some(2));
  assert_eq!(heap.pop(), Some(3));
  assert_eq!(heap.pop(), Some(4));
  assert_eq!(heap.pop(), None);
}

#[test]
fn test_pop_3() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(2.0, 2);
  heap.push(3.0, 3);
  heap.push(1.0, 1);
  heap.push(4.0, 4);

  assert_eq!(heap.pop(), Some(1));
  assert_eq!(heap.pop(), Some(2));
  assert_eq!(heap.pop(), Some(3));
  assert_eq!(heap.pop(), Some(4));
  assert_eq!(heap.pop(), None);
}

#[test]
fn test_pop_capped1() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(1.0, 1);
  heap.push(2.0, 2);
  heap.push(3.0, 3);
  heap.push(4.0, 4);
  heap.push(5.0, 5);
  heap.push(6.0, 6);
  heap.push(7.0, 7);
  heap.push(8.0, 8);

  assert_eq!(heap.pop(), Some(4));
  assert_eq!(heap.pop(), Some(5));
  assert_eq!(heap.pop(), Some(6));
  assert_eq!(heap.pop(), Some(7));
  assert_eq!(heap.pop(), Some(8));
  assert_eq!(heap.pop(), None);
}

#[test]
fn test_pop_capped2() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(8.0, 8);
  heap.push(7.0, 7);
  heap.push(6.0, 6);
  heap.push(5.0, 5);
  heap.push(4.0, 4);
  heap.push(3.0, 3);
  heap.push(2.0, 2);
  heap.push(1.0, 1);

  assert_eq!(heap.pop(), Some(4));
  assert_eq!(heap.pop(), Some(5));
  assert_eq!(heap.pop(), Some(6));
  assert_eq!(heap.pop(), Some(7));
  assert_eq!(heap.pop(), Some(8));
  assert_eq!(heap.pop(), None);
}

#[test]
fn test_pop_capped3() {
  let mut heap: CappedHeap<i32> = CappedHeap::new(5);
  heap.push(3.0, 3);
  heap.push(2.0, 2);
  heap.push(6.0, 6);
  heap.push(1.0, 1);
  heap.push(8.0, 8);
  heap.push(7.0, 7);
  heap.push(4.0, 4);
  heap.push(5.0, 5);

  assert_eq!(heap.pop(), Some(4));
  assert_eq!(heap.pop(), Some(5));
  assert_eq!(heap.pop(), Some(6));
  assert_eq!(heap.pop(), Some(7));
  assert_eq!(heap.pop(), Some(8));
  assert_eq!(heap.pop(), None);
}
