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

DEFINED_INITIAL = bytearray(bytes([0x52, 0x4f, 0x46, 0x4c]))

def Q(input: bytearray) -> bytearray:
    assert len(input) == 4 # needs to be 32 bit


    return input

def H(input: bytearray) -> bytearray:


    return input

def main():
    pass

if __name__ == "__main__":
    main()
