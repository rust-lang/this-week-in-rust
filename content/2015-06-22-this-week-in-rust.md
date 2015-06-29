Title: This Week in Rust 84
Date: 2015-06-22
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

This week's edition was edited by: [Brian Anderson](https://github.com/brson), [Vikrant Chaudhary](https://github.com/nasa42), [Andrew Gallant](https://github.com/BurntSushi), and [mdinger](https://github.com/mdinger).

# From the Blogosphere

* [Exceptional results: error handling with C# and Rust](https://ruudvanasseldonk.com/2015/06/17/exceptional-results-error-handling-in-csharp-and-rust). Exceptions based error handling in C# vs Rust's monadic approach.
* [Rust Torrent](http://pietro.menna.net.br/recurse-center/rust/2015/06/19/rusty-torrent/). Pietro Menna shares his experiece of writing a BitTorrent client in Rust.
* [Exploring Rust](http://www.wilfred.me.uk/blog/2015/06/18/exploring-rust/). A brief look at state of affairs in Rust 1.0.
* [Rust using Visual Studio Code](https://mobiarch.wordpress.com/2015/06/16/rust-using-visual-studio-code/). Setup Visual Studio Code for Rust development.

# Tips & Tricks

* [How to pass a closure into a trait object](http://camjackson.net/post/rust-lang-how-to-pass-a-closure-into-a-trait-object).
* [Rust error stacktraces](http://phildawes.net/blog/2015/06/17/rust-stacktrace/). Get stacktrace from errors in production code.
* [Benchmarking in Rust with `libtest`](https://llogiq.github.io/2015/06/16/bench.html).

# In the News

* [Google Bazel added support for Rust](https://github.com/google/bazel/tree/master/tools/build_rules/rust).
* [Impala: a Rust dialect that can partially evaluate functions at compile time and produce GPU code](http://compilers.cs.uni-saarland.de/papers/ppl14_web.pdf).
* [Rust: Announcing the community subteam](https://internals.rust-lang.org/t/announcing-the-community-subteam/2248).

# New Releases & Project Updates

* [RustLex](https://github.com/LeoTestard/rustlex). Lexical analysers generator for Rust.
* [rsedis](https://github.com/seppo0010/rsedis). Redis re-implemented in Rust.
* [cargo add](https://github.com/withoutboats/cargo-add). A utility for adding cargo dependencies from the command line.
* [volley](https://github.com/jonhoo/volley). A benchmarking tool for measuring the performance of server networking stacks.
* [Rust Dispatcher](https://github.com/timonv/rdispatcher). Dispatcher for Rust, broadcast and subscribe many to many.
* [rust-vim-setup](https://github.com/ivanceras/rust-vim-setup). Use VIM as your Rust IDE - set of bash scripts and a customised `vimrc` for Rust development.
* [Herd](https://github.com/imjacobclark/herd). An experimental HTTP load testing application written in Rust.
* [MaidSafe's Rust rewrite is going well](https://forum.safenetwork.io/t/maidsafe-dev-update-8th-june-2015/4069).
* [claxon](https://github.com/ruud-v-a/claxon). A FLAC decoder.

[Rust by example](http://rustbyexample.com/) has received a number of
improvements recently:

* February 15, 2015: The [flow control
  section](http://rustbyexample.com/flow_control.html) was
  [created](https://github.com/rust-lang/rust-by-example/pull/421) to
  house all flow control operations together.
* March 21, 2015: The [formatting section](http://rustbyexample.com/hello/print.html) was
  [revised](https://github.com/rust-lang/rust-by-example/pull/496) so
  new users are immediately confronted with the distinction of `Debug`
  and `Display` and how to deal with them.
* May 2, 2015: The table of contents was
  [reorganized](https://github.com/rust-lang/rust-by-example/pull/561)
  so examples are sorted consistently by categories.
* May 23, 2015: The [generics
  section](http://rustbyexample.com/generics.html) was majorly
  [expanded](https://github.com/rust-lang/rust-by-example/pull/572).
* June 15, 2015: The [closures
  section](http://rustbyexample.com/fn/closures.html) was completely
  rewritten and
  [expanded](https://github.com/rust-lang/rust-by-example/pull/594).

# What's cooking on master?

112 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-06-15..2015-06-22

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [Don't panic when stdout doesn't
  exist](https://github.com/rust-lang/rust/pull/26168). See [RFC
  1014](https://github.com/rust-lang/rfcs/blob/master/text/1014-stdout-existential-crisis.md). This
  is breaking because it changes the behavior of stdio, but in ways
  that are expected to be less surprising. Considered a bugfix.

# Other Changes

* Thanks to Ashesh Kumar for pointing out that rust-lang.org was not
  configured with DMARC to prevent spoofing. The misconfiguration has
  now been corrected.
* [Optimize implementations of FromIterator and Extend for
  Vec](https://github.com/rust-lang/rust/pull/22681).
* [Result - Add an `expect` method that prints a message and the `Err`
  value](https://github.com/rust-lang/rust/pull/25359).
* [Break apart global unstable
  features](https://github.com/rust-lang/rust/pull/26192). Some of the
  catch-all feature names, `core`, `std_misc`, `collections`, `alloc`,
  are split into smaller, better-named features.
* The regex crate received a [rewrite that includes a big performance
  improvement](https://github.com/rust-lang/regex/pull/91).
* [Avoid deref/ref cycles for no-op conversions between unsafe
  pointers](https://github.com/rust-lang/rust/pull/26336). Reduces the
  amount of IR rustc generates.
* [Pass fat pointers in two immediate
  arguments](https://github.com/rust-lang/rust/pull/26411). More
  codegen improvements from dotdash.
* [Add FromRow{Fd,Handle,Socket} to os
  preludes](https://github.com/rust-lang/rust/pull/26413).
* [Custom Debug impl for
  io::Error](https://github.com/rust-lang/rust/pull/26416).

# New Contributors

* David Stygstra
* Gulshan Singh
* Jake Hickey
* joliv
* Markus
* Steven Walter
* Yongqian Li

# Approved RFCs

* [Update RFC 195 to account for RFC
  246](https://github.com/rust-lang/rfcs/pull/865). Just accounting
  for the `const`/`static` distinction in the associated items RFC.
* [Clarify cast rules, especially regarding fat
  pointers](https://github.com/rust-lang/rfcs/pull/1052). Updates RFC
  401: coercions.
* [RFC 1156: Adjust default object
  bounds](https://github.com/rust-lang/rfcs/blob/master/text/1156-adjust-default-object-bounds.md). This
  is a stable breaking change (the first) to the default lifetime
  bounds of trait objects.

# Final Comment Period

Every week the teams announce a 'final comment period' for RFCs which
are close to reaching a conclusion. Express your opinions now. [This
week's][fcp] RFCs entering FCP are:

[fcp]: https://github.com/rust-lang/rfcs/pulls?q=is%3Aopen+is%3Apr+label%3Afinal-comment-period

* [Allow closure expressions to expand to a `&` or `&mut` temporary](https://github.com/rust-lang/rfcs/pull/756).
* [Allow macros in types](https://github.com/rust-lang/rfcs/pull/873).
* [read_all](https://github.com/rust-lang/rfcs/pull/980).
* [Add read_into_buf and get_buf to BufRead](https://github.com/rust-lang/rfcs/pull/1015).
* [Rename `connect` to `join`](https://github.com/rust-lang/rfcs/pull/1102).
* [Implement `FromIterator` for the unit type](https://github.com/rust-lang/rfcs/pull/1130).
* [Add some of `[T]`'s methods to strings and vice-versa](https://github.com/rust-lang/rfcs/pull/1152).

# New RFCs

* [Make `size` an associated constant](https://github.com/rust-lang/rfcs/pull/1168).

# Upcoming Events

* [6/23. Hannover](http://blog.thoughtram.io/rust/2015/06/17/anouncing-hanovers-second-rust-meetup.html)
* [6/24. Columbus Rust Society](http://www.meetup.com/columbus-rs/)
* [6/29. Sydney](http://www.meetup.com/Rust-Sydney/events/222811456/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

