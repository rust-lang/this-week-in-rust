Title: This Week in Rust 139
Number: 139
Date: 2016-07-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [Mozilla is shipping its first production Rust code in Firefox 48](https://hacks.mozilla.org/2016/07/shipping-rust-in-firefox/).
* [Linux perf gets Rust symbol demangling support](https://git.kernel.org/cgit/linux/kernel/git/tip/tip.git/commit/?id=cae15db74999edb96dd9f5bbd4d55849391dd92b).
* [Testing strategies for Corrode](http://jamey.thesharps.us/2016/07/testing-strategies-for-corrode.html). Corrode is a C to Rust translator.
* [Rust for Node.js developers - Part 3](http://fredrik.anderzon.se/rust-for-node-js-developers-part-3-crates-modules-and-the-web/). Crates, Modules and the web.
* [How to package Rust applications to RPM using vendoring](https://czanik.blogs.balabit.com/2016/07/how-to-package-rust-applications-to-rpm-using-vendoring/).
* [A Rust-powered public web page in 5 minutes](https://medium.com/@rap2h/a-rust-powered-public-website-in-5-minutes-b682d8527b6b).

## New Crates & Project Updates

* [Rust Project changelog for
  2016-07-15](https://users.rust-lang.org/t/rust-project-changelog-for-2016-07-15/6555/1). Updates
  to rustup, libc, net2, regex, websites.
* [rustup 0.3
  released](https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/144). Includes
  fixes for downloading old releases, various cleanups, and
  preliminary (non-functional) rustls support.
* [Lyon](https://github.com/nical/lyon). GPU-based 2D graphics rendering experiments in Rust.
* [Gluon](https://github.com/Marwes/gluon). A static, type inferred and embeddable language written in Rust.
* [Tango](https://github.com/pnkfelix/tango). Markdown-based Literate programming in Rust, integrated with Cargo.
* [ScreenRuster](https://github.com/meh/screenruster). X11 screen saver and locker.
* [These weeks in Servo 71](https://blog.servo.org/2016/07/18/twis-71/).
* [This week in Rust docs 13](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-13).
* [This week in Ruma - July 17, 2016](https://www.ruma.io/news/this-week-in-ruma-2016-07-17/).
* [What's coming up in imag 11](http://beyermatthias.de/blog/2016/07/15/what-s-coming-up-in-imag-11/).

# Crate of the Week

This week has a belated Crate of the Week with Vincent Esche's self-submitted [cargo-modules](https://crates.io/crates/cargo-modules), which gives us the `cargo modules` subcommand that shows the module structure of our crates in a tree view, optionally warning of orphans. Thanks, Vincent!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [imag: Make `imag` forward `--debug` and `--verbose` to subcommands](https://github.com/matthiasbeyer/imag/issues/506).
* [moderate] [rust: Very confusing error on attempt to pass
  `path::Path` by
  value](https://github.com/rust-lang/rust/issues/23286). This is bad
  error message that is hit often. Good bug to get familiar with the
  compiler.
* [easy] [rust: move coerce_match, coerce_calls and related tests into
  run-pass-valgrind](https://github.com/rust-lang/rust/issues/21696). Just
  moving tests around. Easy introduction to the build system.
* [easy] rustbyexample.com is in need of maintainers. Good first tasks
  are [writing Mutex examples](https://github.com/rust-lang/rust-by-example/issues/105)
  and [Arc examples](https://github.com/rust-lang/rust-by-example/issues/104).
* [hard] [rustup: Write a GUI installer for rustup on
  Windows](https://github.com/rust-lang-nursery/rustup.rs/issues/253). This
  is involved but should be fun. It's an integration problem, writing
  a Windows GUI that hooks into the MSI installation system and calls
  into the rustup libraries. Required for rustup 1.0.
* [easy] [cargo: Warn on the duplicate entry points for lib and
  bin](https://github.com/rust-lang/cargo/issues/2800).
* [easy] [cargo: Can't specify precise crate version if there are
  multiple versions](https://github.com/rust-lang/cargo/issues/2773).
* [easy] [error-chain: Display implementation should show the error's
  Display, not just the
  description](https://github.com/brson/error-chain/issues/2). Looks
  like a simple fix.
* [easy] [rust: Parsing inconsistencies (lambda, proc,
  return)](https://github.com/rust-lang/rust/issues/28784). This bug
  identifies some bugs where the rustc parser disagrees with the
  reference parser. Good first bug for someone interested in parsers.
* [easy] [imag: `--version` and `--versions` yield helptext instead of version(s)](https://github.com/matthiasbeyer/imag/issues/540).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

105 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-07-11..2016-07-18

* [Match whole statements in macros](https://github.com/rust-lang/rust/pull/34886)
* [Harder floats on MIPS](https://github.com/rust-lang/rust/pull/34841)
* [New method `new_parser_from_ts`](https://github.com/rust-lang/rust/pull/34829)
* [Non-Squiggly-braced Macros now need Semicolon](https://github.com/rust-lang/rust/pull/34660)
* [Simplify Macro Hygiene](https://github.com/rust-lang/rust/pull/34570)
* [Stable order for handling type projection bounds](https://github.com/rust-lang/rust/pull/34805)
* [calling directly imported trait methods no longer crashes rustc](https://github.com/rust-lang/rust/pull/34797)
* [Unicode 9.0 update](https://github.com/rust-lang/rust/pull/34599)
* [Simplify error reporting](https://github.com/rust-lang/rust/pull/34789) (potentially plugin-breaking)
* [`readdir` now also works on Solaris](https://github.com/rust-lang/rust/pull/34776)
* [String interner cleanup](https://github.com/rust-lang/rust/pull/34772)
* [`&Mutex` is now `RefUnwindSafe`](https://github.com/rust-lang/rust/pull/34756)
* [Simplify `LinkedList` with `Shared` instead of `Box`](https://github.com/rust-lang/rust/pull/34608)
* [Deprecated API spring clean](https://github.com/rust-lang/rust/pull/34705)
* [Cargo can now enable dependencies' features](https://github.com/rust-lang/cargo/pull/2876)
* [`cargo publish --dry-run`](https://github.com/rust-lang/cargo/pull/2849)


## New Contributors

* abhi
* Aravind Gollakota
* Ben Boeckel
* Ben Stern
* David
* Dridi Boukelmoune
* Isaac Andrade
* Zhen Zhang

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1574: Introduce more conventions around documenting Rust projects](https://github.com/rust-lang/rfcs/pull/1574).
* [RFC 1644: Default and expanded errors for rustc](https://github.com/rust-lang/rfcs/pull/1644).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Promote `!` to a type](https://github.com/rust-lang/rfcs/pull/1216)
* [Add language support for bitfields](https://github.com/rust-lang/rfcs/pull/1449).
* [Add support for 128-bit integers](https://github.com/rust-lang/rfcs/pull/1504).
* [Add space-friendly arguments](https://github.com/rust-lang/rfcs/pull/1509). Add `-C link-arg` and `-C llvm-arg` which allow you to pass along argument with spaces.
* [Exclude macros from importing with `#[macro_use(not(...))]`](https://github.com/rust-lang/rfcs/pull/1517).
* [Add `global_asm!` for module-level inline assembly](https://github.com/rust-lang/rfcs/pull/1548).
* [Allow all literals in attributes](https://github.com/rust-lang/rfcs/pull/1559).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [Replace synchronization primitives with those from parking_lot](https://github.com/rust-lang/rfcs/pull/1632).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [Add `assert_ne` to compliment `assert_eq`](https://github.com/rust-lang/rfcs/pull/1653).
* [Introduce non-panicking borrow methods on `RefCell<T>`](https://github.com/rust-lang/rfcs/pull/1660).
* [Propose asserts](https://github.com/rust-lang/rfcs/pull/1662). This rfc proposes that the following macros be added: `assert_gt`, `assert_lt`, `assert_ge`, and `assert_le`.

## New RFCs

* [Procedural macros 1.1](https://github.com/rust-lang/rfcs/pull/1681).
* [Startup initialized statics](https://github.com/rust-lang/rfcs/pull/1674). Introduce the ability to initialize (i.e., mutate) static items (even non-mut ones) at the beginning of main in a compiler-guaranteed safe manner.
* [Unified machine word trait](https://github.com/rust-lang/rfcs/pull/1676). Unify functionality peculiar to `i8`…`i64` and `u8`…`u64` in a trait containing the family of `overflowing`/`checked`/`wrapping`/`saturating` variants of arithmetic operations, as well as a few new ones.
* [Add non-panicking `abs()` functions to all signed integer types](https://github.com/rust-lang/rfcs/pull/1678).
* [Add "panic-safe" or "total" alternatives to the existing panicking indexing syntax](https://github.com/rust-lang/rfcs/pull/1679).

# Upcoming Events

* 7/20. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [7/21. Rust Hack & Learn Karlsruhe](http://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/232621692/).
* 7/27. Rust Community Team Meeting at #rust-community on irc.mozilla.org.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust developer at The Blackbird](https://rust.jobboard.io/jobs/394482-rust-developer-at-the-blackbird).
* [Engineering positions at Zcash mention Rust](https://z.cash/blog/hiring.html).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> fzammetti:
> Am I the only one that finds highly ironic the naming of something that's supposed to be new and cutting-edge after a substance universally synonymous with old, dilapidated and broken down?
>
> paperelectron:
> Rust is as close to the bare metal as you can get.

On [/r/programming](https://www.reddit.com/r/programming/comments/4sgzk5/shipping_rust_in_firefox/d59d2lp).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
