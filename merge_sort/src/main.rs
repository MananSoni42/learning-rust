use merge_sort::{merge_seq, merge_par};

fn main() {
    let mut v = vec![7,6,5,4,3,2,1];
    v = merge_par(v);
    println!("{:?}",v);
}
