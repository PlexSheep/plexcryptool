/// # basic implementation of a feistel network
///
/// This module implements a feistel network according to an exercise at DHBW Mannheim.
/// For demonstration purposes only, do not use this in a secure environment.
///
/// ___
/// Author:     Christoph J. Scherr <software@cscherr.de>
/// License:    MIT
/// Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>


pub fn inner(input: u16, key: u16) -> u16 {
    let blocks = [
        input & 0xf000, 
        input & 0x0f00, 
        input & 0x00f0,
        input & 0x000f,
    ];
    dbg!(blocks);


    panic!("Not implemented");
}
