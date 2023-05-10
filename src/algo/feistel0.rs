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

fn swap_bits(n: u8, p1: u8, p2: u8) -> u8 {
    let bit1 = (n >> p1) & 1;
    let bit2 = (n >> p2) & 1;
    let mut x = (bit1 ^ bit2);
    x = (x << p1) | (x << p2);
 
    n ^ x
}

pub fn inner(input: u16, key: u16) -> u16 {
    // cut into u8 blocks
    let mut blocks: [u8; 4] = [
        ((input & 0xf000) >> 12).to_be_bytes()[1],
        ((input & 0x0f00) >> 08).to_be_bytes()[1],
        ((input & 0x000f) >> 00).to_be_bytes()[1],
        ((input & 0x00f0) >> 04).to_be_bytes()[1],
    ];

    dbg!(&blocks);

    // each block must not be more than 0xf
    for block in blocks {
        if block > 0xf {
            panic!("Block is more than 0xf: {block}");
        }
    }

    // inner blocks into sbox
    blocks[1] = SBOX[blocks[1] as usize];
    blocks[2] = SBOX[blocks[2] as usize];

    dbg!(&blocks);

    // swap position and endianess for outer blocks
    let tmp = blocks[0];
    blocks[0] = u8::swap_bytes(blocks[3]);
    blocks[3] = u8::swap_bytes(tmp);

    dbg!(&blocks);
    
    println!("{:04b} {:04b} {:04b} {:04b}", blocks[0], blocks[1], blocks[2], blocks[3]);
    println!("{:x}{:x}{:x}{:x}", blocks[0], blocks[1], blocks[2], blocks[3]);

    panic!("Not implemented");
}
