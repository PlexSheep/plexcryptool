#!/usr/bin/env python3

"""
This script is really bad and hacky, don't expect it to do anything.
you also need the chiffrat.txt file in the same directory.

License: MIT
author: Christoph J. Scherr <software@cscherr.de>
"""

MAX_IN_LINE = 16
DE_MOST_COMMON = ['E', 'N', 'I', 'R', 'A']
EN_MOST_COMMON = ['E', 'T', 'A', 'O', 'I']

def main():
    with open("chiffrat.txt", "r") as file:
        text = file.read().split(" ")
        intarr = []
        chars_in_text: int = len(text)
        for item in text:
            intarr.append(int(item, 16))
        print('=' * 80)
        print("dumping cyphertext (%d chars)" % chars_in_text)
        print('=' * 80)
        print(dump_intarr(intarr))
        freq = freq_analysis(intarr)
        print('=' * 80)
        print("Top 5 occurences:")
        print('=' * 80)
        dump_frequencies(freq, chars_in_text)
        print('=' * 80)
        print("Trying to find key from frequencies")
        print('=' * 80)
        key = find_key(freq, DE_MOST_COMMON, intarr)
        if not key:
            print("The key could not be determined.")
        if key:
            print('=' * 80)
            print("decrypt (key %d):" % key)
            print('=' * 80)
            decry = shift_intarr(intarr, key)
            print(dump_intarr(decry))

def freq_analysis(intarr: list):
    freql: dict = {}
    for i in range(0, 255):
        freql[i] = 0
    # list of tuples with all possible values and their frequency
    for item in intarr:
        freql[item] += 1
    return dict(sorted(freql.items(), key=lambda item: item[1], reverse=True))

def find_key(freq: dict, language_reference: list, full_text: list):
    # caesar
    caesar_keys = [0] * len(language_reference)
    for index, common_char in enumerate(language_reference):
        # this is probably garbage
        caesar_keys[index] = list(freq.values())[index] - ord(common_char) - 64

    all_same = False
    for i in range(1, len(caesar_keys)):
        if all_same:
            all_same = caesar_keys[i-1] == caesar_keys[i]
        else: break
    if all_same:
        print("found a perfect match in the caesar checks: %s" % caesar_keys)
        return caesar_keys[0]
    else:
        print("Caesar keys ambiguos: %s" % caesar_keys)
    # was not caesar encrypted, continue with XOR checks
    # XOR
    #for xor_key in range(0, 2**7):
    #    decrypt = bytes(xor_key ^ common_char for common_char in full_text)
    #    try:
    #        print(("decry for kex %x:\t%s" % (xor_key, decrypt.decode(errors="backslashreplace"))).replace("\n", ""))
    #    except:
    #        print(("decry for kex %x:\t%s" % (xor_key, decrypt.hex())).replace("\n", ""))
    # -> manual review of the output of the above confirmed that the key for my cyphertext is 0x15
    THE_KEY = 0x15
    
    # dump the text:
    decrypted: list = []
    for c in full_text:
        decrypted.append((c ^ THE_KEY))
    print("dump of text repr:")
    output = ""
    for c in decrypted:
        output += chr(c)
    print(output)
    print("dumping decrypted with 0x15")
    print(dump_intarr(decrypted))
    exit()


    return THE_KEY

    all_same = False
    for i in range(1, len(xor_keys)):
        if all_same:
            all_same = xor_keys[i-1] == xor_keys[i]
        else: break
    if all_same:
        print("found a perfect match in the caesar checks: %s" % xor_keys)
        return xor_keys[0]
    else:
        # ????????
        print("xor keys ambiguos: %s" % xor_keys)

    return False
    

def dump_frequencies(frequencies, chars_in_text: int = -1, print_top_x = 5):
    for i in range(print_top_x):
        print("Top %s:\t'%s' with %s\toccurences (%s%%)" % (
            i+1, 
            hex(list(frequencies.keys())[i]), 
            list(frequencies.values())[i],
            (list(frequencies.values())[i] / chars_in_text) * 100,
            ))

def shift_intarr(cypher_text_arr, key: int = 0):
    decrypt = [0] * len(cypher_text_arr)
    for index, item in enumerate(cypher_text_arr):
        decrypt[index] = item + key
    return decrypt

def dump_intarr(intarr) -> str:
    output = ""
    for index, item in enumerate(intarr) :
        if index % MAX_IN_LINE == 0:
            output += ("%02x\t| %02x " % (index,item))
        elif index % MAX_IN_LINE == MAX_IN_LINE - 1:
            output += ("%02x\n" %item)
        else:
            output += ("%02x " %item)
    # now dump as chars
    output += ("\n" + '=' * 80 + "\n")
    output += ("Char Representation\n")
    output += ('=' * 80 + "\n")
    char_max = MAX_IN_LINE * 4
    for index, item in enumerate(intarr) :
        if index % char_max == 0:
            output += ("%s\t| %c " % (hex(index),item))
        elif index % char_max == char_max - 1:
            output += ("%c\n" %item)
        else:
            output += ("%c " %item)
    return output

if __name__ == "__main__":
    main()
