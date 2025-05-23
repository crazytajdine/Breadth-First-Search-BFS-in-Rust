pub fn find_k<T: PartialEq + Copy>(l: &[T], k: T) -> Option<T> {
    for &value in l {
        if value == k {
            return Some(k);
        }
    }
    None
}
