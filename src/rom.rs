use std::{mem::{self, MaybeUninit}, ops::{Deref, DerefMut, Index, IndexMut}, ptr, slice::SliceIndex};

// smw rom
pub struct Rom {
    data: Box<[u8; Self::DATA_LEN]>,
}

impl Rom {
    const DATA_LEN: usize = 0x80000;
    const SMW_J_SHA1: [u8; 20] = [0xf9, 0x77, 0xaf, 0xab, 0xf2, 0x4e, 0xd2, 0x69, 0xd8, 0x63, 0x66, 0x20, 0x9a, 0x46, 0x04, 0x50, 0xbb, 0xc3, 0x7e, 0x76];

    pub fn new(data: &[u8]) -> Self {
        // validate data
        if data.len() != Self::DATA_LEN  || sha1sum(data) != Self::SMW_J_SHA1 {
            panic!("Please supply a valid ROM");
        }

        let mut new_data = Box::new_uninit();
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), new_data.as_mut_ptr() as _, Self::DATA_LEN);
            Self {
                data: new_data.assume_init(),
            }
        }
    }
}

impl Deref for Rom {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data[..]
    }
}

impl DerefMut for Rom {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..]
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for Rom {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for Rom {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

// sha1 stuff
// todo: write sha1 function myself

#[inline]
fn sha1sum(data: &[u8]) -> [u8; 20] {
    unsafe {
        let mut md = [MaybeUninit::<u8>::uninit(); 20];
        SHA1(data.as_ptr(), data.len(), md[0].as_mut_ptr());
        mem::transmute(md)
    }
}

#[link(name = "crypto", kind = "static")]
unsafe extern "C" {
    unsafe fn SHA1(d: *const u8, n: usize, md: *mut u8) -> *mut u8;
}
