#!/usr/bin/env python3
"""
A custom authentication hash function aswell as an automated extension attack for it.

Since this (auth) hash did not have a name before, I gave it the name 'authur1'

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/plexcryptool/src/branch/master/plexcryptool/authur1.py
"""
import argparse
import random
import sys
import time
from types import NoneType

# FIXME make proper pyi Implementation for the rust part
# only used for bit rotation
# your editor might complain here, because it can only find a pyi file with type annotations.
# rest assured, you just need to compile the rust part with maturin develop and you will be fine.
from plexcryptool import binary

# constants for authur1
SHIFT_LENGTH = 17
DEFINED_INITIAL = bytearray([0x52, 0x4f, 0x46, 0x4c])
PADDING = 0xff
EXT_TEST_STR: str = "ef"

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

def inner_authur1_optimized(input: int) -> int:
    # plexcryptool.binary uses u32 for shifting
    output: int = input ^ (binary.rotl32(input, SHIFT_LENGTH))
    return output

def authur1(input: bytearray, 
            verbose: bool = False, 
            first_state: bytearray = DEFINED_INITIAL, 
            return_last_state: bool = False) -> bytearray:
    internal_buffer: bytearray = bytearray()
    accumulator: bytearray = first_state
    if verbose:
        print("input: %s" % input.hex())
        print("first internal state: %s" % accumulator.hex())
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
        accumulator: bytearray = bytearray(inner_authur1_optimized(int.from_bytes(accumulator) ^ int.from_bytes(internal_buffer)).to_bytes(4))
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
    accumulator: bytearray = bytearray(inner_authur1_optimized(int.from_bytes(accumulator) ^ int.from_bytes(internal_buffer)).to_bytes(4))

    assert len(accumulator) == 4, "accumulator too long: %d bytes" % len(accumulator)
    if verbose:
        print("last internal state: %s" % accumulator.hex())
    # now Q the accumulator and return
    # if input = "" this step breaks things, just remove it.
    if len(input) != 0 and not return_last_state:
        accumulator: bytearray = bytearray(inner_authur1(int.from_bytes(accumulator)).to_bytes(4))
    return accumulator

def authur1_optimized(input: bytearray, first_state: bytearray = DEFINED_INITIAL) -> bytearray:
    internal_buffer: bytearray = bytearray()
    accumulator: bytearray = first_state
    for in_byte in input:
        if len(internal_buffer) < 4:
            internal_buffer.append(in_byte)
            continue
        accumulator: bytearray = bytearray(inner_authur1_optimized(int.from_bytes(accumulator) ^ int.from_bytes(internal_buffer)).to_bytes(4))
        internal_buffer.clear()
        internal_buffer.append(in_byte)
    while len(internal_buffer) < 4:
        internal_buffer.append(PADDING)
    accumulator: bytearray = bytearray(inner_authur1_optimized(int.from_bytes(accumulator) ^ int.from_bytes(internal_buffer)).to_bytes(4))

    # ignore the case of ""
    #if len(input) != 0:
    accuint: int = int.from_bytes(accumulator)
    accuint: int = inner_authur1(accuint)
    accumulator: bytearray = bytearray(accuint.to_bytes(4))
    return accumulator

def keyed_hash(message: bytearray, key: bytearray, verbose: bool = False, return_last_state: bool = False) -> bytearray:
    assert len(key) == 16, "key is not 16 Byte long: %s" % len(key)
    # prepend only
    input: bytearray = key + message
    mic: bytearray = authur1(input, verbose, return_last_state=return_last_state)
    return mic

