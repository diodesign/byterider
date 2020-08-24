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

    /* return offsets into the top-most byte */
    pub fn offset32(&self) -> u32 { self.data.len() as u32 }
    pub fn offset64(&self) -> u64 { self.data.len() as u64 }

    /* convert the given u32 value to the byte order for storing in memory.
    it works in reverse: convert word in memory from byte order */
    fn order_u32(&self, value: u32) -> u32
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

    /* convert the given u64 value to the byte order for storing in memory.
    it works in reverse: convert word in memory from byte order */
    fn order_u64(&self, value: u64) -> u64
    {
        match self.ordering
        {
            Ordering::LittleEndian => value.to_le(),
            Ordering::BigEndian => value.to_be()
        }
    }

    /* add a string as a series of bytes. will not add a null terminator!
    do this yourself using add_null_terminator(), or use add_null_term_string() */
    pub fn add_string(&mut self, to_add: &str)
    {
        for byte in to_add.bytes()
        {
            self.data.push(byte);
        }
    }

    /* add a NULL (zero) byte terminator */
    pub fn add_null_terminator(&mut self)
    {
        self.data.push(0);
    }

    /* add a null-terminated string as a series of bytes, including the terminator */
    pub fn add_null_term_string(&mut self, to_add: &str)
    {
        self.add_string(to_add);
        self.add_null_terminator();
    }

    /* zero pad a byte array to the nearest whole 32-bit word */
    pub fn pad_to_u32(&mut self)
    {
        let word_size = size_of::<u32>();
        let padding = self.data.len();
        for _ in 0..(word_size - (padding % word_size))
        {
            self.data.push(0);       
        }
    }

    /* add a byte to the end of the array */
    pub fn add_u8(&mut self, value: u8) { self.data.push(value) }

    /* add a 32-bit word to the end of the array.
    value = word to write into memory using array's byte ordering */
    pub fn add_u32(&mut self, value: u32)
    {
        let value = self.order_u32(value);
        self.add_u8(((value >>  0) & 0xff) as u8);
        self.add_u8(((value >>  8) & 0xff) as u8);
        self.add_u8(((value >> 16) & 0xff) as u8);
        self.add_u8(((value >> 24) & 0xff) as u8);
    }

    /* add a 64-bit word to the end of the array.
    value = word to write into memory using array's byte ordering */
    pub fn add_u64(&mut self, value: u64)
    {
        let value = self.order_u64(value);
        self.add_u8(((value >>  0) & 0xff) as u8);
        self.add_u8(((value >>  8) & 0xff) as u8);
        self.add_u8(((value >> 16) & 0xff) as u8);
        self.add_u8(((value >> 24) & 0xff) as u8);
        self.add_u8(((value >> 32) & 0xff) as u8);
        self.add_u8(((value >> 40) & 0xff) as u8);
        self.add_u8(((value >> 48) & 0xff) as u8);
        self.add_u8(((value >> 56) & 0xff) as u8);
    }

    /* read a byte from the given byte offset,
    or None if offset is out of bounds */
    pub fn read_u8(&self, offset: usize) -> Option<u8>
    {
        match self.data.get(offset)
        {
            Some(byte) => Some(*byte),
            None => None
        }
    }

    /* read a 32-bit word from the given byte offset, 
    using the array's byte ordering. returns None if offset is out of bounds */
    pub fn read_u32(&self, offset: usize) -> Option<u32>
    {
        match self.data.get(offset..(offset + size_of::<u32>()))
        {
            Some(bytes) =>
            {
                return Some
                (
                    self.order_u32
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

    /* read a 64-bit word from the given byte offset, 
    using the array's byte ordering. returns None if offset is out of bounds */
    pub fn read_u64(&self, offset: usize) -> Option<u64>
    {
        match self.data.get(offset..(offset + size_of::<u64>()))
        {
            Some(bytes) =>
            {
                return Some
                (
                    self.order_u64
                    (
                        (bytes[0] as u64) <<  0 |
                        (bytes[1] as u64) <<  8 |
                        (bytes[2] as u64) << 16 |
                        (bytes[3] as u64) << 24 |
                        (bytes[4] as u64) << 32 |
                        (bytes[5] as u64) << 40 |
                        (bytes[6] as u64) << 48 |
                        (bytes[7] as u64) << 56
                    )
                )
            },
            None => return None
        }
    }

    /* alter a byte in the array at the given offset.
    returns true if successful, or false if out of bounds */
    pub fn alter_u8(&mut self, offset: usize, new_value: u8) -> bool
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
    pub fn alter_u32(&mut self, offset: usize, new_value: u32) -> bool
    {
        let new_value = self.order_u32(new_value);
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

    /* alter a 64-bit word in the array at the given offset.
    new_value = word to write into memory using array's ordering
    returns true if successful, or false if out of bounds */
    pub fn alter_u64(&mut self, offset: usize, new_value: u64) -> bool
    {
        let new_value = self.order_u64(new_value);
        match self.data.get_mut(offset..(offset + size_of::<u64>()))
        {
            Some(ptr) =>
            {                
                ptr[0] = ((new_value >>  0) & 0xff) as u8;
                ptr[1] = ((new_value >>  8) & 0xff) as u8;
                ptr[2] = ((new_value >> 16) & 0xff) as u8;
                ptr[3] = ((new_value >> 24) & 0xff) as u8;
                ptr[4] = ((new_value >> 32) & 0xff) as u8;
                ptr[5] = ((new_value >> 40) & 0xff) as u8;
                ptr[6] = ((new_value >> 48) & 0xff) as u8;
                ptr[7] = ((new_value >> 56) & 0xff) as u8;

                true
            },
            None => false
        }           
    }
}