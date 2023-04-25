#!/usr/bin/env python3
"""
A custom authentication hash function aswell as an automated extension attack for it.

Since this (auth) hash did not have a name before, I gave it the name 'authur1'

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/python-dhbw/src/branch/master/src/authur1.py
"""
import argparse
import random

# FIXME make proper pyi Implementation for the rust part
# only used for bit rotation
# your editor might complain here, because it can only find a pyi file with type annotations.
# rest assured, you just need to compile the rust part with maturin develop and you will be fine.
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

    # plexcryptool.binary uses u32 for shifting
    output: int = input ^ (binary.rotl32(input, SHIFT_LENGTH))

    assert output.bit_length() <= 32, "output length is <= 32: %d" % output.bit_length()

    return output

def authur1(input: bytearray, verbose: bool = False) -> bytearray:
    if verbose:
        print("input: %s" % input)
    internal_buffer: bytearray = bytearray()
    accumulator: bytearray = DEFINED_INITIAL
    for in_byte in input:
        if verbose:
            print("current in_byte: %s" % chr(in_byte))
            print("current buffer: %s" % internal_buffer.hex())
        if len(internal_buffer) < 4:
            if verbose:
                print("loading buffer")
            internal_buffer.append(in_byte)
            continue
        # else
        assert len(internal_buffer) == 4, "internal buffer of authur1 not 4 byte long"
        accuint: int = int.from_bytes(accumulator)
        accuint: int = inner_authur1(accuint ^ int.from_bytes(internal_buffer))
        accumulator: bytearray = bytearray(accuint.to_bytes(4))
        internal_buffer.clear()
        assert len(internal_buffer) == 0
        internal_buffer.append(in_byte)
        assert len(accumulator) == 4, "accumulator too long: %d bytes" % len(accumulator)
    if verbose:
        print("internal state after the bytes were read: %s" % accumulator.hex())
    # finished loading input bytes into the hash, fill with padding and do it one last time
    if verbose:
        print("buffer pre last fill: %s" % internal_buffer.hex())
    while len(internal_buffer) < 4:
        internal_buffer.append(PADDING)
    if verbose:
        print("buffer after last fill: %s" % internal_buffer.hex())
    assert len(internal_buffer) == 4, "internal buffer of authur1 not 4 byte long"
    # same as above, one last time
    assert len(accumulator) == 4, "accumulator too long: %d bytes" % len(accumulator)
    accuint: int = int.from_bytes(accumulator)
    accuint: int = inner_authur1(accuint ^ int.from_bytes(internal_buffer))
    accumulator: bytearray = bytearray(accuint.to_bytes(4))

    assert len(accumulator) == 4, "accumulator too long: %d bytes" % len(accumulator)
    if verbose:
        print("returning state: %s" % accumulator.hex())
    # now Q the accumulator and return
    # if input = "" this step breaks things, just remove it.
    if len(input) != 0:
        accuint: int = int.from_bytes(accumulator)
        accuint: int = inner_authur1(accuint)
        accumulator: bytearray = bytearray(accuint.to_bytes(4))
    return accumulator

def test():
    init: int = int.from_bytes(DEFINED_INITIAL)
    a: int = inner_authur1(init)
    b: int = inner_authur1(a)
    c: int = inner_authur1(b)
    assert a == 0xded7e2d2, "Q(S0) returns wrong value: %s" % hex(a)
    assert b == 0x1b725f7d, "Q(Q(S0)) returns wrong value: %s" % hex(b)
    assert c == 0xa5886999, "Q(Q(Q(S0))) returns wrong value: %s" % hex(c)

    print("Q aka inner_authur1 passed the test")

    ha: bytearray = authur1(bytearray(0))
    hb: bytearray = authur1(bytearray(b'A'))
    hc: bytearray = authur1(bytearray(b'AB'))
    hd: bytearray = authur1(bytearray(b'ABC'))
    he: bytearray = authur1(bytearray(b'ABCD'))
    hf: bytearray = authur1(bytearray(b'ABCDE'))
    assert int.from_bytes(ha) == 0xded7e2d2, "H(\"\") returns wrong value: %s" % ha.hex()
    assert int.from_bytes(hb) == 0x5d725f7f, "H(\"A\") returns wrong value: %s" % hb.hex()
    assert int.from_bytes(hc) == 0x5f3b5f7f, "H(\"AB\") returns wrong value: %s" % hc.hex()
    assert int.from_bytes(hd) == 0x5f39137f, "H(\"ABC\") returns wrong value: %s" % hd.hex()
    assert int.from_bytes(he) == 0x5f391128, "H(\"ABCD\") returns wrong value: %s" % he.hex()
    assert int.from_bytes(hf) == 0x2f69af58, "H(\"ABCDE\") returns wrong value: %s" % hf.hex()

    print("H aka authur1 passed the test")
    print("All tests passed!")

def keyed_hash(message: bytearray, key: bytearray) -> bytearray:
    assert len(key) == 16, "key is not 16 Byte long: %s" % len(key)
    input: bytearray = key + message
    mic: bytearray = authur1(input)
    return mic

def main():
    parser = argparse.ArgumentParser(prog="authur1 authentication hash", description='Implementation and attack for the custom authur1 hash. Don\'t actually use this hash!')
    parser.add_argument('-i', '--hash', type=str,
                    help='an input that should be hashed')
    parser.add_argument('-k', '--key', type=str,
                    help='an key that should be used with auth mode')
    parser.add_argument('-t', '--test', action="store_true",
                    help='perform tests')
    parser.add_argument('-v', '--verbose', action="store_true",
                    help='print many things')
    parser.add_argument('-a', '--auth', action="store_true",
                    help='generate a message integrity code (mic), needs a value to be hashed. If no key is specified, a random key will be generated.')
    args = parser.parse_args()

    if args.test:
        test()
        exit()
    elif args.auth and args.hash:
        if args.key:
            key: bytearray = bytearray(args.key.encode())
            if len(key) < 16:
                print("Your key is not long enough and will be padded with random bytes.")
                key.extend(random.randbytes(16 - len(key)))
            elif len(key) > 16:
                print("Your key is too long!")
                exit()
        else:
            key: bytearray = bytearray(random.randbytes(16))
        my_bytes: bytearray = bytearray(str.encode(args.hash))
        mic: bytearray = keyed_hash(my_bytes, key)
        print("KEY (str): %s" % key.decode(errors="replace"))
        print("KEY (hex): %s" % key.hex())
        print("MIC: %s" % mic.hex())
        exit()
    elif args.hash:
        my_bytes: bytearray = bytearray(str.encode(args.hash))
        hashed: bytearray = authur1(my_bytes, args.verbose)
        print("hash for \"%s\" is:\n%s" % (args.hash, hashed.hex()))
        exit()
    parser.print_help()

if __name__ == "__main__":
    main()
