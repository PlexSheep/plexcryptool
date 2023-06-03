"""
Rsa padding script for croptology lectures

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/plexcryptool/
"""

import argparse

import plexcryptool
import common

# predefined constants
P = 2**206 - 5
Q = 2**226 -5
N = P*Q
D = 0xaffe0815
CIPHERTEXTS: list[int] = [
    0x78766a52455329b486aaa414c3a029834a7e4b6ed87019dce4056f4d8999b137404d9ec4df28da201c9b0bc142deb1d86ff94d83becc,
    0x670b865216dfd0aacd5f7fa8802e704fa82f3fb9c7dbe3eb5a9ec308a1a2288648b15d5cc8ba2f54b245a972aea977932c9c84cf6422,
    0x61d5f2a4298bff3d6ebcd78830fb9181d97235623819eb7c60b92dcdf836a6cf731c60187e72f471c05d1c6eab216c3f6032af3c5370,
    0x3651009d02a0c72b9bc206c57d12277594d9eaad28bb3de5d661670b42f1cfafe688b9674e34d4ad79db898205417086e7e1877b9ef1,
    0x96e51d4675c6be5b14ec0cf2a9e9a9610a99d632723b3f1fcfc6b36806f5d74045f47622817cc35f6ffe9afe29f0aa236cbe12371651,
    ]

def padding_check(cihpertext: int, private_key: tuple[int, int], verbose: bool = False) -> bool:
    """
    Check the padding of a given cihpertext
    """
    decrypted: int = pow(cihpertext, private_key[0], private_key[1])
    # the ciphertexts seem to all be 54B long, so I have just hardcoded that
    cleartext: bytearray = common.int_to_bytearray(decrypted, 54)
    if verbose:
        print(f"original:\t{cihpertext:054x}")
        print(f"decrypted:\t{cleartext.hex()}")

    index: int = 0
    padding_active: bool = False
    for b in cleartext:

        if index == 0 and b != 0:
            # the first byte has to be 0x00
            if verbose:
                print(f"error:\t\tThe first byte is not 00: {b:02x}")
            return False
        elif index == 1:
            # the second byte is the padding type, we only accept 01 and 02 type padding
            if b == 1:
                padding_active = True
                pad_type = b
                if verbose:
                    print(f"type:\t\t01 deterministic padding")
            elif b == 2:
                padding_active = True
                pad_type = b
                if verbose:
                    print(f"type:\t\t02 nondeterministic padding")
            else:
                if verbose:
                    print(f"error:\t\tThe second byte is not 01 or 02 (block type): {b:02x}")
                return False
        elif padding_active:
            if b == 0:
                # padding has ended
                padding_active = False
                continue
            if pad_type == 1 and b != 0xff:
                if verbose:
                    print(f"error:\t\tPadding of type 01 contains non ff byte: {b:02x}")
                return False
            elif pad_type == 2:
                if b == 0:
                    # must be random non 00
                    if verbose:
                        print(f"error:\t\tPadding of type 02 contains 00 byte: {b:02x}")
                    return False



        index += 1
    
    if padding_active:
        if verbose:
            print(f"error:\t\tPadding has not been ended with 00 byte")


    return True

def main():
    parser = argparse.ArgumentParser(prog="rsa-padding", description='validates a rsa padding')
    parser.add_argument('-v', '--verbose', action="store_true",
                    help='append some auth hashed stuff')
    args = parser.parse_args()

    results: list[bool] = []
    for c in CIPHERTEXTS:
        b = padding_check(c, (D, N), args.verbose)
        print(f"Passed:\t\t{b}")
        plexcryptool.cplex.printing.seperator()


if __name__ == "__main__":
    main()
