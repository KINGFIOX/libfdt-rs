pub fn align8(val: usize) -> usize {
    (val + 7) & !7
}
