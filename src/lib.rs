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

/* define the word byte ordering of the data stored in memory */
#[derive(Clone, Copy)]
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

    /* set the word byte ordering for the data in memory. the default is the host's ordering. */
    pub fn set_ordering(&mut self, order: Ordering) { self.ordering = order }

    /* create a new Bytes object and copy in the given byte slice */
    pub fn from_slice(bytes: &[u8]) -> Bytes
    {
        let mut b = Bytes::new();
        b.data = bytes.to_vec();
        return b;
    }

    /* access the data as a borrowed immutable slice */
    pub fn as_slice(&self) -> &[u8] { self.data.as_slice() }

    /* clone the data as a vector */
    pub fn as_vec(&self) -> Vec<u8> { self.data.clone() }

    /* return the length of the array in bytes */
    pub fn len(&self) -> usize { self.data.len() }

    /* convert the given word value to the byte order for storing in memory.
    it works in reverse: convert word in memory from byte order */
    fn order_word(&self, value: u32) -> u32
    {
        /* to_le() is a no-op on little endian machines
        and to_be() is a no-op on big endian machines. otherwise
        they do a byte swap */
        match self.ordering
        {
            Ordering::LittleEndian => value.to_le(),
            Ordering::BigEndian => value.to_be()
        }
    }

    /* add a byte to the end of the array */
    pub fn add_byte(&mut self, value: u8) { self.data.push(value) }

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

    /* add a 32-bit word to the end of the array.
    value = word to write into memory using array's byte ordering */
    pub fn add_word(&mut self, value: u32)
    {
        let value = self.order_word(value);
        self.add_byte(((value >>  0) & 0xff) as u8);
        self.add_byte(((value >>  8) & 0xff) as u8);
        self.add_byte(((value >> 16) & 0xff) as u8);
        self.add_byte(((value >> 24) & 0xff) as u8);
    }

    /* read a 32-bit word from the given byte offset, 
    using the array's byte ordering. returns None if offset is out of bounds */
    pub fn read_word(&self, offset: usize) -> Option<u32>
    {
        match self.data.get(offset..(offset + size_of::<u32>()))
        {
            Some(bytes) =>
            {
                return Some
                (
                    self.order_word
                    (
                        (bytes[0] as u32) <<  0 |
                        (bytes[1] as u32) <<  8 |
                        (bytes[2] as u32) << 16 |
                        (bytes[3] as u32) << 24
                    )
                )
            },
            None => return None
        }
    }

    /* alter a byte in the array at the given offset.
    returns true if successful, or false if out of bounds */
    pub fn alter_byte(&mut self, offset: usize, new_value: u8) -> bool
    {
        match self.data.get_mut(offset)
        {
            Some(ptr) =>
            {
                *ptr = new_value;
                true
            },
            None => false
        }
    }

    /* alter a 32-bit word in the array at the given offset.
    new_value = word to write into memory using array's ordering
    returns true if successful, or false if out of bounds */
    pub fn alter_word(&mut self, offset: usize, new_value: u32) -> bool
    {
        let new_value = self.order_word(new_value);
        match self.data.get_mut(offset..(offset + size_of::<u32>()))
        {
            Some(ptr) =>
            {                
                ptr[0] = ((new_value >>  0) & 0xff) as u8;
                ptr[1] = ((new_value >>  8) & 0xff) as u8;
                ptr[2] = ((new_value >> 16) & 0xff) as u8;
                ptr[3] = ((new_value >> 24) & 0xff) as u8;

                true
            },
            None => false
        }           
    }
}