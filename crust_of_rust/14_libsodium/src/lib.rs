#![allow(dead_code)]

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[unsafe(no_mangle)]
extern "C" fn this_is_rust(arg: std::os::raw::c_int) -> std::os::raw::c_char {
    println!("Hello, Rust!");
    '9' as i8
}

// use std::cell::OnceCell;

use std::{cell::UnsafeCell, mem::MaybeUninit, ptr};

pub use ffi::sodium_init;

// static HAS_BEEN_INIT: OnceCell<bool> = OnceCell::new(false);

#[non_exhaustive]
#[derive(Clone, Debug)]
struct Sodium;

impl Sodium {
    pub fn new() -> Result<Self, ()> {
        if unsafe { sodium_init() } < 0 {
            Err(())
        } else {
            Ok(Self)
        }
    }

    pub fn crypto_generichash<'a>(
        &self,
        input: &[u8],
        key: Option<&[u8]>,
        out: &'a mut [MaybeUninit<u8>],
    ) -> Result<&'a mut [u8], ()> {
        assert!(out.len() >= usize::try_from(ffi::crypto_generichash_BYTES_MIN).unwrap());
        assert!(out.len() <= usize::try_from(ffi::crypto_generichash_BYTES_MAX).unwrap());

        if let Some(key) = key {
            assert!(key.len() >= usize::try_from(ffi::crypto_generichash_BYTES_MIN).unwrap());
            assert!(key.len() <= usize::try_from(ffi::crypto_generichash_BYTES_MAX).unwrap());
        }

        let (key, keylen) = if let Some(key) = key {
            (key.as_ptr(), key.len())
        } else {
            (ptr::null(), 0)
        };

        // SAFETY: We've checked the requirements of the function (MIN/MAX), and
        //   the presence of &self means that init has been called
        let res = unsafe {
            ffi::crypto_generichash(
                out.as_mut_ptr() as *mut u8,
                out.len(),
                input.as_ptr(),
                input.len() as u64,
                key,
                keylen,
            )
        };

        if res < 0 {
            Err(())
        } else {
            Ok(unsafe { &mut *(out as *mut _ as *mut [u8]) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Sodium::new().unwrap();
    }

    #[test]
    fn it_hashes() {
        let s = Sodium::new().unwrap();
        let input = b"Arbitrary data to hash";
        let key = None;
        let mut out = [MaybeUninit::uninit(); ffi::crypto_generichash_BYTES as usize];
        let out = s.crypto_generichash(input, key, &mut out).unwrap();
        println!("{}", hex::encode(&out));
    }
}
