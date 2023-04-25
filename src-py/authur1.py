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

from plexcryptool import binary

# constants for authur1
SHIFT_LENGTH = 17
DEFINED_INITIAL = bytearray([0x52, 0x4f, 0x46, 0x4c])
PADDING = 0xff

# constants for Circular shifting
# constant value defined in limits.h, it's 8 (bit) on my machine, on yours probably too.
CHAR_BIT = 8
# python is being a dynamic dumbass, do a 32 bit shift ~ 4 byte
VALUE_SIZE = 4

def inner_authur1(input: int) -> int:
    """
    passes all tests
    """
    assert input.bit_length() <= 32, "input length is <= 32: %d" % input.bit_length()
    output: int

    # plexcryptool.binary uses u32 for shifting
    output = input ^ (binary.rotl32(input, SHIFT_LENGTH))

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
        accuint = int.from_bytes(accumulator)
        accuint = inner_authur1(accuint ^ int.from_bytes(internal_buffer))
        accumulator = bytearray(accuint.to_bytes(4))
        assert len(accumulator) == 4, "accumulator too long: %d bytes" % len(accumulator)
    # finished loading input bytes into the hash, fill with padding and do it one last time
    while not len(internal_buffer) == 4:
        internal_buffer.append(PADDING)
    assert len(internal_buffer) == 4, "internal buffer of authur1 not 4 byte long"
    # same as above, one last time
    assert len(accumulator) == 4, "accumulator too long: %d bytes" % len(accumulator)
    accuint = int.from_bytes(accumulator)
    accuint = inner_authur1(accuint ^ int.from_bytes(internal_buffer))
    accumulator = bytearray(accuint.to_bytes(4))

    assert len(accumulator) == 4, "accumulator too long: %d bytes" % len(accumulator)
    return accumulator

def test():
    init = int.from_bytes(DEFINED_INITIAL)
    a = inner_authur1(init)
    b = inner_authur1(a)
    c = inner_authur1(b)
    assert a == 0xded7e2d2, "Q(S0) returns wrong value: %s" % hex(a)
    assert b == 0x1b725f7d, "Q(Q(S0)) returns wrong value: %s" % hex(b)
    assert c == 0xa5886999, "Q(Q(Q(S0))) returns wrong value: %s" % hex(c)

    print("Q aka inner_authur1 passed the test")

    ha = authur1(bytearray(0))
    hb = authur1(bytearray(b'A'))
    hc = authur1(bytearray(b'AB'))
    hd = authur1(bytearray(b'ABC'))
    he = authur1(bytearray(b'ABCD'))
    hf = authur1(bytearray(b'ABCDE'))
    assert int.from_bytes(ha) == 0xded7e2d2, "H(\"\") returns wrong value: %s" % ha.hex()
    assert int.from_bytes(hb) == 0x5d725f7f, "H(\"A\") returns wrong value: %s" % hb.hex()
    assert int.from_bytes(hc) == 0x5f3b5f7f, "H(\"AB\") returns wrong value: %s" % hc.hex()
    assert int.from_bytes(hd) == 0x5f39137f, "H(\"ABC\") returns wrong value: %s" % hd.hex()
    assert int.from_bytes(he) == 0x5f391128, "H(\"ABCD\") returns wrong value: %s" % he.hex()
    assert int.from_bytes(hf) == 0x2f69af58, "H(\"ABCDE\") returns wrong value: %s" % hf.hex()

    print("H aka authur1 passed the test")
    print("All tests passed!")

def main():
    parser = argparse.ArgumentParser(prog="authur1 authentication hash", description='Implementation and attack for the custom authur1 hash')
    parser.add_argument('-i', '--hash', type=str,
                    help='an input that should be hashed')
    parser.add_argument('-t', '--test', action="store_true",
                    help='perform tests')
    args = parser.parse_args()

    if args.hash:
        my_bytes: bytearray = bytearray(str.encode(args.hash))
        hashed = authur1(my_bytes)
        print("hash for \"%s\" is:\n%s" % (args.hash, hashed.hex()))
        exit()
    elif args.test:
        test()
        exit()
    parser.print_help()

if __name__ == "__main__":
    main()
