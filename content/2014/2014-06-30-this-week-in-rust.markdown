Title: This Week in Rust 54
Date: 2014-06-30 12:25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

It's time for 0.11! The [prerelease candidate is
available](https://mail.mozilla.org/pipermail/rust-dev/2014-June/010618.html).

<!-- more -->

# What's cooking on master?

73 pull requests were merged in the last week.

## Breaking Changes

The complete breaking change log is available
[here](https://gist.github.com/cmr/9c3db4bc3f0a96426d49), and you can view it
with `git log --no-merges --grep 'breaking-change' --since 6/21/2014 --until
6/28/2014`. Some important ones:

- `*T` [is now known](https://github.com/rust-lang/rust/pull/15208) as `*const
  T`.
- `struct` literals [are not
  allowed](https://github.com/rust-lang/rust/pull/14885) in expressions that
  precede a block.
- The type of `42` [is no longer
  int](https://github.com/rust-lang/rust/commit/9e3d0b002a5c2e81d43351c9b8550a3f4ccfb8f9)
  but will be inferred as normal.

## Other Changes

- In a series of many PRs, Steve [has started
  work](https://github.com/rust-lang/rust/pull/15183) on a new tutorial, which
  is available (in draft form) [here](http://doc.rust-lang.org/guide.html).
- Tasks [are now reusable](https://github.com/rust-lang/rust/pull/14886), in
  that one can create a task, run a closure in it, and if it did not fail, the
  task can be reused.
- Lints [now have a plugin
  infrastructure](https://github.com/rust-lang/rust/pull/15024), meaning your
  own, arbitrary lints can now be defined!

## New Contributors

- Alex Gaynor
- Arjan Topolovec
- Conrad Kleinespel
- Michael Zhou
- Mike Capp
- Pawel Olzacki
- Robert Buonpastore
- Ruud van Asseldonk
- Zach Pomerantz

# New RFCs

- [Remove the `'` from lifetime
  parameters](https://github.com/rust-lang/rfcs/pull/134)
- [Where clauses for more expressive
  bounds](https://github.com/rust-lang/rfcs/pull/135)
- [Ban private items in public
  APIs](https://github.com/rust-lang/rfcs/pull/136)
- [Objects of type T should be implicitly convertible to
  &T](https://github.com/rust-lang/rfcs/pull/137)
- [Remove cross borrowing
  entirely](https://github.com/rust-lang/rfcs/pull/139)
- [Clarify that removing language features requires an
  RFC](https://github.com/rust-lang/rfcs/pull/140)
- [New lifetime elision rules](https://github.com/rust-lang/rfcs/pull/141)
- [Efficient single inheritance](https://github.com/rust-lang/rfcs/pull/142)
- [FromLiteral](https://github.com/rust-lang/rfcs/pull/143)
- [Memory exploit mitigation](https://github.com/rust-lang/rfcs/pull/145)
- [Scoped attributes for checked
  arithmetic](https://github.com/rust-lang/rfcs/pull/146)

# Community Updates

- There was a meetup in San Fransisco on Thursday, about gamedev. [The
  recording is available](https://air.mozilla.org/rust-meetup-june-2014/).
- Damien Katz [seems to be
  recruiting](https://twitter.com/damienkatz/status/482712736170643457) for
  Rust developers to build a distributed object store.
- The meeting notes [have a new
  home](https://github.com/rust-lang/meeting-minutes).
- [Racer progress update](http://phildawes.net/blog/2014/06/24/racer-update/),
  including vim support!
- [rusticom](https://github.com/breckinloggins/rusticom), a NES emulator.
- [Cargo alpha
  announcement](https://mail.mozilla.org/pipermail/rust-dev/2014-June/010569.html)
- [Static checking of units in
  Servo](https://blog.mozilla.org/research/2014/06/23/static-checking-of-units-in-servo/)
- [A basic dominion simulator](https://github.com/dradtke/rust-dominion)
- [ncurses Gravity Worm
  clone](http://www.reddit.com/r/rust/comments/29dttw/first_attempt_at_ncursesbased_gravity_worm_game/)
- [dash-rust now has a nightly docset
  feed](http://www.reddit.com/r/rust/comments/299mi0/dashrust_now_has_a_nightly_docset_feed/)
- [0.11 prerelease
  testing](https://mail.mozilla.org/pipermail/rust-dev/2014-June/010618.html)
- [Piston game engine
  update](http://www.reddit.com/r/rust/comments/29h27x/the_piston_game_engine_update_notice_on_progress/)

# This Week in Cargo

Cargo is the Mozilla-funded package manager slash build tool for Rust code.
Cargo is being developed by [Tilde](http://www.tilde.io/), in part due to their
previous experience building [Bundler](http://bundler.io/). You can find
Cargo's website at [http://crates.io/](http://crates.io/). Apparently, a
startup is already using `cargo.io`. Darn! The source of the website is
[here](https://github.com/wycats/cargo-website) currently, if you'd like to add
or change anything.

Cargo had its first 'release' on Monday, so this is the very first TWiC! As
part of this, Cargo has [moved to the rust-lang
organization](https://github.com/rust-lang/cargo/commit/382a1033260b5db3aeb8b19207c91775f48cb842).
That commit also serves as the original release. Cargo is 'pre-alpha,' so no
actual releases have been tagged. Yet. Basically, Cargo is at a point where it
is able to be used for basic Rust projects, and dogfooding it will help make
it awesome.

Uptake has been pretty good: [A search on
GitHub](https://github.com/search?q=Cargo.toml&ref=cmdform&type=Code) shows a
large number of code that references Cargo. Of course, some people will still
use other projects to handle this, especially with Cargo's lack of features.

For a short introduction to Cargo, [see my section in the new
Guide](http://doc.rust-lang.org/guide.html#hello,-cargo!). One note that's not
in there, however: you're intended to check your `Cargo.toml` file into version
control, similarly to a `Makefile`.

## Notable additions

Eighteen pull requets landed in this first week. Wow! Here are the bigger ones:

- [Removing Vagrant](https://github.com/rust-lang/cargo/pull/48)
- [Correct usage of DESTDIR](https://github.com/rust-lang/cargo/pull/49)
- [adding a LICENSE (Apache/MIT, just like Rust)](https://github.com/rust-lang/cargo/pull/50)
- [Handle misformatted versions with a nicer error message](https://github.com/rust-lang/cargo/pull/53)
- [Fix Windows tests](https://github.com/rust-lang/cargo/pull/56)
- [Remove 'test' binary from source tree, oops!](https://github.com/rust-lang/cargo/pull/59)
- [Don't recompile nested deps too frequently](https://github.com/rust-lang/cargo/pull/64)
- [Prep work for buildbot: Cargo now also uses bors](https://github.com/rust-lang/cargo/pull/70)
- [`cargo test` command added](https://github.com/rust-lang/cargo/pull/71)
- [Error properly on ssh URLs for dependencies](https://github.com/rust-lang/cargo/pull/72)
- [Use a custom `rm_rf` to paper over Windows git funkiness](https://github.com/rust-lang/cargo/pull/81)
- [Check for the existance of a manifest when reading packages](https://github.com/rust-lang/cargo/pull/83)


## New Contributors

- o11c
- gilles-leblanc
- huonw
- mcpherrinm
- dtrebbien
- halorgium
- Arcterus
- samebchase

