
## Creating the following graph from a one-to-many mapping
```mermaid
  graph TD;
    1 --> 10
    1 --> 2
    1 --> 3
    1 --> 5
    1 --> 8
    10 --> 11
    10 --> 12
    10 --> 13
    2 --> 4
    3 --> 4
    5 --> 6
    5 --> 9
    8 --> 9
    11 --> 14
    11 --> 15
    4 --> 7
    6 --> 7
```

To execute the binary:
```shell
cd examples/from_one_to_many_mapping
cargo run
```

The output:
```
descendants of 1 - [10, 2, 3, 5, 8, 11, 12, 13, 4, 6, 9, 14, 15, 7]
descendants of 10 - [11, 12, 13, 14, 15]
descendants of 3 - [4, 7]
```