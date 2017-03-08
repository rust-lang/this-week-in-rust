Title: This Week in Rust 172
Number: 172
Date: 2017-03-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [Rust's language ergonomics initiative](https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html). To help bring [2017 vision for Rust](https://blog.rust-lang.org/2017/02/06/roadmap.html) to fruition, Rust language team is launching an initiative: improving the ergonomics of the core language.
* [What are sum, product, and pi types](https://manishearth.github.io/blog/2017/03/04/what-are-sum-product-and-pi-types/).
* [The future with Futures](http://asquera.de/blog/2017-03-01/the-future-with-futures/). Writing asynchronous Rust with Futures and Tokio.
* [Designing non-lexical borrows: nested method calls via two-phase borrowing](http://smallcultfollowing.com/babysteps/blog/2017/03/01/nested-method-calls-via-two-phase-borrowing/).
* [Mitigating underhandedness: Fuzzing your code](https://manishearth.github.io/blog/2017/03/02/mitigating-underhandedness-fuzzing-your-code/)
* [What I Learned from Giving a Rust Training](http://asquera.de/blog/2017-02-27/rust-training/).
* [Trying out Rust for graphics programming](http://members.iinet.net.au/~ideasman42/blog/learning_rust.html).
* [Setting up a Rust development environment](http://asquera.de/blog/2017-03-03/setting-up-a-rust-devenv/).
* [Rust faster than C? Not so fast…](https://dennisforbes.ca/index.php/2017/03/03/rust-faster-than-c-not-so-fast/). Follow-up to [Rust beating C in The Computer Language Benchmarks Game](https://benchmarksgame.alioth.debian.org/u64q/performance.php?test=knucleotide).
* [LabelledGeneric in Rust: What, why, how](https://beachape.com/blog/2017/03/04/labelledgeneric-in-rust-what-why-how/)?
* [RESTful microservices in Rust - Part 1](https://codingwithglee.blogspot.de/2017/02/my-shot-at-restful-microservices-in.html). Also available: [Part 2](https://codingwithglee.blogspot.de/2017/03/my-shot-at-restful-microservices-in.html).
* [Rust Language Server alpha 2 released bringing new features, better stability, and an easier installation than the first alpha](http://www.jonathanturner.org/2017/03/rls-alpha-2.html).
* [Redox releases 0.1.1 which adds support for resizing windows, mouse scroll, changing window titles, and improves the look and feel of many Redox applications](https://github.com/redox-os/redox/releases/tag/0.1.1).
* [video] [Videos from March 2017 Rust Meetup at Mozilla SF](https://air.mozilla.org/rust-meetup-march-2017/). GPU glyph rasterization, Rocket, and the orphan rules.
* [video] [LLVM Social Berlin #6: Mull meets Rust. Implementing mutation testing system for Rust](https://www.youtube.com/watch?v=VasSufnFswc).

## Other Weeklies from Rust Community

* [This week in Rust docs 46](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-46).
* [This week in Tock Embedded OS 11](https://www.tockos.org/blog/2017/talking-tock-11/).

# Crate of the Week

This week's crate of the week is [nalgebra](https://crates.io/crates/nalgebra), a linear algebra library in and for Rust. Thanks to [nasa42](https://users.rust-lang.org/users/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rails Girls Summer of Code + Servo](https://blog.servo.org/2017/02/27/rgsoc/).
* [easy] [servo: Looking for something to work on](https://github.com/servo/servo/issues/15162).
* [easy] [clippy:  Extend option-map-unwrap-or-else to Result](https://github.com/Manishearth/rust-clippy/issues/1590).
* [easy] [clippy: Always true expressions in if expressions](https://github.com/Manishearth/rust-clippy/issues/1593).
* [easy] [clippy: Spot wrong usage of bitwise and operator](https://github.com/Manishearth/rust-clippy/issues/1594).
* [easy] [clippy: should_implement_trait should include ToOwned, FromStr](https://github.com/Manishearth/rust-clippy/issues/1600).
* [easy] [crates.io: bad appveyor badges for projects with underscore](https://github.com/rust-lang/crates.io/issues/587).
* [easy] [crates.io: Document applying categories/adding new categories](https://github.com/rust-lang/crates.io/issues/544).
* [medium] [crates.io: Be able to search within a keyword or category](https://github.com/rust-lang/crates.io/issues/491).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

124 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-20..2017-02-27

* [fix LLVM x86 interrupt calling convention](https://github.com/rust-lang/llvm/pull/63)
* [fix ICE in trans](https://github.com/rust-lang/rust/pull/40064)
* [properly display expected details on type mismatch](https://github.com/rust-lang/rust/pull/39905)
* [`-C overflow-checks` option](https://github.com/rust-lang/rust/pull/40037)
* [stabilize `static_recursion`](https://github.com/rust-lang/rust/pull/40027)
* [allow tools to run test runners programmatically](https://github.com/rust-lang/rust/pull/39815)
* [allow coercion of non-capturing closures to `fn`s](https://github.com/rust-lang/rust/pull/40025)
* [improved lib defaults handling](https://github.com/rust-lang/rust/pull/40022)
* [make `Unique<T>` `UnwindSafe` even for unsized types](https://github.com/rust-lang/rust/pull/40020)
* [set LLVM metadata for vtables](https://github.com/rust-lang/rust/pull/39995) (allows better optimizations)
* [track the `-Z sanitizer` flag over incremental compilations](https://github.com/rust-lang/rust/pull/39993)
* [incremental compilation: detect bootstrap outputs](https://github.com/rust-lang/rust/pull/40038)
* [adaptive hashmap: consider displacement instead of shift length](https://github.com/rust-lang/rust/pull/39988)
* [don't treat privately uninhabited types as uninhabited](https://github.com/rust-lang/rust/pull/39980)
* [improved type inference error reporting](https://github.com/rust-lang/rust/pull/39913)
* [verify all sysroot crates are unstable](https://github.com/rust-lang/rust/pull/39851)
* [Cargo: fix required features vs. dependencies' features interference](https://github.com/rust-lang/cargo/pull/3737)
* [`cargo check --all`](https://github.com/rust-lang/cargo/pull/3731)
* [migrate Cargo from rustc-serialize to serde](https://github.com/rust-lang/cargo/pull/3682)
* [docs: port the reference to mkbook](https://github.com/rust-lang/rust/pull/39855)

## New Contributors

* Ben Schreiber
* deso
* lukaramu
* Maik Riechert

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period


* [disposition: merge] [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [`From<&[T]> for Rc<[T]> + From<&str> for Rc<str>`](https://github.com/rust-lang/rfcs/pull/1845).
* [disposition: close] [Add variable-length arrays to the language](https://github.com/rust-lang/rfcs/pull/1808).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).

## New RFCs

* [Intra Rustdoc Links](https://github.com/rust-lang/rfcs/pull/1946).
* [Polymorphic Numeric Constants](https://github.com/rust-lang/rfcs/pull/1945).
* [Improve match ergonomics](https://github.com/rust-lang/rfcs/pull/1944).
* [Guess diagnostics](https://github.com/rust-lang/rfcs/pull/1941). `rustfix` should be the semantic companion to `rustfmt`, which automatically changes code to be more idiomatic.
* [Support the `#[must_use]` attribute on arbitrary functions](https://github.com/rust-lang/rfcs/pull/1940).
* [Allow the `?` operator to be used in `main`, and in `#[test]` functions and doctests](https://github.com/rust-lang/rfcs/pull/1937).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

Issues in final comment period:

* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Other significant issues:

* [types](https://github.com/rust-lang-nursery/fmt-rfcs/issues/15)
* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [`extern` vs `extern "C"`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/52)
* [ranges](https://github.com/rust-lang-nursery/fmt-rfcs/issues/60)

# Upcoming Events

* [Mar  9. San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/237602716/).
* [Mar  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/237525355/).
* [Mar  9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 11. Rust NYC - Rust Hack & Learn](https://www.meetup.com/Rust-NYC/events/238057861/).
* [Mar 13. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/237058819/).
* [Mar 15. Rust Meetup Hamburg - Rust/Ethereum Meetup](https://www.meetup.com/Rust-Meetup-Hamburg/events/237858112/).
* [Mar 15. Rust Los Angeles - Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/237757181/).
* [Mar 15. Rust Dublin - Rust Lightning Talks](https://www.meetup.com/Rust-Dublin/events/237883717/).
* [Mar 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 16. Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/de-DE/Rust-Modern-Systems-Programming-in-Leipzig/events/237780401/).
* [Mar 16. Thompson Rivers University, BC Canada - Get Rusty](https://www.eventbrite.ca/e/get-rusty-tickets-31407199780).
* [Mar 21. Rust Paris Meetup #36](https://www.meetup.com/Rust-Paris/events/238240907/).
* [Mar 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 23. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I sure need to work on my quotes.

— [llogiq on his QotW from last week](https://www.reddit.com/r/rust/comments/5wt2vq/this_week_in_rust_171/deczcbo/).

Thanks to [slashgrin for the suggestion](https://www.reddit.com/r/rust/comments/5wt2vq/this_week_in_rust_171/ded0un1/).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
