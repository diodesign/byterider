/* Byte and multi-byte access to memory
 * 
 * Unit tests
 * 
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

use core::mem::size_of;

 /* number of bytes in a populate_bytes()'d array. keep it less than 256 (see below) */
const BYTE_FILL_SIZE: usize = 255;

/* list of byte orderings to test for. this is only needed for word tests, not bytes */
const ORDERINGS: [crate::Ordering; 2] = [ crate::Ordering::LittleEndian, crate::Ordering::BigEndian ];

/* fill an array with sequentially increasing positive integer byte values,
from 0 to BYTE_FILL_SIZE, eg: 0, 1, 2, 3, 4... BYTE_FILL_SIZE should be less than 256 */
fn populate_bytes() -> crate::Bytes
{
    let mut b = crate::Bytes::new();
    for v in 0..BYTE_FILL_SIZE
    {
        b.add_u8(v as u8);
    }

    b
}

#[test]
fn as_slice()
{
    assert_eq!(populate_bytes().as_slice().len(), BYTE_FILL_SIZE as usize);
}

#[test]
fn from_slice()
{
    let values = [0, 2, 4, 6, 8];
    let b = crate::Bytes::from_slice(&values);
    for i in 0..values.len()
    {
        assert_eq!(b.read_u8(i).unwrap(), values[i]);
    }
}

#[test]
fn len()
{
    assert_eq!(populate_bytes().len(), BYTE_FILL_SIZE as usize);
}

#[test]
fn add_u8()
{
    let b = populate_bytes();
    let s = b.as_slice();
    for v in 0..BYTE_FILL_SIZE
    {
        assert_eq!(s[v], v as u8);
    }
}

#[test]
fn add_u32()
{
    for ordering in &ORDERINGS
    {
        let mut b = crate::Bytes::new();
        b.set_ordering(*ordering);
        b.add_u32(0xaabbccdd);
        b.add_u32(0x11223344);

        let s = b.as_slice();

        match *ordering
        {
            crate::Ordering::LittleEndian =>
            {
                assert_eq!(s[0], 0xdd);
                assert_eq!(s[1], 0xcc);
                assert_eq!(s[2], 0xbb);
                assert_eq!(s[3], 0xaa);
                assert_eq!(s[4], 0x44);
                assert_eq!(s[5], 0x33);
                assert_eq!(s[6], 0x22);
                assert_eq!(s[7], 0x11);
            },

            crate::Ordering::BigEndian =>
            {
                assert_eq!(s[0], 0xaa);
                assert_eq!(s[1], 0xbb);
                assert_eq!(s[2], 0xcc);
                assert_eq!(s[3], 0xdd);
                assert_eq!(s[4], 0x11);
                assert_eq!(s[5], 0x22);
                assert_eq!(s[6], 0x33);
                assert_eq!(s[7], 0x44);
            }
        }
    }
}

#[test]
fn add_u64()
{
    for ordering in &ORDERINGS
    {
        let mut b = crate::Bytes::new();
        b.set_ordering(*ordering);
        b.add_u64(0x11223344aabbccdd);

        let s = b.as_slice();

        match *ordering
        {
            crate::Ordering::LittleEndian =>
            {
                assert_eq!(s[0], 0xdd);
                assert_eq!(s[1], 0xcc);
                assert_eq!(s[2], 0xbb);
                assert_eq!(s[3], 0xaa);
                assert_eq!(s[4], 0x44);
                assert_eq!(s[5], 0x33);
                assert_eq!(s[6], 0x22);
                assert_eq!(s[7], 0x11);
            },

            crate::Ordering::BigEndian =>
            {
                assert_eq!(s[0], 0x11);
                assert_eq!(s[1], 0x22);
                assert_eq!(s[2], 0x33);
                assert_eq!(s[3], 0x44);
                assert_eq!(s[4], 0xaa);
                assert_eq!(s[5], 0xbb);
                assert_eq!(s[6], 0xcc);
                assert_eq!(s[7], 0xdd);
            }
        }
    }
}

