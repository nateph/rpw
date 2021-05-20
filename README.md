# rpw

A simple password generator. Running `rpw` with no options or flags
will default to all characters included, with a length of 16.

```
USAGE:
    rpw [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
    -l, --no-lower      Do not include lower case characters
    -n, --no-numbers    Do not include numbers
    -s, --no-symbols    Do not include symbol characters
    -u, --no-upper      Do not include uppercase case characters
    -V, --version       Prints version information

OPTIONS:
        --exclude <exclude>    Exclude certain characters explicitly. example: --exlclude '4$&'
        --len <length>         Specify a custom length (max 255) [default: 16]
```
