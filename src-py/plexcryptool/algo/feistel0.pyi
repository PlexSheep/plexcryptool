"""
# basic implementation of a feistel network

This module implements a feistel network according to an exercise at DHBW Mannheim.
For demonstration purposes only, do not use this in a secure environment.

___
@Author:     Christoph J. Scherr <software@cscherr.de>
@License:    MIT
@Source:     <https://git.cscherr.de/PlexSheep/plexcryptool/>
"""
def inner(input: int, key: int, verbose: bool) -> int:
    """
    the inner function of the feistel network

    takes input, scrambles it by using two s and p boxes, 
    then xors with the key.

    :param input unsigned 16 bit int
    :param key unsigned 16 bit int
    :param verbose print steps
    """
    ...

def encrypt(plaintext: int, keys: list[int], verbose: bool) -> int:
    """
    encrypt using the feistel0 network

    performes some rounds of the feistelnetwork to encrypt the input.
    This will only encrypt a single block.

    DO NOT USE THIS FOR ACTUAL ENCRYPTION!

    :param plaintext unsigned 32 bit int
    :param keys vec of the round keys, usually 3 diffrent ones.
    :param verbose print steps
    """
    ...

def decrypt(ciphertext: int, keys: list[int], verbose: bool) -> int:
    """
    decrypt using the feistel0 network

    performs encryption backwards more or less

    DO NOT USE THIS FOR ACTUAL ENCRYPTION!

    :param ciphertext unsigned 32 bit int
    :param keys vec of the round keys, usually 3 diffrent ones.
    :param verbose print steps
    """
    ...

def sbox(index: int) -> int:
    """
    returns the value of the sbox at index.

    :param index range 0 - 15
    """
