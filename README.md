# merkle-benchmarks
Benchmarking Merkle tree structures

Preliminary benchmark results (in µs)

Inserting 256 random key-value pairs into the different maps.

|                | insert     | insert persist     |
| -------------- |-----------:| ------------------:|
| kelvin-hamt    | 120.37     | 645.28             |
| kelvin-radix   | 176.08     | 807.46             |
| kelvin-two3    | 226.26     | 1398.5             |
| substrate-trie | 625.03     | N/A                |
