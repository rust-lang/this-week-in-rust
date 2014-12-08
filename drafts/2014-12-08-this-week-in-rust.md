Title: This Week in Rust 60
Date: 2014-12-08
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

58 pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-01..2014-12-08

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* The [definitions of operators have changed][ops] to be more
  flexible. [RFC][ops-rfc].
* `std::sync` has been [redesigned and rewritten][sync] for the nth
  time as a result of the ongoing runtime decimation.
* `HashMap` [no longer shrinks automatically][shrink], and some
  methods for managing the capacity have changed.

[shrink]: https://github.com/rust-lang/rust/pull/18770
[ops]: https://github.com/rust-lang/rust/pull/19167
[ops-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0439-cmp-ops-reform.md
[sync]: https://github.com/rust-lang/rust/pull/19274

## Other Changes

* `if let`, `while let`, and tuple indexing are [no longer
  feature-gated][ungate]. [RFC][ungate-rfc].
* There's [a new syntax for escaping unicode characters][es6]. The old
  will be deprecated briefly. [RFC][es6-rfc].
* [`&[u8]` implements `Reader` and `&mut [u8]` implements
  `Writer`][sliceio].
* [Typechecking has been moved into its own crate][typeck].
* Many `match` expressions in the compiler were [replaced by `if
  let`][iflet], which appears to be a nice improvement in readability.
* The 'expected <foo>, found <bar>' parse errors are [much more
  accurate about what they actually expect][parse].

[sliceio]: https://github.com/rust-lang/rust/pull/18980
[typeck]: https://github.com/rust-lang/rust/pull/19362
[iflet]: https://github.com/rust-lang/rust/pull/19405/files
[ungate]: https://github.com/rust-lang/rust/pull/19472
[ungate-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0450-un-feature-gate-some-more-gates.md
[es6]: https://github.com/rust-lang/rfcs/pull/446
[es6-rfc]: https://github.com/rust-lang/rfcs/pull/446
[parse]: https://github.com/rust-lang/rust/pull/19494

## New Contributors

* Aaron Liblong
* jbranchaud
* jfager
* Jim Apple
* kulakowski
* Mukilan Thiyagarajan
* Oliver Schneider
* Paul Collier
* Victor van den Elzen

# Approved RFC's

* [450: Un-feature-gate `if let`, `while let` and tuple indexing][rfc450].
* [490: Change `Sized?` syntax][rfc490]. From `Sized? T` to `T: Sized?`.

[rfc450]: https://github.com/rust-lang/rfcs/pull/450
[rfc490]: https://github.com/rust-lang/rfcs/pull/490

# New RFC's

* [495: Array pattern adjustments][rfc495]. Fixes several limitations.
* [Ammendment to RFC 439 for grammar ambiguity][rfc498]. There's an
  ambiguity with `..foo` in the previously-accepted slicing RFC.
* [499: NonZero lang item][rfc499]. A way to tell the compiler that a
  value is not 0.
* [501: Consistent no-prelude attribute][rfc501]. Reworks the
  mechanism for opting out of the prelude.
* [502: Remove blanket extension impls][rfc502]. Blanket
  implementations make it impossible to override default methods.
* [503: Stabilize std::prelude][rfc503]. Scrutinizes what is included
  in the prelude, removing many traits.
* [504: Split `Show` into `String` and `Show`][rfc504]. Repurposes
  `{:?}`.

[rfc495]: https://github.com/rust-lang/rfcs/pull/495
[rfc498]: https://github.com/rust-lang/rfcs/pull/498
[rfc499]: https://github.com/rust-lang/rfcs/pull/499
[rfc501]: https://github.com/rust-lang/rfcs/pull/501
[rfc502]: https://github.com/rust-lang/rfcs/pull/502
[rfc503]: https://github.com/rust-lang/rfcs/pull/503
[rfc504]: https://github.com/rust-lang/rfcs/pull/504

# Community

Get your (unofficial) [rustacean t-shirts][tshirts]!

[tshirts]: https://www.reddit.com/r/rust/comments/2o01sd/rustacean_tshirts/

## From the Team

There was no weekly meeting as the team was at a workweek in
Portland. There weren't a lot of coherent minutes taken this time, but
any discussions of substance will result in RFCs. Topics were largely
around stabilization in preparation for 1.0, and this workweek
featured a greater ratio of hacking to talking than previous ones.

## Blog Posts

Zbigniew Siciarz has been writing an informative (and ambitious) series called
"24 Days of Rust". Way to go, Zbigniew!

* [24 Days of Rust - Cargo and crates.io][24days1]. [Reddit][24days1-reddit].
* [24 Days of Rust - slow_primes][24days2]. [Reddit][24days2-reddit].
* [24 Days of Rust - CSV][24days3]. [Reddit][24days3-reddit].
* [24 Days of Rust - docopt][24days4]. [Reddit][24days4-reddit].
* [24 Days of Rust - hyper][24days5]. [Reddit][24days5-reddit].
* [24 Days of Rust - working with JSON][24days6]. [Reddit][24days6-reddit].
* [24 Days of Rust - itertools][24days7]. [Reddit][24days7-reddit].

* [24days1]: https://siciarz.net/24-days-rust-cargo-and-cratesio/
* [24days1-reddit]: https://www.reddit.com/r/rust/comments/2nybtm/24_days_of_rust_cargo_and_cratesio/
* [24days2]: https://siciarz.net/24-days-rust-slow_primes/
* [24days2-reddit]: https://www.reddit.com/r/rust/comments/2o296i/24_days_of_rust_slow_primes/
* [24days3]: https://siciarz.net/24-days-of-rust-csv/
* [24days3-reddit]: https://www.reddit.com/r/rust/comments/2o69pc/24_days_of_rust_csv/
* [24days4]: https://siciarz.net/24-days-of-rust-docopt/
* [24days4-reddit]: https://www.reddit.com/r/rust/comments/2oa78k/24_days_of_rust_docopt/
* [24days5]: https://siciarz.net/24-days-of-rust-hyper/
* [24days5-reddit]: https://www.reddit.com/r/rust/comments/2oe0yg/24_days_of_rust_hyper/
* [24days6]: https://siciarz.net/24-days-of-rust-working-json/
* [24days6-reddit]: https://www.reddit.com/r/rust/comments/2oh6ue/24_days_of_rust_working_with_json/
* [24days7]: https://siciarz.net/24-days-of-rust-itertools/
* [24days7-reddit]: https://www.reddit.com/r/rust/comments/2okqey/24_days_of_rust_itertools/

But that's not all that's going on!

* [The story of my childhood, or: Rust on the
  PSP][psp]. [Reddit][psp-reddit].
* [Progress on Rustdoc source link problems][rustdoc]. lifthrasiir
  spreads the love of rustc metadata. [Reddit][rustdoc-reddit].
* [A Beginner's Thoughts on Programming Languages (Part 1)][beg]. Some mentions of Rust.

[psp]: http://fnordig.de/2014/12/03/a-story-of-hacking-or-rust-on-the-psp/
[psp-reddit]: https://www.reddit.com/r/rust/comments/2o903j/the_story_of_my_childhood_or_rust_on_the_psp/
[rustdoc]: https://lifthrasiir.github.io/rustlog/worklog-2014-12-06.html
[rustdoc-reddit]. https://www.reddit.com/r/rust/comments/2ojnnh/worklog_20141206_progress_on_rustdoc_source_link/
[beg]: https://medium.com/@chcokr/a-beginners-thoughts-on-programming-languages-part-1-e1ad124db3cd

## Discussions

* [Reading Rust aloud][aloud].
* [Clarification on standard library stability and the push to
  1.0][clar].
* [Brainf*uck in Rust's type system (aka type system is Turing
  complete)][brainfuck]. Rust is serious business now.
* [How does `Cow` work][cow].
* [Package naming and grouped packages][pkg].
* [D's proposal for escape-proof references, with some similarities to
  Rust's borrowed references][d]. In which Walter Bright graces our
  subreddit and we totally fail to capitalize.
* [Aren't exceptions mostly implemented for dtors][ex]?
* [C++'s rvalue `&&` references correspond to `&mut`][cxx]. "But then
  life is pain, and C++ doubly so."
* [Why Rust started rather than Ada][ada]? No authoritative answers
  here but some interesting discussion.
* [PSA: Copy is becoming opt-in][copy]. [Reddit][copy-reddit].
* [Could the GC case be made noise-free by default][gc]?
* [Rustdoc: reStructuredText vs. Markdown][rest]. Another round of the
  age-old debate.
* [Repr formatter with ShowRepr trait][repr].

[aloud]: https://www.reddit.com/r/rust/comments/2o5tin/reading_rust_aloud/
[clar]: https://www.reddit.com/r/rust/comments/2o5d9d/clarification_on_standard_library_stability_and/
[brainfuck]: https://www.reddit.com/r/rust/comments/2o6yp8/brainfck_in_rusts_type_system_aka_type_system_is/
[cow]: https://www.reddit.com/r/rust/comments/2oebm5/how_does_cow_work/
[pkg]: https://www.reddit.com/r/rust/comments/2ocz69/package_naming_and_grouped_packages_cratesio_and/
[d]: https://www.reddit.com/r/rust/comments/2od8a8/ds_proposal_for_escapeproof_references_with_some/
[ex]: https://www.reddit.com/r/rust/comments/2of8ox/apologies_in_advance_arent_exceptions_mostly/
[cxx]: https://www.reddit.com/r/rust/comments/2oes6s/cs_rvalue_references_correspond_to_rusts_mut/
[ada]: https://www.reddit.com/r/rust/comments/2og8xf/why_rust_started_rather_than_ada/
[copy]: http://discuss.rust-lang.org/t/psa-copy-is-becoming-opt-in/982
[copy-reddit]: https://www.reddit.com/r/rust/comments/2ogqaj/psa_copy_is_becoming_optin/
[gc]: http://discuss.rust-lang.org/t/could-the-gc-case-be-made-noise-free-by-default/924
[rest]: http://discuss.rust-lang.org/t/rustdoc-restructuredtext-vs-markdown/356
[repr]: http://discuss.rust-lang.org/t/repr-formatter-with-showrepr-trait/926/5

## New Projects

* [BitRust]. A live breaking changes log!
* [Sea Birds' Breakfast][birds]. long_void's Ludum Dare 31 entry.
* [speedtest-rust]. Command-line tool for testing Internet
  speed. [Reddit][speedtest-rust-reddit].
* [rust-once-mutex]. A mutex providing one-time lock and subsequent
  fast access. [Reddit][rust-once-mutex-reddit].
* [osmpbfreader-rs]. OpenStreetMap BPF file
  reader. [Reddit][osmpbfreader-rs-reddit].
* [rust-promise]. A basic promise type. [Reddit][rust-promise-reddit].
* [rust-buildbot]. The scripts running Rust's buildbot instance.
* [matches]. The `matches!` macro is on crates.io.
* [time_calc]. Music and DSP time
  conversion. [Reddit][time_calc-reddit].
* [rust-jwt]. JSON Web Token. [Reddit][rust-jwt-reddit].
* [rust-beanstalkd]. A beanstalkd
  client. [Reddit][rust-beanstalkd-reddit].
* [rusty-tags]. tags generator for cargo
  projects. [Reddit][rusty-tags-reddit].
* [acacia]. Generic spatial tree library. [Reddit][acacia-reddit].

[BitRust]: http://bitrust.octarineparrot.com/
[birds]: http://ludumdare.com/compo/ludum-dare-31/?action=preview&uid=19918
[speedtest-rust]: https://github.com/gkbrk/speedtest-rust
[speedtest-rust-reddit]: https://www.reddit.com/r/rust/comments/2nz8eh/wip_commandline_tool_to_test_your_internet_speed/
[rust-once-mutex]: https://github.com/reem/rust-once-mutex
[rust-once-mutex-reddit]: https://www.reddit.com/r/rust/comments/2o0tm8/a_mutex_providing_a_onetime_lock_then_fast/
[osmpbfreader-rs]: https://github.com/TeXitoi/osmpbfreader-rs
[osmpbfreader-rs-reddit]: https://www.reddit.com/r/rust/comments/2o5506/osmpbfreaderrs_read_openstreetmap_pbf_files_with/
[rust-promise]: https://github.com/viperscape/rust-promise
[rust-promise-reddit]: https://www.reddit.com/r/rust/comments/2o6iz4/promise_sync_object/
[rust-buildbot]: https://github.com/rust-lang/rust-buildbot
[matches]: https://www.reddit.com/r/rust/comments/2obpqv/the_matches_macro_is_on_cratesio/
[time_calc]: https://github.com/RustAudio/time_calc
[time_calc-reddit]: https://www.reddit.com/r/rust/comments/2ogden/time_calc_a_crate_for_music_and_dsp_time/
[rust-jwt]: https://github.com/GildedHonour/rust-jwt
[rust-jwt-reddit]: https://www.reddit.com/r/rust/comments/2ohueq/json_web_token_jwt_in_rust/
[rust-beanstalkd]: https://github.com/schickling/rust-beanstalkd
[rust-beanstalkd-reddit]: https://www.reddit.com/r/rust/comments/2ol9cg/beanstalkd_client_my_first_very_basic_library_in/
[rusty-tags]: https://github.com/dan-t/rusty-tags
[rusty-tags-reddit]: https://www.reddit.com/r/rust/comments/2ol09b/ann_rustytags_create_tags_for_a_cargo_project_and/
[acacia]: https://github.com/aepsil0n/acacia
[acacia-reddit]: https://www.reddit.com/r/rust_gamedev/comments/2oasrs/acacia_a_generic_spatial_tree_library/

## Project Updates

* [New graphics library design - Piston][piston].
* The Rust implementation of docopt now offers [tab completion][docopt].

[piston]: http://blog.piston.rs/2014/12/02/new-graphics-design/
[docopt]: https://github.com/docopt/docopt.rs#tab-completion-support

## Upcoming Meetups

* [2014-12-08 Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)
* [2014-12-15 Rust Paris](http://www.meetup.com/Rust-Paris)
* [2014-12-18 Bay Area Rust - Crypto](http://www.meetup.com/Rust-Bay-Area/events/210632582/)
