Title: This Week in Rust 14
Date: 2013-09-07 18:30
Category: This Week in Rust

Welcome to another *This Week in Rust*.

<!-- more -->

# What's cooking in master?

65 pull requests were merged this week, and bors has had some idle time when
there wasn't anything approved in the queue.

## Breaking changes

- `std::os::glob` has been replaced with a [pure Rust
  version](https://github.com/mozilla/rust/pull/8914), for cross-platform and
  compatability reasons.
- `std::str::from_bytes` has been
  [renamed](https://github.com/mozilla/rust/pull/8997) to
  `std::str::from_utf8`, to be explicit about what it accepts.
- Casting to bool with `as` is [no longer
  allowed](https://github.com/mozilla/rust/pull/8980), and surrogate
  characters are no longer allowed in strings.
- char is [no longer](https://github.com/mozilla/rust/pull/8974) treated as an
  integer type (meaning it can't be casted to/from them), which removes the
  ability for safe code to create invalid characters.
- Opening a listening socket and actually listening on it [have been
  split](https://github.com/mozilla/rust/pull/8954). If you're jiggy with the
  jive, listen and accept are now separate operations. (*ed*: this used to say
  bind and accept wereseparate; thanks to ecr for the correction.)

## Additions

- `let` var hygiene has [landed](https://github.com/mozilla/rust/pull/9026).
  I'm sure this has cool implications, but I don't really know what they are.
- An [`export_name` attribute](https://github.com/mozilla/rust/pull/8903) has
  been added to control what symbol name something is exported as (similar to
  `no_mangle`).
- An `ExactSize` trait [has been
  added](https://github.com/mozilla/rust/pull/8884) to mark an iterator that
  always accurately reports its size in the `size_hint` method.
- `ToStr` has been [implemented](https://github.com/mozilla/rust/pull/8960)
  for char and Ascii.
- Safe accessors of `MutexArc` [have been
  implemented](https://github.com/mozilla/rust/pull/8966)
- A bytes iterator [has been added](https://github.com/mozilla/rust/pull/8935)
  for newrt readers.
- Stream is [automatically
  implemented](https://github.com/mozilla/rust/pull/8984) for types which
  implement Reader and Writer from newrt.
- An `unreachable` macro [has been
  added](https://github.com/mozilla/rust/pull/8992) for better error reporting
  than a function could do.
- newrt [can now do](https://github.com/mozilla/rust/pull/9000) simple DNS
  resolution.
- strptime/strftime [now support](https://github.com/mozilla/rust/pull/9016)
  fractional seconds, out to tenths of a nanosecond.

## Changes etc

- Name mangling [has been
  improved](https://github.com/mozilla/rust/pull/8875).
- `rust_log.cpp` [has been
  converted](https://github.com/mozilla/rust/pull/8880) into pure Rust.
- Debuginfo [now does closure
  capture](https://github.com/mozilla/rust/pull/8855) and very large structs.
- A [bunch](https://github.com/mozilla/rust/pull/8947)
  [of](https://github.com/mozilla/rust/pull/8927)
  [repr](https://github.com/mozilla/rust/pull/8928) improvements landed.

# Meeting

There was no meteting this week listed on the wiki or that I saw.

# Projects

- [rust-nanomsg](https://github.com/glycerine/rust-nanomsg) - bindings to the
  nanomsg library.
