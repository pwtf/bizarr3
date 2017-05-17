# IDF file format
In a glorious pre-formatted ASCII diagram
```
┌─────┬──────┬────┬───────┬───┬────────────┬────────┬───┐
│Magic│Magic*│Type│Offset*│Key│... data ...│TOC size│TOC│
└▲────┴▲─────┴▲───┴▲──────┴▲──┴▲───────────┴▲───────┴▲──┘
 │     │      │    │       │   │            │        │
 │     │      │    │       │   │            │        │
 Four bytes of magic, always "FFFL"         │        │
       │      │    │       │   │            │        │
       │      │    │       │   │            │        │
 Read together with the "Magic", but ends up being   │
 discarded. Always 0x0000000c (mind the endianness). │
 Coincidentally, this is the same size as the next   │
 header group and the amount of bytes left before the   
 actual data. │    │       │   │            │        │
              │    │       │   │            │        │
              │    │       │   │            │        │
              │    │       │   │            │        │
 There are two special cases that are triggered      │
 depending on the file type.   │            │        │
                   │       │   │            │        │
 0x00010000 seems to indicate a file with an unencrypted
 TOC, encryption key is being zeroed. Refer to the "Key"
 description.      │       │   │            │        │
                   │       │   │            │        │
 0x00010100, all the .idf files I've ever seen have this
 type. Makes a truly weird thing to the "Offset".    │
                   │       │   │            │        │
                   │       │   │            │        │
                   │       │   │            │        │
 A TOC offset. Gets XORed with 0x123 if the "Type" is
 0x10100 (read: always).   │   │            │        │
                           │   │            │        │
                           │   │            │        │
                           │   │            │        │
 Encryption key. Decryption part is skipped if the "Type"
 is 0x10000 or the following condition is true:      │
     !(key >> 24) || key >> 24 != 1         │        │
                               │            │        │
 Only the lowest byte is used in the decryption process.
                               │            │        │
                               │            │        │
                               │            │        │
 Infinitely coveted bytes we're here for. The layout is
 described in the "TOC".                    │        │
                                            │        │
                                            │        │
                                            │        │
 Size of the "Table of Contents" portion of the file.
 Don't forget to XOR it with 0x123, LOL. u32.
                                                     │
                                                     │
                                                     │
 A (most likely) encrypted list of 64-byte structures,
 "TOC size" of them:
 - 52-byte (max, null-terminated) file name
 - an undefined u32 (see below)
 - offset, u32
 - size, u32

 An undefined u32 is set to an IDF index (in an order
 they are parsed), once being processed by the game code.
```
