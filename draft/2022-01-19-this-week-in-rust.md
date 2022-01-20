Title: This Week in Rust 426
Number: 426
Date: 2022-01-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

* [gtk-rs 0.15 / gtk4-rs 0.4 release](https://gtk-rs.org/blog/2022/01/16/new-release.html)
* [Announcing Relm4 0.4](https://aaronerhardt.github.io/blog/posts/announcing_relm4_v0.4/)
* [SixtyFPS (GUI crate): Changelog for 16th of January 2022](https://sixtyfps.io/thisweek/2022-01-17.html)
* [Quickwit 0.2 brings full-text search to ClickHouse and Kafka!](https://quickwit.io/blog/quickwit-0.2)
* [Introducing the new DNSimple Rust Client](https://blog.dnsimple.com/2022/01/new-rust-api-client-library/)
* [This week in Databend #25: an elastic and reliable cloud warehouse](https://weekly.databend.rs/2022-01-19-databend-weekly/)
* [Benchmarking relational data in BonsaiDb](https://bonsaidb.io/blog/commerce-benchmark/)

### Observations/Thoughts

### Research

### Rust Walkthroughs

* [How to securely encrypt a file with an insecure password in Rust (using Streaming Encryption + Argon2)](https://kerkour.com/rust-file-encryption-chacha20poly1305-argon2/)
* [video] [I'm learning Rust - Cargo, documentation, user input, and JSON files](https://www.youtube.com/watch?v=ruKV0WMSweA)
* [series] [video] [Writing a Programming Language (in Rust) 10: Implementing Objects](https://www.youtube.com/watch?v=67HtN0PHWUI)
* [series] [video] [Writing a Programming Language (in Rust) 11: Object Destructuring (Part 1)](https://www.youtube.com/watch?v=ay7gzTzMZKo)

### Miscellaneous
* [Creating social sharing images in Rust](https://hashrust.com/blog/creating-social-sharing-images-in-rust/)
* [Writing a Fluent Bit input plugin in Rust](https://fredrik-jansson-se.github.io/fluent-bit-input-rust.html)
* [The 10 books that helped me, as a hobbyist, on my journey to learn Rust to re-code a Django application](https://www.reddit.com/r/rust/comments/s3z7ek/the_10_books_that_helped_me_as_a_hobbyist_on_my/)
* [Rewriting my toy blockchain in Rust](https://ezzeriesa.notion.site/Rewriting-my-toy-blockchain-in-Rust-9a130f225666488491ba497004821fbb)
* [Rust on Apache NuttX OS](https://lupyuen.github.io/articles/rust2)

## Crate of the Week

This week's crate is [cargo-release](https://crates.io/crates/cargo-release), a cargo subcommand that makes every crate release a breeze.

Thanks to [dpc](https://users.rust-lang.org/t/crate-of-the-week/2704/1010) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

289 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-01-10..2022-01-17

* [remove deprecated LLVM-style inline assembly](https://github.com/rust-lang/rust/pull/92816)
* [optimize `impl_read_unsigned_leb128`](https://github.com/rust-lang/rust/pull/92604)
* [reduce use of LocalDefId <-> HirId maps](https://github.com/rust-lang/rust/pull/90146)
* [store a `Symbol` instead of an `Ident` in `VariantDef`/`FieldDef`](https://github.com/rust-lang/rust/pull/92533)
* [partially stabilize `maybe_uninit_extra`](https://github.com/rust-lang/rust/pull/92768)
* [add {`Add`, `Sub`, `Mul`, `Div`, `Rem`, `BitXor`, `BitOr`, `BitAnd`}{, `Assign`} to `Saturating`](https://github.com/rust-lang/rust/pull/92356)
* [futures: fix stacked borrows violations in waker_ref and FuturesUnordered](https://github.com/rust-lang/futures-rs/pull/2550)
* [rustdoc: avoid many `Symbol` to `String` conversions](https://github.com/rust-lang/rust/pull/91948)
* [clippy: new lint: `iter_overeager_cloned`](https://github.com/rust-lang/rust-clippy/pull/8203)
* [clippy: add `manual_bits` lint](https://github.com/rust-lang/rust-clippy/pull/8213)
* [clippy: add borrow_as_ptr lint](https://github.com/rust-lang/rust-clippy/pull/8210)
* [clippy: allow primitive types in `disallowed_methods`](https://github.com/rust-lang/rust-clippy/pull/8112)
* [clippy: apply `not_unsafe_ptr_arg_deref` to type aliases](https://github.com/rust-lang/rust-clippy/pull/8273)
* [clippy: don't lint `if_same_then_else` with `if let` conditions](https://github.com/rust-lang/rust-clippy/pull/8297)
* [clippy: downgrade `mutex_atomic` to nursery](https://github.com/rust-lang/rust-clippy/pull/8260)
* [clippy: erase late bound regions in `iter_not_returning_iterator`](https://github.com/rust-lang/rust-clippy/pull/8287)
* [clippy: fix `cmp_owned` suggestion flipping the comparison](https://github.com/rust-lang/rust-clippy/pull/8299)
* [clippy: fix `deref_addrof`](https://github.com/rust-lang/rust-clippy/pull/8268)
* [clippy: fix `implicit_clone` for `&&T`](https://github.com/rust-lang/rust-clippy/pull/8231)
* [clippy: fix `manual_memcpy`](https://github.com/rust-lang/rust-clippy/pull/8226)
* [clippy: fix cropped `or_fun_call` hint](https://github.com/rust-lang/rust-clippy/pull/8292)
* [clippy: handle implicit named arguments in `useless_format`](https://github.com/rust-lang/rust-clippy/pull/8295)
* [clippy: move `return_self_not_must_use` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/8302)
* [thorin: performance improvements](https://github.com/rust-lang/thorin/pull/14)

### Rust Compiler Performance Triage

A quiet week for regular rustc performance; incremental builds
(particularly ones with little recompilation to do) saw an average 1.5%
improvement. rustdoc also saw several notable optimizations land which improve
performance, particularly on larger benchmarks.

Triage done by **@simulacrum**.
Revision range: [72e74d..7bc7be](https://perf.rust-lang.org/?start=72e74d7b9cf1a7901650227e74650f1fcc797600&end=7bc7be860f99f4a40d45b0f74e2d01b02e072357&absolute=false&stat=instructions%3Au)

3 Regressions, 5 Improvements, 2 Mixed; 2 of them in rollups
30 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-01-18.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Scoped threads in the standard library, take 2](https://github.com/rust-lang/rfcs/pull/3151)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Stabilize arc_new_cyclic](https://github.com/rust-lang/rust/pull/90666)
* [disposition: merge] [Change location of where clause on GATs](https://github.com/rust-lang/rust/pull/90076)
* [disposition: merge] [rustdoc: "Namespace" user-written Markdown headings](https://github.com/rust-lang/rust/issues/91759)
* [disposition: merge] [Stabilize -Z print-link-args as --print link-args](https://github.com/rust-lang/rust/pull/91606)
* [disposition: merge] [Use verbatim paths for process::Command if necessary](https://github.com/rust-lang/rust/pull/92519)
* [disposition: merge] [impl Not for !](https://github.com/rust-lang/rust/pull/91122)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No new RFCs were opened this week.*

## Upcoming Events

Rusty Events between 1/19/2022 - 2/16/2022 🦀

### Online

* [January 19, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out night** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydccbzb)
* [January 20, 2022 | Cardiff, UK | **Rust Book Study Session - Functional Language Features & Cargo and Crates.io** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/283204522/)
* [January 25, 2022 | Dallas, TX, US | **Last Tuesday Meetup** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrydccbhc/)
* [January 25, 2022 | Los Gatos, CA, US | **Book #24 - Rust for Rustaceans - Chapter 3 - Designing Interfaces** | Los Gatos Reading Group](https://www.meetup.com/Los-Gatos-Rust-Reading-Group/events/283352417/)
* [January 27, 2022 | Charlottesville, VA, US | **Minimal Area Bananagrams: a Tale of Needless Optimization** | Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/events/283355090/)
* [January 27, 2021 | Linz, AT | **Rust Meetup Linz - 18th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/283116945/)
* [January 27, 2022 | Nürnberg, DE | **Rust Nürnberg online #9**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/283118050/)
* [January 27, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545254)
* [January 29, 2021 | London, UK | **Rust (Remote) Hack & Learn** | Rust London User Group](https://www.meetup.com/Rust-London-User-Group/events/283335221/)
* [February 1, 2021 | Berlin, DE | **Rust Hack and Learn** | OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/events/283338268/)
* [February 1, 2022 | Buffalo, NY, US | **First Tuesdays: Buffalo Rust User Group** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/283011769)
* [February 8, 2022 | Los Angeles, CA, US | **Rust LA (Topic TBD) [Virtual] Feb. 2022** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/283232930/)
* [February 8, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/283213217/)
* [February 9, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545292)
* [February 15, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/283351974/)
* [February 16, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out night** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/283260386/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Usually Rust figures out the [Sartre question](https://davedevine.wordpress.com/2011/01/20/the-sartre-joke/) by itself

– [kornel on rust-users](https://users.rust-lang.org/t/type-ascription/70214/4)

Thanks to [H2CO3](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1166) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
