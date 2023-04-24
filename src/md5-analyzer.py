#!/usr/bin/env python3
"""
A small script to help analyze the md5 hash function.

@author Christoph J. Scherr <software@cscherr.de>
@license MIT
@source: https://git.cscherr.de/PlexSheep/python-dhbw/src/branch/master/src/md5-analyzer.py
TODO do I need to implement md5 by myself for the assignment?
"""
import argparse
import hashlib

def main():
    parser = argparse.ArgumentParser(prog="md5-analyzer", description='md5 analyzer for a cryptography assignment')
    parser.add_argument('-i', '--input', type=str,
                    help='an input that should be hashed.')
    parser.add_argument('-a', '--print-all', action="store_true",
                    help="print all hashes in iterate mode")
    parser.add_argument('-t', '--iterate', action="store_true",
                    help='iterate 0 to 999999 (chars), generate hashes for this, analyze it\'s hashes.')
    args = parser.parse_args()
    if args.iterate:
        if not args.input:
            args.input = ""
        print('='*80)
        hashlist = []
        max = 999999
        searchbytes = "0000"
        for i in range(0, max + 1):
            # max should be included, so +x for the counter
            input = (args.input + str(i))
            hash = hashlib.md5(input.encode())
            # print only every 1000 lines for performance
            if i % 1000 == 0 or args.print_all:
                print("inp %s\t\t| out %s" % (input, hash.hexdigest()))
            if hash.hexdigest()[0:4] == searchbytes:
                hashlist.append((i, hash.hexdigest()))
                print("^" * 80)
        print('='*80)
        for (index, hash) in hashlist:
            print("ind %i\t\t| has %s" % (index, hash))
        print('='*80)
        size_of_first_x = 2**16
        expected = size_of_first_x / max
        print("found %d items.\nExpected %d (%f%%) from %d" % (len(hashlist), expected, expected / size_of_first_x, max))
        exit()
    if args.input:
        print("Hash for '%s':\n%s" % (
            args.input,
            hashlib.md5(args.input.encode()).digest().hex()
            ))
        exit()
    parser.print_usage()



if __name__ == "__main__":
    main()
