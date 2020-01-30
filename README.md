# merkle-benchmarks
Benchmarking Merkle tree structures

Preliminary benchmark results (in µs)

Inserting 256 random key-value pairs into the different maps, persisting to disk.

|                | insert     | insert and persist |
| -------------- |-----------:| ------------------:|
| kelvin-hamt    | 268        | 1308               |
| kelvin-radix   | 176        | 736                |
| kelvin-two3    | 233        | 1093               |
| substrate-trie | 625        | N/A                |