def extension_attack(valid_pairs: list[tuple[bytearray, bytearray]], extending_message: bytearray, last_internal_state: int | NoneType = None) -> tuple[bytearray,bytearray, bytearray]:
    """
    Extension attack against keyed hash of authur1

    Procedure:
    valid_pairs parsed in main()

    > Consider a case where the length of K || m together with internal 
    > length fields is a multiple of the internal block size.

    > The output value now corresponds tightly with the internal state of H 
    > after the digesting the message

    > This gives rise to a so-called extension attack

    > An attacker observes a message that is the correct length and the attacker 
    > also knows the legitimate (public) MIC of that message

    > They then infer the internal state just before emission of the MIC

    > When the internal state is known at this point in time, the attacker can 
    > simply reset their H to that internal state

    > Then append arbitrary malicious data and compute a valid MIC
    If I understand correctly, this is a new MIC, not one of the valid MICs we already know.

    > Knowledge of the key is not necessary at all, yet valid forgeries can be 
    > produced efficiently by the adversary

    use any convenient message that has low length?
    fill it with padding?
    -> internal block length is 4 byte
    -> we don't have one given in the exercise
    reverse the last state before finalizing in the hash
    -> we have a valid internal state and can continue to append our own whatever to it?
    -> But the mic at the end should change, right?

    (the exercise said the Key K has length 16, which is really handy, so i don't need to calculate for that.)
    (16 % 4 is 0)
    """
    # find a valid message
    target_pair = None
    for msg, mic in valid_pairs:
        print("%s has length %s" % (msg, len(msg)))
        if len(msg) % 4 == 0:
            # we have a message of the right length!
            target_pair = (msg, mic, msg)
        else:
            # for now, just use the first one?
            original_msg = msg
            while len(msg) % 4 != 0:
                msg.append(PADDING)
            target_pair = (msg, mic, original_msg)
    if target_pair is None:
        print("The given originals were not sufficient to perform an extension attack.\n"+
              "We need a message, which has a length that is a multiple of 16 (Bytes).")
        raise Exception("No fitting original value given")

    # now find the last internal state.
    # inner_authur1 cannot be reversed, so we need to brute force it
    # the key space is only 2**32, so it should be possible
    # we need to check everyone of these 2**32 against the valid hashes to find a working one

    KEY_SPACE = 2**32
    # only the last block
    last_block = target_pair[0][-4:]
    extension_msg = last_block
    full_msg = target_pair[0][:-4]
    # given mic for "abcdef" 0f6b8802
    looking_for = target_pair[1]
    print("=" * 120)
    print("extension_msg for bruteforce: %s" % extension_msg)
    print("looking for result: %s" % looking_for.hex())
    print("Bruteforcing the internal state, this might take a while")
    print("=" * 120)
    starttime = time.time()
    if last_internal_state is None:
        for i in range(0, KEY_SPACE):
            mic = authur1_optimized(extension_msg, bytearray(i.to_bytes(4)))
            if i % 0x1000 == 0:
                msg = "state %08x | hash %s | search %s | status %f%% | time %s" % (
                        i, 
                        mic.hex(), 
                        looking_for.hex(), 
                        i / KEY_SPACE,
                        time.time() - starttime
                        )
                sys.stdout.write('%s\r' % msg)
                sys.stdout.flush()
            if mic == looking_for:
                print("\n" + "=" * 120)
                print("FOUND THE THING")
                last_internal_state = i
                break
    print("IT IS %08x" % last_internal_state)
    print("=" * 120)
    forged_input: bytearray = extending_message
    print("Trying to forge a mic for an extended version with input:\n%08x\n(%s)" % (
        int.from_bytes(forged_input),
        forged_input.decode(errors="replace")
        ))
    print("=" * 120)

    hacked_mic = authur1_optimized(forged_input, first_state=bytearray(last_internal_state.to_bytes(4)))
    print("Hacked MIC: %s" % hacked_mic.hex())
    full_msg.extend(forged_input)
    return (forged_input, hacked_mic, full_msg)

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

    #test_reverse_inner_authur1()
    test_extension_attack()

    print("All tests passed!")

