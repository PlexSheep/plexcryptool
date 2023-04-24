#!/usr/bin/env python3
"""
A small script to help analyze the md5 hash function.

@author Christoph J. Scherr <software@cscherr.de>
@license MIT
@source: https://git.cscherr.de/PlexSheep/python-dhbw/src/branch/master/src/md5-analyzer.py
"""
import argparse
import hashlib

def main():
    parser = argparse.ArgumentParser(prog="md5-analyzer", description='md5 analyzer for a cryptography assignment')
    parser.add_argument('--hash', type=str,
                    help='an input that should be hashed.')
    args = parser.parse_args()
    if args.hash:
        print("Hash for '%s':\n%s" % (
            args.hash,
            hashlib.md5(args.hash.encode()).digest().hex()
            ))



if __name__ == "__main__":
    main()
