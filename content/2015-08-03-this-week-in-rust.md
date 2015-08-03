Title: This Week in Rust 90
Date: 2015-08-03
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# From the Blogosphere

* [A Rusting Rubyist IV](https://medium.com/@mfpiccolo/a-rusting-rubyist-iv-b1d32802944b). Rust Structs and Arrays in Ruby.
* [Rust's Built-in Traits, the When, How & Why](https://llogiq.github.io/2015/07/30/traits.html).
* [Type-checked matrix operations in Rust](https://jadpole.github.io/rust/typechecked-matrix/).
* [Visualizing Rust's type-system](https://jadpole.github.io/rust/type-system/).
* [A web app with Nickel: From first line to Heroku deployment](http://blog.thoughtram.io/rust/2015/07/29/a-web-app-with-nickel-from-first-line-to-heroku-deployment.html).
* [Why you should take a closer look at Rust 1.0](https://jaxenter.com/why-you-should-take-a-closer-look-at-rust-1-0-119191.html).
* [My First PR for the Rust compiler](https://nashenas88.github.io/posts/my-first-pr-for-the-rust-compiler/).
* [book] [The Advanced Rust Programming Language](https://doc.rust-lang.org/nightly/adv-book/).
* [Abstracted Algebra in Rust, part I](http://maniagnosis.crsr.net/2015/07/abstracted-algebra-in-rust.html) and [part II](http://maniagnosis.crsr.net/2015/07/more-abstracted-algebra-in-rust.html). 

# New Releases & Project Updates

* [Racer is now at v1.0](http://phildawes.net/blog/2015/07/29/racerv1/)
* [Raft implementation is ready to start experimenting with](http://hoverbear.org/2015/08/01/raft-examples/).
* [annotated-std-rs](https://github.com/brson/annotated-std-rs). An annotation of the Rust standard library.
* [rust-syslog](https://github.com/Geal/rust-syslog). Send syslog messages from Rust.
* [unrar.rs](https://github.com/muja/unrar.rs). Rust library for extracting, listing and testing RAR archives.
* [msgpack-rust](https://github.com/3Hren/msgpack-rust). MessagePack implementation for Rust.

# Slides and talks from RustCamp!

[RustCamp](http://rustcamp.com) was on Saturday, August 1st. It was
lovely event populated by lovely people. If you couldn't make it here
are the slides from some of the talks. Hopefully the remainder of
slides will become available this week. Video recordings will be
available at an indeterminate future date.

* [pdf] [Keynote](http://rustcamp.com/RustCampKeynote.pdf) by [Aaron Turon](http://www.mpi-sws.org/~turon/) and [Niko Matsakis](https://twitter.com/nikomatsakis).
* [notes] [Navigating the Open Seas](http://carol-nichols.com/2015/08/01/rustcamp-talk-notes/) by [Carol (Nichols || Goulding)](http://carol-nichols.com/).
* [pdf] [Making tools for Rust](https://dl.dropboxusercontent.com/u/74741329/rust-tools.pdf) by [Nick Cameron](http://www.ncameron.org/).
* [pdf] [Mio: Fast Async IO for Rust](https://www.dropbox.com/s/fzf7tiukacyft3b/Rustcamp%20-%20Mio.pdf) by [Carl Lerche](https://twitter.com/carllerche).
* [slides] [Who Owns This Stream of Data?](http://cglab.ca/~abeinges/talks/iter/) by [Alexis Beingessner](https://twitter.com/gankro).

# What's cooking on nightly?

130 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-07-27..2015-08-03

* Gankro landed [The Advanced Rust Programming
  Language](https://github.com/rust-lang/rust/pull/27032), which was
  quickly renamed to [The
  Rustonomicon](https://github.com/rust-lang/rust/pull/27444). This
  book is about unsafe Rust and will become Gankro's master's
  thesis. [Read it
  online](http://doc.rust-lang.org/nightly/adv-book/).
* [Nonzeroing move
  hints](https://github.com/rust-lang/rust/pull/26173). This patch
  adds drop flags to the stack as part of the [non-zeroing drop
  RFC](https://github.com/rust-lang/rfcs/blob/master/text/0320-nonzeroing-dynamic-drop.md). It
  does not yet remove the drop flag from objects.
* [@azerupi](http://github.com/azerupi) cleaned up the design of
  gitbook, the tool that generates The Book.
* [Deprecated
  `VecMap`](https://github.com/rust-lang/rust/pull/26734). This
  little-used unstable collection has been moved to the `vec_map`
  crate.
* [Deprecate a number of unstable
  features](https://github.com/rust-lang/rust/pull/26914). The list is
  sizable.
* [Implement `Clone` for `Box<T> where T:
  Clone`](https://github.com/rust-lang/rust/pull/26934)
* [Make `AtomicPtr` `Send`](https://github.com/rust-lang/rust/pull/27052)
* [Make `Rc` and `Arc` `mem::forget`
  safe](https://github.com/rust-lang/rust/pull/27174). This closes
  some corner-case soundness holes in reference counting.
* [More extended errors!](https://github.com/rust-lang/rust/pull/27230).
* [Add a `--cap-lints` flag to the
  compiler](https://github.com/rust-lang/rust/pull/27260). This flag
  [will be used by Cargo to avoid breaking dependencies when lints
  change](https://github.com/rust-lang/rfcs/blob/master/text/1193-cap-lints.md).
* [Rewrite dropck to be more
  correct](https://github.com/rust-lang/rust/pull/27261).
* [Improve SipHash performance for longer
  data](https://github.com/rust-lang/rust/pull/27280).
* [Stabilize a number of small
  APIs](https://github.com/rust-lang/rust/pull/27370).
* [Implement `Clone` for
  `Box<str>`](https://github.com/rust-lang/rust/pull/27371).
* [Gate associated type
  defaults](https://github.com/rust-lang/rust/pull/27382). 1.1 stable
  mistakenly allowed default associated types to be written, but their
  use was completely broken. With other outstanding questions about
  the feature it has been gated for 1.2.

# New Contributors

* Agoston Szepessy
* Andrew
* Andrew Kuchev
* Blake Loring
* Daniel Albert
* diaphore
* Jeehoon Kang
* Kieran Hunt
* krumelmonster
* Mark Buer
* Nicolette Verlinden
* Ralf Jung
* Taliesin Beynon

# Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1183: Allow changing the default allocator](https://github.com/rust-lang/rfcs/blob/master/text/1183-swap-out-jemalloc.md)
* [RFC 1184: Stabilize the `#[no_std]` attribute](https://github.com/rust-lang/rfcs/blob/master/text/1184-stabilize-no_std.md)

# Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-07-27..2015-07-03

* [Allow renaming imports when importing a group of symbols from a module.](https://github.com/rust-lang/rfcs/pull/1219).
* [Clarify rules for projections and well-formedness](https://github.com/rust-lang/rfcs/pull/1214).
* [Introduce a mid-level IR (MIR) in the compiler](https://github.com/rust-lang/rfcs/pull/1211).
* [Update FOLLOW set for `ty` tokens](https://github.com/rust-lang/rfcs/pull/1209).
* [Add `cargo install`](https://github.com/rust-lang/rfcs/pull/1200).
* [Improve ordered query API](https://github.com/rust-lang/rfcs/pull/1195).
* [Add item recovery collection APIs](https://github.com/rust-lang/rfcs/pull/1194).
* [Implement raw fat pointer comparisons](https://github.com/rust-lang/rfcs/pull/1135).
* [Add `read_exact` to `Read` trait](https://github.com/rust-lang/rfcs/pull/980).
* [Allow `#[must_use]` on functions, rather than just types.](https://github.com/rust-lang/rfcs/pull/886).

# New RFCs

* [Place left arrow syntax (`place <-
  expr`)](https://github.com/rust-lang/rfcs/pull/1228). Another
  attempt at coming up with a pleasing 'placement new' syntax.
* [Turn statically-known erroneous code into a warning and
  unconditional panic](https://github.com/rust-lang/rfcs/pull/1229).
* [Add `Box::leak` to leak `Box<T>` to a `&'static mut
  T`](https://github.com/rust-lang/rfcs/pull/1233).
* [Specify that `CoerceUnsized` should ignore `PhantomData`
  fields](https://github.com/rust-lang/rfcs/pull/1234).

# Internals discussions

* [Pre-RFC?: rustc UX guidelines](https://internals.rust-lang.org/t/pre-rfc-rustc-ux-guidelines/2419/1).

# Upcoming Events

* [8/5. Montreal](http://www.meetup.com/Montreal-Rust-Language-Meetup/events/224148410/).
* [8/10. Seattle](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [8/11. San Diego](http://www.meetup.com/San-Diego-Rust/events/223766853/).
* [8/19. Los Angeles](http://www.meetup.com/Rust-Los-Angeles/events/224231575/).
* [8/26. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [8/31. Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

There are some jobs writing Rust! This week's listings:

* Assistant Researcher in Karlsruhe, Germany for embedded development on ARM stm32. Contact [Oliver Schneider][oli_obk]

[oli_obk]: mailto:oliver.schneider@kit.edu

# Quote of the Week

> It should be noted that the authentic Rust learning experience involves
> writing code, having the compiler scream at you, and trying to figure out
> what the heck that means. I will be carefully ensuring that this occurs as
> frequently as possible.

From @Gankro's [Learning Rust With Entirely Too Many Linked
Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/).

Thanks to @carols10cents for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
