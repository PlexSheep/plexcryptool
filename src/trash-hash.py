#!/usr/bin/env python3
"""
Dirty hash we were covered in an excercise for Week 16 of 2023 in cryptography
"""
import math
import random

DEFINED_INITIAL = bytearray(b'\xa5\xa5\xa5\xa5\x5a\x5a\x5a\x55\x55\x55\xaa\xaa\xaa')

def trash_hash(input: bytearray) -> bytearray:
    print("original len is %s" % len(input))
    # extend with 0xff if length is not multiple of 16
    while len(input) % 16 != 0:
        input.append(0xff)

    # set n
    n: int = math.ceil(len(input)/16)
    print("len is %s" % len(input))
    print("n is %s" % n)

    # cut input into blocks with size 16
    blocks = [bytearray(16)] * n  # initializes with 0x00s
    # print the empty blocks
    #for block in blocks:
    #    print("block: %s" % block.hex())
    #print('='*80)

    for i in range(0, n):
        blocks[i] = input[i*16:i*16 + 16]

    # print the filled blocks
    #for block in blocks:
    #    print("block: %s" % block.hex())

    # initilaize accumulator A_0 with the following constant values:
    A = DEFINED_INITIAL

    # iterate over blocks
    for index, block in enumerate(blocks):
        if index == 0:
            pass
        thing = bytes(by0 ^ by1 for by0, by1 in zip(A, block))
        A = bytearray(thing)

    return A

def use():
    some_bytes = bytearray(b'AAAA');
    print("hashed: %s" % some_bytes.hex())
    print('='*80)
    hashed = trash_hash(some_bytes)
    print('='*80)
    print("hashed: %s" % hashed.hex())

def test_collision(a: bytearray, b: bytearray) -> bool:
    return trash_hash(a) == trash_hash(b)

def main():
    payload_a = bytearray(b"AAAA")
    payload_b = bytearray(random.randbytes(122))
    print("a: %s\tb: %s" % (trash_hash(payload_a).hex(), trash_hash(payload_b).hex()))

if __name__ == "__main__":
    main()
