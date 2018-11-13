Title: This Week in Rust 44
Date: 2014-04-05 05:06
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

0.10 [was released][ten] this week, and with it comes a redesign of the
websites and official nightlies.

[ten]: https://mail.mozilla.org/pipermail/rust-dev/2014-April/009387.html

<!-- more -->

# What's cooking on master?

62 pull requests were merged this week.

## Breaking Changes

- The `map` and `flat_map` methods [have been
  removed](https://github.com/mozilla/rust/pull/13203) from `Vec<T>`, the are
  superseded by their iterator equivalents (of the same name).
- `FromIterator::from_iterator` [has been
  renamed](https://github.com/mozilla/rust/pull/13220) to `from_iter`.
- `std::vec::{append, append_one}` [have been turned into
  methods](https://github.com/mozilla/rust/pull/13221).
- struct fields [are now private by
  default](https://github.com/mozilla/rust/pull/13184). Similarly, fields of
  tuple structs [are also private by
  default](https://github.com/mozilla/rust/pull/13237).
- `std::num` [has seen some
  cleanup](https://github.com/mozilla/rust/pull/13225). The `cmath` module has
  been removed from the public API and the various wrapper functions on float
  types have been removed.
- `Rng::shuffle_mut` [has been
  renamed](https://github.com/mozilla/rust/pull/13177) to `shuffle`.
- Vectors, arrays, and slices [require `uint` indices
  now](https://github.com/mozilla/rust/pull/13257). Previously they accepted
  any integer type.
- Static string constants [no longer include a null
  terminator](https://github.com/mozilla/rust/pull/13291). This was
  accidentally leftover from when all strings were implicitly
  null-terminated. This may break code that incorrectly assumed strings were
  null-terminated.
- The `concat_idents` macro [is now feature
  gated](https://github.com/mozilla/rust/pull/13295).
- `RefCell::get` and `RefCell::set` [have been
  removed](https://github.com/mozilla/rust/pull/13301).

## Other Changes

- rustc [now gives file paths](https://github.com/mozilla/rust/pull/13284)
  when reporting duplicate crates found.
- `std::cmp` [has some nice new
  documentation](https://github.com/mozilla/rust/pull/12956).
- The manual [now has](https://github.com/mozilla/rust/pull/13207) an updated
  list of what attributes are valid and what they do.
- Built-in syntax extensions [are now
  documented](https://github.com/mozilla/rust/pull/13255).
- `AtomicInt` and `AtomicUint` [now have atomic bitwise
  operations](https://github.com/mozilla/rust/pull/12964).
- The `Show` implementation for `Cell` [has been
  fixed](https://github.com/mozilla/rust/pull/13243) to actually print the
  contents of the Cell.
- Macros in statement and expression position [can now be delimited by square
  brackets](https://github.com/mozilla/rust/pull/13234).
- The `k-nucleotide` benchmark [has been fixed to pass the official shootout
  tests](https://github.com/mozilla/rust/pull/13206).
- A nasty bug which would cause `Arc`s to leak [has been
  fixed](https://github.com/mozilla/rust/pull/13211).

## New Contributors

- Benjamin Adamson
- Christopher Kendell
- Falco Hirschenberger
- Gábor Lehel
- Ivan Petkov
- Scott Jenkins
- Timothée Ravier

# Weekly Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-04-01)
discussed Vec versus `~[T]`, the intrinsics RFC, and StrBuf.

# RFCs

Some new RFCs:

- [Check actual type parameters against their
  bounds](https://github.com/rust-lang/rfcs/pull/34)
- [Change return type of str::replace to
  MaybeOwned](https://github.com/rust-lang/rfcs/pull/33)
- [Bit fields and matching](https://github.com/rust-lang/rfcs/pull/29)
- [Use different keywords for declaring tagged unions and C-style
  enums](https://github.com/rust-lang/rfcs/pull/27)
- [Remove the `priv` keyword](https://github.com/rust-lang/rfcs/pull/26)
- [Unify and nest structs and
  enums](https://github.com/rust-lang/rfcs/pull/24)

# Project Updates

- [rustlex](https://github.com/LeoTestard/rustlex), a syntax extension for
  generating regular expression-based lexers.
- farcaller [has shown a
  demo](http://www.reddit.com/r/rust/comments/21qogc/im_making_a_note_here_huge_embedded_success/)
  of Rust on an ARM dev board.
- [A sample Asterisk
  module](http://www.reddit.com/r/rust/comments/21tplw/writing_an_asterisk_module_using_rust/),
  in Rust
- [Parallel JS
  Compression](http://alan-andrade.github.io/rust/javascript/2014/03/31/parallel_js_compression.html)
  in Rust.
- [A Brainfuck
  Interpreter](https://github.com/tedsta/rust-brainfuck/blob/master/main.rs)
  in 43 lines.
- [rustfix](https://github.com/Geal/rustfix), a simple regex-based upgrading
  tool.
- [rust-iteratorcomprehensions](http://www.reddit.com/r/rust/comments/229fze/rustiteratorcomprehensions_nest_filter_and_map/),
  a list comprehension syntax for iterators.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

In the last week, we landed 18 PRs.

## Notable additions

- Patrick Walton (along with commits from Juneyoung Cho and Hyun June Kim) landed ACID2 support in [#1988](https://github.com/mozilla/servo/pull/1988)
- ms2ger landed a new Rust upgrade, bringing us to late March in [#2041](https://github.com/mozilla/servo/pull/2041)
- Martin Robinson move the active timers for the `Window` object into a `HashMap` in [#2002](https://github.com/mozilla/servo/pull/2002)
- Manish Goregaokar made attribute getter/setters case insensitive in [#2043](https://github.com/mozilla/servo/pull/2043)
- Tetsuharu Ohzeki cleaned up `Document::create_collection` in [#2031](https://github.com/mozilla/servo/pull/2031)
- Peiyong Lin got rid of match statements in layout queries in [#2022](https://github.com/mozilla/servo/pull/2022)
- Bruno de Oliveira Abinader added support for the ASCII whitespace check in [#2032](https://github.com/mozilla/servo/pull/2032)

## New contributors

- Martin Robinson

## Meetings and Notes

In this week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-03-31), we
discussed how to demo ACID2, gave a quick overview of our Q2 agenda, talked
about the tentative Servo workweek date (June 2), and wrapped up where the
rest of the layout features are at now that we have closed down the ACID2
push.

Courtesy of Manish Goregaokar, we now have a
[badge](https://badges.mozilla.org/en-US/badges/badge/Servo-Rust-upgrade) for
those brave memebers of the Servo community who participate in the Sisyphean
task of upgrading the version of Rust used in Servo. Thanks, Manish!

# Community

- [Rust vs Go](http://www.reddit.com/r/rust/comments/21m5jf/rust_vs_go/)
- [Rust is now on Facebook](https://www.facebook.com/rustlang)
- [How similar is Rust to
  Go?](http://www.reddit.com/r/rust/comments/21ofma/how_similar_is_rust_to_go/)
- [Safe Synchronization Primitives and their
  implementation](http://www.reddit.com/r/rust/comments/21t8n8/safe_synchronization_primitives_and_their/)
- [Simple Type-Based Alias Analysis for
  Rust](http://www.reddit.com/r/rust/comments/21wu1c/simple_typebased_alias_analysis_for_rust/)
- Reminder: `~[T]` [is not going away](http://www.reddit.com/r/rust/comments/2213vw/reminder_t_is_not_going_away/)
- [Higher-kinded
  polymorpihsm](http://www.reddit.com/r/rust/comments/2212j2/higherkinded_polymorphism/)
- [A More Detailed Tour of the Rust
  Compiler](http://www.reddit.com/r/rust/comments/226ax6/a_more_detailed_tour_of_the_rust_compiler/)
- [Another brain dump - data
  structures](http://www.reddit.com/r/rust/comments/228ou3/another_brain_dump_data_structures/)
- [Rust Me, I'm a
  Developer!](http://www.reddit.com/r/rust/comments/227o5n/rust_me_im_a_developer_slides_and_commentary_from/),
  (slides and commentary from a talk given at Stir Trek 2014)
