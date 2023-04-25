#!/usr/bin/env python3
"""
A small script to help analyze the md5 hash function.

@author: Christoph J. Scherr <software@cscherr.de>
@license: MIT
@source: https://git.cscherr.de/PlexSheep/python-dhbw/src/branch/master/src/md5-analyzer.py
TODO do I need to implement md5 by myself for the assignment?
"""
import argparse
import hashlib

def main():
    parser = argparse.ArgumentParser(prog="md5-analyzer", description='md5 analyzer for a cryptography assignment')
    parser.add_argument('-i', '--input', type=str,
                    help='an input that should be hashed or used with iterate')
    parser.add_argument('-a', '--print-all', action="store_true",
                    help="print all hashes in iterate mode")
    parser.add_argument('-t', '--iterate', action="store_true",
                    help='iterate 0 to 999999 (chars), generate hashes for this, analyze it\'s hashes.')
    parser.add_argument('-q', '--quiet', action="store_true",
                    help="print less")
    parser.add_argument('-m', '--max', action="store", type=int, default=999999,
                    help="max value for iteration")
    args = parser.parse_args()

    if args.iterate:
        if not args.input:
            args.input = ""
        if not args.quiet:
            print('='*80)
        hashlist = []
        searchbytes = "0000"
        for i in range(0, args.max + 1):
            # args.max should be included, so +x for the counter
            input = (args.input + str(i))
            hash = hashlib.md5(input.encode())
            found = False
            # print only every 1000 lines for performance
            if hash.hexdigest()[0:4] == searchbytes:
                found = True
                hashlist.append((input, hash.hexdigest()))
            if not args.quiet and (found or i % 1000 == 0 or args.print_all):
                print("inp %s\t\t| out %s" % (input, hash.hexdigest()))
                if found:
                    print("^" * 80)
        print('='*80)
        for (index, hash) in hashlist:
            print("inp %s\t\t| has %s" % (index, hash))
        print('='*80)
        size_searchbytes = 16**4
        expected = args.max / size_searchbytes # 32 hex value chars in an md5 hash
        print("found %d items (%f%%) from %d" % (len(hashlist), len(hashlist) / size_searchbytes, args.max))
        print("Expected %f (%f%%) from %d" % (expected, expected / size_searchbytes, args.max))
        exit()

    if args.input:
        print("Hash for '%s':\n%s" % (
            args.input,
            hashlib.md5(args.input.encode()).digest().hex()
            ))
        exit()

    # else print help
    parser.print_help()



if __name__ == "__main__":
    main()
