/* Byte and multi-byte access to memory
 * 
 * This does not require the standard library,
 * though it does require a dynamic allocator to create binary structures.
 * 
 * (c) Chris Williams, 2020
 *
 * See LICENSE for usage and copying.
 */

#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;
use core::mem::size_of;

#[cfg(test)]
mod tests;

struct Bytes
{
    data: Vec<u8>
}

impl Bytes
{
    pub fn new() -> Bytes
    {
        Bytes
        {
            data: Vec::<u8>::new()
        }
    }

    /* access the data as a borrowed immutable slice */
    pub fn as_slice(&self) -> &[u8]
    {
        return self.data.as_slice();
    }

    /* return the length of the array in bytes */
    pub fn len(&self) -> usize
    {
        return self.data.len();
    }

    /* add a byte to the end of the array */
    pub fn add_byte(&mut self, val: u8)
    {
        self.data.push(val);
    }

    /* add a 32-bit big endian word to the end of the array */
    pub fn add_be_word(&mut self, val: u32)
    {
        self.add_byte(((val >> 24) & 0xff) as u8);
        self.add_byte(((val >> 16) & 0xff) as u8);
        self.add_byte(((val >>  8) & 0xff) as u8);
        self.add_byte(((val >>  0) & 0xff) as u8);
    }

    /* add a 32-bit little endian word to the end of the array */
    pub fn add_le_word(&mut self, val: u32)
    {
        self.add_byte(((val >>  0) & 0xff) as u8);
        self.add_byte(((val >>  8) & 0xff) as u8);
        self.add_byte(((val >> 16) & 0xff) as u8);
        self.add_byte(((val >> 24) & 0xff) as u8);
    }

    /* read a 32-bit big endian word from the given byte offset,
    or None if offset is out of bounds */
    pub fn read_be_word(&self, offset: usize) -> Option<u32>
    {
        match self.read_le_word_at_offset(offset)
        {
            None => return None,
            Some(v) =>
            {
                let value = (v & 0xff000000) >> 24 |
                            (v & 0x00ff0000) >>  8 |
                            (v & 0x0000ff00) <<  8 |
                            (v & 0x000000ff) << 24;
                return Some(value);
            }
        }
    }

    /* read a 32-bit little endian word from the given byte offset,
    or None if offset is out of bounds */
    pub fn read_le_word(&self, offset: usize) -> Option<u32>
    {
        return self.read_le_word_at_offset(offset);
    }

    /* read a 32-bit little endian word from the given byte offset,
    or None if offset is out of bounds */
    fn read_le_word_at_offset(&self, offset: usize) -> Option<u32>
    {
        if (offset + size_of::<u32>()) > self.len()
        {
            return None;
        }

        let array = self.data.as_slice();
        let value = (array[offset + 0] as u32) <<  0 |
                    (array[offset + 1] as u32) <<  8 |
                    (array[offset + 2] as u32) << 16 |
                    (array[offset + 3] as u32) << 24;
        return Some(value);
    }
}

