Title: This Week in Rust 46
Date: 2014-04-26 14:06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This issue combines this week and last, since I was very busy with school last
week, and did not have time to write.

<!-- more -->

# What's cooking on master?

153 pull requests were merged in the last two weeks.

## Breaking Changes

- Auto-rooting of `@` [has been
  removed](https://github.com/mozilla/rust/pull/13559). The exact fallout of
  this isn't obvious to me, but presumably this makes some uses of `@` not
  work.
- `std::task::task` [has been
  renamed](https://github.com/mozilla/rust/pull/13675) to `TaskBuilder::new`.
- Closures can [no longer be
  applied](https://github.com/mozilla/rust/pull/13686) through a `&`-pointer.
  This fixes some memory unsafety.
- The `Round` trait [has been
  removed](https://github.com/mozilla/rust/pull/13597), and is now part of
  `Float`, and `Float` now takes things by-value.
- `Unsafe<T>` [is now always
  `Share`](https://github.com/mozilla/rust/pull/13583), regardless of whether
  or not the contained type is `Share`.
- Modulo (`%` operator) on float types [has been
  removed](https://github.com/mozilla/rust/pull/13410), use the `rem` method
  instead.
- `~[T]` [is no longer growable](https://github.com/mozilla/rust/pull/13588).
- Some `Bitv` method names [have
  changed](https://github.com/mozilla/rust/pull/13572).
- The `priv` keyword [is no longer
  used](https://github.com/mozilla/rust/pull/13547), but is still reserved.
- Some cases where destructors were not run [have been
  fixed](https://github.com/mozilla/rust/pull/13390).
- `unwrap` and `unwrap_err` on `Result` [now require the wrapper type to
  implement `Show`](https://github.com/mozilla/rust/pull/13479).
- Some return types in `std::comm` [have been made
  consistent](https://github.com/mozilla/rust/pull/13448).

## Other Changes

- There is now a [pure-Rust regular expression
  library](https://github.com/mozilla/rust/pull/13700) in the standard
  library. As I've come to expect from burntsushi, the docs are fantastic.
- [Unix sockets](https://github.com/mozilla/rust/pull/13723) and [TCP
  sockets](https://github.com/mozilla/rust/pull/13688) now support accept with
  a timeout. [TcpStream::connect](https://github.com/mozilla/rust/pull/13604) can also
  take a timeout.
- [64-bit Windows is now partially
  supported](https://github.com/mozilla/rust/pull/13692). Unwinding still
  doesn't work.
- `&&` [is now parsed as `& &`](https://github.com/mozilla/rust/pull/13576)
  when appropriate.
- Errors about use of moved values [are much nicer
  now](https://github.com/mozilla/rust/pull/13418).
- Cloning vectors [is now much much
  faster](https://github.com/mozilla/rust/pull/13539).
- SipHash [has also been
  optimized](https://github.com/mozilla/rust/pull/13522).
- Steve Klabnik's 30 minute introduction to Rust [has been added as official
  documentation](https://github.com/mozilla/rust/pull/13416).

## New Contributors

- Aaron Turon
- Adolfo Ochagavía
- Andrew Gallant
- Brandon Waskiewicz
- Brendan McLoughlin
- Chris Shea
- Jacob Hegna
- James Sanders
- John Fresco
- John Simon
- Manish Goregaokar
- Meyer S. Jacobs
- Michael Fairley
- Richo Healey
- Ryan Mulligan
- Rüdiger Sonderfeld
- Thomas Backman
- iancormac84
- mdinger

# Weekly Meeting

- [Two weeks
  ago](https://github.com/mozilla/rust/wiki/Meeting-weekly-2014-04-15), a
  bunch of RFCs were discussed, as well as a breaking change log.
- [Last week], some more RFCs were discussed, notably the regex crate, numeric
  type inference, and disableable asserts.

# RFCs

- [Linker placement attribute](https://github.com/rust-lang/rfcs/pull/44)
- [Avoiding integer overflow](https://github.com/rust-lang/rfcs/pull/45)
- [Writer size hints](https://github.com/rust-lang/rfcs/pull/46)
- [Revised trait matching](https://github.com/rust-lang/rfcs/pull/48)
- [Disableable assertions](https://github.com/rust-lang/rfcs/pull/50)
- [Macro name resolution](https://github.com/rust-lang/rfcs/pull/51)
- [Private trait items](https://github.com/rust-lang/rfcs/pull/52)
- [Coroutines](https://github.com/rust-lang/rfcs/pull/53)

# Community Updates

- For all Mac users, there is now
  [dash-rust](https://github.com/indirect/dash-rust/), for Rust API docs in
  Dash.
- Another [Rust By Example](http://rustbyexample.github.io/) has been created.
  This one is much more complete and also looks pretty nice.
- [Teepee](http://chrismorgan.info/blog/introducing-teepee.html) has been
  announced, the successor to `rust-http`.
- [zinc](https://mail.mozilla.org/pipermail/rust-dev/2014-April/009618.html),
  a bare-metal Rust stack.
- [An IntelliJ Rust plugin](https://github.com/Vektah/idea-rust).
- [Rust for C++
  Programmers](http://featherweightmusings.blogspot.co.nz/search/label/rust-for-c).
- [A very fast n-queens solver](https://github.com/reem/rust-n-queens).

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test cases for the Rust language.

In the last week, we landed 29 PRs. There are several very large PRs waiting to land behind an impending Rust upgrade, which will bring us April 10th.

## Notable additions

- Matt Brubeck worked around a long-standing issue causing Servo to look crunched on HIDPI displays in [#2224](https://github.com/mozilla/servo/pull/2224)
- Harry Maclean made `Node.Normalize()` work on all its descendants in [#2221](https://github.com/mozilla/servo/pull/2221)
- jgraham cleaned up the Web Platform Tests integration in [#2216](https://github.com/mozilla/servo/pull/2216)
- ms2ger, among many other things, added support for the `Any` type in dictionaries in [#2225](https://github.com/mozilla/servo/pull/2225)
- Tetsuharu Ohzeki added helpers that significantly cleaned up script's layout queries in [#2210](https://github.com/mozilla/servo/pull/2210)
- jdm brought back the "I tried" star for failed pages in [#2200](https://github.com/mozilla/servo/pull/2200)
- Peiyong Lin implemented `Element.localName` in [#2209](https://github.com/mozilla/servo/pull/2209)
- Tom Schuster implemented `ParentNode.children` in [#2192](https://github.com/mozilla/servo/pull/2192)
- jdm also added a basic browser context in [#2111](https://github.com/mozilla/servo/pull/2111)
- Manish Goregaokar added support for tracking the WPT manifest in [#2187](https://github.com/mozilla/servo/pull/2187)
- Sankha Guria implemented `Element.prefix` in [#2199](https://github.com/mozilla/servo/pull/2199)
- Bruno Abinader implemented `createDocument` in [#2072](https://github.com/mozilla/servo/pull/2072)

## New Contributors
- Harry Maclean (hazz)

## Meetings and Notes

In this week's [meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-04-21) we went over the Rust upgrade status, some medium-sized project brainstorming we've been doing, the Web Platform Tests support in Servo, and fixing iframes.

# This Week in Servo
Servo is a web browser engine written in Rust and is one of the primary test cases for the Rust language.

In the last week, we landed 39 PRs.

## Notable additions

- Manish Goregaokar landed support of the Web Platform Tests in [#2089](https://github.com/mozilla/servo/pull/2089)
- ms2ger improved the integration of WPT with our build system in [#2162](https://github.com/mozilla/servo/pull/2162) and [#2180](https://github.com/mozilla/servo/pull/2180)
- Philip Horger handled treating HTTPS request as a network error in [#2166](https://github.com/mozilla/servo/pull/2166)
- Peiyong Lin cleaned up some parser code in [#2157](https://github.com/mozilla/servo/pull/2157)
- James Sanders associated ResourceTask with URLProvenance in [#2152](https://github.com/mozilla/servo/pull/2152)
- Josh Matthews added `Traceable` and `Untraceable` types to clean up rooting in [#2147](https://github.com/mozilla/servo/pull/2147)
- Lars Bergstrom changed the default rendering mode to CPU on Android [#2148](https://github.com/mozilla/servo/pull/2148)
- Simon Sapin removed some unnecessary `unsafe` code in [#2145](https://github.com/mozilla/servo/pull/2145)
- Matthew Brubeck fixed some terrible bugs in [#2135](https://github.com/mozilla/servo/pull/2135) and [#2134](https://github.com/mozilla/servo/pull/2134) and [#2130](https://github.com/mozilla/servo/pull/2130)
- Sanhka Guria added attribute setters and getters for `HTMLImageElement` in [#2054](https://github.com/mozilla/servo/pull/2054)


## Meetings and Notes

In this week's [meeting](https://github.com/mozilla/servo/wiki/Meeting-2014-04-14) we went over our Rust upgrade strategy, linking, embedding, rooting, Android support, and the commit we missed landing in one submodule for Acid2.
