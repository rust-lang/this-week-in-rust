Title: This Week in Rust 40
Date: 2014-03-11 23:45
Category: This Week in Rust


Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This week was the Winter Workweek.

<!-- more -->

# What's cooking on master?

66 pull requests were merged this week.

## Breaking Changes

- `Any` [has been cleaned up](https://github.com/mozilla/rust/pull/12667), and
in particular the `as_void_ptr` and `as_mut_void_ptr` methods are gone.
- Patterns are [no longer allowed in extern function
declarations](https://github.com/mozilla/rust/pull/12715). Previously, they
either silently passed or ICEd.
- Comparison traits [have been
reworked](https://github.com/mozilla/rust/pull/12520).
- DeepClone [has been removed](https://github.com/mozilla/rust/pull/12706).
- Compound assignment (eg, `+=`) [is no longer
allowed](https://github.com/mozilla/rust/pull/12733) on potentially
uninitialized variables.
- `std::rand` [has been moved into a
`librand`](https://github.com/mozilla/rust/pull/12650).
- debuginfo flags [have changed
slightly](https://github.com/mozilla/rust/pull/12714), and more limited
debuginfo is supported again (only line number information).

## Other Changes

- Dereferencing [is now
overloadable](https://github.com/mozilla/rust/pull/12491). This is another
part of the smart pointer changes. Code like `let x = *Rc::new(5);` is now
valid. There is a follow-up pull request that will automatically dereference
smart pointers where appropriate, to avoid expressions like
`foo.borrow().get().borrow_mut().get()`.
- Hexadecimal floating point literals [are now
available](https://github.com/mozilla/rust/pull/12652) through a syntax
extension.
- Support for creating binary installer tarballs [has
landed](https://github.com/mozilla/rust/pull/12793).
- Linker arguments [are no longer
deduplicated](https://github.com/mozilla/rust/pull/12688).
- Weak linkage etc [is now
possible](https://github.com/mozilla/rust/pull/12556) via a `linkage`
attribute.

## New Contributors

- Dmitry Promsky
- Mike Boutin
- Robert Gawdzik

# Weekly Meeting

There was no weekly meeting due to the workweek. There are [notes and
minutes](https://github.com/mozilla/rust/wiki/Meeting-workweek-2014-03-03),
however, and there will be many [RFCs](https://github.com/rust-lang/rfcs) from
it.

# Announcements, etc

- [Leveraging tuples to make a statically-typed, concatenative
EDSL](http://www.reddit.com/r/rust/comments/20143y/leveraging_tuples_to_make_a_statically_typed/)
- [Subtyping and coercion in
Rust](http://featherweightmusings.blogspot.com/2014/03/subtyping-and-coercion-in-rust.html)
- [Rust support for the Atom
editor](http://www.reddit.com/r/rust/comments/1ztahv/rust_language_support_in_atom/)
