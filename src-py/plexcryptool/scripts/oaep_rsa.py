#!/usr/bin/env python3
"""
Perform RSA-OAEP

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/plexcryptool/src/branch/master/plexcryptool/trash-hash.py
"""

import argparse
import hashlib
import random
from math import floor

# the given key in the assignment
GIVEN_PUB_KEY = (0xAF5466C26A6B662AC98C06023501C9DF6036B065BD1F6804B1FC86307718DA4048211FD68A06917DE6F81DC018DCAF84B38AB77A6538BA2FE6664D3FB81E4A0886BBCDAB071AD6823FE20DF1CD67D33FB6CC5DA519F69B11F3D48534074A83F03A5A9545427720A30A27432E94970155A026572E358072023061AF65A2A18E85, 0x10001)
GIVEN_MASK_FOR_DB = bytearray.fromhex("""
ea600669f6f16b3a2ad05d4b6d9b23911c8cc432fddd8d34a68d88af
3d787b7eebf6cd1b720812086758ce56e24ab819ccd8fb5eedb1cae9
f6f895667d7f89d0454b828777ecabc040a649c8956e78ec1c721370
663065cbc343deabad9eb6f2aceab6bfed5beb232cc55413bfffa06e
68627d7ec3ded5
""")
GIVEN_DB = bytearray.fromhex("""
00000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000001466f6f626172203132
33343536373839
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
GIVEN_SEED = 0xaa1122fe0815beef
GIVEN_SEED_BYTES = bytearray.fromhex("aa1122fe0815beef")
GIVEN_MSG = bytearray.fromhex("466f6f62617220313233343536373839")
GIVEN_OAEP = bytearray.fromhex("""
00db2040f6425bb082ea600669f6f16b3a2ad05d4b6d9b23911c8cc4
32fddd8d34a68d88af3d787b7eebf6cd1b720812086758ce56e24ab8
19ccd8fb5eedb1cae9f6f895667d7f89d0454b828777ecabc040a649
c8956e78ec1c721370663065cbc343deabad9eb6f2aceab6bfed5bea
6543aa3672cddf915c5b564848f4e6ec
""")
GIVEN_C = bytearray.fromhex("""
1b57819fa11340ac8b1843c87db7adb126daa8b6dde1feefd7af721c
ee8f46b6e2c361fc04ac055406a342187388b019dba0bc3f6503f267
b848f7cc86b29a3d0b32730ccf04c5a8a3e1255708cbc6a6a648015e
30f38b1c1c7aa9d2b0e67a775c7ad1cb72ff76c000af46e7cada3c3b
45b5f4d1ec8e0596928cc9b46ee2b53d
""")

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

def byte_xor(ba0: bytearray, ba1: bytearray):
    """
    helper function for bytewise xor
    """
    ba2: bytearray = bytearray(0)
    for (b0, b1) in zip(ba0, ba1):
        ba2.append((b0 ^ b1))
    #print("xored:\n%s" % ba2.hex())
    return ba2

def calclen(n: int) -> int:
    """
    helper function to calculate the length in bytes
    """
    len = n.bit_length() / 8
    if len > floor(len):
        return 1 + floor(len)
    else: 
        return floor(len)

def rsa_oaep_inner(seed: bytearray, block: bytearray, verbose: bool) -> tuple[bytearray, bytearray]:
    """
    inner function of rsa-oaep
    """
    assert type(seed) == bytearray
    mgf_seed = mgf1(seed, len(block))
    masked_db = byte_xor(mgf_seed, block)
    mask_seed = mgf1(masked_db, len(seed))
    masked_seed = byte_xor(seed, mask_seed)
    if verbose:
        print("mgf1(seed):\n%s" % mgf_seed.hex())
        print("mgf1(seed) ^ block:\n%s" % masked_db.hex())
        print("mgf1(mgf1(seed) ^ block):\n%s" % mask_seed.hex())
        print("mgf1(mgf1(seed) ^ block) ^ seed:\n%s" % masked_seed.hex())
    return (masked_seed, masked_db)

def test_rsa_oaep_inner():
    result = rsa_oaep_inner(GIVEN_SEED_BYTES, GIVEN_DB, True)

    assert result[0] == GIVEN_MASKED_SEED, "is\n%s\ninstead of\n%s" % (result[0].hex(), GIVEN_MASKED_SEED.hex())
    assert result[1] == GIVEN_MASKED_DB, "is\n%s\ninstead of\n%s" % (result[1].hex(), GIVEN_MASKED_DB.hex())

def rsa_oaep_noenc(ha: bytearray, m: bytearray, verbose: bool, seed: int = random.randint(0, 2**64 - 1)):
    """
    rsa-oaep without encryption
    """
    # generate a seed
    assert calclen(seed) == 8, "seed is wrong length: %d" % calclen(seed)
    l_seed: bytearray = bytearray(seed.to_bytes(calclen(seed), 'big'))
    # build the message
    block: bytearray = bytearray(0)
    assert len(block) == 0
    maxlen = calclen(GIVEN_PUB_KEY[0]) - 1 - len(l_seed)
    curlen = 0
    curlen += len(ha)
    curlen += len(m)
    block += ha
    block += bytearray(maxlen - curlen - 1)
    block += bytearray(0x01.to_bytes())
    block += m

    assert len(block) == maxlen
    if verbose:
        print("block:\n%s" % block.hex())
    assert type(l_seed) == bytearray
    # do the inner function
    result = rsa_oaep_inner(seed=l_seed, block=block, verbose=verbose)
    if verbose:
        print()
        print(result[0].hex())
        print(result[1].hex())
        print()
    return bytearray(1) + result[0] + result[1]

def test_rsa_oaep_noenc():
    r = rsa_oaep_noenc(bytearray(0), GIVEN_MSG, True, GIVEN_SEED)
    assert r == GIVEN_OAEP
    print(r.hex())

def test_rsa_oaep():
    r = rsa_oaep(bytearray(0), GIVEN_MSG, True, GIVEN_SEED)
    assert r == GIVEN_C
    print(r.hex())

def rsa_oaep(
        ha: bytearray, 
        m: bytearray, 
        verbose: bool, 
        seed: int = random.randint(0, 2**64 - 1)
        ) -> bytearray:
    """
    rsa-oaep with encryption
    """
    r: bytearray = rsa_oaep_noenc(ha, m, verbose, seed)
    ri = int.from_bytes(r, 'big')
    c = pow(ri, GIVEN_PUB_KEY[1], GIVEN_PUB_KEY[0])
    return bytearray(c.to_bytes(calclen(c), 'big'))

def main():
    parser = argparse.ArgumentParser(prog="oaep-rsa", description='A hacky Implementation of rsa-oaep')
    parser.add_argument('-m', '--message', type=str, metavar="MSG",
                    help='the message of the oaep')
    parser.add_argument('-rs', '--random-seed', action="store_true",
                    help='a random seed')
    parser.add_argument('-s', '--seed', type=int,
                    help='a custom seed')
    parser.add_argument('-a', '--hashed-data', type=str,
                    help='append some auth hashed stuff')
    parser.add_argument('-v', '--verbose', action="store_true",
                    help='append some auth hashed stuff')
    parser.add_argument('-t', '--test', action="store_true",
                    help='perform tests')
    args = parser.parse_args()

    if args.test:
        test_rsa_oaep_inner()
        test_rsa_oaep_noenc()
        test_rsa_oaep()
        exit()
    if args.hashed_data:
        ha = bytearray.fromhex(args.hashed_data)
    else:
        ha = bytearray(0)
    if args.message:
        m = bytearray.fromhex(args.message)
    else:
        m = bytearray.fromhex("466f6f62617220313233343536373839")
    if args.seed:
        seed: int = args.seed
        r = rsa_oaep(ha, m, args.verbose, seed)
    elif args.random_seed:
        seed = random.randint(0, 2**64 - 1)
        r = rsa_oaep(ha, m, args.verbose, seed)
    else:
        seed = GIVEN_SEED
        r = rsa_oaep(ha, m, args.verbose, seed)
    print("result:\n%s" % r.hex())

if __name__ == "__main__":
    main()
