# kaws key

`kaws key` groups commands for managing OpenPGP keys.

## Synopsis

```
USAGE:
	kaws key [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

SUBCOMMANDS:
    export    Exports an OpenPGP public key from the local keyring into the pubkeys directory
    help      Prints this message
```

## Subcommands

### export

`kaws key export` exports an OpenPGP public key from the local keyring into the `pubkeys` directory.

```
USAGE:
	kaws key export [FLAGS] <uid>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
        --verbose    Outputs additional information to the standard output

ARGS:
    uid    The OpenPGP UID of the key to export
```

