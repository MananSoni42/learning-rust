use merge_sort::{merge_seq, merge_parallel};

fn main() {
    let mut v = vec![7,6,5,4,3,2,1];
    merge_seq(&mut v);
    println!("{:?}",v);
}
