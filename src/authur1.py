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

# constants for authur1
SHIFT_LENGTH = 17
DEFINED_INITIAL = bytearray([0x52, 0x4f, 0x46, 0x4c])
PADDING = 0xff

# constants for Circular shifting
# constant value defined in limits.h, it's 8 (bit) on my machine, on yours probably too.
CHAR_BIT = 8
# python is being a dynamic dumbass, do a 32 bit shift ~ 4 byte
VALUE_SIZE = 4

"""
The rotations are tested agains a c implementation and are still garbage. FIXME
"""
def rotl(value: int, count: int) -> int:
    # FIXME
    mask: int = CHAR_BIT * VALUE_SIZE - 1;
    print("mask length: %d" % mask.bit_length())
    count = count & mask
    result = (value << count) | (value >> (-count & mask));
    assert result.bit_length() <= 32, "python made the numbers too big: %d bit" % result.bit_length()
    return result

def rotr(value: int, count: int) -> int:
    # FIXME
    mask: int = CHAR_BIT * VALUE_SIZE - 1;
    print("mask length: %d" % mask.bit_length())
    count = count & mask
    result = (value >> count) | (value << (-count & mask));
    assert result.bit_length() <= 32, "python made the numbers too big: %d bit" % result.bit_length()
    return result

"""
Now for the actual implementation of authur1
"""
def inner_authur1(input: int) -> int:
    # should really be 32 bit block
    # python sucks for binary operations
    # assert input.bit_length() == 32, "not a 32 bit int :("
    assert input.bit_length() <= 32, "input length is <= 32: %d" % input.bit_length()
    output: int

    output = input ^ (rotr(input, SHIFT_LENGTH))

    assert output.bit_length() <= 32, "output length is <= 32: %d" % output.bit_length()

    return output

def authur1(input: bytearray) -> bytearray:
    internal_buffer: bytearray = bytearray()
    accumulator: bytearray = DEFINED_INITIAL
    for in_byte in input:
        if not len(internal_buffer) == 4:
            internal_buffer.append(in_byte)
            continue
        # else
        assert len(internal_buffer) == 4, "internal buffer of authur1 not 4 byte long"
        accumulator = bytearray(
                inner_authur1(
                    # accumulator
                    int.from_bytes(accumulator, byteorder='big', signed=False)
                    # XOR
                    ^
                    # internal_buffer
                    int.from_bytes(internal_buffer, byteorder='big', signed=False)
                    )
                .to_bytes(length=2**16, byteorder="big", signed=False)
                )
    # finished loading input bytes into the hash, fill with padding and do it one last time
    while not len(internal_buffer) == 4:
        internal_buffer.append(PADDING)
    assert len(internal_buffer) == 4, "internal buffer of authur1 not 4 byte long"
    # same as above, one last time
    accumulator = bytearray(
            inner_authur1(
                # accumulator
                int.from_bytes(accumulator, byteorder='big', signed=False)
                # XOR
                ^
                # internal_buffer
                int.from_bytes(internal_buffer, byteorder='big', signed=False)
                )
                .to_bytes(length=2**16, byteorder="big", signed=False)
            )

    return accumulator

def main():
    parser = argparse.ArgumentParser(prog="authur1 authentication hash", description='Implementation and attack for the custom authur1 hash')
    parser.add_argument('-i', '--hash', type=str,
                    help='an input that should be hashed')
    args = parser.parse_args()

    if args.hash:
        my_bytes: bytearray = bytearray(str.encode(args.hash))
        hashed = authur1(my_bytes)
        print("hash for \"%s\" is:\n%s" % (args.hash, hashed.hex()))
    parser.print_help()

if __name__ == "__main__":
    main()
