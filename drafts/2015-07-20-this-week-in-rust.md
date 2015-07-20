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

XXX pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-05-18..2015-06-07

# New Contributors



# Approved RFCs



# Final Comment Period

Every week the teams announce a 'final comment period' for RFCs and
key PRs which are reaching a decision. Express your opinions
now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-07-06..2015-07-13

* TODO

# New RFCs


# Internals discussions

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
