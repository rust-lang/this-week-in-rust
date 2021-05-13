Title: This Week in Rust 391
Number: 391
Date: 2021-05-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

# Crate of the Week

This week's crate is [tokio-console](https://github.com/tokio-rs/console), a "top"-like utility to view your tasks run.

Thanks to [Simon Farnsworth](https://users.rust-lang.org/t/crate-of-the-week/2704/910) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

324 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-03..2021-05-10

* [fix duplicate unknown lint errors](https://github.com/rust-lang/rust/pull/85053)
* [improve diagnostics for functions in `struct` definitions](https://github.com/rust-lang/rust/pull/76808)
* [ensure failing promoteds in const/static bodies are handled correctly](https://github.com/rust-lang/rust/pull/85112)
* [implement Native link modifiers](https://github.com/rust-lang/rust/pull/83507) (RFC [#2951](https://rust-lang.github.io/rfcs/2951-native-link-modifiers.html))
* [deduplicate native libs before they are passed to the linker](https://github.com/rust-lang/rust/pull/84794)
* [retain data in vectorized registers for longer](https://github.com/rust-lang/rust/pull/84915)
* [only compute Obligation `cache_key` once in register_obligation_at](https://github.com/rust-lang/rust/pull/84923)
* ["const" initialized thread locals in rustc](https://github.com/rust-lang/rust/pull/84833)
* [streamline `try_start` code](https://github.com/rust-lang/rust/pull/84806)
* [simplify `chdir` implementation and minimize unsafe block](https://github.com/rust-lang/rust/pull/84712)
* [optimize `BufWriter`](https://github.com/rust-lang/rust/pull/79930)
* [provide `io::Seek::rewind`](https://github.com/rust-lang/rust/pull/85146)
* [stablize {`HashMap`, `BTreeMap`}`::into_`{`keys`, `values`}](https://github.com/rust-lang/rust/pull/84328)
* [futures: add FuturesUnordered::into_iter, make iter_pin_ref public](https://github.com/rust-lang/futures-rs/pull/2423)
* [regex: fix compilation with pattern feature](https://github.com/rust-lang/regex/pull/772)
* [datafrog: speed up Relation::merge](https://github.com/rust-lang/datafrog/pull/29)
* [cargo: improve two error messages](https://github.com/rust-lang/cargo/pull/9472)
* [rustdoc: fix source code line number display and make it clickable again](https://github.com/rust-lang/rust/pull/85148)
* [compiletest: add `needs-unwind` and beginning of support for testing `panic=abort` std](https://github.com/rust-lang/rust/pull/84734)
* [clippy: fix stack overflow issue in `redundant_pattern_matching`](https://github.com/rust-lang/rust-clippy/pull/7170)
* [clippy: fix `eval_order_dependence` async false positive](https://github.com/rust-lang/rust-clippy/pull/7174)
* [clippy: fix `unused_unit` macro false positive](https://github.com/rust-lang/rust-clippy/pull/7167)
* [clippy: fix `needless_quesiton_mark` false positive](https://github.com/rust-lang/rust-clippy/pull/7165)
* [clippy: for `to_*` variant don't lint in trait impl taking `self` when non-`Copy` type](https://github.com/rust-lang/rust-clippy/pull/7182)
* [clippy: fix `unnecessary_filter_map` false positive](https://github.com/rust-lang/rust-clippy/pull/7175)
* [clippy needless_collect: Lint cases with type annotations for indirect usage and recognize `BinaryHeap`](https://github.com/rust-lang/rust-clippy/pull/7163)

## Rust Compiler Performance Triage

Not much change overall - both regressions and improvements were all minor, apart from the 2x compile-time improvement for libcore from PR [#83278](https://github.com/rust-lang/rust/issues/83278).

Triage done by **@pnkfelix**.
Revision range: [7a0f..382f](https://perf.rust-lang.org/?start=7a0f1781d04662041db5deaef89598a8edd53717&end=382f748f23979e37e3e012b090e5a0313463f182&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 0 Mixed
0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-05-11.md).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [A new prelude for the 2021 edition (trait-only edition)](https://github.com/rust-lang/rfcs/pull/3114)
* [disposition: postpone] [Allow Overloading || and &&](https://github.com/rust-lang/rfcs/pull/2722)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize const_fn_unsize](https://github.com/rust-lang/rust/pull/85078)
* [disposition: merge] [FCP poll for ErrorKind::OutOfMemory](https://github.com/rust-lang/rust/issues/84916)
* [disposition: merge] [impl FromStr for proc_macro::Literal](https://github.com/rust-lang/rust/pull/84717)
* [disposition: merge] [stabilize member constraints](https://github.com/rust-lang/rust/pull/84701)
* [disposition: merge] [Uplift the invalid_atomic_ordering lint from clippy to rustc](https://github.com/rust-lang/rust/pull/84039)
* [disposition: merge] [Stabilize "RangeFrom" patterns](https://github.com/rust-lang/rust/pull/83918)
* [disposition: merge] [Stabilize extended_key_value_attributes](https://github.com/rust-lang/rust/pull/83366)
* [disposition: merge] [Add functions `Duration::try_from_secs_{f32, f64}`](https://github.com/rust-lang/rust/pull/82179)

## New RFCs

* [Scrape code examples from examples/ directory for Rustdoc](https://github.com/rust-lang/rfcs/pull/3123)

# Upcoming Events

### Online
* [May 12, Online - Rust Meetup May 2021 - Rust Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSf_hz-ZDwYEhVmIH0uzJ0uH41aXWZ_zRDsI0XENpfkKHvh_Jg/viewform)
* [May 13, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/2021/05/13/rust-hack-and-learn.html)
* [May 15 - June 7, Online - Solana Season Hackathon - Registration open now](https://twitter.com/solana/status/1387411221717176323?s=20)
* [May 17, 2021, Cardiff, UK - Rust and Cpp Cardiff :: v2.0 - Rust and C++ Cardiff](https://secure.meetup.com/register/?referrer_n=event&referrer_i=278002832&ctx=ref)
* [May 18, 2021, Washington, DC, US - Mid-month Rustful: rust4ml - Rust DC](https://www.meetup.com/RustDC/events/ntvrgsycchbxb)
* [May 19, 2021, Vancouver, BC - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zppkjsycchbzb/)
* [May 20, 2021, Online - Go vs Rust | Round table discussion](https://rustlab.it/en/rust-vs-go/)
* [May 20, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrycchbhc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> You won’t appreciate Rust unless you spend few weeks building something in it. The initial steep learning curve could be frustrating or challenging depending on how you see it, but once past that it’s hard not to love it. It’s a toddler with superpowers after all 💗

– [Deepu K Sasidharan on their blog](https://deepu.tech/my-second-impression-of-rust)

Thanks to [Phlopsi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1047) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
