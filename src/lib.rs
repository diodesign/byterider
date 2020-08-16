/* Byte and multi-byte access to memory
 * 
 * This does not require the standard library,
 * though it does require a dynamic allocator
 * to create binary structures.
 * 
 * Not thread safe: access Bytes exclusively.
 * 
 * (c) Chris Williams, 2020.
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

pub enum Ordering
{
    LittleEndian,
    BigEndian
}

pub struct Bytes
{
    ordering: Ordering,
    data: Vec<u8>
}

impl Bytes
{
    /* create a new, empty Bytes object, ordering defaults to host ordering */
    pub fn new() -> Bytes
    {
        Bytes
        {
            ordering: if cfg!(target_endian = "little")
            {
                Ordering::LittleEndian
            }
            else
            {
                Ordering::BigEndian
            },
            data: Vec::<u8>::new()
        }
    }

    /* set the byte ordering for the data in memory. this data is automatically converted from
    the host ordering to the chosen in-memory ordering when writing, and from the chosen
    in-memory ordering to the host ordering when reading. the default is the host's ordering. */
    pub fn set_ordering(&mut self, order: Ordering)
    {
        self.ordering = order;
    }

    /* create a new Bytes object and copy in the given byte slice */
    pub fn from_slice(bytes: &[u8]) -> Bytes
    {
        let mut b = Bytes::new();
        b.data = bytes.to_vec();
        return b;
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
    pub fn add_byte(&mut self, value: u8)
    {
        self.data.push(value);
    }

    /* read a byte from the given byte offset,
    or None if offset is out of bounds */
    pub fn read_byte(&self, offset: usize) -> Option<u8>
    {
        match self.data.get(offset)
        {
            Some(byte) => Some(*byte),
            None => None
        }
    }

    /* add a 32-bit word to the end of the array */
    pub fn add_word(&mut self, value: u32)
    {
        /* handling byte ordering */
        if cfg!(target_endian = "little")
        {
            /* and self.ordering is the order of the data saved in memory */
            let ordered = match self.ordering
            {
                Ordering::LittleEndian => value,
                Ordering::BigEndian => value.swap_bytes()
            };

            self.add_byte(((ordered >>  0) & 0xff) as u8);
            self.add_byte(((ordered >>  8) & 0xff) as u8);
            self.add_byte(((ordered >> 16) & 0xff) as u8);
            self.add_byte(((ordered >> 24) & 0xff) as u8);
        }
        else
        {
            let ordered = match self.ordering
            {
                Ordering::LittleEndian => value.swap_bytes(),
                Ordering::BigEndian => value
            };

            self.add_byte(((ordered >> 24) & 0xff) as u8);
            self.add_byte(((ordered >> 16) & 0xff) as u8);
            self.add_byte(((ordered >>  8) & 0xff) as u8);
            self.add_byte(((ordered >>  0) & 0xff) as u8);
        }
    }

    /* read a 32-bit word from the given byte offset,
    or None if offset is out of bounds */
    pub fn read_word(&self, offset: usize) -> Option<u32>
    {
        match self.data.get(offset..(offset + size_of::<u32>()))
        {
            Some(bytes) =>
            {
                Some
                (
                    (bytes[0] as u32) <<  0 |
                    (bytes[1] as u32) <<  8 |
                    (bytes[2] as u32) << 16 |
                    (bytes[3] as u32) << 24
                )
            },
            None => None
        }
    }
}
