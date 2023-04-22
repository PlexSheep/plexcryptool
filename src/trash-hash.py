#!/usr/bin/env python3
"""
Dirty hash we covered in an excercise for Week 16 of 2023 in cryptography

author: Christoph J. Scherr <software@cscherr.de>
version control at: https://git.cscherr.de/PlexSheep/python-dhbw/src/branch/master/src/trash-hash.py
License: MIT
"""
import math
import random

DEFINED_INITIAL =   bytearray(b'\xa5\xa5\xa5\xa5\x5a\x5a\x5a\x5a\x55\x55\x55\x55\xaa\xaa\xaa\xaa')

def trash_hash(input: bytearray) -> bytearray:
    #print("original len is %s" % len(input))
    # extend with 0xff if length is not multiple of 16
    while len(input) % 16 != 0:
        input.append(0xff)

    # set n
    n: int = math.ceil(len(input)/16)
    #print("len is %s" % len(input))
    #print("n is %s" % n)

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
    payload_a = bytearray(b"AAAA")
    # works, but is too cheap
    #payload_b = bytearray(b"AAAA\xff\xff")
    payload_b = bytearray(b'\xb2\xef\x82t<~<\xbe\x8d\xca\xe2\t\xdc7E\x10')
    print("a: %s\nb: %s" % (trash_hash(payload_a).hex(), trash_hash(payload_b).hex()))
    print("identical: %s" % test_collision(payload_a, payload_b))

def test_collision(a: bytearray, b: bytearray) -> bool:
    return trash_hash(a) == trash_hash(b)

def test_against_hash(input: bytearray, target_hash: bytearray) -> bool:
    print("given input:\t%s" % input.hex())
    print("given input (repr):\t%s" % input.decode(errors="ignore"))
    hashed = trash_hash(input)
    print("hashed variant:\t%s" % hashed.hex())
    print("should be:\t%s" % THE_HASH.hex())
    return trash_hash(input) == target_hash

# the hash we want to find in the preimage attacks
# comes from 'AAAA'
#THE_HASH_ORIGIN = bytearray(b'AAAA')
#THE_HASH = bytearray(b'\xe4\xe4\xe4\xe4\xa5\xa5\xa5\xa5\xaa\xaa\xaa\xaa\x55\x55\x55\x55')
# any custom hash you want to find a collision for:
# needs to be 16 bytes long
# TODO fill with padding if not long enough
THE_HASH_ORIGIN = bytearray(b'1249239473289754927513214 21421 4124 214 21')
THE_HASH = trash_hash(THE_HASH_ORIGIN)

def first_preimage():
    print("Trying to find a message that produces %s" % THE_HASH.hex())
    target = bytearray(b'\00' * 16)
    input = bytearray(b'\00' * 16)
    between = bytearray(16)
    # this is an arbituary target
    # should work for anything
    target[0] = ord('A')

    for i in range(0, 16):
        
        between[i] = THE_HASH[i] ^ DEFINED_INITIAL[i]
        #print("%d:\tbtw %s\ttar %s\thash %s\n\tini %s\tinp %s" % (i, between.hex(), target.hex(), THE_HASH.hex(), DEFINED_INITIAL.hex(), input.hex()))
        input[i] = target[i] ^ between[i]
        #print("%d:\tbtw %s\ttar %s\thash %s\n\tini %s\tinp %s" % (i, between.hex(), target.hex(), THE_HASH.hex(), DEFINED_INITIAL.hex(), input.hex()))
        assert THE_HASH[i] == DEFINED_INITIAL[i] ^ between[i], "xor circle is broken: %s vs %s" % (hex(THE_HASH[i]), hex(input[i] ^ between[i]))
    
    input: bytearray = bytearray(bytes(a ^ b for a, b in zip(input, target)))
    print("for input '%s':\n %s" % (input, test_against_hash(input, THE_HASH)))

    assert test_collision(input, THE_HASH_ORIGIN), "not the same thing: %s and %s" % (
            trash_hash(input).hex(), trash_hash(THE_HASH_ORIGIN).hex())

def main():
    first_preimage()

def bruteForce() -> bool:
    payload_a = bytearray(b"AAAA")
    foundCollision = False
    while not foundCollision:
        current = bytearray(random.randbytes(16))
        foundCollision = test_collision(payload_a, current)
        if random.randint(1, 65535) % 65535 == 0:
            print(current)
    print("found one!") 
    print(current)
    return True

if __name__ == "__main__":
    main()
