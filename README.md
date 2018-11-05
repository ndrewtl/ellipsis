# ellipsis

[![Build Status](https://travis-ci.org/ndrewtl/ellipsis.svg?branch=master)](https://travis-ci.org/ndrewtl/ellipsis)

a peace-of-mind dotfile manager

### The ellipsis home directory

ellipsis chooses its 'home directory', the location in which it stores _all_
persistent state.

The home directory for any invocation of the `ellipsis` executable is defined to
be:

1. If `XDG_DATA_HOME` is defined, `$XDG_DATA_HOME/ellipsis`.
2. `$HOME/.ellipsis` otherwise.

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
