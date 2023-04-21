# this is literal garbage
# does not even work for the bad hash
def calcHash():
    message = list('A' * 4)

    accu = int("a5a5a5a55a5a5a5a55555555aaaaaaaa", 16)

    for i in range(0, len(message)):
        message[i] = hex(ord(message[i]))[2:]

    if len(message) % 16 != 0:
        remainder = -((len(message) % 16) - 16);
        for i in range(0, remainder):
            message.append("FF")

    chunks = []

    for i in range(0, len(message), 16):
        chunk = "".join(message[i:i+16])
        chunks.append(chunk)

    for B in chunks:
        accu = accu ^ int(B, 16)

    print(accu)
    return accu

calcHash()
