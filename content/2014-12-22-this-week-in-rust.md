Title: This Week in Rust 62
Date: 2014-12-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on Github](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

149 pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-15..2014-12-22

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* Macros using parens and square brackets (`macro!()`, macro![]`) are
  [parsed as expressions][mac] if not followed by a semicolon. This
  makes expressions like `vec![1i, 2, 3].len();` work as
  expected. [RFC][mac-rfc].
* [Binops take their arguments by value][binops], including `Add`,
  `Sub`, `Mul`, `Div` `Rem`, `BitAnd`, `BitOr`, `BitXor`, `Shl`,
  `Shr`. This breaks all existing implementations!  `String` and `Vec`
  addition now reuse the LHS buffer. [RFC][binops-rfc].
* The `Neg` and `Not` unary ops [also take their arguments by
  value][unops].
* [A number of changes to the runtime appeared][rt], including
  renaming `std::task` to `std::thread`, that maps more directly to OS
  threads.
* The in-tree `getopts`, `log`, `regex`, and `regex_macros` crates are
  [deprecated][crates] in favor of the ones from `crates.io`. Nearly
  everything that isn't std is moving to the registry.
* Command-line flags to `rustc` are [being adjusted][rustcflags] with
  an eye towards stabilizing and futureproofing. See PR for details.
* `TreeMap`, `TreeSet`, `TrieMap`, `TrieSet`, `LruCache` and `EnumSet`
  have all been [removed from the `collections` crate][rmcoll], and
  can for now live in [collect-rs]. [RFC][rmcoll-rfc].
* The `VecMap` iterators are now [newtypes instead of type
  aliases][vecmap] to encapsulate the implementation.
* The `BTreeMap`, `BTreeSet`, `HashMap`, and `HashSet` iterators are
  [also newtypes][morenewtypes]..
* [`is_power_of_two` no longer incorrectly considers 0 to be a power
  of 2][two].
* `regex::Captures::at` and `Captures::name` [return `Option`][regex].
* Many, many public reexports of enum variants [have been removed][enums].
* The `HashSet` iterators [have been
  renamed](https://github.com/rust-lang/rust/pull/19993).

[binops]: https://github.com/rust-lang/rust/pull/19448
[binops-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0439-cmp-ops-reform.md
[unops]: https://github.com/rust-lang/rust/pull/19899
[rt]: https://github.com/rust-lang/rust/pull/19654
[two]: https://github.com/rust-lang/rust/pull/19640
[vecmap]: https://github.com/rust-lang/rust/pull/19720
[morenewtypes]: https://github.com/rust-lang/rust/pull/19770
[regex]: https://github.com/rust-lang/rust/pull/19818
[crates]: https://github.com/rust-lang/rust/pull/19820
[rustcflags]: https://github.com/rust-lang/rust/pull/19900
[rmcoll]: https://github.com/rust-lang/rust/pull/19955
[collect-rs]: https://github.com/Gankro/collect-rs/
[rmcoll-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0509-collections-reform-part-2.md
[mac]: https://github.com/rust-lang/rust/pull/19984
[mac-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0378-expr-macros.md
[enums]: https://github.com/rust-lang/rust/pull/19842

## Other Changes

* Backtraces are [demangled correctly on Windows][bt].
* `RingBuf` now exposes its buffers via the [`as_slices`
  method][as_slices].
* A number of collections implement [a new method called
  `drain`][drain] which removes all members of the collection without
  deallocating the underlying buffers.
* Work is progressing on [generalized where clauses][where] which is
  necessary for associated types to be fully useful. [RFC][where-rfc].

[bt]: https://github.com/rust-lang/rust/pull/19819
[as_slices]: https://github.com/rust-lang/rust/pull/19903
[drain]: https://github.com/rust-lang/rust/pull/19946
[where]: https://github.com/rust-lang/rust/pull/20073
[where-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0135-where.md

## New Contributors

* Aaron Friel
* Akos Kiss
* Andrew Wagner
* Eric Kidd
* Jacob Edelman
* Jake Goulding
* Jared Roesch
* Ken Tossell
* Kevin Yap
* Martin Pool
* Mathieu Poumeyrol
* Mike Pedersen
* Niels Egberts
* Pedro Larroy
* Sean Collins
* Seth Pollack
* mchaput

# Approved RFC's

- [503: prelude stabilization][503]. Removes a ton of stuff from the prelude,
  and moves what remains to a `std::prelude::v1` module.
- [474: path reform][474]. Tightens up the `Path` API, and makes some changes
  around their internal representation.
- [504: `Show` stabilization][504]. Splits `Show` into `Show` and `String`,
  with `Show` being tied to the re-introduced `{:?}` format string specifier
  for debugging purposes.
- [453: macro reform][453]. Tightens up how macro import/export "works", and
  solves the "curious inner module" problem with a `$crate` meta-variable.
- [486: `std::ascii` reform][486]. Removes the `Ascii` newtype and implements
  the old functionality on various forms of arrays of `u8` instead.
- [509: collections reform part 2][509]. Finalizes the fate of
  `std::collections`, marking some as stable and moving some to the external
  `collect-rs` crate.
- [520: new array repeat syntax][520]. Due to an ambiguity with range
  notation, the fixed-length array syntax has changed to `[T; N]` for the type
  and `[expr; N]` for the constructor.
- [522: Allow `Self` to be used in impls][522]. As it sounds. Reduces
  verbosity.

[520]: https://github.com/rust-lang/rfcs/blob/master/text/0520-new-array-repeat-syntax.md
[522]: https://github.com/rust-lang/rfcs/blob/master/text/0522-self-impl.md
[509]: https://github.com/rust-lang/rfcs/blob/master/text/0509-collections-reform-part-2.md
[486]: https://github.com/rust-lang/rfcs/blob/master/text/0486-std-ascii-reform.md
[504]: https://github.com/rust-lang/rfcs/blob/master/text/0504-show-stabilization.md
[503]: https://github.com/rust-lang/rfcs/blob/master/text/0503-prelude-stabilization.md
[474]: https://github.com/rust-lang/rfcs/blob/master/text/0474-path-reform.md
[453]: https://github.com/rust-lang/rfcs/blob/master/text/0453-macro-reform.md

# New RFC's

- [Mark `std::mem::drop` as unstable until negative bounds are
  implemented](https://github.com/rust-lang/rfcs/pull/536).
- [Rename `std::mem::drop`](https://github.com/rust-lang/rfcs/pull/535).
- [Rename the `deriving` syntax extension to
  `derive`](https://github.com/rust-lang/rfcs/pull/534).
- [Remove certain array elem moves to prepare for non-zeroing
  drop](https://github.com/rust-lang/rfcs/pull/533).
- [`self` in `use`](https://github.com/rust-lang/rfcs/pull/532).
- [Amend RFC process with a defined
  scope](https://github.com/rust-lang/rfcs/pull/531).
- [Generic conversion traits](https://github.com/rust-lang/rfcs/pull/529).
- [Generic string pattern matching
  API](https://github.com/rust-lang/rfcs/pull/528).
- [Statically enforce Unicode in
  `std::fmt`](https://github.com/rust-lang/rfcs/pull/526).

# Community

The [video from the Thursday's SF Meetup][meetup] about crypto has
several interesting presentations.

[meetup]: https://air.mozilla.org/bay-area-rust-meetup-december-2014/

## From the Team

* [Weekly-meetings/2014-12-16][mtg]:  [Reddit][mtg-reddit].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-12-16.md
[mtg-reddit]: http://www.reddit.com/r/rust/comments/2pjmve/weekly_meeting_20141216/

## Blog Posts

* [Diving into concurrency][jvns]: trying out mutexes and atomics (in Rust and
  C). [Reddit][jvns-reddit].
* [Comparing Rust and C++][kuku]. [Reddit][kuku-reddit]. Note that this
  appears to be a translation of an [older post in
  Russian](http://habrahabr.ru/post/225003/).
* [Updating Github Pages with Travis
  CI](http://words.steveklabnik.com/update-github-pages-with-travis-ci).
  [Reddit](http://www.reddit.com/r/rust/comments/2phrge/updating_github_pages_with_travis_ci/)
* [This Week in Servo 15](http://blog.servo.org/2014/12/15/twis-15/).
  [Reddit](http://www.reddit.com/r/rust/comments/2pj6bv/this_week_in_servo_15/).
* [Columnarization in
  Rust](http://www.frankmcsherry.org/columnarization/serialization/rust/2014/12/15/Columnarization-in-Rust.html).
  [Reddit](http://www.reddit.com/r/rust/comments/2pozxt/columnarization_in_rust/).
* [Introducing Hyper](http://seanmonstar.com/post/105541782562/hyper).
  [Reddit](http://www.reddit.com/r/rust/comments/2pqnjn/hyper_intro/).
* [Rewriting Rust Serialization, Part 3.1: Another Performance
  Digression](http://erickt.github.io/blog/2014/12/13/performance-digression/).
  [Reddit](http://www.reddit.com/r/rust/comments/2pxx52/rewriting_serialization_part_31_another/).

[jvns]: http://jvns.ca/blog/2014/12/14/fun-with-threads/
[jvns-reddit]: http://www.reddit.com/r/rust/comments/2pabnd/diving_into_concurrency_trying_out_mutexes_and/
[kuku]: http://kukuruku.co/hub/rust/comparing-rust-and-cpp
[kuku-reddit]: http://www.reddit.com/r/rust/comments/2pbzug/comparing_rust_and_c/


## 24 Days of Rust continues!

* [nalgebra](https://siciarz.net/24-days-of-rust-nalgebra/).
  [Reddit](http://www.reddit.com/r/rust/comments/2pa7md/24_days_of_rust_nalgebra/)
* [FUSE filesystems, part
  1](https://siciarz.net/24-days-of-rust-fuse-filesystems-part-1/).
  [Reddit](http://www.reddit.com/r/rust/comments/2pdxbe/24_days_of_rust_fuse_filesystems_part_1/).
* [FUSE filesystems, part
  2](https://siciarz.net/24-days-of-rust-fuse-filesystems-part-2/).
* [`from_fn`](https://siciarz.net/24-days-of-rust-from_fn/).
  [Reddit](http://www.reddit.com/r/rust/comments/2pljk9/24_days_of_rust_from_fn/).
* [redis](https://siciarz.net/24-days-of-rust-redis/).
  [Reddit](http://www.reddit.com/r/rust/comments/2ppltp/24_days_of_rust_redis/).
* [zeromq](https://siciarz.net/24-days-of-rust-zeromq/).
  [Reddit](http://www.reddit.com/r/rust/comments/2pwt70/24_days_of_rust_zeromq/).
* [rusti](https://siciarz.net/24-days-of-rust-rusti/)
* [rust-crypto](https://siciarz.net/24-days-of-rust-rust-crypto/).
  [Reddit](http://www.reddit.com/r/rust/comments/2q07jv/24_days_of_rust_rustcrypto/).

## Discussions

* [Anyone working on C/C++ to Rust
  transpilers?](http://www.reddit.com/r/rust/comments/2pyg8f/anyone_working_on_cc_to_rust_transpilers/)
* [Rust ruined C++ for
  me](http://www.reddit.com/r/rust/comments/2pi3ju/rust_ruined_c_for_me/)
* [Well written rust code to read and learn
  from?](http://www.reddit.com/r/rust/comments/2pmaqz/well_written_rust_code_to_read_and_learn_from/)

## New Projects

* [rusql](https://github.com/mttr/rusql), "A naive, SQL based RDBMS written in
  Rust."
* [Rusthon](https://github.com/rusthon/Rusthon), "a python-like language that
  converts and compiles into: Rust, C++, and JavaScript."
* [rbencode](https://github.com/asamy45/rbencode), a Bencode implementation.
* [wtftw](https://github.com/Kintaro/wtftw), "Window Tiling For The Win. A
  tiling window manager written in Rust."
* [Custom Logger with time-stamp, file name and line
  number](http://joshitech.blogspot.com/2014/12/rust-customer-logger.html).
* [cpal](https://crates.io/crates/cpal), pure Rust audio renderer.
* [rusti reborn](https://github.com/murarth/rusti), a REPL for Rust
* [nadeko](https://github.com/klutzy/nadeko), "an experimental syntax
  extension which converts functions into amd64 assembly code."
* [traverse](https://github.com/reem/rust-traverse), "proof-of-concept trait
  for internal iterators called traversals."

## Upcoming Events

Nothing on the calendar for the next two weeks!
