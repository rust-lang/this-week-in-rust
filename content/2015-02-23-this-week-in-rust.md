Title: This Week in Rust 71
Date: 2015-02-23
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

# The big news

Rust [1.0.0-alpha.2] was released on Friday, but keep using
nightlies. Six more weeks until the beta, which should become
1.0. *Only* six more weeks.

[1.0.0-alpha.2]: http://blog.rust-lang.org/2015/02/20/Rust-1.0-alpha2.html

# What's cooking on master?

127 pull requests were [merged in the last week][merged], and XXX [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-16..2015-02-23
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-16..2015-02-23

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* A [variety of tweaks][thread] have been made to `std::thread` before
  declaring it stable.
* The `vec!` macro accepts a new form, `vec![element; len]`, which
  [produces a vector containing a number of clones of the same
  element][elem].
* The `CString` type has seen a few modifications, as well as the
  [introduction of the `CStr` type][cstr].
* `IntoIterator` now has an [associated type for specifying the element
  type][into].
* `ExactSizeIterator` is [no longer implemented][exact] for 64-bit
  ranges.
* In target specs, `target-word-size` [is now called
  `target-pointer-width`][tpw], to match the recently changed
  `target_pointer_width` cfg attribute.
* [`fmt::Writer` was renamed to `fmt::Write`][write].
* A [number][audit1] of [modules][audit2] have been [audited][audit3]
  for incorrect use of `isize` and `usize`.
* Use of arbitrary attributes not defined by the language [is feature
  gated][attr].

[cstr]: https://github.com/rust-lang/rust/pull/22482
[elem]: https://github.com/rust-lang/rust/pull/22455
[thread]: https://github.com/rust-lang/rust/pull/22435
[tpw]: https://github.com/rust-lang/rust/pull/22191
[exact]: https://github.com/rust-lang/rust/pull/22299
[write]: https://github.com/rust-lang/rust/pull/22311
[into]: https://github.com/rust-lang/rust/pull/22313
[audit1]: https://github.com/rust-lang/rust/pull/22339
[audit2]: https://github.com/rust-lang/rust/pull/22401
[audit3]: https://github.com/rust-lang/rust/pull/22485
[attr]: https://github.com/rust-lang/rust/pull/22364

## Other Changes

* [`Send` no longer requires `'static`][send], which make it possible
  to express various data-parallel scenarious by sharing interior
  pointers. [RFC][send-rfc].
* The ['Macros'] chapter of TRPL has been overhauled and a new
  ['Advanced Macros'] chapter added.
* `rustc --version` now reports the [build date] in addition to the
  commit date, to make it more clear what nightly you are on.
* Florian Hahn [added a new category of 'parse-fail' tests][parse] to
  the test suite. This makes it easier to compare model parsers to the
  production parser.

[send]: https://github.com/rust-lang/rust/pull/22319
[send-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0458-send-improvements.md
[build date]: https://github.com/rust-lang/rust/pull/22201
[parse]: https://github.com/rust-lang/rust/pull/22118
['Macros']: http://doc.rust-lang.org/book/macros.html
['Advanced Macros']: http://doc.rust-lang.org/book/advanced-macros.html

## New Contributors



## Approved RFCs



## New RFCs


# Friend of the Tree

The Rust Team likes to occassionally recognize people who have made
outstanding contributions to The Rust Project, its ecosystem, and its
community. These people are 'friends of the tree'.

This week's friend of the tree was ... Toby Scrace.

"Today I would like to nominate Toby Scrace as Friend of the
Tree. Toby emailed me over the weekend about a login vulnerability on
crates.io where you could log in to whomever the previously logged in
user was regardless of whether the GitHub authentication was
successful or not. I very much appreciate Toby emailing me privately
ahead of time, and I definitely feel that Toby has earned becoming
Friend of the Tree."

# Quote of the Week

```
15:35 <Binero> so a Cow is a String that clones as soon as you try to mutate it?
13:53 <scott> it clones when you try to mootate it
```

# Notable Links

* [Weekly-meetings/2014-18-11][mtg]: what? [Reddit][mtg-reddit].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-18-11.md
[mtg-reddit]:


# Project Updates



# Upcoming Events

* [What?]

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar] or [Brian Anderson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: Erick Tryzelaar <erick.tryzelaar@gmail.com>
[brson] Brian Anderson <banderson@mozilla.com>
