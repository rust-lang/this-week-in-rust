Title: This Week in Rust 384
Number: 384
Date: 2021-03-31
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official
* [Announcing Rust 1.51.0](https://blog.rust-lang.org/2021/03/25/Rust-1.51.0.html)
* [Foundation] [Introducing Mark Rousskov](https://foundation.rust-lang.org/posts/2021-03-25-introducing-mark-rousskov/)
* [Foundation] [Introducing Nell Shamrell-Harrington](https://foundation.rust-lang.org/posts/2021-03-25-introducing-nell-shamrell-harrington/)

### Project/Tooling Updates
* [rust-analyzer changelog #70](https://rust-analyzer.github.io/thisweek/2021/03/29/changelog-70.html)
* [IntelliJ Rust Changelog #144](https://intellij-rust.github.io/2021/03/29/changelog-144.html)
* [Knurling-rs Financial Update and Call for Funding](https://ferrous-systems.com/blog/knurling-financial-update/)
* [Ockam | End-to-end encrypted messaging and mutual authentication between distributed applications](https://github.com/ockam-network/ockam)
* [Announcing the Deno Company](https://deno.com/blog/the-deno-company)

### Observations/Thoughts
* [Using Rust to corrode insane Python run-times](https://www.vortexa.com/insight/using-rust-to-corrode-insane-python-run-times)
* [The current state of Rust web frameworks](https://blog.logrocket.com/the-current-state-of-rust-web-frameworks/)
* [GhostCell: Separating Permissions from Data in Rust](http://plv.mpi-sws.org/rustbelt/ghostcell/)
* [Using const generics in slipstream](https://vorner.github.io/2021/03/28/const-generic-slipstreem.html)
* [Rust iterators tips and tricks](https://robinmoussu.gitlab.io/blog/post/2021-03-25_rust_iterators_tips_and_tricks/)

### Rust Walkthroughs
* [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)
* [Embedding Rust Into Zephyr Firmware using C-bindgen](https://www.jaredwolff.com/embedding-rust-into-zephyr-using-cbindgen/)
* [Parser combinators in Rust](https://doma.dev/blog/parsing-stuff-in-rust/)
* [Grids in Rust, part 2: const generics](https://blog.adamchalmers.com/grids-2/)
* [Ergonomic error handling with Rust](https://dev.to/senyeezus/ergonomic-error-handling-with-rust-13bj)
* [Implementing SHA2 (256/512) algorithm with Rust const generics](https://dev.to/dandyvica/implementing-sha2-256-512-algorithm-with-rust-const-generics-5ap)
* [Analisando alocações de memória em Rust utilizando GNU Debugger](https://dev.to/ignaciojvig/analisando-alocacoes-de-memoria-em-rust-utilizando-gnu-debugger-34kb)
* [Testing an embedded application](https://ferrous-systems.com/blog/test-embedded-app/)
* [Using Const Generics To Model An Electronics Graph](https://mkhan45.github.io/2021/03/28/Using-const-generics-to-model-an-electronics-graph.html)
* [Rust's Module System Explained](https://aloso.github.io/2021/03/28/module-system.html)
* [series] [Working with the trust-dns-resolver crate](https://dev.to/basman/series/11934)
* [video] [Safer Rust: Program Verification with Creusot](https://youtu.be/BPt987BRdDw)

### Miscellaneous
* [Linus Torvalds weighs in on Rust language in the Linux kernel](https://arstechnica.com/gadgets/2021/03/linus-torvalds-weighs-in-on-rust-language-in-the-linux-kernel/)
* [Ownership Concept Diagram](https://www.reddit.com/r/rust/comments/mgh9n9/ownership_concept_diagram/)

# Crate of the Week

This week's crate is [tide-acme](https://github.com/http-rs/tide-acme), a crate for automatic HTTPS certificaion using Let's Encrypt for Tide.

Thanks to [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/894) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [AWS Rust lambdas and Bioinformatics](https://umccr.org/blog/aws-bioinformatics-rust/)
* [darpi-rs/darpi is looking for users and contributors](https://github.com/darpi-rs/darpi)
* [RoaringBitmap/roaring-rs seeks a review of this Pull Request](https://github.com/RoaringBitmap/roaring-rs/pull/92)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

327 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-03-22..2021-03-29

* [coverage bug fixes and optimization support](https://github.com/rust-lang/rust/pull/83307)
* [ban custom inner attributes in expressions and statements](https://github.com/rust-lang/rust/pull/83488)
* [`GenericParam` does not need to be a HIR owner](https://github.com/rust-lang/rust/pull/83424)
* [remove assignments to ZST places instead of marking ZST return place as unused](https://github.com/rust-lang/rust/pull/83177)
* [run analyses before thir-tree dumps](https://github.com/rust-lang/rust/pull/83050)
* [import small cold functions](https://github.com/rust-lang/rust/pull/82980)
* [implement `feature(const_generics_defaults)`](https://github.com/rust-lang/rust/pull/75384)
* [stabilize `debug_non_exhaustive`](https://github.com/rust-lang/rust/pull/83041)
* [simplify encoder and decoder](https://github.com/rust-lang/rust/pull/83273)
* [remove (lots of) dead code](https://github.com/rust-lang/rust/pull/83185)
* [use `TrustedRandomAccess` for in-place iterators where possible](https://github.com/rust-lang/rust/pull/79846)
* [instruct LLVM that `binary_search` returns a valid index](https://github.com/rust-lang/rust/pull/81354)
* [make `NonNull::as_ref` (and friends) return refs with unbound lifetimes](https://github.com/rust-lang/rust/pull/80771)
* [add function `core::iter::zip`](https://github.com/rust-lang/rust/pull/82917)
* [revert reverting of stabilizing `integer::BITS`](https://github.com/rust-lang/rust/pull/82565)
* [generalize and inline `slice::fill` specializations](https://github.com/rust-lang/rust/pull/83245)
* [add `Result::into_err` where the Ok variant is the never type](https://github.com/rust-lang/rust/pull/83421)
* [remove `Option::`{`unwrap_none`, `expect_none`}](https://github.com/rust-lang/rust/pull/83349)
* [futures: add `AsyncSeekExt::stream_position`](https://github.com/rust-lang/futures-rs/pull/2380)
* [cargo: default macOS targets to unpacked debuginfo](https://github.com/rust-lang/cargo/pull/9298)
* [rustdoc: sidebar trait items order](https://github.com/rust-lang/rust/pull/83051)
* [docs.rs: stop displaying and serving authorship information](https://github.com/rust-lang/docs.rs/pull/1322)

## Rust Compiler Performance Triage

An overall busy but decent week for performance. While there were some performance regressions they were mostly small, and they were outnumbered by performance gains. Perhaps the most interesting news is not a compiler performance improvement but rather the introduction of no-alias optimizations at the LLVM level. This slightly hurts optimized build time performance in some cases, but it should make some workloads run faster after compilation.

Triage done by **@rylev**.
Revision range: [f24ce9b0..9b6339e4](https://perf.rust-lang.org/?start=f24ce9b0140d9be5a336954e878d0c1522966bb8&end=9b6339e4b9747d473270baa42e77e1d2fff39bf4&absolute=false&stat=instructions%3Au)

2 Regressions, 5 Improvements, 3 Mixed

1 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Declarative macro metavariable expressions](https://github.com/rust-lang/rfcs/pull/3086)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Raw Keywords](https://github.com/rust-lang/rfcs/pull/3098)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add strong_count mutation methods to Rc](https://github.com/rust-lang/rust/pull/83476)
* [disposition: merge] [Turn old edition lint (anonymous-parameters) into warn-by-default on 2015](https://github.com/rust-lang/rust/pull/82918)
* [disposition: merge] [reduce threads spawned by ui-tests](https://github.com/rust-lang/rust/pull/81942)
* [disposition: merge] [Stabilize `peekable_peek_mut`](https://github.com/rust-lang/rust/pull/81938)
* [disposition: merge] [Stabilize `rustdoc::bare_urls` lint](https://github.com/rust-lang/rust/pull/81764)
* [disposition: merge] [Adding diesel to the cargotest suite](https://github.com/rust-lang/rust/pull/81507)
* [disposition: merge] [Stabilize `cmp_min_max_by`](https://github.com/rust-lang/rust/pull/81047)
* [disposition: merge] [Allow qualified paths in struct construction (both expressions and patterns)](https://github.com/rust-lang/rust/pull/80080)
* [disposition: merge] [Tracking issue for RFC 2457, "Allow non-ASCII identifiers"](https://github.com/rust-lang/rust/issues/55467)

## New RFCs

* [RFC: Add a standard trait for getting many &mut to places](https://github.com/rust-lang/rfcs/pull/3100)
* [RFC: `cargo`-`miri` integration](https://github.com/rust-lang/rfcs/pull/3099)
* [Raw Keywords](https://github.com/rust-lang/rfcs/pull/3098)
* [rustdoc URL conflict resolution](https://github.com/rust-lang/rfcs/pull/3097)

# Upcoming Events

### Online
* [April 1, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccgbcb/)
* [April 6, Buffalo, NY, US - Buffalo Rust User Group - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/276717867/)
* [April 7, Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/277133126/)
* [April 7, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/jhfstryccgbkb/)
* [April 13, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccgbrb/)

### North America

* [April 8, Columbus, OH, US - Monthly Meetup - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccgblb/)
* [April 14, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccgbsb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**e.ventures**

* [Rust backend engineer (Remote, the Americas)](https://old.reddit.com/r/rust/comments/mfstaz/official_rrust_whos_hiring_thread_for_jobseekers/gspq9v1/)

**Wallaroo**

* [Software Engineer (Remote)](https://wallaroo.breezy.hr/p/30939dc4e5c7-software-engineer)

**Ockam**

* [Hiring for several Rust positions](https://www.ockam.io/team#open-roles)


*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Despite all the negative aspects, I must say that I do generally really like the poll-based approach that Rust is taking. Most of the problems encountered are encountered not because of mistakes, but because no other language really has pushed this principle this far. Programming language design is first and foremost an “artistic” activity, not a technical one, and anticipating the consequences of design choices is almost impossible.

– [tomaka on medium](https://tomaka.medium.com/a-look-back-at-asynchronous-rust-d54d63934a1c)

Thanks to [Michael Howell](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1028) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/mhkhmw/this_week_in_rust_384/)</small>
