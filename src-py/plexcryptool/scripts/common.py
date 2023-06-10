"""
Common tools that may be used by many

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/plexcryptool/src/branch/master/plexcryptool/trash-hash.py
"""

from math import floor
from typing import Literal

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

def int_to_bytearray(n: int, size: int|None = None, endianness = 'big') -> bytearray:
    """
    convert an integer to a bytearray

    you can specify a size if you want, if the number is too big an Exception will be raised.
    size is in bytes
    """
    brepr: bytes = n.to_bytes(calc_byte_len(n), endianness)
    ba: bytearray = bytearray(brepr)
    if size is not None and len(ba) > size:
        raise Exception("Number is larger than requested")
    while size is not None and size > len(ba):
        ba: bytearray = bytearray(1) + ba

    return ba

def ba_to_int(ba: bytearray) -> int:
    """
    convert bytearray to int
    """
    return int(ba.hex(), 16)
