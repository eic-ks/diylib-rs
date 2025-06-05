// x以下の最大の数をsliceとして渡された配列のleft,rightの範囲から二分探索で探す
pub fn upperbound(slice: &[isize], left: usize, right: usize, x: isize) -> isize {
    let mut l = left;
    let mut r = right;
    while r - l > 1 {
        let mut mid = slice[(l + r)/2] as isize;

        if mid > x {
            r = (l + r)/2;
        }else {
            l = (l + r)/2;
        }
    }
    if slice[l] <= x {return slice[l];}
    return isize::MIN;
}
// x以上の最小の数をsliceとして渡された配列のleft,rightの範囲から二分探索で探す
pub fn lowerbound(slice: &[isize], left: usize, right: usize, x: isize) -> isize {
    let mut l = left;
    let mut r = right;
    while r - l > 1 {
        let mut mid = slice[(l + r)/2] as isize;

        if mid < x {
            l = (l + r)/2;
        }else {
            r = (l + r)/2;
        }
    }
    if slice[r] >= x {return slice[r];}
    return isize::MAX;
}