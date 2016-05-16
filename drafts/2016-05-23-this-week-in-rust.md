Title: This Week in Rust 131
Number: 131
Date: 2016-05-23
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

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts



## New Crates & Project Updates



# Crate of the Week

This week's Crate of the Week is [cargo-profiler](https://github.com/pegasos1/cargo-profiler), which lets us profile our code directly from cargo using a variety of tools, notably valgrind's callgrind and cachegrind. Thanks to [kbknapp](https://users.rust-lang.org/users/kbknapp) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Participate in 2016 State of Rust Survey](http://blog.rust-lang.org/2016/05/09/survey.html).
* [easy] [redox: Participate in Redox Survey](http://goo.gl/forms/RUOfIsxXsv).
* [easy] [clippy: Participate in Rust Clippy Survey](https://docs.google.com/forms/d/1k0wuWgGwDhuUL3q_cONGVxQ6PJSYq5JRZOHKc0itLbg/viewform?c=0&w=1).
* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [medium] [rustup: Make `rustup default x86_64-unknown-linux-gnu` do something smarter](https://github.com/rust-lang-nursery/rustup.rs/issues/411).
* [easy] [servo: Remove `--no-ssl` option](https://github.com/servo/servo/issues/11197).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

132 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-05-09..2016-05-16

* [custom panic runtimes](https://github.com/rust-lang/rust/pull/32900) (finally implements [RFC #1513](https://github.com/rust-lang/rfcs/blob/master/text/1513-less-unwinding.md) – embedded Rust users rejoice!)
* [cache ast-ty-to-ty again](https://github.com/rust-lang/rust/pull/33596)
* [AtomicBools are now 8 bits wide](https://github.com/rust-lang/rust/pull/33579) (used to surprisingly be one ptr wide)
* [reduce LLVM IR bloat in simple switches for large enums](https://github.com/rust-lang/rust/pull/33566)
* [reduce LLVM IR bloat on CFG simplification](https://github.com/rust-lang/rust/pull/33544)
* [split the type context into a local/global one](https://github.com/rust-lang/rust/pull/33425) for some memory savings
* [MIR: CFG block merging](https://github.com/rust-lang/rust/pull/33552)
* [MIR: allow mutable slicing of arbitrarily long arrays](https://github.com/rust-lang/rust/pull/33620)
* [MIR: Prettier MIR pretty printing](https://github.com/rust-lang/rust/pull/33607)
* [more MIR blockers fixed](https://github.com/rust-lang/rust/pull/33488)
* [save-analysis format changes](https://github.com/rust-lang/rust/pull/33370)
* [correct temp drop scheduling order](https://github.com/rust-lang/rust/pull/33239)
* [don't use `env::current_exe with libbacktrace](https://github.com/rust-lang/rust/pull/33554)
* [Better derived impls for large (mostly) C-like enums](https://github.com/rust-lang/rust/pull/33593)
* [std::sync::OnceState now public](https://github.com/rust-lang/rust/pull/33563) (though unstable)
* [normalize trait refs before use](https://github.com/rust-lang/rust/pull/33478/files)
* [const_eval can now cast byte slices to bare ptrs](https://github.com/rust-lang/rust/pull/33457)
* [earlier name resolution](https://github.com/rust-lang/rust/pull/33443)
* [allow repr attribute on single variant enum](https://github.com/rust-lang/rust/pull/33355)
* [`impl (Partial)Ord for Cell<T>+RefCell<T> where T: (Partial)Ord](https://github.com/rust-lang/rust/pull/33306)
* [`std::process::ExitStatus::from_raw(_)`](https://github.com/rust-lang/rust/pull/33224)
* [integer atomic types](https://github.com/rust-lang/rust/pull/33048)
* [fix gdb pretty printing](https://github.com/rust-lang/rust/pull/33612)
* [the playpen now uses the new error format](https://github.com/rust-lang/rust-playpen/pull/207)
* [rustdoc no longer skips blanket impls](https://github.com/rust-lang/rust/pull/33514)
* [make lifetime error reporting less confusing](https://github.com/rust-lang/rust/pull/33544)
* There were a lot of documentaion improvements, too numerous to mention each.

## New Contributors

* billyevans
* bnewbold
* Brian Green
* Cristian Oliveira
* Dan Fockler
* Geordon Worley
* Haiko Schol
* mrmiywj
* Pavel Sountsov
* silvo38
* Stefan Schindler
* Steven Burns

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1358: Add `#[repr(align = "N")]`](https://github.com/rust-lang/rfcs/pull/1358).
* [RFC 1492: Permit the `..` pattern fragment in more contexts](https://github.com/rust-lang/rfcs/pull/1492).
* [Amendment to RFC 1214: All but the last field of a tuple must be Sized](https://github.com/rust-lang/rfcs/pull/1592).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

*No RFCs are currently in final comment period.*

## New RFCs

* [Let Cargo put data into platform-specific directories](https://github.com/rust-lang/rfcs/pull/1615).
* [Add an owning "borrowed" pointer type `&move`](https://github.com/rust-lang/rfcs/pull/1617).

# Upcoming Events

* [5/17. Moscow Rust Conference](https://rustycrate.ru/%D0%BD%D0%BE%D0%B2%D0%BE%D1%81%D1%82%D0%B8/2016/04/25/colaboratory-rust.html).
* [5/18. Rust Meetup Taipei #2](http://www.meetup.com/RUST-TW/events/230709491/).
* 5/18. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/18. Rust Berlin: Rust 1.0 Anniversary Meetup](http://www.meetup.com/Rust-Berlin/events/230810678/).
* [5/24. Rust Brisbane Meetup #1](http://www.meetup.com/Rust-Brisbane/events/230676018/).
* [5/24. Rust Barcelona - Cross compiling for ARM linux targets + accessing the GPIO in a Raspberry Pi](http://www.meetup.com/Rust-Barcelona/events/230638451/).
* 5/25. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/25. Boston Rust Meetup w/ Steve Klabnik and Andrew Gallant](http://www.meetup.com/BostonRust/events/230419544/).
* [5/26. Rust DC - Inaugural Meetup](http://www.meetup.com/RustDC/events/227138240/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
