use rand::Rng;
use pad::{PadStr, Alignment};

fn new_heap() -> Vec<i32> {
    vec![42]
}

fn parent(elem: usize) -> usize {
    (elem as f32 / 2f32).ceil() as usize
}

fn children(elem: usize) -> (usize, usize) {
    (elem * 2, elem * 2 + 1)
}

fn push_heap(elem: i32, mut heap: Vec<i32>) -> Vec<i32> {
    heap.push(elem);
    let mut eidx = heap.len() - 1;
    loop {
        let pidx = parent(eidx);
        if heap[pidx] < heap[eidx] {
            heap.swap(eidx, pidx);
            eidx = pidx;
        } else {
            break;
        }
    };
    heap
}

fn dump_aux(heap: &Vec<i32>, idx: usize, indent: usize) {
    let step = 16;
    if idx >= heap.len() {
        return;
    }
    let (a, b) = children(idx);
    dump_aux(heap, a, indent + step);
    let elem = heap[idx];
    println!("{}", elem.to_string().pad_to_width_with_alignment(indent, Alignment::Right));
    println!();
    dump_aux(heap, b, indent + step)
}

fn heap_dump(heap: &Vec<i32>) {
    dump_aux(heap, 1, 0)
}

fn main() {
    let range = 20;
    let mut rng = rand::thread_rng();
    let mut heap = new_heap();
    for _ in 0..range {
        heap = push_heap(rng.gen_range(0..range), heap);
    }
    heap_dump(&heap);
}
