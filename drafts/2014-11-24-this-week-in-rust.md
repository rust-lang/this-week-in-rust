Title: This Week in Rust 58
Date: 2014-11-24
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

The big news this week was that the [central crate
repository][crates.io] is [now live][crates.io-blog].  There was much
discussion on [Hacker News][crates.io-hn], [/r/rust][crates.io-reddit1] and
[/r/programming][crates.io-reddit2].

[crates.io]: https://crates.io/
[crates.io-blog]: http://blog.rust-lang.org/2014/11/20/Cargo.html
[crates.io-hn]: http://news.ycombinator.com/item?id=8637493
[crates.io-reddit1]: https://www.reddit.com/r/rust/comments/2mwice/cratesio_has_shipped/
[crates.io-reddit2]: https://www.reddit.com/r/programming/comments/2mwidh/rusts_central_package_repository_is_up/

We also have a new [guide to error handling][error].

[error]: http://doc.rust-lang.org/guide-error-handling.html

# What's cooking on master?

xxx pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-17..2014-11-23

## Breaking Changes

* Enum variants are now [namespaced by their type name][enumns], a
  major change. Enums in the standard library have been reexported in
  their old locations, for now at
  least. [RFC][enumns-rfc]. [Reddit][enumns-reddit].
* As part of ongoing [DST-ification][dst], `AsSlice` has been
  [modified to work with unsized types][unsized-asslice]. This breaks
  existing code because `AsSlice` type vars should now be taken by
  reference. At the same time, some of the ops have been extended to
  work with unsized types.
* The `str` method `slice_shift_char`'s return type [has changed
  slightly][slice_shift_char].
* The `find_copy` and `get_copy` methods of `HashMap` [are
  deprecated][cloned].  `find_copy` can be performed with
  `map.get(&key).cloned()`, which converts the `Option<&T>` returned
  by `get` to `Option<T>`, and `get_copy` to simply
  `map[key].clone()`, which calls `.clone()` on the value returned by
  reference from the indexing operator.
* [Runtime removal continues][rt]. Breaking changes here are mostly
  to code invoking the runtime directly.
* The `overloaded_calls` and `unboxed_closure_sugar` feature gates
  [have been combined][gates] into a single `unboxed_closures` gate.
* Formatting has [seen a stability pass][fmt] with some minor breaking
  changes. [RFC][fmt-rfc].
* The little-used `col!` macro is [renamed to `column!`][column].
* Non-ASCII lifetime identifiers are [behind the `non_ascii_idents`
  feature gate][ascii] as intended.
* [Struct variants can not be matched as if they were tuple
  variants][varmatch].

