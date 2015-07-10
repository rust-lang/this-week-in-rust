Title: This Week in Rust 87
Date: 2015-07-13
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

# Tips & Tricks

# In the News

# New Releases & Project Updates

# What's cooking on nightly?

XXX pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-07-06..2015-07-13

* [Linux installation will try harder to set up the dynamic linker](https://github.com/rust-lang/rust-installer/pull/41), fixing a bad first-run issue where Fedora systems can't run rustc out of the box.
* The `#[prelude_import]` attribute, which is employed by rustc to perform [dark](https://github.com/rust-lang/rust/blob/6a3b385cbd6b9044b4447da96aad066e8b257ddf/src/libsyntax/std_inject.rs#L164) and [mysterious](https://github.com/rust-lang/rust/blob/6a3b385cbd6b9044b4447da96aad066e8b257ddf/src/librustc_resolve/build_reduced_graph.rs#L292-L294) acts, but is not supposed to be stable. This is not known to break real code.
* `rustc` on Windows now [looks in the registry](https://github.com/rust-lang/rust/pull/26741) to find the location of the MSVC linker.

# New Contributors



# Approved RFCs

* [RFC 1058: Replace `slice.tail()`, `slice.init()` with new methods `slice.split_first()`, `slice.split_last()`](https://github.com/rust-lang/rfcs/blob/master/text/1058-slice-tail-redesign.md).
* [RFC 1102: Rename `connect` to `join`](https://github.com/rust-lang/rfcs/blob/master/text/1102-rename-connect-to-join.md).

# Final Comment Period

Every week the teams announce a 'final comment period' for RFCs which
are reaching a decision. Express your opinions now. [This week's][fcp]
RFCs entering FCP are:

[fcp]: https://github.com/rust-lang/rfcs/pulls?q=is%3Aopen+is%3Apr+label%3Afinal-comment-period

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

* [What?]

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*"Quote"*

Explanation and link.

Thanks to XXX for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
