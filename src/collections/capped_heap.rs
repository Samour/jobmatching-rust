#[derive(Debug)]
struct HeapNode<T> {
  value: f64,
  item: T,
}

#[derive(Debug)]
pub struct CappedHeap<T> {
  size: usize,
  push_head: usize,
  data: Vec<Option<HeapNode<T>>>,
}

impl<T> CappedHeap<T> {
  pub fn new(size: usize) -> CappedHeap<T> {
    let mut data = Vec::with_capacity(size + 1);
    for _ in 0..size + 1 {
      data.push(None);
    }
    CappedHeap {
      size,
      push_head: 0,
      data,
    }
  }

  fn swap(&mut self, h_idx1: usize, h_idx2: usize) -> () {
    let n1 = std::mem::replace(&mut self.data[h_idx1 - 1], None);
    let n2 = std::mem::replace(&mut self.data[h_idx2 - 1], n1);
    self.data[h_idx1 - 1] = n2;
  }

  fn float_node(&mut self, h_idx: usize) -> () {
    if h_idx == 1 {
      return;
    }
    if self.data[h_idx - 1].as_ref().unwrap().value
      < self.data[(h_idx / 2) - 1].as_ref().unwrap().value
    {
      self.swap(h_idx, h_idx / 2);
      self.float_node(h_idx / 2);
    }
  }

  pub fn push(&mut self, value: f64, item: T) -> () {
    if self.push_head == self.size && value <= self.data[0].as_ref().unwrap().value {
      return;
    }

    self.data[self.push_head] = Some(HeapNode { value, item });
    self.float_node(self.push_head + 1);
    self.push_head += 1;

    if self.push_head == self.size + 1 {
      self.pop();
    }
  }

  fn get_node_safe(&self, h_idx: usize) -> Option<&HeapNode<T>> {
    Option::Some(h_idx)
      .filter(|i| *i <= self.size + 1)
      .map(|i| self.data[i - 1].as_ref())
      .filter(|n| n.is_some())
      .map(|n| n.unwrap())
  }

  fn sink_node(&mut self, h_idx: usize) -> () {
    let node = self.data[h_idx - 1].as_ref().unwrap();
    let h_left = h_idx * 2;
    let h_right = h_left + 1;
    let n_left = self.get_node_safe(h_left).filter(|n| n.value < node.value);
    let n_right = self.get_node_safe(h_right).filter(|n| n.value < node.value);

    if n_left.is_some() && n_right.is_some() {
      if n_left.unwrap().value < n_right.unwrap().value {
        self.swap(h_idx, h_left);
        self.sink_node(h_left);
      } else {
        self.swap(h_idx, h_right);
        self.sink_node(h_right);
      }
    } else if let Some(_) = n_left {
      self.swap(h_idx, h_left);
      self.sink_node(h_left);
    } else if let Some(_) = n_right {
      self.swap(h_idx, h_right);
      self.sink_node(h_right);
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    let result = std::mem::replace(&mut self.data[0], None).map(|n| n.item);
    if self.push_head <= 1 {
      self.push_head = 0;
      return result;
    }

    self.data[0] = std::mem::replace(&mut self.data[self.push_head - 1], None);
    self.push_head -= 1;
    self.sink_node(1);

    result
  }
}
