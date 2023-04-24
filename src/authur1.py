#!/usr/bin/env python3
"""
A custom authentication hash function aswell as an automated extension attack for it.

Since this (auth) hash did not have a name before, I gave it the name 'authur1'

@author Christoph J. Scherr <software@cscherr.de>
@license MIT
@source: https://git.cscherr.de/PlexSheep/python-dhbw/src/branch/master/src/authur1.py
"""
import math
import argparse
from ctypes import c_uint32

# constants for authur1
SHIFT_LENGTH = 17
DEFINED_INITIAL = bytes([0x52, 0x4f, 0x46, 0x4c])

"""
C implementation for bit rotating:
see https://en.wikipedia.org/wiki/Circular_shift#Implementing_circular_shifts

```c
/*
 * Shift operations in C are only defined for shift values which are
 * not negative and smaller than sizeof(value) * CHAR_BIT.
 * The mask, used with bitwise-and (&), prevents undefined behaviour
 * when the shift count is 0 or >= the width of unsigned int.
 */

#include <stdint.h>  // for uint32_t, to get 32-bit-wide rotates, regardless of the size of int.
#include <limits.h>  // for CHAR_BIT

uint32_t rotl32 (uint32_t value, unsigned int count) {
    const unsigned int mask = CHAR_BIT * sizeof(value) - 1;
    count &= mask;
    return (value << count) | (value >> (-count & mask));
}

uint32_t rotr32 (uint32_t value, unsigned int count) {
    const unsigned int mask = CHAR_BIT * sizeof(value) - 1;
    count &= mask;
    return (value >> count) | (value << (-count & mask));
}
```
"""

"""
The rotations are tested agains a c implementation and seem to work fine.
"""
# constant value defined in limits.h, it's 8 (bit) on my machine, on yours probably too.
CHAR_BIT = 8
# python is being a dynamic dumbass, do a 32 bit shift / 4 byte
VALUE_SIZE = 4
def rotl(value: int, count: int) -> int:
    mask: int = CHAR_BIT * VALUE_SIZE - 1;
    count &= mask;
    return (value << count) | (value >> (-count & mask));

def rotr(value: int, count: int) -> int:
    mask: int = CHAR_BIT * VALUE_SIZE - 1;
    count &= mask;
    return (value >> count) | (value << (-count & mask));

def Q(input: c_uint32) -> c_uint32:
    output: c_uint32

    output = input ^ (rot_r(input, SHIFT_LENGTH))

    return output

def H(input: bytes) -> bytes:


    return input

def main():
    a: str = input()
    ai = int(a)
    print((ai))
    print((rotr(ai,17)))

if __name__ == "__main__":
    main()
