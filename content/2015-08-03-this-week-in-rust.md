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

*some intro here*

* [pdf] [Keynote](http://rustcamp.com/RustCampKeynote.pdf) by [Aaron Turon](http://www.mpi-sws.org/~turon/) and [Niko Matsakis](https://twitter.com/nikomatsakis).
* [notes] [Navigating the Open Seas](http://carol-nichols.com/2015/08/01/rustcamp-talk-notes/) by [Carol (Nichols || Goulding)](http://carol-nichols.com/).
* [pdf] [Making tools for Rust](https://dl.dropboxusercontent.com/u/74741329/rust-tools.pdf) by [Nick Cameron](http://www.ncameron.org/).
* [pdf] [Mio: Fast Async IO for Rust](https://www.dropbox.com/s/fzf7tiukacyft3b/Rustcamp%20-%20Mio.pdf) by [Carl Lerche](https://twitter.com/carllerche).
* [slides] [Who Owns This Stream of Data?](http://cglab.ca/~abeinges/talks/iter/) by [Alexis Beingessner](https://twitter.com/gankro).

# What's cooking on nightly?

XXX pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-05-18..2015-06-07

# New Contributors



# Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:


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


# Friend of the Tree

[The Rust Team](http://www.rust-lang.org/team.html) likes to
occassionally recognize people who have made
outstanding contributions to The Rust Project, its ecosystem, and its
community. These people are 'friends of the tree'.

[This week's friend of the tree](TODO) was ...


# Subteam reports

Every week [The Rust Team](http://www.rust-lang.org/team.html) release
a report on what is going on in their corner of the project. Here are
the highlights from [this week's report](TODO).

* TODO

# Internals discussions

# Crate of the Week

There are so many crates! It's easy to lose track of the good ones,
like [THING].

THING is a ...


# Upcoming Events

* [What?]

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

There are some jobs writing Rust! This week's listings:

* TODO

(Don't forget to re-list last-week's).

# Quote of the Week

*"Quote"*

Explanation and link.

Thanks to XXX for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
