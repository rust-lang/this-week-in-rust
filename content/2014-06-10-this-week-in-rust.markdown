Title: This Week in Rust 51
Date: 2014-06-10 22:46
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This issue combines two weeks of Rust and three weeks of Servo. Additionally,
in a little over a month month, I'll be releasing [an entirely reworked TWiR
website](http://blog.octayn.net/blog/2014/06/09/future-of-twir/). Until then,
TWiR might be a bit lighter than usual.

<!-- more -->

# What's cooking on master?

126 pull requests were merged in the last week.

## Breaking Change Log

- std: Remove the `as_utf16_p` functions

```
These functions are all much better expressed via RAII using the to_utf16()
method on strings. This refactoring also takes this opportunity to properly
handle when filenames aren't valid unicode when passed through to the windows
I/O layer by properly returning I/O errors.

All previous users of the `as_utf16_p` or `as_utf16_mut_p` functions will need
to convert their code to using `foo.to_utf16().append_one(0)` to get a
null-terminated utf16 string.

[breaking-change]
```


- Implement `#[plugin_registrar]`

```
See RFC 22.

[breaking-change]
```


- core: Move the collections traits to libcollections

```
This commit moves Mutable, Map, MutableMap, Set, and MutableSet from
`core::collections` to the `collections` crate at the top-level. Additionally,
this removes the `deque` module and moves the `Deque` trait to only being
available at the top-level of the collections crate.

All functionality continues to be reexported through `std::collections`.

[breaking-change]
```


- core: Rename `container` mod to `collections`. Closes #12543

```
Also renames the `Container` trait to `Collection`.

[breaking-change]
```


- std: Extract librustrt out of libstd

```
As part of the libstd facade efforts, this commit extracts the runtime interface
out of the standard library into a standalone crate, librustrt. This crate will
provide the following services:

* Definition of the rtio interface
* Definition of the Runtime interface
* Implementation of the Task structure
* Implementation of task-local-data
* Implementation of task failure via unwinding via libunwind
* Implementation of runtime initialization and shutdown
* Implementation of thread-local-storage for the local rust Task

Notably, this crate avoids the following services:

* Thread creation and destruction. The crate does not require the knowledge of
  an OS threading system, and as a result it seemed best to leave out the
  `rt::thread` module from librustrt. The librustrt module does depend on
  mutexes, however.
* Implementation of backtraces. There is no inherent requirement for the runtime
  to be able to generate backtraces. As will be discussed later, this
  functionality continues to live in libstd rather than librustrt.

As usual, a number of architectural changes were required to make this crate
possible. Users of "stable" functionality will not be impacted by this change,
but users of the `std::rt` module will likely note the changes. A list of
architectural changes made is:

* The stdout/stderr handles no longer live directly inside of the `Task`
  structure. This is a consequence of librustrt not knowing about `std::io`.
  These two handles are now stored inside of task-local-data.

  The handles were originally stored inside of the `Task` for perf reasons, and
  TLD is not currently as fast as it could be. For comparison, 100k prints goes
  from 59ms to 68ms (a 15% slowdown). This appeared to me to be an acceptable
  perf loss for the successful extraction of a librustrt crate.

* The `rtio` module was forced to duplicate more functionality of `std::io`. As
  the module no longer depends on `std::io`, `rtio` now defines structures such
  as socket addresses, addrinfo fiddly bits, etc. The primary change made was
  that `rtio` now defines its own `IoError` type. This type is distinct from
  `std::io::IoError` in that it does not have an enum for what error occurred,
  but rather a platform-specific error code.

  The native and green libraries will be updated in later commits for this
  change, and the bulk of this effort was put behind updating the two libraries
  for this change (with `rtio`).

* Printing a message on task failure (along with the backtrace) continues to
  live in libstd, not in librustrt. This is a consequence of the above decision
  to move the stdout/stderr handles to TLD rather than inside the `Task` itself.
  The unwinding API now supports registration of global callback functions which
  will be invoked when a task fails, allowing for libstd to register a function
  to print a message and a backtrace.

  The API for registering a callback is experimental and unsafe, as the
  ramifications of running code on unwinding is pretty hairy.

* The `std::unstable::mutex` module has moved to `std::rt::mutex`.

* The `std::unstable::sync` module has been moved to `std::rt::exclusive` and
  the type has been rewritten to not internally have an Arc and to have an RAII
  guard structure when locking. Old code should stop using `Exclusive` in favor
  of the primitives in `libsync`, but if necessary, old code should port to
  `Arc<Exclusive<T>>`.

* The local heap has been stripped down to have fewer debugging options. None of
  these were tested, and none of these have been used in a very long time.

[breaking-change]
```


- Rename Iterator::len to count

```
This commit carries out the request from issue #14678:

> The method `Iterator::len()` is surprising, as all the other uses of
> `len()` do not consume the value. `len()` would make more sense to be
> called `count()`, but that would collide with the current
> `Iterator::count(|T| -> bool) -> unit` method. That method, however, is
> a bit redundant, and can be easily replaced with
> `iter.filter(|x| x < 5).count()`.
> After this change, we could then define the `len()` method
> on `iter::ExactSize`.

Closes #14678.

[breaking-change]
```


- auto merge of #14538 : alexcrichton/rust/libcollections, r=brson

```
As with the previous commit with `librand`, this commit shuffles around some
`collections` code. The new state of the world is similar to that of librand:

* The libcollections crate now only depends on libcore and liballoc.
* The standard library has a new module, `std::collections`. All functionality
  of libcollections is reexported through this module.

I would like to stress that this change is purely cosmetic. There are very few
alterations to these primitives.

There are a number of notable points about the new organization:

* std::{str, slice, string, vec} all moved to libcollections. There is no reason
  that these primitives shouldn't be necessarily usable in a freestanding
  context that has allocation. These are all reexported in their usual places in
  the standard library.

* The `hashmap`, and transitively the `lru_cache`, modules no longer reside in
  `libcollections`, but rather in libstd. The reason for this is because the
  `HashMap::new` contructor requires access to the OSRng for initially seeding
  the hash map. Beyond this requirement, there is no reason that the hashmap
  could not move to libcollections.

  I do, however, have a plan to move the hash map to the collections module. The
  `HashMap::new` function could be altered to require that the `H` hasher
  parameter ascribe to the `Default` trait, allowing the entire `hashmap` module
  to live in libcollections. The key idea would be that the default hasher would
  be different in libstd. Something along the lines of:

      // src/libstd/collections/mod.rs

      pub type HashMap<K, V, H = RandomizedSipHasher> =
            core_collections::HashMap<K, V, H>;

  This is not possible today because you cannot invoke static methods through
  type aliases. If we modified the compiler, however, to allow invocation of
  static methods through type aliases, then this type definition would
  essentially be switching the default hasher from `SipHasher` in libcollections
  to a libstd-defined `RandomizedSipHasher` type. This type's `Default`
  implementation would randomly seed the `SipHasher` instance, and otherwise
  perform the same as `SipHasher`.

  This future state doesn't seem incredibly far off, but until that time comes,
  the hashmap module will live in libstd to not compromise on functionality.

* In preparation for the hashmap moving to libcollections, the `hash` module has
  moved from libstd to libcollections. A previously snapshotted commit enables a
  distinct `Writer` trait to live in the `hash` module which `Hash`
  implementations are now parameterized over.

  Due to using a custom trait, the `SipHasher` implementation has lost its
  specialized methods for writing integers. These can be re-added
  backwards-compatibly in the future via default methods if necessary, but the
  FNV hashing should satisfy much of the need for speedier hashing.

A list of breaking changes:

* HashMap::{get, get_mut} no longer fails with the key formatted into the error
  message with `{:?}`, instead, a generic message is printed. With backtraces,
  it should still be not-too-hard to track down errors.

* The HashMap, HashSet, and LruCache types are now available through
  std::collections instead of the collections crate.

* Manual implementations of hash should be parameterized over `hash::Writer`
  instead of just `Writer`.

[breaking-change]
```


- std: Recreate a `collections` module

```
As with the previous commit with `librand`, this commit shuffles around some
`collections` code. The new state of the world is similar to that of librand:

* The libcollections crate now only depends on libcore and liballoc.
* The standard library has a new module, `std::collections`. All functionality
  of libcollections is reexported through this module.

I would like to stress that this change is purely cosmetic. There are very few
alterations to these primitives.

There are a number of notable points about the new organization:

* std::{str, slice, string, vec} all moved to libcollections. There is no reason
  that these primitives shouldn't be necessarily usable in a freestanding
  context that has allocation. These are all reexported in their usual places in
  the standard library.

* The `hashmap`, and transitively the `lru_cache`, modules no longer reside in
  `libcollections`, but rather in libstd. The reason for this is because the
  `HashMap::new` contructor requires access to the OSRng for initially seeding
  the hash map. Beyond this requirement, there is no reason that the hashmap
  could not move to libcollections.

  I do, however, have a plan to move the hash map to the collections module. The
  `HashMap::new` function could be altered to require that the `H` hasher
  parameter ascribe to the `Default` trait, allowing the entire `hashmap` module
  to live in libcollections. The key idea would be that the default hasher would
  be different in libstd. Something along the lines of:

      // src/libstd/collections/mod.rs

      pub type HashMap<K, V, H = RandomizedSipHasher> =
            core_collections::HashMap<K, V, H>;

  This is not possible today because you cannot invoke static methods through
  type aliases. If we modified the compiler, however, to allow invocation of
  static methods through type aliases, then this type definition would
  essentially be switching the default hasher from `SipHasher` in libcollections
  to a libstd-defined `RandomizedSipHasher` type. This type's `Default`
  implementation would randomly seed the `SipHasher` instance, and otherwise
  perform the same as `SipHasher`.

  This future state doesn't seem incredibly far off, but until that time comes,
  the hashmap module will live in libstd to not compromise on functionality.

* In preparation for the hashmap moving to libcollections, the `hash` module has
  moved from libstd to libcollections. A previously snapshotted commit enables a
  distinct `Writer` trait to live in the `hash` module which `Hash`
  implementations are now parameterized over.

  Due to using a custom trait, the `SipHasher` implementation has lost its
  specialized methods for writing integers. These can be re-added
  backwards-compatibly in the future via default methods if necessary, but the
  FNV hashing should satisfy much of the need for speedier hashing.

A list of breaking changes:

* HashMap::{get, get_mut} no longer fails with the key formatted into the error
  message with `{:?}`, instead, a generic message is printed. With backtraces,
  it should still be not-too-hard to track down errors.

* The HashMap, HashSet, and LruCache types are now available through
  std::collections instead of the collections crate.

* Manual implementations of hash should be parameterized over `hash::Writer`
  instead of just `Writer`.

[breaking-change]
```


- auto merge of #14610 : alexcrichton/rust/issue-14008, r=brson

```
This commit removes the <M: Any + Send> type parameter from Option::expect in
favor of just taking a hard-coded `&str` argument. This allows this function to
move into libcore.

Previous code using strings with `expect` will continue to work, but code using
this implicitly to transmit task failure will need to unwrap manually with a
`match` statement.

[breaking-change]
Closes #14008
```


- std: Remove generics from Option::expect

```
This commit removes the <M: Any + Send> type parameter from Option::expect in
favor of just taking a hard-coded `&str` argument. This allows this function to
move into libcore.

Previous code using strings with `expect` will continue to work, but code using
this implicitly to transmit task failure will need to unwrap manually with a
`match` statement.

[breaking-change]
Closes #14008
```


- std: Drop Total from Total{Eq,Ord}

```
This completes the last stage of the renaming of the comparison hierarchy of
traits. This change renames TotalEq to Eq and TotalOrd to Ord.

In the future the new Eq/Ord will be filled out with their appropriate methods,
but for now this change is purely a renaming change.

[breaking-change]
```


- auto merge of #14556 : sfackler/rust/kill-workcache, r=alexcrichton

```
This was only ever used by rustpkg and is very unmaintained.

[breaking-change]
```


- Remove libworkcache

```
This was only ever used by rustpkg and is very unmaintained.

[breaking-change]
```


- std: Rename {Eq,Ord} to Partial{Eq,Ord}

```
This is part of the ongoing renaming of the equality traits. See #12517 for more
details. All code using Eq/Ord will temporarily need to move to Partial{Eq,Ord}
or the Total{Eq,Ord} traits. The Total traits will soon be renamed to {Eq,Ord}.

cc #12517

[breaking-change]
```


- librustc: Fix snake case errors.

```
A number of functions/methods have been moved or renamed to align
better with rust standard conventions.

rustc::back::link::WriteOutputFile => write_output_file
rustc::middle::ty::EmptyBuiltinBounds => empty_builtin_bounds
rustc::middle::ty::AllBuiltinBounds => all_builtin_bounds
rustc::middle::liveness::IrMaps => IrMaps::new
rustc::middle::liveness::Liveness => Liveness::new
rustc::middle::resolve::NameBindings => NameBindings::new
rustc::middle::resolve::PrimitiveTypeTable => PrimitiveTypeTable::new
rustc::middle::resolve::Resolver => Resolver::new
rustc::middle::trans::datum::Datum => Datum::new
rustc::middle::trans::datum::DatumBlock => DatumBlock::new
rustc::middle::trans::datum::Rvalue => Rvalue::new
rustc::middle::typeck::infer::new_ValsAndBindings => ::infer::unify::ValsAndBindings::new
rustc::middle::typeck::infer::region_inference::RegionVarBindings => RegionVarBindings::new

[breaking-change]
```


- lib{serialize, uuid}: Fix snake case errors.

```
A number of functions/methods have been moved or renamed to align
better with rust standard conventions.

serialize::ebml::reader::Doc => seriaize::ebml::Doc::new
serialize::ebml::reader::Decoder => Decoder::new
serialize::ebml::writer::Encoder => Encoder::new

[breaking-change]
```


- lib{std,core,debug,rustuv,collections,native,regex}: Fix `snake_case` errors.

```
A number of functions/methods have been moved or renamed to align
better with rust standard conventions.

std::reflect::MovePtrAdaptor => MovePtrAdaptor::new
debug::reflect::MovePtrAdaptor => MovePtrAdaptor::new
std::repr::ReprVisitor => ReprVisitor::new
debug::repr::ReprVisitor => ReprVisitor::new
rustuv::homing::HomingIO.go_to_IO_home => go_to_io_home

[breaking-change]
```


- libsyntax: Fix `snake_case` errors.

```
A number of functions/methods have been moved or renamed to align
better with rust standard conventions.

syntax::ext::mtwt::xorPush => xor_push
syntax::parse::parser::Parser => Parser::new

[breaking-change]
```


- auto merge of #14511 : Sawyer47/rust/osrng-rename, r=alexcrichton

```
According to Rust's style guide acronyms in type names should be
CamelCase.

[breaking-change]
```


- Rename OSRng to OsRng

```
According to Rust's style guide acronyms in type names should be
CamelCase.

[breaking-change]
```


- auto merge of #14427 : alexcrichton/rust/librand, r=huonw

```
This commit shuffles around some of the `rand` code, along with some
reorganization. The new state of the world is as follows:

* The librand crate now only depends on libcore. This interface is experimental.
* The standard library has a new module, `std::rand`. This interface will
  eventually become stable.

Unfortunately, this entailed more of a breaking change than just shuffling some
names around. The following breaking changes were made to the rand library:

* Rng::gen_vec() was removed. This has been replaced with Rng::gen_iter() which
  will return an infinite stream of random values. Previous behavior can be
  regained with `rng.gen_iter().take(n).collect()`

* Rng::gen_ascii_str() was removed. This has been replaced with
  Rng::gen_ascii_chars() which will return an infinite stream of random ascii
  characters. Similarly to gen_iter(), previous behavior can be emulated with
  `rng.gen_ascii_chars().take(n).collect()`

* {IsaacRng, Isaac64Rng, XorShiftRng}::new() have all been removed. These all
  relied on being able to use an OSRng for seeding, but this is no longer
  available in librand (where these types are defined). To retain the same
  functionality, these types now implement the `Rand` trait so they can be
  generated with a random seed from another random number generator. This allows
  the stdlib to use an OSRng to create seeded instances of these RNGs.

* Rand implementations for `Box<T>` and `@T` were removed. These seemed to be
  pretty rare in the codebase, and it allows for libcore to not depend on
  liballoc.  Additionally, other pointer types like Rc<T> and Arc<T> were not
  supported.  If this is undesirable, librand can depend on liballoc and regain
  these implementations.

* The WeightedChoice structure is no longer built with a `Vec<Weighted<T>>`,
   but rather a `&mut [Weighted<T>]`. This means that the WeightedChoice
   structure now has a lifetime associated with it.

cc #13851

[breaking-change]
```


- std: Recreate a `rand` module

```
This commit shuffles around some of the `rand` code, along with some
reorganization. The new state of the world is as follows:

* The librand crate now only depends on libcore. This interface is experimental.
* The standard library has a new module, `std::rand`. This interface will
  eventually become stable.

Unfortunately, this entailed more of a breaking change than just shuffling some
names around. The following breaking changes were made to the rand library:

* `Rng::gen_vec()` was removed. This has been replaced with `Rng::gen_iter()`
  which will return an infinite stream of random values. Previous behavior can
  be regained with `rng.gen_iter().take(n).collect()`

* `Rng::gen_ascii_str()` was removed. This has been replaced with
  `Rng::gen_ascii_chars()` which will return an infinite stream of random ascii
  characters. Similarly to `gen_iter()`, previous behavior can be emulated with
  `rng.gen_ascii_chars().take(n).collect()`

* {IsaacRng, Isaac64Rng, XorShiftRng}::new() have all been removed. These all
  relied on being able to use an OSRng for seeding, but this is no longer
  available in librand (where these types are defined). To retain the same
  functionality, these types now implement the `Rand` trait so they can be
  generated with a random seed from another random number generator. This allows
  the stdlib to use an OSRng to create seeded instances of these RNGs.

* Rand implementations for `Box<T>` and `@T` were removed. These seemed to be
  pretty rare in the codebase, and it allows for librand to not depend on
  liballoc.  Additionally, other pointer types like Rc<T> and Arc<T> were not
  supported.  If this is undesirable, librand can depend on liballoc and regain
  these implementations.

* The WeightedChoice structure is no longer built with a `Vec<Weighted<T>>`,
  but rather a `&mut [Weighted<T>]`. This means that the WeightedChoice
  structure now has a lifetime associated with it.

* The `sample` method on `Rng` has been moved to a top-level function in the
  `rand` module due to its dependence on `Vec`.

cc #13851

[breaking-change]
```


- Rename UTF16Item[s] to Utf16Item[s]

```
According to Rust's style guide acronyms should be CamelCase.

[breaking-change]
```


- auto merge of #14364 : alexcrichton/rust/libdebug, r=brson

```
This commit moves reflection (as well as the {:?} format modifier) to a new
libdebug crate, all of which is marked experimental.

This is a breaking change because it now requires the debug crate to be
explicitly linked if the :? format qualifier is used. This means that any code
using this feature will have to add `extern crate debug;` to the top of the
crate. Any code relying on reflection will also need to do this.

Closes #12019

[breaking-change]
```


- Move std::{reflect,repr,Poly} to a libdebug crate

```
This commit moves reflection (as well as the {:?} format modifier) to a new
libdebug crate, all of which is marked experimental.

This is a breaking change because it now requires the debug crate to be
explicitly linked if the :? format qualifier is used. This means that any code
using this feature will have to add `extern crate debug;` to the top of the
crate. Any code relying on reflection will also need to do this.

Closes #12019

[breaking-change]
```

- std: Rename strbuf operations to string
- std: Remove `String::from_owned_str` as it's redundant


## Other Changes

- Primitive types [now have a home in the
  documnetation](https://github.com/mozilla/rust/pull/14513), where their
  methods will be listed, and the traits they implement.
- References to owned vectors in the docs [have been mercilessly
  destroyed](https://github.com/mozilla/rust/pull/14553).
- There is now a function [for parsing PATH environment variable-looking
  strings](https://github.com/mozilla/rust/pull/14544) into their component
  paths.
- `Rc::make_unique` [has been
  added](https://github.com/mozilla/rust/pull/14522).
- Debuginfo representation of enums [has
  improved](https://github.com/mozilla/rust/pull/14486) to be more consistent.
- Macros [can now be expanded in
  patterns](https://github.com/mozilla/rust/pull/14298).
- There is now a lint [that warns when one enum variant is vastly larger than
  the others](https://github.com/mozilla/rust/pull/14300). It is allow by
  default.
- Regular expressions [can now be stored in
  statics](https://github.com/mozilla/rust/pull/14423).
- rustdoc [will now inline
  documentation](https://github.com/mozilla/rust/pull/14391) for reexported
  items.
- A new [language design
  FAQ](http://doc.rust-lang.org/complement-design-faq.html) [has been
  added](https://github.com/mozilla/rust/pull/14370).

## New Contributors

- Ahmed Charles
- Ariel Ben-Yehuda
- Christoph Burgdorf
- Florian Hartwig
- Jonathan Reem
- Nikita Pekin
- Randati
- Reilly Watson
- Ryman
- Santiago Rodriguez
- Sean Gillespie
- Sergio Benitez
- Sylvestre Ledru
- Tom Jakubowski
- Utkarsh Kukreti
- fort

# New RFCs

- [Coercible and HasPrefix for Zero Cost
  Coercions](https://github.com/rust-lang/rfcs/pull/91)
- [Disambiguate enum variant
  names](https://github.com/rust-lang/rfcs/pull/94)
- [Filling in the details around unboxed
  closures](https://github.com/rust-lang/rfcs/pull/97)
- [Uninitialized pointers](https://github.com/rust-lang/rfcs/pull/98)
- [Add a `partial_cmp` method to
  `TotalOrd`](https://github.com/rust-lang/rfcs/pull/100)
- [Allow multiple fixed-size subslices borrows in one
  pattern](https://github.com/rust-lang/rfcs/pull/101)
- [Auto-dereferencing non-raw
  pointers](https://github.com/rust-lang/rfcs/pull/102)
- [Add a freestanding target](https://github.com/rust-lang/rfcs/pull/106)
- [Pattern guards with
  bind-by-move](https://github.com/rust-lang/rfcs/pull/107)
- [Convenience syntax for module
  imports](https://github.com/rust-lang/rfcs/pull/108)
- [Remove `#[crate_id]`](https://github.com/rust-lang/rfcs/pull/109)
- [Resolve `::foo::...` when compiling crates named
  `foo`](https://github.com/rust-lang/rfcs/pull/110)
- [Index traits](https://github.com/rust-lang/rfcs/pull/111)
- [Remove cross-borrowing](https://github.com/rust-lang/rfcs/pull/112)
- [Provide a common API across `Option` and the `Ok` and `Err` variants of
  `Result`](https://github.com/rust-lang/rfcs/pull/113)

# Community Updates

- [Are we web yet? Not
  really.](http://www.reddit.com/r/rust/comments/26oxl4/are_we_web_yet_not_really/)
- [We need hardware traps for integer
  overflow](http://www.reddit.com/r/rust/comments/26ss21/embedded_in_academia_we_need_hardware_traps_for/)
- [Piston game engine: update on
  progress](http://www.reddit.com/r/rust/comments/26u192/piston_game_engine_update_notice_on_progress/)
- [Practicality with Rust: Error
  Handling](http://www.reddit.com/r/rust/comments/26xt8j/practicality_with_rust_error_handling/)
- [Debug validation vs
  safety](http://www.reddit.com/r/rust/comments/26z1xa/debug_validation_vs_safety/)
- [rust-core now deprecated in favor of libcore
  upstream](https://github.com/thestinger/rust-core)
- [Emacs Rust Development
  Setup](http://bjbell.wordpress.com/2014/05/31/emacs-rust-development-setup/)
- [What was the result of the
  mutpocalypse?](http://www.reddit.com/r/rust/comments/273gq2/what_was_the_result_of_the_mutpocalypse/)
  I am happy that it just fizzled out and died.
- [Swift - initial commentary from Graydon
  Hoare](http://www.reddit.com/r/rust/comments/276n30/swift_initial_commentary_from_graydon_hoare/)
- [Systems Programming in 2014 and
  Beyond](http://channel9.msdn.com/Events/Lang-NEXT/Lang-NEXT-2014/Panel-Systems-Programming-Languages-in-2014-and-Beyond),
  a panel discussion featuring our very own Niko Matsakis, as well as Andrei
  Alexandrescu (D developer), Rob Pike, and Bjarne Stroustroup.
- [Open Call for Contributions: 7 High Priority Rust Libraries That Need To Be
  Written](http://www.reddit.com/r/rust/comments/27cphk/an_open_call_for_contributions_7_high_priority/)
- [Piston 0.1
  Released](http://www.reddit.com/r/rust/comments/27ig84/piston_01_released/)
- [The definitive, end-all source for why Rust is named
  "Rust"](http://www.reddit.com/r/rust/comments/27jvdt/internet_archaeology_the_definitive_endall_source/)
- [A tool for viewing `RUST_LOG` output](http://cmr.github.io/rust-log-viewer/)
- [Snowmew's Architecture: Part
  1](http://www.reddit.com/r/rust/comments/27qn6y/snowmews_architecture_part_1/)

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

In the last three weeks, we landed 77 PRs.

## Notable additions

- Patrick Walton improved Wikipedia paint time by 40x through display list optimization.
- Jack and many others landed a rust upgrade.
- Patrick Walton added reference counted flows, on the path to incremental layout support.
- Edit Balint reduced the number of times that we invoke rustc during the Servo build.
- Martin Robinson fixed many Linux rendering issues, especially on NVidia cards.

## New Contributors

- Edit Balint (ebalint)

## Meetings and Notes

There was a [workweek](https://github.com/mozilla/servo/wiki/Workweek-roadmap)
last week, and our previous
[two](https://github.com/mozilla/servo/wiki/Meeting-2014-05-19)
[meetings](https://github.com/mozilla/servo/wiki/Meeting-2014-05-13) covered
the Rust upgrade, 32-bit support, the HTML parser, and gfx rendering.
