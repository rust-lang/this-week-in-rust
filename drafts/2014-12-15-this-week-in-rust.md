Title: This Week in Rust 61
Date: 2014-12-15
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

XXX pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-08..2014-12-15

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* Implementations of `Copy` must [now be declared
  explicitly][optin]. This is part of the [opt-in-builtin traits RFC
  (colloquially knows as OIBIT)][optin-rfc] which is supposed to
  remove some potential footguns from the typesystem.
* Most use of closures have been [converted][unboxed] to 'unboxed
  closures', the new, more flexible, closure model. As a result of API
  changes some downstream code will break, the PR has detailed
  instructions for the transition.
* Slight adjustments have been made to the `fmt` API to make them
  [safe][fmt]. These APIs are rarely used directly.
* It's now [impossible to explicitly call the `call` method][call] of
  the closure types without turning on a feature
  gate. Futureproofing. Doesn't affect the normal calling syntax.
* The `Option` and `Result` variants are [no longer reexported from
  their respective modules][reexp]. This won't break most code
  because the variants are part of the prelude.

[optin]: https://github.com/rust-lang/rust/pull/19566
[optin-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
[fmt]: https://github.com/rust-lang/rust/pull/19506
[call]: https://github.com/rust-lang/rust/pull/19587
[reexp]: https://github.com/rust-lang/rust/pull/19653
[unboxed]: https://github.com/rust-lang/rust/pull/19467

## Other Changes

* Barosl [taught bors how to do rollups][rollup]!!!!!
* Type bounds can be [constrained by the type of an associated
  type][assoc-eq], as in `fn sum_uints<I>(iter: I) where I: Iterator,
  I::A = uint { ... }`. [RFC][assoc-eq-rfc].
* Lifetime elision [works on unboxed closure type sugar][sugar].
* The [testing guide][testing] has been overhauled.
* [`unsafe impl` and `unsafe trait` have landed][unsafe] as port of
  [OIBIT][oibit-rfc]. This is required to convert `Send` and `Sync`
  into library types.
* [`BTreeSet` implements `BitOr`, `BitAnd`, `BitXor`, and `Sub`][btreeset].
* The `recursion_limit` attribute [can control how deeply various
  algorithms in the compiler recurse][recur]. It can be used to
  convince the typechecker to keep working on particularly complicated
  types.
* [`String` implements `FromIterator<&str>` and
  `Extend<&str>`][extend], which means that iterators of `&str` can be
  collected into or appended on to a single string, e.g. `let s:
  String = vec!["foo", "bar"].collect();`, `let s = String::new();
  s.extend(vec!["foo", "bar"]);`.
* New `os::unix` and `os::windows` modules provide [platform-specific
  interop with `std::io`][io].
* The `TupleN` traits [are deprecated][tuplen] because tuple indexing
  is part of the language.

[testing]: http://doc.rust-lang.org/guide-testing.html
[sugar]: https://github.com/rust-lang/rust/pull/19589
[recur]: https://github.com/rust-lang/rust/pull/19466
[btreeset]: https://github.com/rust-lang/rust/pull/19514
[extend]: https://github.com/rust-lang/rust/pull/19626
[io]: https://github.com/rust-lang/rust/pull/19169
[assoc-eq]: https://github.com/rust-lang/rust/pull/19391
[assoc-eq-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md#constraining-associated-types
[tuplen]: https://github.com/rust-lang/rust/pull/19677
[unsafe]: https://github.com/rust-lang/rust/pull/19703
[oibit-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
[rollup]: https://github.com/graydon/bors/pull/47

## New Contributors



# Approved RFC's

None.

# New RFC's

* [505: API comment conventions][rfc505]. Standardizing current
  conventions.
* [507: Release channels take 2][rfc507]. Describes the release train,
  feature staging and in this iteration merges stability attributes
  with feature gates.
* [509: Collections reform part 2][rfc509]. This RFC shores up the
  finer details of collections reform. In particular, where the
  previous RFC focused on general conventions and patterns, this RFC
  focuses on specific APIs. It also patches up any errors that were
  found during implementation of part 1. Some of these changes have
  already been implemented, and simply need to be ratified.
* [517: `io` and `os` reform][rfc517]. This RFC proposes a significant
  redesign of the std::io and std::os modules in preparation for API
  stabilization.
* [519: Panicking tasks should abort process if not handled][rfc519].
* [520: Change array syntax to prevent ambiguity introduced by RFC
  439][rfc520]. An alternative to [RFC 498]rfc498] that fixes the
  ambiguity by changing the array type syntax.
* [522: Allow the `Self` type to be used in impls][rfc522]. In the
  implemantion of a trait, instead of writing the explicit type for
  which a trait is implemented, just write `Self`.

[rfc498]: https://github.com/rust-lang/rfcs/pull/498
[rfc505]: https://github.com/rust-lang/rfcs/pull/505
[rfc507]: https://github.com/rust-lang/rfcs/pull/507
[rfc509]: https://github.com/rust-lang/rfcs/pull/509
[rfc517]: https://github.com/rust-lang/rfcs/pull/517
[rfc519]: https://github.com/rust-lang/rfcs/pull/519
[rfc520]: https://github.com/rust-lang/rfcs/pull/520
[rfc522]: https://github.com/rust-lang/rfcs/pull/522

# Community

[rustbyexample.com] is an [official rust-lang project][rbe]
now. Contributions highly encouraged. [Reddit][rbe-reddit].

[rbe]: https://github.com/rust-lang/rust-by-example
[rde-reddit]: https://www.reddit.com/r/rust/comments/2onzq0/rust_by_example_has_been_transferred_to_rustlang/

## From the Team

* [Rust 1.0: Scheduling the Trains][trains]. About the 1.0 release
  schedule. [Reddit][trains-reddit]. [HN][trains-hn].
* [Yehuda Katz and Steve Klabnik are joining the Rust Core
  Team][core]. [Reddit][core-reddit]. [HN][core-hn].
* [Weekly-meetings/2014-18-11][mtg]: process changes; box patterns;
  macros; issues in FIXMEs; placement box. [Reddit][mtg-reddit].

[trains]: http://blog.rust-lang.org/2014/12/12/1.0-Timeline.html
[trains-reddit]: https://www.reddit.com/r/rust/comments/2p35dk/rust_10_scheduling_the_trains/
[trains-hn]: https://news.ycombinator.com/item?id=8740751
[core]: http://blog.rust-lang.org/2014/12/12/Core-Team.html
[core-reddit]: https://www.reddit.com/r/rust/comments/2p47ee/yehuda_katz_and_steve_klabnik_are_joining_the/
[core-hn]: https://news.ycombinator.com/item?id=8742953
[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-12-09.md
[mtg-reddit]:

## Blog Posts

* [Fun with Threads][funthreads]: Comparing basic parallel programming in C and Rust, from a beginner's perspective. [Reddit][funthreads-reddit] 
* [Rewriting Rust Serialization, Part 3: Introducing Serde][rustserial]: Part 3 of Erick's great series on serialization. [Reddit][rustserial-reddit] 
* [Using Rust to Make a Safer Interface for Yahoo’s Fast MDBM Database][mdbm]: [Reddit][mdbm-reddit] 
* [Bootstrapping Rust][bootstrapping]: A quick look at the issues of compiling a compiler written in itself. [Reddit][bootstrapping-reddit] 
* [A Beginner’s Thoughts on Programming Languages (Part 2 — RAII, GC, Ownership, noexcept)][begginers]: [Reddit][begginers-reddit] 
* [Working with AI behavior trees][behaviour]: Using some of the AI features found in Piston. [Reddit][behaviour-reddit] 


### 24 Days of Rust continues!

* [24 Days of Rust - racer][24days8]. [Reddit][24days8-reddit].
* [24 Days of Rust - anymay][24days9]. [Reddit][24days9-reddit].
* [24 Days of Rust - the glorious tau][24days10]. [Reddit][24days10-reddit].
* [24 Days of Rust - postgres][24days11]. [Reddit][24days11-reddit].
* [24 Days of Rust - image][24days12]. [Reddit][24days12-reddit].
* [24 Days of Rust - uuid][24days13]. [Reddit][24days13-reddit].
* [24 Days of Rust - nalgebra][24days14]. [Reddit][24days14-reddit].

[funthreads]: http://jvns.ca/blog/2014/12/14/fun-with-threads/
[funthreads-reddit]: http://www.reddit.com/r/rust/comments/2pabnd/diving_into_concurrency_trying_out_mutexes_and/
[rustserial]: http://erickt.github.io/blog/2014/12/13/rewriting-rust-serialization/
[rustserial-reddit]: http://www.reddit.com/r/rust/comments/2p85za/rewriting_rust_serialization_part_3_introducing/
[mdbm]: http://erickt.github.io/blog/2014/12/13/rust-and-mdbm/
[mdbm-reddit]: http://www.reddit.com/r/rust/comments/2p70sj/using_rust_to_make_a_safer_interface_for_yahoos/
[bootstrapping]: http://aidancully.blogspot.com/2014/12/bootstrapping-rust.html
[bootstrapping-reddit]: http://www.reddit.com/r/rust/comments/2ovuko/bootstrapping_rust/
[begginers]: https://medium.com/@chcokr/a-beginners-thoughts-on-programming-languages-part-2-raii-gc-ownership-noexcept-1c08f5dfe353
[begginers-reddit]: http://www.reddit.com/r/rust/comments/2ov7e2/a_beginners_thoughts_on_programming_languages/
[behaviour]: http://blog.piston.rs/2014/12/09/working-with-ai-behavior-trees/
[behaviour-reddit]: http://www.reddit.com/r/rust/comments/2orssg/working_with_ai_behavior_trees_piston/

[24days8]: https://siciarz.net/24-days-of-rust-racer/
[24days8-reddit]: https://www.reddit.com/r/rust/comments/2oo1n5/24_days_of_rust_racer/
[24days9]: https://siciarz.net/24-days-of-rust-anymap/
[24days9-reddit]: https://siciarz.net/24-days-of-rust-anymap/
[24days10]: https://siciarz.net/24-days-of-rust-glorious-tau/
[24days10-reddit]: https://www.reddit.com/r/rust/comments/2ow3jm/24_days_of_rust_the_glorious_tau/
[24days11]: https://siciarz.net/24-days-of-rust-postgres/
[24days11-reddit]: https://www.reddit.com/r/rust/comments/2ozzeg/24_days_of_rust_postgres/
[24days12]: https://siciarz.net/24-days-of-rust-image/
[24days12-reddit]: https://www.reddit.com/r/rust/comments/2p3mjf/24_days_of_rust_image/
[24days13]: https://siciarz.net/24-days-of-rust-uuid/
[24days13-reddit]: https://www.reddit.com/r/rust/comments/2p6kvf/24_days_of_rust_uuid/
[24days14]: https://siciarz.net/24-days-of-rust-nalgebra/
[24days14-reddit]: https://www.reddit.com/r/rust/comments/2pa7md/24_days_of_rust_nalgebra/

## New Projects
* [Elliptic Curve Crypto][ecc]: Elliptic curve arithmetic and cryptography library in pure Rust
* [Rustache][rustache]: a flexible template engine for Rust
* [checked_cast!][checked]: A small macro to deal with the libc typedef hell
* [Iota][iota]: A simple text editor written in Rust
* [Metafactory][metafactory]: value construction pipeline builder
* [superchan][superchan]: A Rust crate containing types for sending data across a network
* [wxRust][wxRust]: A Rust binding of the wxWidgets cross platform toolkit

[ecc]: https://github.com/Bren2010/ecc
[rustache]: http://rustache.github.io/
[checked]: https://github.com/Jurily/rust-checked-cast
[iota]: https://github.com/gchp/iota
[metafactory]: https://github.com/Nercury/metafactory-rs
[superchan]: https://github.com/dradtke/superchan
[wxRust]: https://github.com/kenz-gelsoft/wxRust

## Project Updates

* [This Week in Servo 14][twis]. Servo has adopted [hyper]. [Reddit][twis-reddit].
* [Servo meeting notes from Mozlandia][servo-mozlandia]. Notes from the workweek.
* [Servo running on Firefox OS][fxos]. [Reddit][fxos-reddit].
* [rusty-tags now supports emacs][rusty-tags]

[twis]: http://blog.servo.org/2014/12/09/twis-14/
[twis-reddit]: https://www.reddit.com/r/rust/comments/2orabe/this_week_in_servo_15/
[hyper]: https://github.com/hyperium/hyper
[servo-mozlandia]: https://groups.google.com/forum/#!topic/mozilla.dev.servo/zK2H8a2dTGQ
[fxos]: https://twitter.com/larsberg_/status/539937229049581568
[fxos-reddit]: https://www.reddit.com/r/rust/comments/2orhhh/servo_running_on_firefox_os/
[rusty-tags]: https://github.com/dan-t/rusty-tags

## Upcoming Events

* There will be a [talk about Rust at PyCon][pycon].
* [12/18 Bay Area Rust: Crypto][bay].

[pycon]: https://us.pycon.org/2015/schedule/presentation/411/
[bay]: http://www.meetup.com/Rust-Bay-Area/events/210632582/
