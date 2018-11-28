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

## How it works

### The ellipsis home directory

ellipsis chooses its 'home directory', the location in which it stores _all_
persistent state.

The home directory for any invocation of the `ellipsis` executable is defined to
be:

1. If `XDG_DATA_HOME` is defined, `$XDG_DATA_HOME/ellipsis`.
2. `$HOME/.ellipsis` otherwise.

This tool functions by storing all _actual_ configuration files in this
centralized directories, and then symlinking them into the proper locations.

### Configuration and Linking

During linking, ellipsis identifies its configuration file, located at
`$(ellipsis home)/.dot.json`. The file looks like this:

```json
{
  "links" : {
    "~/.bashrc" : ".bashrc",
    "~/xmonad/xmonad.hs" : "xmonad.hs",
    "~/.config/nvim/init.vim" : "init.vim"
  }
}
```

Running `ellipsis link` creates a symlink from each of the right hand files
(resolved relative to the home directory) to each of the left-hand files
(specified as absolute paths.) Running `ellipsis unlink` deletes the links.

As you can see, each local file, such as `$(ellipsis home)/.bashrc` is linked
into the proper location, such as `~/.bashrc`. If you want to edit the file,
just run `ellipsis edit .bashrc`, and your configuration will be updated
automatically.

But all computers aren't identical, right? What if you need a different
configuration for different computers?

Each computer self-identifies with a different hostname. During linking,
ellipsis also identifies a file called `[hostname].dot.json` in the ellipsis
home directory. This file is identical in format to the global `.dot.json`, but
_overrides_ it in any points of conflict. In fact, it's often easier to start
by _just_ using `.dot.json` and adding per-computer configurations as the need
arises. This allows you to synchronize configurations and maintain the structure
you want, across any computer.

## Example workflow

What does this look like in practice?

Suppose we have an apple computer with hostname `fuji` and a linux box with
hostname `emperor`.

### Startup

Initialize configuration on `fuji`:

```sh
handle@fuji $ ellipsis init
Creating new git repository in /Users/handle/.local/share/ellipsis
done
handle@fuji $ ellipsis edit .bashrc # add your configuration
handle@fuji $ echo '{ "links" : { "~/.bashrc" : ".bashrc" } }' > $(ellipsis home)/.dot.json
handle@fuji $ ellipsis link
```

Now, synchronize your configuration across computers:
```sh
handle@fuji $ ellipsis git commit -am"Add bash configuration"
handle@fuji $ ellipsis git remote add origin https://gitlab.org/handle/your-dotfiles.git
handle@fuji $ ellipsis git push -u origin master
```

On your second machine, clone in your configuration:
```sh
handle@emperor $ ellipsis init https://gitlab.org/handle/your-dotfiles.git
Cloning https://gitlab.org/handle/your-dotfiles.git into /home/handle/.local/share/ellipsis
done
handle@emperor $ ellipsis link
```

Now, we're synchronized across machines!

### Per-machine configurations

Suppose we want colorized output whenever we type `ls`. Mac computers by default
use the BSD command line tools, which have different flags than the GNU userland
utilities. If we type `ls --color` on a Linux machine, it'll work fine:
```sh
handle@emperor $ ls --color
Documents Downloads Mail Public ... (more colored output)
```

But on a mac, we get an error:
```sh
handle@fuji $ ls --color
/bin/ls: illegal option -- -
usage: ls [-ABCFGHLOPRSTUWabcdefghiklmnopqrstuwx1] [file ...]
```

As it turns out, on  a mac, the correct flag is `ls -G` We want to `alias ls='ls
--color` on one machine, and `alias ls='ls -G` on the other.

How do we accomplish this? Decompose your dotfiles!

First, load up the two configurations:
```sh
handle@emperor $ echo "alias ls='ls --color'" >> $(ellipsis home)/aliases.gnu.bash
handle@emperor $ echo "alias ls='ls -G'" >> $(ellipsis home)/aliases.bsd.bash
handle@emperor $ git commit -am"Add ls coloring aliases"
handle@emperor $ git push
```

Edit the following lines

```sh
handle@emperor $ ellipsis edit .bashrc
```

```diff
+ source $HOME/.config/bash/aliases.bash
```


Now, let's set up our computers to link properly. Let's assume that by default,
our computers will use the GNU coreutils.

Change `.dot.json` to look like the following:
```json
{
  "links" : {
    "~/.bashrc" : ".bashrc",
    "~/.config/bash/aliases.bash" : "aliases.gnu.bash"
  }
}
```

For Mac computers, however, we want to override this. Change `fuji.dot.json` to
look like this:
```json
{
  "links" : {
    "~/.config/bash/aliases.bash" : "aliases.bsd.bash"
  }
}
```

Make sure you use git to sync the proper files across your machines and run
`ellipsis link` whenever you add new files.

Now, when you work on a mac, your computer should automatically use the correct
`ls` command on both machines.

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