#[test]
fn read_u8()
{
    let b = populate_bytes();
    for i in 0..BYTE_FILL_SIZE
    {
        assert_eq!(b.read_u8(i).unwrap(), i as u8);
    }
}

#[test]
fn read_u32()
{
    let mut b = crate::Bytes::new();

    b.add_u8(0x11);
    b.add_u8(0x22);
    b.add_u8(0x33);
    b.add_u8(0x44);

    b.add_u8(0x55);
    b.add_u8(0x66);
    b.add_u8(0x77);
    b.add_u8(0x88);

    for ordering in &ORDERINGS
    {
        b.set_ordering(*ordering);

        match *ordering
        {
            crate::Ordering::LittleEndian =>
            {
                assert_eq!(b.read_u32(0).unwrap(), 0x44332211);
                assert_eq!(b.read_u32(2).unwrap(), 0x66554433);
                assert_eq!(b.read_u32(4).unwrap(), 0x88776655);
            },

            crate::Ordering::BigEndian =>
            {
                assert_eq!(b.read_u32(0).unwrap(), 0x11223344);
                assert_eq!(b.read_u32(2).unwrap(), 0x33445566);
                assert_eq!(b.read_u32(4).unwrap(), 0x55667788);
            }
        }
    }
}

#[test]
fn read_u64()
{
    let mut b = crate::Bytes::new();

    b.add_u8(0x11);
    b.add_u8(0x22);
    b.add_u8(0x33);
    b.add_u8(0x44);
    b.add_u8(0x55);
    b.add_u8(0x66);
    b.add_u8(0x77);
    b.add_u8(0x88);
    let value: u64 = 0x8877665544332211;

    for ordering in &ORDERINGS
    {
        b.set_ordering(*ordering);

        match *ordering
        {
            crate::Ordering::LittleEndian => assert_eq!(b.read_u64(0).unwrap(), value.to_le()),
            crate::Ordering::BigEndian => assert_eq!(b.read_u64(0).unwrap(), value.to_be())
        }
    }
}

#[test]
fn alter_u8()
{
    let mut b = populate_bytes();
    for i in 0..BYTE_FILL_SIZE
    {
        let new_value = (BYTE_FILL_SIZE - i) as u8;
        assert_eq!(b.alter_u8(i, new_value), true);
        assert_eq!(b.read_u8(i).unwrap(), new_value);
    }
}

#[test]
fn alter_u32()
{
    let words: [u32; 4] =
    [
        0x11223344, 0x55667788, 0x99aabbcc, 0xddeeff00
    ];
    let new_value = 0xff00ff00;

    for ordering in &ORDERINGS
    {
        let mut b = crate::Bytes::new();
        b.set_ordering(*ordering);
        for w in &words
        {
            b.add_u32(*w);
        }

        for i in 0..words.len()
        {
            assert_eq!(b.alter_u32(i * size_of::<u32>(),  new_value), true);
        }

        for i in 0..words.len()
        {
            assert_eq!(b.read_u32(i * size_of::<u32>()).unwrap(), new_value);
        }
    }
}

#[test]
fn alter_u64()
{
    let words: [u64; 2] =
    [
        0x1122334455667788, 0x99aabbccdeeff00
    ];
    let new_value = 0xff00ff00ff00ff00;

    for ordering in &ORDERINGS
    {
        let mut b = crate::Bytes::new();
        b.set_ordering(*ordering);
        for w in &words
        {
            b.add_u64(*w);
        }

        for i in 0..words.len()
        {
            assert_eq!(b.alter_u64(i * size_of::<u64>(),  new_value), true);
        }

        for i in 0..words.len()
        {
            assert_eq!(b.read_u64(i * size_of::<u64>()).unwrap(), new_value);
        }
    }
}