[enumns]: https://github.com/rust-lang/rust/pull/18973
[enumns-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0390-enum-namespacing.md
[enumns-reddit]: https://www.reddit.com/r/rust/comments/2ml4oo/switch_to_purely_namespaced_enums/
[dst]: http://smallcultfollowing.com/babysteps/blog/2014/01/05/dst-take-5/
[unsized-asslice]: https://github.com/rust-lang/rust/pull/18638
[slice_shift_char]: https://github.com/rust-lang/rust/pull/18911
[cloned]: https://github.com/rust-lang/rust/pull/18914
[rt]: https://github.com/rust-lang/rust/pull/18967
[gates]: https://github.com/rust-lang/rust/pull/18993
[fmt]: https://github.com/rust-lang/rust/pull/19040
[fmt-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0380-stabilize-std-fmt.md
[column]: https://github.com/rust-lang/rust/pull/19071
[ascii]: https://github.com/rust-lang/rust/pull/19073
[varmatch]: https://github.com/rust-lang/rust/pull/19087

## Other Changes

* Rust now [supports higher-ranked trait bounds][hrtb]. This is
  necessary to make unboxed closures work, but the implications are
  quite complex. Read the [RFC][hrtb-rfc] for details.
* `Vec` [implements `Writer`][vec-writer] and `MemWriter` is deprecated.
* All statics now [support the `#[linkage]` attribute][linkage], which
  is behind the `linkage` feature gate.
* Parts of rustc have been [pulled into a new `rustc_trans`
  crate][rustc_trans] to reduce memory pressure.
* All idents following literals are [tokenized specially][litid] now
  as future proofing. [RFC][litid-rfc].
* [Unboxed closures can be written with the sugared syntax][sugar].

[hrtb]: https://github.com/rust-lang/rust/pull/18993
[hrtb-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md
[vec-writer]: https://github.com/rust-lang/rust/pull/18885
[linkage]: https://github.com/rust-lang/rust/pull/18890
[rustc_trans]: https://github.com/rust-lang/rust/pull/19070
[litid]: https://github.com/rust-lang/rust/pull/19103
[litid-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0463-future-proof-literal-suffixes.md
[sugar]: https://github.com/rust-lang/rust/pull/19113

## New Contributors




# Approved RFC's
* [Higher-ranked trait bounds][rfc387]: Makes the type-system powerful enough to deal with
unboxed closures as well as boxed ones.
* [RFC to restrict placing an identifier after a literal.][rfc463]: Future-proofs syntax for literals
* [Finalizing more naming conventions][rfc430]: snake_case-type stuff and `unwrap` vs `into_inner`
* [Change precedence of `+` in type grammar][rfc438]: Cleans up a some weird parsing interactions
* [cmp and ops reform][rfc439]: DST-ify operators, add multi-dispatch, make them by-value for
flexibility, add IndexSet, unify slice operators with `..` notation sugaring to special types,
kill Equiv
* [Add a thread local storage module, std::tls][rfc461]: Introduces Scoped TLS and Owned TLS in a
new thread_local module to replace the old design. Should have better perf, be more flexible.

[rfc387]: https://github.com/rust-lang/rfcs/pull/387
[rfc463]: https://github.com/rust-lang/rfcs/pull/463
[rfc430]: https://github.com/rust-lang/rfcs/pull/430
[rfc438]: https://github.com/rust-lang/rfcs/pull/438
[rfc439]: https://github.com/rust-lang/rfcs/pull/439
[rfc461]: https://github.com/rust-lang/rfcs/pull/461


# New RFC's

* [Release channels and feature staging][rfc475]: This RFC describes changes to the Rust release
process, primarily the division of Rust's time-based releases into 'release channels', following
the 'release train' model used by e.g. Firefox and Chrome; as well as 'feature staging', which
enables the continued development of experimental language features and libraries APIs while
providing strong stability guarantees in stable releases.
* [path reform][rfc474]: This RFC reforms the design of the std::path module in preparation for API
stabilization. The path API must deal with many competing demands, and the
current design handles many of them, but suffers from some significant problems
given in "Motivation" below. The RFC proposes a redesign modeled loosely on the
current API that addresses these problems while maintaining the advantages of
the current design.
* [placement box with Placer trait for overloading][rfc470]: Add user-defined placement in
expression (more succinctly, "an in expression"), an operator analogous to "placement new"
in C++. This provides a way for a user to specify (1.) how the backing storage for some
datum should be allocated, (2.) that the allocation should be ordered before the evaluation
of the datum, and (3.) that the datum should preferably be stored directly into the backing
storage (rather than allocating temporary storage on the stack and then copying the datum
from the stack into the backing storage).
* [Feature gate box patterns][rfc469]: Move box patterns behind a feature gate.The general idea is
good, but the semantics aren't baked enough for 1.0.
* [Add "function name macro"][rfc466]: This RFC proposes the addition of a function! macro that
expands to the function it's used in. This will greatly help error reporting.

[rfc474]: https://github.com/rust-lang/rfcs/pull/474
[rfc475]: https://github.com/rust-lang/rfcs/pull/475
[rfc470]: https://github.com/rust-lang/rfcs/pull/470
[rfc469]: https://github.com/rust-lang/rfcs/pull/469
[rfc466]: https://github.com/rust-lang/rfcs/pull/466

# Community

Karen Rustad found a wild [rustacean](https://twitter.com/whoisaldeka/status/535679593353854976).

## From the Team

* [Weekly-meetings/2014-11-18][mtg]: cmp/ops; TLS; future-proofing
  literal parsing; ungating tuple indexing, if/while let; naming
  conventions; struct variants matching; for syntax for higher-order
  lifetimes; macros; type parameter grammar; better shepherding
  [Reddit][mtg-reddit].
* [Cargo: Rust's community crate host][cargo].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-18.md
[mtg-reddit]: https://www.reddit.com/r/rust/comments/2mwdhn/weekly_meeting_20141118/
[cargo]: http://blog.rust-lang.org/2014/11/20/Cargo.html

## Videos

* [Introduction to programming safe and efficient systems in Rust][vid]. Jakub Bukaj at Øredev.

[vid]: http://vimeo.com/111852387

## Blog Posts

* [Rust, Lifetimes, and Collections][coll]. Alexis Beingessner (aka
  Gankro), who has been driving the current modernization of the
  collections libs, talks about the design of collections in
  Rust. [Reddit][coll-reddit]. [HN][coll-hn].
* [Chasing an EPROTOTYPE Through Rust, Sendto, and the OSX Kernel With
  C-Reduce][erickt]. [Reddit][erickt-reddit].
* [Rust tools][tools]. Nick Cameron has been thinking about the state
  of Rust tooling. [Reddit][tools-reddit].
* [Roguelike Tutorial in Rust: Part 5: Combat! Part III][rogue]. [Reddit][rogue-reddit].

[coll]: http://cglab.ca/~abeinges/blah/rust-lifetimes-and-collections/
[coll-hn]: http://news.ycombinator.com/item?id=8629789
[coll-reddit]: https://www.reddit.com/r/rust/comments/2mqwdm/rust_lifetimes_and_collections/
[erickt]: https://erickt.github.io/blog/2014/11/19/adventures-in-debugging-a-potential-osx-kernel-bug/
[erickt-reddit]: https://www.reddit.com/r/rust/comments/2mslk8/chasing_an_eprototype_through_rust_sendto_and_the/
[tools]: https://gist.github.com/nick29581/a3bbf6dd1b14ce57f18c
[tools-reddit]: https://www.reddit.com/r/rust/comments/2mqkwk/rust_tools/
[rogue]: http://jaredonline.svbtle.com/roguelike-tutorial-in-rust-part-5
[rogue-reddit]: https://www.reddit.com/r/rust/comments/2mlq0h/roguelike_tutorial_in_rust_part_5_combat_part_iii/

## Discussions

* [What are the advantages of Rust over modern C++?][c++]
* [What does 'unwrap' mean in Rust?][unwrap]
* [How would a Rust application be able to properly react to low level failures, like memory allocation failure?][ll]
* [The Race Towards 1.0 and The Standard Library][race].
* [Is Rust recommended for beginners with 0 programming language experience?][newb]. TL;DR it depends.

[c++]: https://www.reddit.com/r/rust/comments/2mwpie/what_are_the_advantages_of_rust_over_modern_c/
[unwrap]: https://www.reddit.com/r/rust/comments/2mw2ns/what_does_unwrap_mean_in_rust/
[ll]: https://www.reddit.com/r/rust/comments/2mthq2/how_would_a_rust_application_be_able_to_properly/
[race]: https://www.reddit.com/r/rust/comments/2mo0zb/the_race_towards_10_and_the_standard_library/
[newb]: https://www.reddit.com/r/rust/comments/2mlq18/is_rust_recommended_for_beginners_with_0/

## New Projects

* [rust-eh]. Python-like error tracebacks, from mitsuhiko.
* `sl`, the classic Unix command, [in Rust][sl].
* [img_hash]. Perceptual hashing of images.
* [raw-rs]. Utilities for manipulation of Rust core types.
* [rust-uchardet]. Encoding detection.
* [Rust-Relay]. IRC client library. Start your bots!
* [cargo-do]. Cargo plugin for running multiple commands at once.

[rust-eh]: https://www.reddit.com/r/rust/comments/2mjqzi/rusteh_python_like_error_tracebacks_for_rust/
[sl]: https://www.reddit.com/r/rust/comments/2mrep5/sl1_reimplementation_in_rust/
[img_hash]: https://www.reddit.com/r/rust/comments/2mq3dg/img_hash_a_simple_rust_library_for_performing/
[raw-rs]: https://www.reddit.com/r/rust/comments/2mp0il/rawrs_utilities_for_unsafe_manipulation_of_core/
[rust-uchardet]: https://www.reddit.com/r/rust/comments/2mpedc/rustuchardet_encoding_detection_wrapper_using_ffi/
[Rust-Relay]: https://www.reddit.com/r/rust/comments/2miyr2/rustrelay_an_ircv3_client_library_looking_for/
[cargo-do]: https://www.reddit.com/r/rust/comments/2mwzah/cargo_subcommand_plugin_to_run_multiple_commands/

## Project Updates

* [This Week in Servo 12][twis].
* [Glium's design choices][glium]. Glium is a safe OpenGL wrapper.

[twis]: http://blog.servo.org/2014/11/18/twis-12/
[glium]: https://www.reddit.com/r/rust_gamedev/comments/2mkbs9/gliums_design_choices/

