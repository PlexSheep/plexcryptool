"""
Common tools that may be used by many

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/plexcryptool/src/branch/master/plexcryptool/trash-hash.py
"""

from math import floor

def byte_xor(ba0: bytearray, ba1: bytearray) -> bytearray:
    """
    helper function for bytewise xor
    """
    ba2: bytearray = bytearray(0)
    for (b0, b1) in zip(ba0, ba1):
        ba2.append((b0 ^ b1))
    #print("xored:\n%s" % ba2.hex())
    return ba2

def calc_byte_len(n: int) -> int:
    """
    helper function to calculate the length in bytes
    """
    len = n.bit_length() / 8
    if len > floor(len):
        return 1 + floor(len)
    else: 
        return floor(len)
