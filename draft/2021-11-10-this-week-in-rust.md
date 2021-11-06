Title: This Week in Rust 416
Number: 416
Date: 2021-11-10
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

### Project/Tooling Updates

### Research Papers

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

* [What does `&mut &[T]` mean?](https://ihatereality.space/04-what-mutref-to-slice-ref-means/)

## Crate of the Week

This week's crate is [roogle](https://github.com/hkmatsumoto/roogle), a type-based Rust API search engine inspired by Haskell's Hoogle.

Thanks to [Hirochika Matsumoto](https://users.rust-lang.org/t/crate-of-the-week/2704/978) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

316 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-10-25..2021-11-01

* [fix CVE-2021-42574](https://github.com/rust-lang/rust/pull/90462)
* [add LLVM Control Flow Integrity support to the Rust compiler](https://github.com/rust-lang/rust/pull/89652)
* [add `-Z no-unique-section-names` to reduce ELF header bloat](https://github.com/rust-lang/rust/pull/89581)
* [fix: inner attribute followed by outer attribute causing ICE](https://github.com/rust-lang/rust/pull/90267)
* [skipping verbose diagnostic suggestions when calling `.as_ref()` on type not implementing `AsRef`](https://github.com/rust-lang/rust/pull/90399)
* [improve and test cross-crate hygiene](https://github.com/rust-lang/rust/pull/90202)
* [use `SortedMap` in HIR](https://github.com/rust-lang/rust/pull/90145)
* [stabilize `is_symlink()` for `Metadata` and `Path`](https://github.com/rust-lang/rust/pull/89677)
* [stabilize `option_result_unwrap_unchecked`](https://github.com/rust-lang/rust/pull/89951)
* [impl `Pattern` for `char` array](https://github.com/rust-lang/rust/pull/86336)
* [make most `std::ops` traits `const` on numeric types](https://github.com/rust-lang/rust/pull/89876)
* [codegen\_gcc: remove unused dependency on object](https://github.com/rust-lang/rustc_codegen_gcc/pull/102)
* [codegen\_gcc: fix negation operation](https://github.com/rust-lang/rustc_codegen_gcc/pull/108)
* [bindgen: don't generate 2^64 byte padding fields on unions](https://github.com/rust-lang/rust-bindgen/pull/2108)
* [bindgen: avoid case of a self-referential type alias](https://github.com/rust-lang/rust-bindgen/pull/2109)
* [cargo: change `--scrape-examples` flag to `-Z rustdoc-scrape-examples`](https://github.com/rust-lang/cargo/pull/10017)
* [cargo: scrape code examples from `examples/` directory for rustdoc](https://github.com/rust-lang/cargo/pull/9525)
* [rustdoc: fix generics generation in search index](https://github.com/rust-lang/rust/pull/88268)
* [rustdoc: use better highlighting for `*const`, `*mut`, and `&mut`](https://github.com/rust-lang/rust/pull/90278)
* [rustdoc: remove flicker during page load](https://github.com/rust-lang/rust/pull/90333)
* [clippy: move `if_then_panic` to pedantic and rename to `manual_assert`](https://github.com/rust-lang/rust-clippy/pull/7810)
* [clippy: fix false positive in `match_overlapping_arm`](https://github.com/rust-lang/rust-clippy/pull/7847)
* [clippy: fix `question_mark` false positive on custom error type](https://github.com/rust-lang/rust-clippy/pull/7860)
* [clippy: add `unit-hash  lint](https://github.com/rust-lang/rust-clippy/pull/7875)
* [clippy: new lint: `string-slice`](https://github.com/rust-lang/rust-clippy/pull/7878)
* [clippy: ignore references to type aliases in `ptr_arg`](https://github.com/rust-lang/rust-clippy/pull/7890)
* [clippy: fix ICE in `undocumented_unsafe_blocks`](https://github.com/rust-lang/rust-clippy/pull/7891)
* [clippy: disable `if_not_else` lints from firing on `else`-`if`s](https://github.com/rust-lang/rust-clippy/pull/7895)
* [rustfmt: prevent trailing whitespace in where clause bound predicate](https://github.com/rust-lang/rustfmt/pull/5019)
* [rustfmt: retain trailing comments in module when using `rustfmt::skip` attribute](https://github.com/rust-lang/rustfmt/pull/5035)

### Rust Compiler Performance Triage

The only significant regressions were 1. two PRs that slowed down doc
generation, and 2. some slowdown from the new lints to flag occurrences of
Unicode bidirectional control characters. The doc generation regression is being
investigated.

Triage done by **@pnkfelix**.
Revision range: [3c8f00..6384dc](https://perf.rust-lang.org/?start=3c8f001d454b1b495f7472d8430ef8fdf10aac11&end=6384dca100f3cedfa031a9204586f94f8612eae5&absolute=false&stat=instructions%3Au)

6 Regressions, 3 Improvements, 1 Mixed; 4 of them in rollups
39 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-11-02.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Multiple artifact deps on the same crate with different names, for different targets](https://github.com/rust-lang/rfcs/pull/3176)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Static async fn in traits](https://github.com/rust-lang/rfcs/pull/3185)
* [disposition: merge] [Constrained Naked Functions](https://github.com/rust-lang/rfcs/pull/2972)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize format args capture](https://github.com/rust-lang/rust/pull/90473)
* [disposition: merge] [Stabilize -Z symbol-mangling-version=v0 as -C symbol-mangling-version=v0](https://github.com/rust-lang/rust/pull/90128)
* [disposition: merge] [Stabilize -Z strip as -C strip](https://github.com/rust-lang/rust/pull/90058)
* [disposition: merge] [Stabilize `const_raw_ptr_deref` for `*const T`](https://github.com/rust-lang/rust/pull/89551)
* [disposition: merge] [Clarification of default socket flags](https://github.com/rust-lang/rust/pull/88805)
* [disposition: merge] [use CLOCK_BOOTTIME in `Instant::now`](https://github.com/rust-lang/rust/pull/88714)
* [disposition: merge] [GATs: Decide whether to have defaults for `where Self: 'a`](https://github.com/rust-lang/rust/issues/87479)

### New RFCs

* [take on bool](https://github.com/rust-lang/rfcs/pull/3189)
* [New Cargo and Rust options to support embedding Natvis into a PDB](https://github.com/rust-lang/rfcs/pull/3191)

## Upcoming Events

### Online

* [November 3, 2021, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/281258179)
* [November 9, 2021, San Diego, CA, US - San Diego Rust November 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/281801412)
* [November 9, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [November 9, 2021, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccpbmb/)
* [November 10, 2021, Malaysia - Rust Meetup - Rust Malaysia](https://discord.gg/9Xj8H2EXTD)
* [November 11, 2021 - Rust For Linux: Writing Safe Abstractions & Drivers - The Linux Foundation](https://linuxfoundation.org/webinars/rust-for-linux-writing-abstractions-and-drivers/)
* [November 17, 2021, Vancouver, BC, CA - Borrowing and Lifetimes through Metaphors - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccpbwb/)

### North America

* [November 10, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccpbnb/)
* [November 10, 2021, Mesa, AZ, US - Booze.rs - Desert Rust](https://www.meetup.com/Desert-Rustaceans/events/281729697)

### Europe

* [November 11, 2021, Belgrade, RS - First! - Belgrade Rust Meetup Group](https://www.meetup.com/belgrade-rust-meetup-group/events/281523208/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I always tell myself that code quickly written just to compile looks like Order 66 executed on Christmas day 
>
> [...]
>
> Clones and unwrapping as far as the eye can see.

– [Dhghomon on /r/rust](https://www.reddit.com/r/rust/comments/qjgwhr/whats_your_vote_for_funniest_feature_of_rust/hiq37zq)

Thanks to [UtherII](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1129) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
