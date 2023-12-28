use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Display;
// Priority Queue
//use priority_queue::PriorityQueue;
use std::time::Instant;
use rand::Rng;

#[derive(Debug, Eq, Ord, Copy, Clone)]
struct PriorityPair {
    priority: i64,
    value: i64,
}

impl PriorityPair {
    fn new(value: i64, priority: i64) -> Self {
        Self {
            priority,
            value,
        }
    }
}

impl PartialEq<Self> for PriorityPair {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl PartialOrd<Self> for PriorityPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

fn main() {
    let range: i64 = 10_000_000;
    let mut rng = rand::thread_rng();
    let mut simple_prio = BinaryHeap::new();
    let mut test_out = String::new();

    // Test dequeue order
    simple_prio.push(PriorityPair::new(1, 1));
    simple_prio.push(PriorityPair::new(2,1));
    simple_prio.push(PriorityPair::new(3,1));
    simple_prio.push(PriorityPair::new(4,5));
    simple_prio.push(PriorityPair::new(5,9));

    while !simple_prio.is_empty() {
        test_out = [test_out, simple_prio.pop().unwrap_or(PriorityPair::new(0,0)).value.to_string()].join(" ");
    }

    println!("Simple Priority Queue Dequeued: {}",test_out);
    
    // Speed Test Enqueue
    let mut pairs: Vec<(i64, i64)> = Vec::new();
    for n in 0..range {
        pairs.push((rng.gen_range(0..range),rng.gen_range(1..9)));
    }
    let start_enqueue = Instant::now();
    for pair in pairs {
        simple_prio.push(PriorityPair::new(pair.0,pair.1));
    }
    let duration_enqueue = start_enqueue.elapsed();
    println!("Simple Priority Queue Enqueue time: {:?}", duration_enqueue);

    // Speed Test Dequeue 
    let mut number_dequeued: i64 = 0;
    let start_dequeue = Instant::now();
    while !simple_prio.is_empty() {
        simple_prio.pop();
        number_dequeued += 1;
    }
    let duration_dequeue = start_dequeue.elapsed();
    println!("Simple Priority Queue Dequeued Items: {}", number_dequeued);
    println!("Simple Priority Queue Dequeue time: {:?}", duration_dequeue);
}