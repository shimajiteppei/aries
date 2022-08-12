use core::ops::Range;

pub trait Slice: Sized {
    fn copy_slice(&self, range: Range<usize>) -> Self;
}

impl Slice for Vec<u8> {
    #[inline(always)]
    fn copy_slice(&self, range: Range<usize>) -> Self {
        let mut slice = Vec::new();
        slice.extend(self[range].iter().copied());
        slice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _equal_array() {
        let vec = [0, 1, 2, 3];
        assert_eq!(true, [1, 2, 3] == vec[1..4]);
    }

    #[test]
    fn _slice() {
        let origin: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
        let sliced: Vec<u8> = vec![0, 1, 2];
        assert_eq!(sliced, origin.copy_slice(0..3));
    }
}