def test_extension_attack():
    """
    Test the attack against a known last state
    """
    # this key produces a mic that is fast to bruteforce with my code
    # (in combination with "AAAAaa" as input ofc)
    # test with these values, they are some of the first the brute force algo tries
    #key = bytearray(0x289488ae6d71c82da1502c0130ec04e0.to_bytes(16))
    #message = bytearray(b'AAAAaa')
    key = bytearray(0x289488ae6d71c82da1502c0130ec04e0.to_bytes(16))
    message = bytearray(b'AAAAaa')
    # we need to bruteforce this, skip for the test
    mic = keyed_hash(message, key, False)
    print("testing against mic: %s" % mic.hex())
    #last_internal_state = keyed_hash(message, key, return_last_state=True)
    forged_data = extension_attack([(message, mic)])
    print("extension attack returned the following:\n%s\n%s" % (forged_data[0].decode(errors="replace"), forged_data[1].hex()))
    print("=" * 120)
    message = message[:4]
    message.extend(forged_data[0])
    print("generating real mic for\n%x\n(%s)" % (int.from_bytes(message), message.decode(errors="replace")))
    validated_mic = keyed_hash(message, key, False)
    assert validated_mic == forged_data[1], "forged mic %s is not valid. (%s)" % ( forged_data[1].hex(), validated_mic.hex() )
    print("Manual extension attack with known last internal state works (%s == %s)" % (forged_data[1].hex(), validated_mic.hex()))
    print("=" * 120)
    print("Forged a valid MIC for the following message:\n\n%s\n\nMIC: %s" % (message.decode(errors="replace"), validated_mic.hex()))
    print("=" * 120)
    
def main():
    parser = argparse.ArgumentParser(prog="authur1", description='Implementation and attack for the custom authur1 hash. Don\'t actually use this hash!')
    parser.add_argument('-i', '--hash', type=str, metavar="MSG",
                    help='an input that should be hashed')
    parser.add_argument('-k', '--key', type=str,
                    help='an key that should be used with auth mode')
    parser.add_argument('-t', '--test', action="store_true",
                    help='perform tests')
    parser.add_argument('-v', '--verbose', action="store_true",
                    help='print many things')
    parser.add_argument('-e', '--extension-attack', metavar="ORIGINALS", type=str,
                        help='perform an extension attack, this option requires known mics in the form: "msg1:deadbeed,msg2:abababab,msg3:ecbadf,..."')
    parser.add_argument('-s', '--last-state-attack', metavar="HEX", type=str,
                        help='don\'t bruteforce the internal state and use a given one.')
    parser.add_argument('-m', '--ext-message', metavar="MSG", type=str,
                        help='the message that is appended to the valid one')
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
                exit(1)
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
    elif args.extension_attack and args.ext_message:
        msg_bytes = bytearray(args.ext_message.encode())
        original_strs: list = args.extension_attack.split(",")
        # will store our processed given messages and mics as tuples of bytearrays
        valid_pairs: list = []   
        for pair in original_strs:
            try:
                (msg, mic) = pair.split(":")
                assert (len(mic) == 8), "given mic '%s' formatted incorrectly" % mic
                mic_int: int = int(mic, 16)
                valid_pairs.append((bytearray(msg.encode()),bytearray(mic_int.to_bytes(4))))
            except Exception as e:
                print(e)
                print("given pair '%s' formatted incorrectly" % pair)
                exit(1)

        if args.last_state_attack:
            state = int(args.last_state_attack, 16)
            forged_data = extension_attack(valid_pairs, msg_bytes, state)

        else:
            forged_data = extension_attack(valid_pairs, msg_bytes)
        print("Forged a valid delta:")
        print("=" * 20 + "BEGIN FORGED AUTHENTICATED TEXT" + "=" * 20)
        print(forged_data[2].decode())
        print("=" * 20 + "END FORGED AUTHENTICATED TEXT" + "=" * 22)
        print("MIC: %s" % forged_data[1].hex())
        exit()

    parser.print_help()

if __name__ == "__main__":
    main()
