# BSON format Nushell plugin

Adds `from bson` and `to bson` commands to Nushell.

```nushell
{a: 1 b: 2} | to bson
# => Length: 27 (0x1b) bytes | printable whitespace ascii_other non_ascii
# => 00000000:   1b 00 00 00  12 61 00 01  00 00 00 00  00 00 00 12   •000•a0•0000000•
# => 00000010:   62 00 02 00  00 00 00 00  00 00 00                   b0•00000000
```

With these commands in place, Nushell automatically and transparently handles `.bson` files.

```nushell
{a: 1 b: 2} | save test.bson

open test.bson
# => ╭───┬───┬───╮
# => │ # │ b │ a │
# => ├───┼───┼───┤
# => │ 0 │ 2 │ 1 │
# => ╰───┴───┴───╯

open test.bson | describe
# => table<b: int, a: int> (stream)
```

## Plugin Installation

In Nushell, plugins are registered executables. See commands `plugin list`, `plugin add`, `plugin use`.

To install the `nu_plugin_bson` plugin, you can use `cargo` (Rust development toolchain) to build and install your own version, or download a pre-built executable from the [latest GitHub release](https://github.com/Kissaki/nu_plugin_bson/releases/latest).

See also [Nushell Installing Plugins documentation](https://www.nushell.sh/book/plugins.html#installing-plugins).

### Build and Install Using Cargo

```nushell
cargo install --path . --locked
```

if you have `CARGO_HOME` defined, otherwise the path defaults to `~/.cargo`

```nushell
plugin add $"($env.CARGO_HOME)/bin/nu_plugin_bson.exe"
plugin use bson
```

After add and use, to update the plugin, replacing the executable is enough. To do so, call the `cargo install` command documented above.

## Check Plugin Status

```nushell
plugin list | where name == 'bson' | table -e
```
