# ellipsis

[![Build Status](https://travis-ci.org/ndrewtl/ellipsis.svg?branch=master)](https://travis-ci.org/ndrewtl/ellipsis)

a peace-of-mind dotfile manager

ellipsis is influenced by `pass`, the standard Unix Password Manager. Like pass,
it uses git to export and share personal information across multiple domains.
However, uniquely, configuration files can vary across various devices, so there
must be a mechanism to _broadly_ share configurations across multiple devices,
while still allowing fine customization between each one.

Like many other dotfile managers, ellipsis solves this problem by using
symlinks. "Actual" configuration files are stored in the ellipsis home
directory, and then linked into the correct locations. Each computer has a
different linking scheme, so individual changes can be made by linking slightly
different files in each.

### The ellipsis home directory

ellipsis chooses its 'home directory', the location in which it stores _all_
persistent state.

The home directory for any invocation of the `ellipsis` executable is defined to
be:

1. If `XDG_DATA_HOME` is defined, `$XDG_DATA_HOME/ellipsis`.
2. `$HOME/.ellipsis` otherwise.

### Configuration and Linking

Each computer self-identifies with a different hostname. During linking,
ellipsis identifies a file called `[hostname].dot.json` in the ellipsis home
directory. This file contains a specification of what links to use:
```json
{
  "links" : {
    ".bashrc"   : "~/.bashrc",
    "xmonad.hs" : "~/xmonad/xmonad.hs",
    "init.vim"    : "~/.config/nvim/init.vim"
  }
}
```

Running `ellipsis link` creates a symlink from each of the left hand files
(resolved relative to the home directory) to each of the destination files
(specified as absolute paths.) Running `ellipsis unlink` deletes the links.

## Example workflow

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

# Caveats
In order to choose which configurations are relevant for each given device,
ellipsis relies on self-identified hostnames for each. If you have a
public-facing git repository to store your dotfiles, a `.dot.json` file
containing your hostname may be visible to the public.

At the moment, there is not an alternative scheme to differentiate between
computers. If you have another proposal for this, please submit an issue!

If you are concerned, you can keep your `*.dot.json` files ignored by git, which
prevents any sharing whatsoever. Each computer really only needs its own
configuration, so there is no need to share these files across the internet
anyway. This is currently the workflow preferred by the author.
