/// # basic implementation of a feistel network
///
/// This module implements a feistel network according to an exercise at DHBW Mannheim.
/// For demonstration purposes only, do not use this in a secure environment.
///
/// ___
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>

const SBOX: [u8; 0x10] = [0x4, 3, 9, 0xa, 0xb, 2, 0xe, 1, 0xd, 0xc, 8, 6, 7, 5, 0, 0xf];

#[test]
/// test inner against the given values
fn test_inner() {
    assert!(inner(0x1234, 0x0000) == 0x29a8);
    assert!(inner(0x1234, 0x2345) == 0x0aed);
    assert!(inner(0xabcd, 0xbeef) == 0x089a);
    assert!(inner(0x9876, 0xfedc) == 0x93c5);
}

pub fn inner(input: u16, key: u16) -> u16 {
    // cut into u8 blocks
    let mut blocks: [u8; 4] = [
        ((input & 0xf000) >> 12).to_be_bytes()[1],
        ((input & 0x0f00) >> 08).to_be_bytes()[1],
        ((input & 0x00f0) >> 04).to_be_bytes()[1],
        ((input & 0x000f) >> 00).to_be_bytes()[1],
    ];

    println!("{:04b} {:04b} {:04b} {:04b}", blocks[0], blocks[1], blocks[2], blocks[3]);
    println!("{:x}{:x}{:x}{:x}", blocks[0], blocks[1], blocks[2], blocks[3]);

    // each block must not be more than 0xf
    for block in blocks {
        if block > 0xf {
            panic!("Block is more than 0xf: {block}");
        }
    }

    // inner blocks into sbox
    blocks[1] = SBOX[blocks[1] as usize];
    blocks[2] = SBOX[blocks[2] as usize];

    println!("{:04b} {:04b} {:04b} {:04b}", blocks[0], blocks[1], blocks[2], blocks[3]);
    println!("{:x}{:x}{:x}{:x}", blocks[0], blocks[1], blocks[2], blocks[3]);

    // swap position and endianess for outer blocks
    let mut b0 = 0;
    let mut b3 = 0;
    b0 |= (blocks[0] & 0b1000) >> 3;
    b0 |= (blocks[0] & 0b0100) >> 1;
    b0 |= (blocks[0] & 0b0010) << 1;
    b0 |= (blocks[0] & 0b0001) << 3;
    b3 |= (blocks[3] & 0b1000) >> 3;
    b3 |= (blocks[3] & 0b0100) >> 1;
    b3 |= (blocks[3] & 0b0010) << 1;
    b3 |= (blocks[3] & 0b0001) << 3;

    blocks[0] = b3;
    blocks[3] = b0;
    
    println!("{:04b} {:04b} {:04b} {:04b}", blocks[0], blocks[1], blocks[2], blocks[3]);
    println!("{:x}{:x}{:x}{:x}", blocks[0], blocks[1], blocks[2], blocks[3]);

    for block in blocks {
        if block > 0xf {
            panic!("Block is more than 0xf: {block}");
        }
    }

    let mut result: u16 = 0;
    result |= (blocks[0] as u16) << 12;
    result |= (blocks[1] as u16) << 08;
    result |= (blocks[2] as u16) << 04;
    result |= (blocks[3] as u16) << 00;

    return result ^ key
}

pub fn sbox(index: u8) -> u8 {
    assert!(index < 0xf);
    return SBOX[index as usize];
}
