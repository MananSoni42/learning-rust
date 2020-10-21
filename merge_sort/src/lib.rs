use std::cmp;

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

pub fn merge_seq(v: &mut Vec<i32>) {
    let mut curr_size=1;
    let n = v.len();

    if n == 0 { return; }

    while curr_size <= n-1 {
        for left in (0..n-1).step_by(2*curr_size) {
            let mid = cmp::min(left + curr_size - 1, n-1);
            let right = cmp::min(left + 2*curr_size - 1, n-1);
            merge(v, left, mid, right);
        }
        curr_size = curr_size*2;
    }
}

pub fn merge_parallel(v: &mut Vec<i32>) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seq_sample() {
        let mut v = vec![3,2,4,5,1];
        merge_seq(&mut v);
        assert_eq!(vec![1,2,3,4,5], v);
    }

    #[test]
    fn seq_empty() {
        let mut v = vec![3];
        merge_seq(&mut v);
        assert_eq!(vec![3], v);
    }

    #[test]
    fn seq_one_val() {
        let mut v: Vec<i32> = vec![];
        merge_seq(&mut v);
        assert_eq!(Vec::<i32>::new(), v);
    }
}
