Title: This Week in Rust 95
Number: 95
Date: 2015-09-07
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

This week's edition was edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# From the Blogosphere

* [Designing a GC in Rust](https://manishearth.github.io/blog/2015/09/01/designing-a-gc-in-rust/).
* [Rusty Radio: Episode 3](http://rustyrad.io/podcast/3/). Who Develops the Developer Tools? Featuring Phil Dawes (Racer), Nathan Sobo (Atom), and Christian Hergert (Builder).
* [Combine 1.0.0 and a simple INI parser](https://marwes.github.io/2015/08/28/combine-1.0.0.html).
* [Boilerplate Beginning - Piccolo Part 1](https://polyfractal.com/post/boilerplate-beginning-piccolo-part-1/). Writing a key-value store in Rust.
* [Clippy vs. Rust](https://llogiq.github.io/2015/09/06/clippy.html). Running Clippy on rustc and stdlib.
* [`rust-gnome` is now `Gtk-rs`](http://gtk-rs.org/blog/2015/09/06/the-name-is-gtk-rs.html).

# New Releases & Project Updates

* [swiboe](https://github.com/swiboe/swiboe). The text editor for the next 25 years.
* [cargo-clippy](https://github.com/arcnmx/cargo-clippy) - runs clippy on the current project.
* [ring](https://github.com/briansmith/ring). Simplified libcrypto (from BoringSSL) for Rust, C/C++, etc.
* [Ideone](https://ideone.com) added support for Rust.
* [Pipeline](https://github.com/johannhof/pipeline.rs). A macro collection to pipe your functions calls, like in F# or Elixir.


# What's cooking on nightly?

114 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-08-31..2015-09-07

See the [subteam report for 2015-09-04][subteam] for details.

[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-09-04/2600

# New Contributors

* Aleksey Kladov
* AlexDenisov
* benshu
* christopherdumas
* Hunan Rostomyan
* Jack Wilson
* John Thomas
* Jørn Lode
* Viacheslav Chimishuk
* Xiao Chuan Yu

# Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 953: overloaded assignment operations `a += b`](https://github.com/rust-lang/rfcs/pull/953).
* [RFC 1135: Implement raw fat pointer comparisons](https://github.com/rust-lang/rfcs/pull/1135).
* [RFC 1192: Allow a `x...y` expression to create an inclusive range](https://github.com/rust-lang/rfcs/pull/1192).
* [RFC 1229: Turn statically known erroneous code into a warning and continue normal code-generation](https://github.com/rust-lang/rfcs/pull/1229).

# Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-08-31..2015-09-07

* [Expand the `std::net` module to bind more low-level interfaces](https://github.com/rust-lang/rfcs/pull/1158).
* [Add `Box::leak` to leak `Box<T>` to `&'static mut T`](https://github.com/rust-lang/rfcs/pull/1233).
* [Forbid wildcard dependencies on crates.io](https://github.com/rust-lang/rfcs/pull/1241).
* [References into `repr(packed)` structs should be `unsafe`](https://github.com/rust-lang/rfcs/pull/1240).
* [Revise the Drop Check (`dropck`) part of Rust's static analyses](https://github.com/rust-lang/rfcs/pull/1238).
* [Update the RFC process with sub-teams, amongst other things.](https://github.com/rust-lang/rfcs/pull/1224).
* [Lay the ground work for building powerful SIMD functionality.](https://github.com/rust-lang/rfcs/pull/1199).

# New RFCs

* [Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [Allow library authors to use a `#[deprecate]` attribute](https://github.com/rust-lang/rfcs/pull/1270).

# Crate of the Week

This is a new part of this weekly installation, where we will write about a crate that some of you may not know.
Please nominate a crate of your choice at the [rust-users thread](https://users.rust-lang.org/t/crate-of-the-week/2704/15) so we can write about it next week.

For this first installment, the most votes went to [Serde](https://github.com/serde-rs/serde). Despite many of you probably knowing it, we accepted the popular vote because of the great benefit those who don't yet know it may get. Thanks go to [bstrie](https://users.rust-lang.org/users/bstrie) for the suggestion.

[**Serde**](https://github.com/serde-rs/serde) (de)serializes arbitrary Rust data to a number of formats, including [JSON](https://crates.io/crates/serde_json), [XML](https://github.com/serde-rs/xml), [YAML](https://github.com/serde-rs/yaml/), [Bincode](https://crates.io/crates/bincode) and [MessagePack](https://crates.io/crates/rmp). It is possible to write encoder/decoder pairs to work with Serde that don't need to know too much about Rust objects, and Rust data structures only need a pair of auto-derived traits to work with Serde. Now what can one use serializaton for? Storing Rust objects in files or database blobs, sending them between processes, over language-barriers or over the network, for one.

In short, Serde is _the_ Rust framework for your (de)serialization needs.

# Upcoming Events

* [9/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [9/15. San Diego Rust Meetup #8](http://www.meetup.com/San-Diego-Rust/events/224577039/).
* [9/17. Rust Meetup Hamburg: Show, Tell and Drink](http://www.meetup.com/Rust-Meetup-Hamburg/events/225116081/).
* [9/21. Rust Paris](http://www.meetup.com/Rust-Paris).
* [9/23. Columbus Rust Society](http://www.meetup.com/columbus-rs/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*"[Rust] the language had to dedicate so much real estate to this (difficult) problem alone, it became a disharmonic creature with one bulging muscle and little of anything else."* — [Andrei Alexandrescu (one of designers of D)](https://www.reddit.com/r/programming/comments/3ioy9b/andrei_alexandrescu_c_guru_leaves_facebook_to/cuj0csn).

Thanks to [llogiq](https://users.rust-lang.org/users/llogiq) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
