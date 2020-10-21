use std::cmp;
use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;

pub fn merge(v: &mut Vec<i32>, l: usize, m: usize, h:usize) {
    let mut lvec: Vec<i32> = vec![];
    let mut rvec: Vec<i32> = vec![];

    for i in l..m+1 {
        lvec.push(v[i]);
    }
    for i in m+1..h+1 {
        rvec.push(v[i]);
    }
    let (mut i, mut j, mut k) = (0,0,l);
    while i < m-l+1 && j < h-m {
        if lvec[i] <= rvec[j] {
            v[k] = lvec[i];
            i = i+1;
        } else  {
            v[k] = rvec[j];
            j = j+1;
        }
        k = k+1;
    }

    while i < m-l+1 {
        v[k] = lvec[i];
        i = i+1; k = k+1;
    }

    while j < h-m {
        v[k] = rvec[j];
        j = j+1; k = k+1;
    }
}

pub fn merge_seq(mut v: Vec<i32>) -> Vec<i32> {
    if v.len() == 0 { return v; }

    let n = v.len();

    let mut curr_size=1;
    while curr_size <= n-1 {
        for left in (0..n-1).step_by(2*curr_size) {
            let mid = cmp::min(left + curr_size - 1, n-1);
            let right = cmp::min(left + 2*curr_size - 1, n-1);
            merge(&mut v, left, mid, right);
        }
        curr_size = curr_size*2;
    }

    v.to_vec()
}

pub fn merge_par(v: Vec<i32>) -> Vec<i32> {
    if v.len() == 0 { return v; }

    let n = v.len();
    let data = Arc::new(Mutex::new(v));

    let mut curr_size=1;
    while curr_size <= n-1 {
        let mut handles = vec![];
        for left in (0..n-1).step_by(2*curr_size) {
            let counter = Arc::clone(&data);
            let mid = cmp::min(left + curr_size - 1, n-1);
            let right = cmp::min(left + 2*curr_size - 1, n-1);
            let handle = thread::spawn(move || {
                merge(&mut counter.lock().unwrap(), left, mid, right);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        curr_size = curr_size*2;
    }

    Arc::clone(&data).lock().unwrap().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_sample() {
        let v = vec![3,2,4,5,1];
        let v = merge_seq(v);
        assert_eq!(vec![1,2,3,4,5], v);
    }

    #[test]
    fn seq_empty() {
        let v = vec![3];
        let v = merge_seq(v);
        assert_eq!(vec![3], v);
    }

    #[test]
    fn seq_one_val() {
        let v: Vec<i32> = vec![];
        let v = merge_seq(v);
        assert_eq!(Vec::<i32>::new(), v);
    }

    #[test]
    fn par_sample() {
        let v = vec![3,2,4,5,1];
        let v = merge_par(v);
        assert_eq!(vec![1,2,3,4,5], v);
    }

    #[test]
    fn par_empty() {
        let v = vec![3];
        let v = merge_par(v);
        assert_eq!(vec![3], v);
    }

    #[test]
    fn par_one_val() {
        let v: Vec<i32> = vec![];
        let v = merge_par(v);
        assert_eq!(Vec::<i32>::new(), v);
    }
}
