# Bright
A scriptable tool for transforming and transpiling Luau code

## Non-goals
- Drop-in replacement for Darklua - Bright is designed to be a much more complex transformer and will support more
  complex rule definitions, but in exchange this means that it won't act as a simple drop-in
- Simplicity - since Bright is scriptable, it will support a wide variety of transformations that you can apply to code,
  which means that writing a transformer will not be simple; clear documentation should be prioritised over simplicity

## Rough around the edges?
This is my first time writing a proper Rust project! Things are definitely going to be rough around the edges to begin
with, but I'm aiming to learn as I go, and this is a fun learning experience. Overkill for a first proper project? Yes.
But worth it? Yes.

## Credits
A lot of Bright's runtime code was derived from the [Lune](https://github.com/lune-org/lune) project. Whilst nothing in
Bright is 1:1, Lune was used as either inspiration or the base to write the runtime. It's been handy to learn how they
set up things such as Luau LSP typings as well.
