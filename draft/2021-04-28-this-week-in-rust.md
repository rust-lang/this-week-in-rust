Title: This Week in Rust 388
Number: 388
Date: 2021-04-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No papers/research projects this week.

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

# Crate of the Week

This week's crate is [deltoid](https://github.com/jjpe/deltoid), another crate for delta-compressing Rust data structures.

Thanks to [Joey Ezechiëls](https://users.rust-lang.org/t/crate-of-the-week/2704/904) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No calls for participation this week*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

292 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-04-12..2021-04-19

* [detect when suggested paths enter extern crates more rigorously](https://github.com/rust-lang/rust/pull/84113)
* [don't set fast-math for the SIMD operations we set it for previously](https://github.com/rust-lang/rust/pull/84274)
* [add lint `deref_nullptr` detecting when a null ptr is dereferenced](https://github.com/rust-lang/rust/pull/83948)
* [fix suggestion for unsized function parameters](https://github.com/rust-lang/rust/pull/84313)
* [suggest to borrow after failing to cast from `T` to `*const/mut T`](https://github.com/rust-lang/rust/pull/84228)
* [stabilize `non-ascii-idents`](https://github.com/rust-lang/rust/pull/83799)
* [stabilize `is_subnormal`](https://github.com/rust-lang/rust/pull/84086)
* [stabilize `duration_zero`](https://github.com/rust-lang/rust/pull/84084)
* [stabilize `nonzero_leading_trailing_zeros`](https://github.com/rust-lang/rust/pull/84082)
* [stabilize `bufreader_seek_relative`](https://github.com/rust-lang/rust/pull/82992)
* [stabilize `BTree`{`Map`, `Set`}`::retain`](https://github.com/rust-lang/rust/pull/84121)
* [fix aliasing violations in `thread_local_const_init`](https://github.com/rust-lang/rust/pull/84291)
* [fix `join_paths` error display](https://github.com/rust-lang/rust/pull/84177)
* [merge same condition branch in vec `spec_extend`](https://github.com/rust-lang/rust/pull/84209)
* [improve `vecdeque_binary_search`](https://github.com/rust-lang/rust/pull/84145/files)
* [regex: shrink size of `Inst`](https://github.com/rust-lang/regex/pull/760)
* [cargo: don't re-use rustc cache when `RUSTC_WRAPPER` changes](https://github.com/rust-lang/cargo/pull/9348)
* [clippy: split `is_diagnostic_assoc_item`](https://github.com/rust-lang/rust-clippy/pull/7074)
* [clippy: fix `single_match`](https://github.com/rust-lang/rust-clippy/pull/7093)
* [clippy: fix a false negative on `needless_return`](https://github.com/rust-lang/rust-clippy/pull/7067)
* [clippy: fix a false positive in `missing_const_for_fn`](https://github.com/rust-lang/rust-clippy/pull/7076)
* [clippy: fix false positive in `wrong_self_convention` lint](https://github.com/rust-lang/rust-clippy/pull/7064)
* [clippy: fix `redundant_pattern_matching` drop order](https://github.com/rust-lang/rust-clippy/pull/6568)
* [clippy: un-double `return` on `try_err`](https://github.com/rust-lang/rust-clippy/pull/7108)

## Rust Compiler Performance Triage

Another quiet week with very small changes to compiler performance.

Triage done by **@rylev**.
Revision range: [5258a74..6df26f](https://perf.rust-lang.org/?start=5258a74c887f8ae14717e1f98b652b470877ce4e&end=6df26f897cffb2d86880544bb451c6b5f8509b2d&absolute=false&stat=instructions%3Au)

1 Regressions, 0 Improvements, 1 Mixed

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [try_trait_v2: A new design for the ? desugaring](https://github.com/rust-lang/rfcs/pull/3058)


## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)
* [disposition: merge] [add const-ub RFC](https://github.com/rust-lang/rfcs/pull/3016)
* [disposition: merge] [Target tier policy](https://github.com/rust-lang/rfcs/pull/2803)
* [disposition: postpone] [RFC: Custom DSTs](https://github.com/rust-lang/rfcs/pull/2594)
* [disposition: postpone] [Enum variant types](https://github.com/rust-lang/rfcs/pull/2593)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Cautiously add IntoIterator for arrays by value](https://github.com/rust-lang/rust/pull/84147)
* [disposition: merge] [Stabilize Duration::MAX](https://github.com/rust-lang/rust/pull/84120)
* [disposition: merge] [Stabilize `impl From<[(K, V); N]`> for HashMap`](https://github.com/rust-lang/rust/pull/84111)
* [disposition: merge] [Allow setting `target_family` to multiple values, and implement `target_family="wasm"`](https://github.com/rust-lang/rust/pull/84072)
* [disposition: close] [Exiting a process calls exit() which isn’t thread-safe](https://github.com/rust-lang/rust/issues/83994)
* [disposition: merge] [Stabilize `:pat_param` but leave :pat2021 gated](https://github.com/rust-lang/rust/pull/83386)
* [disposition: merge] [parser: Remove support for inner attributes on non-block expressions](https://github.com/rust-lang/rust/pull/83312)
* [disposition: merge] [Update BARE_TRAIT_OBJECT and ELLIPSIS_INCLUSIVE_RANGE_PATTERNS to errors in Rust 2021](https://github.com/rust-lang/rust/pull/83213)
* [disposition: merge] [Tracking Issue for vec_extend_from_within](https://github.com/rust-lang/rust/issues/81656)
* [disposition: merge] [Tracking Issue for 'ordering helpers'](https://github.com/rust-lang/rust/issues/79885)
* [disposition: merge] [Tracking issue for array::from_ref and array::from_mut](https://github.com/rust-lang/rust/issues/77101)
* [disposition: merge] [Tracking Issue for {HashMap,BTreeMap}::into_{keys,values}](https://github.com/rust-lang/rust/issues/75294)
* [disposition: merge] [Tracking issue for x86 bittest intrinsics](https://github.com/rust-lang/rust/issues/59414)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [April 21, Vancouver, BC, CA - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsyccgbcc/)
* [April 27, Berlin, DE - Rust and Tell - Rust Berlin](https://www.meetup.com/Rust-Berlin/events/277590271)
* [April 27, London, UK - LDN Virtual Talks Apr 2021 - Red Badger Takeover - Rust London User Group](https://www.meetup.com/Rust-London-User-Group/events/277520645/)
* [April 27, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccgbkc/)
* [April 28, Online - Ockam Open Source Community Call - Live coding walkthrough of building end-to-end encrypted communication in Rust](https://github.com/ockam-network/ockam/discussions/1303)
* [May 4, Buffalo, NY, US - Buffalo Rust User Group, Tues May 4th - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/277402612/)

### Europe
* [April 21, Moscow, RU - Monthly Meetup - Rust Moscow](https://www.meetup.com/ru-RU/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/277259838/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> We feel that Rust is now ready to join C as a practical language for implementing the \[Linux\] kernel. It can help us reduce the number of potential bugs and security vulnerabilities in privileged code while playing nicely with the core kernel and preserving its performance characteristics.

– [Wedson Almeida Filho on the Google Security Blog](https://security.googleblog.com/2021/04/rust-in-linux-kernel.html)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1040) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
