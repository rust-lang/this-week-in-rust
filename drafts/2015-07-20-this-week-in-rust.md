Title: This Week in Rust 88
Date: 2015-07-20
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

This week's edition was edited by: Brian Anderson

# From the Blogosphere

* [Two reasons the Rust language will succeed](http://www.infoworld.com/article/2947214/open-source-tools/two-reasons-the-rust-language-will-succeed.html). _As good as the Rust project may be, its community is even better_.
* [Holy std::borrow::Cow](https://llogiq.github.io/2015/07/09/cow.html), also [Redux](https://llogiq.github.io/2015/07/09/cow.html): Llogiq investigates Cow usage and implementation.
* [`ijson` in Rust](http://softwaremaniacs.org/blog/category/ijson-in-rust/en/). A series of posts on implementing a streaming JSON parser.
* [A Rusting Rubyist III](https://medium.com/@mfpiccolo/a-rusting-rubyist-iii-5db9915e3269). Rust HTTP Requests in a Ruby Module.
* [Profiling Rust applications on Linux](https://llogiq.github.io/2015/07/15/profiling.html).
* [Understanding Lifetime in Rust – Part II](https://mobiarch.wordpress.com/2015/07/08/understanding-lifetime-in-rust-part-ii-3/).
* [Traits as Higher-order Functions](http://sproul.io/blog/posts/traits-as-hof-in-rust.html).
* [Understanding `mio` and Asynchronous IO](http://hermanradtke.com/2015/07/12/my-basic-understanding-of-mio-and-async-io.html).
* [Building a Random Friend Dialer Part 1: Getting Started with Rust and Iron](https://www.twilio.com/blog/2015/07/building-a-random-friend-dialer-part-1-getting-started-with-rust-and-iron.html).
* [Parsing ISO8601 dates using nom](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/).
* [podcast] [Rusty radio - episode 2](http://rustyrad.io/podcast/2/). Featuring Alex Newman, Ben Striegel, Carl Lerche of [tilde.io](http://tilde.io), and Jonathan Reem of [terminal.com](http://terminal.com).
* [video] [Rust: A type system you didn't know you wanted](https://www.youtube.com/watch?v=Q7lQCgnNWU0).

# New Releases & Project Updates

* [chan](https://github.com/BurntSushi/chan). Multi-producer, multi-consumer concurrent channel for Rust.
* [cargo-check](https://github.com/rsolomo/cargo-check). Wrapper around `cargo rustc -- -Zno-trans`.
* [Raft: New Crates!](http://hoverbear.org/2015/07/16/raft-new-crates/). Two new crates `wrapped_enum` (use multiple `try!()` with different errors) and `scoped_log` (log log context to logs) from Raft developers.
* [rust-memalloc](https://github.com/reem/rust-memalloc). Raw allocation APIs in stable rust.
* [newtype_macros](https://github.com/arienmalec/newtype_macros). Tuple structs with a single member, intended to be used for wrapping types to create new semantics for an underlying type.
* [capnp-ffi](https://github.com/waynenilsen/capnp-ffi). Use Cap'n Proto as a better method of FFI communication.
* [rust-farmhash](https://github.com/seiflotfy/rust-farmhash). Port of Google's Farmhash version 1.1 to pure Rust.

# What's cooking on nightly?

[bors](https://github.com/bors) underwent a [sudden unscheduled
uprade](https://internals.rust-lang.org/t/buildbot-is-down-for-a-bit/2365/4)
this week, incurring unusual ammounts of downtime. Thankfully, [Manish
crafted an epic rollup](https://github.com/rust-lang/rust/pull/27066)
to make up some of the slack.

XXX pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-07-13..2015-07-20

* [Implement `DerefMut` for
 `String`](https://github.com/rust-lang/rust/pull/26241)
* [Add specializations of read_to_end for Stdin and File using
  uninitialised buffers](https://github.com/rust-lang/rust/pull/26950)
* [Allow semi tokens after macro
  ty/path](https://github.com/rust-lang/rust/pull/27000). See [the
  test
  case](https://github.com/alexcrichton/rust/blob/af556238ebe72d58adbcf339bd2fa0aef4e3caf9/src/test/run-pass/semi-after-macro-ty.rs)
  for an example of what this means.
* [LLVM was updated to
  3.7](https://github.com/rust-lang/rust/pull/27076). Includes
  improved 32-bit MSVC, archive writing, and some performance
  improvements and minor fixes.

# New Contributors



# Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC for creation of `IntoRaw{Fd, Socket, Handle}` trait to
  complement
  `AsRaw*`](https://github.com/rust-lang/rfcs/pull/1174). This
  provides interop between the standard library's I/O and other
  out-of-tree platform-specific APIs.

# Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-07-06..2015-07-13

* TODO

# New RFCs


# Internals discussions

# Subteam reports

Every week the [Rust teams](http://www.rust-lang.org/team.html)
release a report on what is going on in their corner of the
project. Here are the highlights from [This week's report](TODO).

* TODO

# Friend of the Tree

The Rust Team likes to occassionally recognize people who have made
outstanding contributions to The Rust Project, its ecosystem, and its
community. These people are 'friends of the tree'.

This week's friend of the tree was ...


# Crate of the Week

There are so many crates! It's easy to lose track of the good ones,
like [THING].

THING is a ...

# Upcoming Events

* [7/22. Columbus Rust Society](http://www.meetup.com/columbus-rs/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*Rust is very much about only paying for what you need, and often you don't need much, but when you do need something, Rust is more than ready to rummage in your wallet for loose change.* — [Manish Goregaokar](https://www.reddit.com/r/rust/comments/3cj69b/why_go_and_rust_are_competitors/csw5t5v)

Thanks to [llogiq](https://users.rust-lang.org/users/llogiq) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
