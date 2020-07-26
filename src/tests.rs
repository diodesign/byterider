/* Byte and multi-byte access to memory
 * 
 * Unit tests
 * 
 * (c) Chris Williams, 2020.
 *
 * See LICENSE for usage and copying.
 */

#[test]
fn as_slice()
{
    let count = 42;
    let mut b = crate::Bytes::new();
    for _ in 0..count
    {
        b.add_byte(0x41);
    }
    
    assert_eq!(b.as_slice().len(), count);
}

#[test]
fn len()
{
    let count = 666;
    let mut b = crate::Bytes::new();
    for _ in 0..count
    {
        b.add_byte(0xaa);
    }
    
    assert_eq!(b.len(), count);
}

#[test]
fn add_byte()
{
    let count = 256;
    let mut b = crate::Bytes::new();
    for v in 0..256
    {
        b.add_byte(v as u8);
    }

    let s = b.as_slice();
    for v in 0..count
    {
        assert_eq!(s[v], v as u8);
    }
}

#[test]
fn add_word()
{
    let mut b = crate::Bytes::new();
    b.add_word(0xaabbccdd);

    let s = b.as_slice();
    assert_eq!(s[0], 0xdd);
    assert_eq!(s[1], 0xcc);
    assert_eq!(s[2], 0xbb);
    assert_eq!(s[3], 0xaa);
}

#[test]
fn read_word()
{
    let mut b = crate::Bytes::new();
    b.add_byte(0x11);
    b.add_byte(0x22);
    b.add_byte(0x33);
    b.add_byte(0x44);

    b.add_byte(0x55);
    b.add_byte(0x66);
    b.add_byte(0x77);
    b.add_byte(0x88);

    assert_eq!(b.read_word(0).unwrap(), 0x44332211);
    assert_eq!(b.read_word(4).unwrap(), 0x88776655);
    assert_eq!(b.read_word(2).unwrap(), 0x66554433);
}

#[test]
fn from_slice()
{
    let values = [0, 2, 4, 6, 8];
    let b = crate::Bytes::from_slice(&values);
    assert_eq!(b.read_byte(0).unwrap(), 0);
    assert_eq!(b.read_byte(1).unwrap(), 2);
    assert_eq!(b.read_byte(2).unwrap(), 4);
    assert_eq!(b.read_byte(3).unwrap(), 6);
    assert_eq!(b.read_byte(4).unwrap(), 8);
}

#[test]
fn read_byte()
{
    let mut b = crate::Bytes::new();
    b.add_byte(0xaa);
    b.add_byte(0xbb);
    b.add_byte(0xcc);
    b.add_byte(0xdd);

    assert_eq!(b.read_byte(0).unwrap(), 0xaa);
    assert_eq!(b.read_byte(1).unwrap(), 0xbb);
    assert_eq!(b.read_byte(2).unwrap(), 0xcc);
    assert_eq!(b.read_byte(3).unwrap(), 0xdd);
}