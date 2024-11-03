pub struct BitVec {
    vec: Vec<usize>,
    len: usize, // in bits
}

pub const BITS_PER_USIZE: usize = 8 * std::mem::size_of::<usize>();

impl BitVec {
    #[inline]
    pub fn new(default: bool, len: usize) -> Self {
        Self {
            vec: vec![
                if default {
                    std::usize::MAX
                } else {
                    std::usize::MIN
                };
                if len % BITS_PER_USIZE == 0 {
                    len / BITS_PER_USIZE
                } else {
                    len / BITS_PER_USIZE + 1
                }
            ],
            len,
        }
    }

    #[inline]
    pub fn get(&self, i: usize) -> bool {
        self.vec[i / BITS_PER_USIZE] & (1 << (i % BITS_PER_USIZE)) != 0
    }

    #[inline]
    pub fn set(&mut self, i: usize, v: bool) {
        let mask: usize = 1 << (i % BITS_PER_USIZE);
        if v {
            self.vec[i / BITS_PER_USIZE] |= mask;
        } else {
            self.vec[i / BITS_PER_USIZE] &= !mask;
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn into_inner(self) -> (Vec<usize>, usize) {
        (self.vec, self.len)
    }
}
