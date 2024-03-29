Title: This Week in Rust 7
Date: 2013-07-21 09:36
Category: This Week in Rust

Hello and welcome to the seventh issue of *This Week in Rust*, a weekly
overview of Rust and its community. Things are calming down quite a bit, in
that nothing extraordinarily exciting is happening. Lots of great work is
being done everywhere, and good progress is being made in both bugfixes and
cleanup. It has been a good week!

<!-- more -->

# What's cooking on master?

Issue churn this week was -15. A total of 59 PRs were merged.

## Breaking changes

- **[The semantics of `range_rev` have
  changed](https://github.com/mozilla/rust/pull/7684). This will break your
  code without warning.** It is now, to use [interval
  notation](https://en.wikipedia.org/wiki/Interval_%28mathematics%29#Notations_for_intervals),
  `(hi, lo]` rather than `[hi, lo)`.
- `pub extern` and `priv extern` [have been
  removed](https://github.com/mozilla/rust/pull/7896) from the language. This
  matches the previous similar change to `impl`. Place the visibility
  qualifier (`pub`/`priv`) on each item in the `extern` block instead.
- `&T` (besides `&'static T`) is [no longer allowed in
  `@T`](https://github.com/mozilla/rust/pull/7894).
- The `ThreadPerCore` spawn mode [has been
  removed](https://github.com/mozilla/rust/pull/7856), as it doesn't make
  sense with the new scheduler.
- The `consume` methods of the hash containers [has been replaced with an
  external iterator](https://github.com/mozilla/rust/pull/7833). The method
  name is the same, though.
- Moved values can [no longer be captured
  twice](https://github.com/mozilla/rust/pull/7849). This was a blatant
  soundness issue.
- The `swap_unwrap` method of Option has been [renamed to
  `take_unwrap`](https://github.com/mozilla/rust/pull/7831).
- `debug!` statements [generate no
  code](https://github.com/mozilla/rust/pull/7822) unless you pass `--cfg
  debug` to `rustc`. This should help keep code size down and make your
  programs a (tiny bit) faster. Now you don't have to feel bad about having
  `debug!` in hot code.
- The `mutate_values` method of HashMap [has been
  removed](https://github.com/mozilla/rust/pull/7815).

## Notable library additions, bugfixes, and cleanup

- An iterator adaptor was added that [endlessly repeats the iterator it is
  called on](https://github.com/mozilla/rust/pull/7882).
- Generated test runners [now have a
  `-h`/`--help`](https://github.com/mozilla/rust/pull/7840) option.
- Metric capturing + racheting [has been
  added](https://github.com/mozilla/rust/pull/7829) for benchmarks.
- `local_data` [now has a
  `get_mut`](https://github.com/mozilla/rust/pull/7841) function.
- `extra::semver` [has been updated to SemVer
  2.0.0](https://github.com/mozilla/rust/pull/7726).
- Consuming iterators [have been
  added](https://github.com/mozilla/rust/pull/7806) for the hash structures.
- `extra::ringbuf` [now implements
  DoubleEndedIterator](https://github.com/mozilla/rust/pull/7808).
- `Eq` [now has a default implementation of
  `ne`](https://github.com/mozilla/rust/pull/7799).
- `extra::term` [now knows how to handle more
  attributes](https://github.com/mozilla/rust/pull/7716).
- More containers [implement
  FromIter](https://github.com/mozilla/rust/pull/7788). This means you can use
  `.collect()` to gather the elements from an iterator into those containers.
- [Task killing, failure, and exit code
  propagation](https://github.com/mozilla/rust/pull/7858) in the new runtime
  has been implemented.

## Notable compiler additions, bugfixes, and cleanup

- `syntax::attr` [has been
  modernized](https://github.com/mozilla/rust/pull/7902).
- [Tons of debuginfo work](https://github.com/mozilla/rust/pull/7710) from mw
  this week!
- Trait data structures [have been cleaned
  up](https://github.com/mozilla/rust/pull/7886), as well as a default method
  fix.
- Intrinsics [now have much better
  codegen](https://github.com/mozilla/rust/pull/7851).
- A `no_implicit_prelude` attribute [has been
  added](https://github.com/mozilla/rust/pull/7844), which prevents prelude
  injection in the module hierarchy starting at the item which that attribute
  is added to.
- C-style enum variants are [now
  allowed](https://github.com/mozilla/rust/pull/7827) in `[T, ..n]`
  expressions.
- All language items are [now
  optional](https://github.com/mozilla/rust/pull/7828). The compiler emits an
  error if a language item is used but not provided.
- The removal of `spanned<T>` [has
  begun](https://github.com/mozilla/rust/pull/7826).
- Headers [have been removed](https://github.com/mozilla/rust/pull/7816) for
  `~str` and `~[T]` where `T` is unmanaged.

## Documentation, tools, and other stuff

- rustpkg [now works when you don't give it a package
  ID](https://github.com/mozilla/rust/pull/7419). It builds/installs/cleans
  the package in the current directory, *iff* the current directory is in a
  rustpkg workspace.
- `--quiet` is [no longer passed](https://github.com/mozilla/rust/pull/7847)
  to git during submodule operations, so you can see the progress of the huge
  LLVM download.
- Documentation of the [lint-controlling
  attributes](https://github.com/mozilla/rust/pull/7823) was added.
- rustpkg [now handles cloning from local git
  repos](https://github.com/mozilla/rust/pull/7681).
- The GtkSourceView highlighting file [was
  improved](https://github.com/mozilla/rust/pull/7795).

# Meetings

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-07-16)
discussed nothing at all of importance.

# Discussion + Blog posts

- [A pure-Rust memory allocator
  (malloc)](http://www.reddit.com/r/rust/comments/1ibd48/proofofconcept_pure_rust_malloc_implementation/)
- [mw's fourth weekly
  update](http://michaelwoerister.github.io/2013/07/12/Status-Update-4.html).
- [mw's fifth weekly
  update](http://michaelwoerister.github.io/2013/07/20/Status-Update-5.html)
- [Discussion and slides from Niko's presentation at the Northeastern
  University Programming Language
  Seminar](http://www.reddit.com/r/rust/comments/1imeac/guaranteeing_memory_safety_in_rust_niko_matsakis/)
- [A nightly Ubuntu
  PPA](http://thread.gmane.org/gmane.comp.lang.rust.devel/4829)
- [SIMD
  discussion](http://www.reddit.com/r/rust/comments/1igvye/vision_for_rust_simd/)
- [Rust on bare metal
  ARM](https://mail.mozilla.org/pipermail/rust-dev/2013-July/004841.html)
- [dherman's OSCON
  presentation](http://www.oscon.com/oscon2013/public/schedule/detail/28741)
