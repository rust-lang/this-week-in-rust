Title: This Week in Rust 83
Date: 2015-06-15
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: Brian Anderson.

# What's cooking on master?

160 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-06-07..2015-06-15

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [Prevent raw pointers from being used as explicit
  self](https://github.com/rust-lang/rust/pull/26225). Probably nobody
  has ever tried to write such code, and the current implemented
  behavior is bonkers.

# Other Changes

* [Add `Result::expect`](https://github.com/rust-lang/rust/pull/25359/files).
* [Add
  `CString::from_ptr/into_ptr`](https://github.com/rust-lang/rust/pull/25777). For
  transfering ownership of C strings across the FFI.
* [Implement `str::split_at`](https://github.com/rust-lang/rust/pull/25839).
* [Stabilize a number of new `fs` features](https://github.com/rust-lang/rust/pull/25844).
* [Parallel code generation works
  again](https://github.com/rust-lang/rust/pull/26018). Pass `rustc -C
  codegen-units=4` to try.
* [The `to_uppercase` and `to_lowercase` functions now support complex
  case mapping](https://github.com/rust-lang/rust/pull/26039). This
  changes the behavior of the *stable*
  `char::to_uppercase/to_lowercase` and also stabilizes
  `str::to_uppercase/to_lowercase`.
* [Implement `Extend<&T> where: T: Copy` for a variety of collection
  types](https://github.com/rust-lang/rust/pull/25989).
* [The unstable `String::from_str` is
  deprecated](https://github.com/rust-lang/rust/pull/26077). Use
  `String::from`.
* [Heuristics for detecting identifier typos are improved](https://github.com/rust-lang/rust/pull/26087).

# New Contributors



# Approved RFCs

* [RFC 1105. Policy on API
  evolution](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md). Describes
  how the Rust project evaluates changes to the libraries, and which are allowed when.
* [RFC 1119. `Result::expect`](https://github.com/rust-lang/rfcs/pull/1119).
* [RFC 1122. Semantic
  versioning](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md). Describes
  how the language itself is allowed to change.
* [RFC 1123. Introduce `str::split_at`](https://github.com/rust-lang/rfcs/blob/master/text/1123-str-split-at.md)
* [RFC 1131. `likely` intrinsic](https://github.com/rust-lang/rfcs/blob/master/text/1131-likely-intrinsic.md). For hinting hot and cold branches.

# New RFCs

* [Adjust default object
bounds](https://github.com/rust-lang/rfcs/pull/1156). This fixes some
dumb rules that made it into 1.0, but is a breaking change that
affects relatively little code.
* [Expand the `std::net` module](https://github.com/rust-lang/rfcs/pull/1158).

# Betawatch!

The current beta is `1.0.0-beta.3 (5241bf9c3 2015-04-25)`.

There were X PRs this week landing backports to beta.

* [NNNNN](link).

# Friend of the Tree

The Rust Team likes to occassionally recognize people who have made
outstanding contributions to The Rust Project, its ecosystem, and its
community. These people are 'friends of the tree'.

This week's friend of the tree was ...

# Notable Links

* [Weekly-meetings/2014-18-11][mtg]: what? [Reddit][mtg-reddit].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-18-11.md
[mtg-reddit]:


# Project Updates


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
