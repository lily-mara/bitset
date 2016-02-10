#![feature(plugin)]
#![plugin(stainless)]
#![allow(dead_code)]

#[cfg(test)]
#[allow(unused_mut)]
mod test;

extern crate libc;

use std::{ ptr };

pub const DEFAULT_SIZE: usize = 256;

const BYTE_SIZE: usize = 8;

const MASKS: [u8; BYTE_SIZE] = [
    0b00000001,
    0b00000010,
    0b00000100,
    0b00001000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b10000000,
];

#[derive(Debug)]
pub struct BitSet {
    data: *mut u8,
    capacity: usize,
}

impl BitSet {
    fn new() -> BitSet {
        BitSet::new_with_capacity(DEFAULT_SIZE)
    }

    fn new_with_capacity(capacity: usize) -> BitSet {
        let cap = if capacity % BYTE_SIZE == 0 {
            capacity
        } else {
            BYTE_SIZE - capacity % BYTE_SIZE
        };

        unsafe {
            let p = libc::malloc(cap) as *mut u8;
            assert!(!p.is_null());

            BitSet { data: p, capacity: capacity }
        }
    }

    fn contains(&self, x: usize) -> bool {
        if x >= self.capacity {
            return false;
        }
        let offset = x / BYTE_SIZE;
        let index = x % BYTE_SIZE;
        let mask = MASKS[index];

        unsafe {
            let p = self.data.offset(offset as isize);
            let d = ptr::read(p) ^ mask;

            d == 0
        }
    }

    fn insert(&mut self, x: usize) {
        assert!(x <= self.capacity);
        let offset = x / BYTE_SIZE;
        let index = x % BYTE_SIZE;
        let mask = MASKS[index];

        unsafe {
            let p = self.data.offset(offset as isize);
            let d = ptr::read(p);

            ptr::write(p, d | mask);
        }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Drop for BitSet {
    fn drop(&mut self) {
        self.capacity = 0;

        unsafe {
            libc::free(self.data as *mut libc::c_void);
        }
    }
}
