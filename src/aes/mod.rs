#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bytes::BytesMut;
use std::sync::{Arc, Mutex};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[allow(dead_code)]
pub fn decrypt<T>(buffer: &mut T, iv: Arc<Mutex<BytesMut>>, size: usize)
where
    T: AsMut<[u8]>,
{
    unsafe {
        ffi_decrypt(
            buffer.as_mut().as_mut_ptr(),
            iv.lock().unwrap().as_mut_ptr(),
            size as ::std::os::raw::c_ushort,
        );
    }
}
#[allow(dead_code)]
pub fn encrypt(buffer: &mut [u8], iv: &mut [u8], size: usize) {
    unsafe {
        ffi_encrypt(
            buffer.as_mut_ptr(),
            iv.as_mut_ptr(),
            size as ::std::os::raw::c_ushort,
        );
    }
}
#[allow(dead_code)]
pub fn create_packet_header(buffer: &mut [u8], iv: Arc<Mutex<BytesMut>>, size: usize) {
    unsafe {
        ffi_create_packet_header(
            buffer.as_mut_ptr(),
            iv.lock().unwrap().as_mut_ptr(),
            size as ::std::os::raw::c_ushort,
        );
    }
}
#[allow(dead_code)]
pub fn get_packet_length(buffer: &mut [u8]) -> usize {
    unsafe { ffi_get_packet_length(buffer.as_mut_ptr()) as usize }
}
