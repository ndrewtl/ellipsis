# ellipsis

[![Build Status](https://travis-ci.org/ndrewtl/ellipsis.svg?branch=master)](https://travis-ci.org/ndrewtl/ellipsis)

a peace-of-mind dotfile manager

## Testing

Because the operations carried out by `ellipsis` modify both the filesystem and
various environment variables, there is a danger that various tests will exhibit
race conditions on environment variables if run concurrently.

Thus, these processes ust be run one after another. To test, run:
```sh
cargo test --verbose -- --test-threads 1
```

Or, simply
```sh
./test.sh
```
