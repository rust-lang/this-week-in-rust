Title: This Week in Rust 31
Date: 2014-01-06 01:09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

0.9 is on the brink. Perhaps this week, the next at the latest. Some hefty
changes come this week.

<!-- more -->

# What's cooking on master?

63 pull requests were merged this week.

## Breaking changes

- `@mut` [has been removed](https://github.com/mozilla/rust/pull/11251). The
replacements are `Gc<RefCell<T>>` or, preferably, `Rc<RefCell<T>>`. See also
`Cell<T>` for `Pod` types.
- Dereferencing of unary tuple structs and enums (tuple structs with one field
and enums with one variant) [has been
removed](https://github.com/mozilla/rust/pull/11188). An example:

```
struct Foo(int);

fn bar() {
	let x = Foo(42);
	// previously `*x` would return 42
	let Foo(y) = x; // but now you do this pattern matching
}
```

- `Either` [has been removed](https://github.com/mozilla/rust/pull/11149).
- `extern mod foo (name="bar")` syntax [has been
removed](https://github.com/mozilla/rust/pull/10696). The newer `extern mod
foo = "bar"` syntax replaces it.
- The hashmap iterators [have
changed](https://github.com/mozilla/rust/pull/11242). Use `.keys()` and
`.values()` respectively.
- rustc now outputs [1-based column
numbers](https://github.com/mozilla/rust/pull/11184). This shouldn't
affect any tooling that assume columns are 0-based. But, this brings us in
line with gcc.
- The `cfg` attribute [will now strip struct fields and enum
variants](https://github.com/mozilla/rust/pull/11093).
- The contents of the `crate_type` attribute [are now checked for
validity](https://github.com/mozilla/rust/pull/11264).

## Other changes

- `Reader`/`Writer` have regrown some functionality. It can now [read and
write strings](https://github.com/mozilla/rust/pull/10861), though *only* real
UTF-8 strings. Encoding support will come later. `write_char` [also
reappeared](https://github.com/mozilla/rust/pull/11310).
- Unwinding on ARM [has been
fixed](https://github.com/mozilla/rust/pull/11301). Still no C++!
- Native UDP IO [has been
implemented](https://github.com/mozilla/rust/pull/11186).
- A `rust_fail` function [has been
added](https://github.com/mozilla/rust/pull/11231). Break on this function to
catch failure. Should be easier to remember! Certainly better than
`_Unwind_RaiseException`.
- Some more C [has been converted to
Rust](https://github.com/mozilla/rust/pull/11208).
- volatile load and store intrinsics [have been
added](https://github.com/mozilla/rust/pull/11173).


Additionally, it hasn't landed yet, but [external syntax
extensions](https://github.com/mozilla/rust/pull/11151), aka loadable syntax
extensions, aka procedural macros, seem to be really close to landing. This is
quite exciting!

## New Contributors

- Alan Andrade
- Carl-Anton Ingmarsson
- John Louis Walker
- Peter Zotov
- lyuts
- Matthew Auld

# Meeting

Once again, no meeting due to the holidays. There will be one on January 7,
however.

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

Mozilla Corp. came back from vacation this Thursday, January 2nd. We landed 14
PRs this week.

## Notable additions
- Aydin Kim fixed Android support - hooray! - in
[#1445](https://github.com/mozilla/servo/pull/1445).
- Mike Blumenkrantz cleaned up our configure script to avoid the "configure
smashed my submodule edits" problem in
[#1449](https://github.com/mozilla/servo/pull/1449).
- deokjinkim fixed several font-related issues in
[#1454](https://github.com/mozilla/servo/pull/1454) and
[#1452](https://github.com/mozilla/servo/pull/1452).
- Simon Sapin refactored font styles in
[#1455](https://github.com/mozilla/servo/pull/1455).
- Shamir Khodzha implemented `child_elements` for filtered iteration in
[#1443](https://github.com/mozilla/servo/pull/1443).
- ms2ger landed several changes to attributes in
[#1439](https://github.com/mozilla/servo/pull/1439),
[#1456](https://github.com/mozilla/servo/pull/1456), and
[#1460](https://github.com/mozilla/servo/pull/1460).

## New contributors
- Shamir Khodzha
- Mike Blumenkrantz

# Announcements, etc

- [DST, take
5](http://smallcultfollowing.com/babysteps/blog/2014/01/05/dst-take-5/). I
think this might be the last DST proposal, and I quite like it.
- [rust-redis and
rust-msgpack](https://mail.mozilla.org/pipermail/rust-dev/2014-January/007687.html).
- [A capnproto-rust and zmq
example](http://dwrensha.github.io/capnproto-rust/2014/01/04/zmq-explorers.html).
- [Post-mortem from the OS class taught in
Rust](http://www.reddit.com/r/rust/comments/1ucrfg/using_rust_for_an_undergraduate_os_course/).
- [Using CMake with
Rust](https://mail.mozilla.org/pipermail/rust-dev/2014-January/007659.html)
- [boehm-rs](https://github.com/huonw/boehm-rs), a `Gc<T>` type with a real
GC!
- [rust-OpenBLAS](https://github.com/wellposed/rust-OpenBLAS), a (in-progress)
library for using OpenBLAS and LAPACK.
- [libhttpd](https://github.com/WebeWizard/libhttpd/tree/master), a library
for writing web servers. Intended to be similar to jetty.
