#!/usr/bin/env python3
"""
Perform RSA-OAEP

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/plexcryptool/src/branch/master/plexcryptool/trash-hash.py
"""

import hashlib
from math import floor

# the given key in the assignment
GIVEN_PUB_KEY = (0xAF5466C26A6B662AC98C06023501C9DF6036B065BD1F6804B1FC86307718DA4048211FD68A06917DE6F81DC018DCAF84B38AB77A6538BA2FE6664D3FB81E4A0886BBCDAB071AD6823FE20DF1CD67D33FB6CC5DA519F69B11F3D48534074A83F03A5A9545427720A30A27432E94970155A026572E358072023061AF65A2A18E85, 0x10001)

SEED_LENGTH = 8 # bytes

def mgf1(seed: bytearray, length: int, hash_func=hashlib.sha256) -> bytearray:
    """
    Code stolen from wikipedia
    """
    hLen = hash_func().digest_size
    # https://www.ietf.org/rfc/rfc2437.txt
    # 1.If l > 2^32(hLen), output "mask too long" and stop.
    if length > (hLen << 32):
        raise ValueError("mask too long")
    # 2.Let T  be the empty octet string.
    T = b""
    # 3.For counter from 0 to \lceil{l / hLen}\rceil-1, do the following:
    # Note: \lceil{l / hLen}\rceil-1 is the number of iterations needed,
    #       but it's easier to check if we have reached the desired length.
    counter = 0
    while len(T) < length:
        # a.Convert counter to an octet string C of length 4 with the primitive I2OSP: C = I2OSP (counter, 4)
        C = int.to_bytes(counter, 4, 'big')
        # b.Concatenate the hash of the seed Z and C to the octet string T: T = T || Hash (Z || C)
        T += hash_func(seed + C).digest()
        counter += 1
    # 4.Output the leading l octets of T as the octet string mask.
    return bytearray(T[:length])

def byte_xor(ba1: bytearray, ba2: bytearray):
    a = int(ba1.hex(), 16)
    b = int(ba2.hex(), 16)
    c = a^b
    if c.bit_length() / 8 == floor(c.bit_length() / 8):
        clen = floor(c.bit_length() / 8)
    else:
        clen = floor(c.bit_length() / 8) + 1
    return bytearray(c.to_bytes(clen, 'big'))


def rsa_oaep_inner(seed: bytearray, datablock: bytearray) -> tuple[bytearray, bytearray, bytearray]:
    mgf_seed = mgf1(seed, len(seed))
    masked_db = byte_xor(mgf_seed, datablock)
    print(masked_db.hex())

def test_rsa_oaep_inner():
    seed: bytearray = bytearray.fromhex("aa1122fe0815beef")
    db: bytearray = bytearray.fromhex("""
    00000000000000000000000000000000000000000000000000000000
    00000000000000000000000000000000000000000000000000000000
    00000000000000000000000000000000000000000000000000000000
    00000000000000000000000000000000000001466f6f626172203132
    33343536373839
    """)
    print("seed\t%s" % seed.hex())
    print("db\t%s" % db.hex())

    result = rsa_oaep_inner(seed, db)

    GIVEN_MASK_FOR_DB = bytearray.fromhex("""
    ea600669f6f16b3a2ad05d4b6d9b23911c8cc432fddd8d34a68d88af
    3d787b7eebf6cd1b720812086758ce56e24ab819ccd8fb5eedb1cae9
    f6f895667d7f89d0454b828777ecabc040a649c8956e78ec1c721370
    663065cbc343deabad9eb6f2aceab6bfed5beb232cc55413bfffa06e
    68627d7ec3ded5
    """)
    GIVEN_MASKED_DB = bytearray.fromhex("""
    ea600669f6f16b3a2ad05d4b6d9b23911c8cc432fddd8d34a68d88af
    3d787b7eebf6cd1b720812086758ce56e24ab819ccd8fb5eedb1cae9
    f6f895667d7f89d0454b828777ecabc040a649c8956e78ec1c721370
    663065cbc343deabad9eb6f2aceab6bfed5bea6543aa3672cddf915c
    5b564848f4e6ec
    """)
    GIVEN_MASK_FOR_SEED = bytearray.fromhex("713162084a4e0e6d ")
    GIVEN_MASKED_SEED = bytearray.fromhex("db2040f6425bb082")
    assert result[0] == GIVEN_MASKED_SEED, "is %s" % result[0].hex()
    assert result[1] == GIVEN_MASKED_DB, "is %s" % result[1].hex()

if __name__ == "__main__":
    test_rsa_oaep_inner()
