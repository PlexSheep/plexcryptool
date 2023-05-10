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
const ROUNDS: u8 = 3;

#[test]
/// test inner against the given values
fn test_inner() {
    assert!(inner(0x1234, 0x0000, false) == 0x29a8);
    assert!(inner(0x1234, 0x2345, false) == 0x0aed);
    assert!(inner(0xabcd, 0xbeef, false) == 0x089a);
    assert!(inner(0x9876, 0xfedc, false) == 0x93c5);
}

pub fn inner(input: u16, key: u16, verbose: bool) -> u16 {
    // cut into u8 blocks
    let mut blocks: [u8; 4] = [
        ((input & 0xf000) >> 12).to_be_bytes()[1],
        ((input & 0x0f00) >> 08).to_be_bytes()[1],
        ((input & 0x00f0) >> 04).to_be_bytes()[1],
        ((input & 0x000f) >> 00).to_be_bytes()[1],
    ];

    if verbose {
        println!("{:04b} {:04b} {:04b} {:04b}", blocks[0], blocks[1], blocks[2], blocks[3]);
        println!("{:x}{:x}{:x}{:x}", blocks[0], blocks[1], blocks[2], blocks[3]);
    }

    // each block must not be more than 0xf
    for block in blocks {
        if block > 0xf {
            panic!("Block is more than 0xf: {block}");
        }
    }

    // inner blocks into sbox
    blocks[1] = SBOX[blocks[1] as usize];
    blocks[2] = SBOX[blocks[2] as usize];

    if verbose {
        println!("{:04b} {:04b} {:04b} {:04b}", blocks[0], blocks[1], blocks[2], blocks[3]);
        println!("{:x}{:x}{:x}{:x}", blocks[0], blocks[1], blocks[2], blocks[3]);
    }

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
    
    if verbose {
        println!("{:04b} {:04b} {:04b} {:04b}", blocks[0], blocks[1], blocks[2], blocks[3]);
        println!("{:x}{:x}{:x}{:x}", blocks[0], blocks[1], blocks[2], blocks[3]);
    }

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

/// Boilerplate KSA, returns the same given values everytime.
pub fn key_scheduler(_key: u32) -> Vec<u16> {
    return vec![0xdead, 0xc0ff, 0xee5a]
}

#[test]
/// test the block encryption against the given value
fn test_encrypt() {
    assert_eq!(encrypt(0x12345678, vec![0x1aa2, 0x2bb3, 0x3cc4], true), 0x4313e07a);
}

/// encrypt a block
pub fn encrypt(plaintext: u32, keys: Vec<u16>, verbose: bool) -> u32 {
    assert_eq!(keys.len(), ROUNDS as usize);
    let mut lef: u16 = ((plaintext & 0xffff0000) >> 16).try_into().expect("boom");
    let mut rig: u16 = ((plaintext & 0x0000ffff) >> 0).try_into().expect("boom");
    if verbose {
        println!("input:\n{:08x}\n{:04x}\n    {:04x}", plaintext, lef, rig);
    }
    
    for i in 0..ROUNDS {
        let tmp = rig;
        rig = inner(rig, keys[i as usize], false) ^ lef;
        lef = tmp;
        if verbose {
            println!("{}\trig {:04x}\tlef {:04x}", i, rig, lef);
        }
    }

    let mut ciphertext: u32 = 0;
    ciphertext |= (lef as u32) << 16;
    ciphertext |= rig as u32;
    if verbose {
        println!("returning ciphertext:\n{:08x}", ciphertext);
    }
    return ciphertext;
}

#[test]
/// test the block decryption against the given value
fn test_decrypt() {
    let keys = key_scheduler(0x1337);
    let plaintext = 0xb00b00;
    let ciphertext = encrypt(plaintext, keys.clone(), true);
    let deciphertext = decrypt(ciphertext, keys.clone(), true);
    assert_eq!(plaintext, deciphertext);
}

/// decrypt a given plaintext with a given key vec
pub fn decrypt(ciphertext: u32, mut keys: Vec<u16>, verbose: bool) -> u32 {
    assert_eq!(keys.len(), ROUNDS as usize);

    // swap lef rig
    let mut lef: u16 = ((ciphertext & 0xffff0000) >> 16).try_into().expect("boom");
    let mut rig: u16 = ((ciphertext & 0x0000ffff) >> 0).try_into().expect("boom");
    let tmp = rig;
    rig = lef;
    lef = tmp;
    let mut ciphertext: u32 = 0;
    ciphertext |= (lef as u32) << 16;
    ciphertext |= rig as u32;
    if verbose {
        println!("input:\n{:08x}\n{:04x}\n    {:04x}", ciphertext, lef, rig);
    }

    // reverse keys
    keys.reverse();

    // do the thing
    ciphertext = encrypt(ciphertext, keys, verbose);

    // swap lef rig back
    let mut lef: u16 = ((ciphertext & 0xffff0000) >> 16).try_into().expect("boom");
    let mut rig: u16 = ((ciphertext & 0x0000ffff) >> 0).try_into().expect("boom");
    let tmp = rig;
    rig = lef;
    lef = tmp;
    let mut plaintext: u32 = 0;
    plaintext |= (lef as u32) << 16;
    plaintext |= rig as u32;
    if verbose {
        println!("input:\n{:08x}\n{:04x}\n    {:04x}", plaintext, lef, rig);
    }

    if verbose {
        println!("returning plaintext:\n{:08x}", plaintext);
    }
    return plaintext;
}

/// returns the value of the sbox for any input
///
/// max index is 0xf
pub fn sbox(index: u8) -> u8 {
    assert!(index < 0xf);
    return SBOX[index as usize];
}
