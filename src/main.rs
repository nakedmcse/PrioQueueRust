// Priority Queue 
use priority_queue::PriorityQueue;
use std::time::Instant;
use rand::Rng;

fn main() {
    let range: u64 = 10000000;
    let mut rng = rand::thread_rng();
    let mut simple_prio: PriorityQueue<(i64,u64), i64> = PriorityQueue::new();
    let mut test_out = String::new();

    // Test dequeue order
    simple_prio.push((1,1),1);
    simple_prio.push((2,2),1);
    simple_prio.push((3,3),1);
    simple_prio.push((4,4),5);
    simple_prio.push((5,5),9);

    while !simple_prio.is_empty() {
        test_out = [test_out, simple_prio.pop().unwrap_or(((0,0),0)).0.0.to_string()].join(" ");
    }

    println!("Simple Priority Queue Dequeued: {}",test_out);
    
    // Speed Test Enqueue
    let mut pairs: Vec<(i64, i64)> = Vec::new();
    let mut n:u64 = 0;
    while n<range {
        pairs.push((rng.gen_range(0..10000000),rng.gen_range(1..9)));
        n += 1;
    }
    let start_enqueue = Instant::now();
    n = 0;
    for pair in pairs {
        simple_prio.push((pair.0,n),pair.1);
        n += 1;
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