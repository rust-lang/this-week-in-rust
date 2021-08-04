Title: This Week in Rust 402
Number: 402
Date: 2021-08-04
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
* [Announcing Rust 1.54.0](https://blog.rust-lang.org/2021/07/29/Rust-1.54.0.html)
* [The push for GATs stabilization](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html)

### Project/Tooling Updates
* [ConnectorX: accelerate data loading from databases to dataframes](https://github.com/sfu-db/connector-x)
* [This week in SixtyFPS (a Rust GUI toolkit)](https://sixtyfps.io/thisweek/2021-08-02.html)

### Observations/Thoughts

* [Deep Learning in Rust with GPU using onnxruntime-rs](https://able.bio/haixuanTao/deep-learning-in-rust-with-gpu--26c53a7f)

### Rust Walkthroughs
* [Implementing Base64 from scratch with Rust](https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1)
* [How I built End-to-End Encrypted Messaging in 51 lines of Rust, using the Ockam crate](https://github.com/ockam-network/ockam/tree/develop/documentation/use-cases/end-to-end-encryption-with-rust#readme)

### Research

### Miscellaneous

* [Implementing a BLT parser by hand in Rust (vs pest and combine): OpenTally dev log](https://yingtongli.me/blog/2021/07/30/blt-parser.html)
* [Rust on RISC-V BL602: Is It Sunny?](https://lupyuen.github.io/articles/adc)
* [Rust Design FAQ for C++ Programmers](https://cppfaq.rs)

## Crate of the Week

This week's crate is [sycamore](https://crates.io/crates/sycamore), a crate for making web applications using WebAssembly..

Thanks to [Luke Chu](https://users.rust-lang.org/t/crate-of-the-week/2704/941) for the self-suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Module of the Week

Launching this week is [Rust Module of the Week](https://motw.rs) with [std::fs Part 1](https://motw.rs/blog/2021/08/01/stdfs-part-1/). Contribution and feedback welcome [here](https://github.com/slyons/rust-module-of-the-week).

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [vandenheuvel/relp - A faster implementation of Markowitz' pivot rule in LU factorization](https://github.com/vandenheuvel/relp/issues/15)
* [vandenheuvel/relp - Parsing the LP file format](https://github.com/vandenheuvel/relp/issues/30)
* [ockam-network/ockam - Make Rust Errors more idiomatic](https://github.com/ockam-network/ockam/issues/1655)
* [ockam-network/ockam - File Transfer over end-to-end encrypted secure channels](https://github.com/ockam-network/ockam/issues/1624)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

324 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-07-26..2021-08-02

* [`#[derive(Default)]` on enums with a `#[default]` attribute](https://github.com/rust-lang/rust/pull/86735) (RFC [#3107](https://rust-lang.github.io/rfcs/3107-derive-enum-default.html))
* [fix issue with autofix for ambiguous associated function from Rust 2021 prelude when struct is generic](https://github.com/rust-lang/rust/pull/87557)
* [add flag to configure `large_assignments` lint](https://github.com/rust-lang/rust/pull/86450)
* [make const `panic!("..")` work in Rust 2021](https://github.com/rust-lang/rust/pull/86998)
* [suggest removing unnecessary `&mut` as help message](https://github.com/rust-lang/rust/pull/87453)
* [suggest `;` on parse error where applicable](https://github.com/rust-lang/rust/pull/87436)
* [fix invalid suggestions for non-ASCII characters in byte constants](https://github.com/rust-lang/rust/pull/87659)
* [tweak opaque type mismatch error](https://github.com/rust-lang/rust/pull/87673)
* [bail on any found recursion when expanding opaque types](https://github.com/rust-lang/rust/pull/87546)
* [polonius: compute subset errors everywhere](https://github.com/rust-lang/polonius/pull/156)
* [MIR borrowck does not generate lifetime variables for `'static` lifetimes during opaque type resolution](https://github.com/rust-lang/rust/pull/87483)
* [tweak borrowing suggestion in `for` loop](https://github.com/rust-lang/rust/pull/87559)
* [remove unsound `TrustedRandomAccess` implementations](https://github.com/rust-lang/rust/pull/85874)
* [BTree: lazily locate leaves in rangeless iterators](https://github.com/rust-lang/rust/pull/86031)
* [partially stabilize `const_slice_first_last`](https://github.com/rust-lang/rust/pull/86593)
* [stabilize `core::task::ready!`](https://github.com/rust-lang/rust/pull/81050)
* [stabilize `const_fn_transmute`, `const_fn_union`](https://github.com/rust-lang/rust/pull/85769)
* [implement `fold()` on `array::IntoIter` to improve `flatten().collect()` perf](https://github.com/rust-lang/rust/pull/87431)
* [optimize `fmt::PadAdapter::wrap`](https://github.com/rust-lang/rust/pull/87052)
* [remove `P: Unpin` bound on `impl Future for Pin`](https://github.com/rust-lang/rust/pull/81363)
* [futures: use `futuresordered` in `join_all`](https://github.com/rust-lang/futures-rs/pull/2412)
* [clippy: cover `Result` on `map_flatten` lint](https://github.com/rust-lang/rust-clippy/pull/7522)
* [clippy: fix `while_let_on_iterator`](https://github.com/rust-lang/rust-clippy/pull/7520)

### Rust Compiler Performance Triage

Quiet week for performance, with no large changes. Regressions are limited to just a few benchmarks.

Triage done by **@simulacrum**.
Revision range: [998cfe5..3354a44](https://perf.rust-lang.org/?start=998cfe5aad7c21eb19a4bca50f05a13354706970&end=3354a44d2fa8d5ba6b8d6b40d2596de2c8292ec1&absolute=false&stat=instructions%3Au)

2 Regressions, 0 Improvements, 0 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-08-03.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Stabilize Cargo's weak-dep-features and namespaced-features.](https://github.com/rust-lang/rfcs/pull/3143)
* [RFC: Introduce concat_bytes!() to join [u8] and byte str analogous to concat! for str](https://github.com/rust-lang/rfcs/pull/2509)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `Vec<T>::shrink_to`](https://github.com/rust-lang/rust/pull/86879)
* [disposition: merge] [impl Default, Copy, Clone for std::io::Sink and Empty](https://github.com/rust-lang/rust/pull/86744)
* [disposition: merge] [impl Pattern for char array](https://github.com/rust-lang/rust/pull/86336)
* [disposiiton: merge] [Weaken guarantee around advancing underlying iterators in zip](https://github.com/rust-lang/rust/pull/83791)

### New RFCs

* [RFC: Backtrace in core](https://github.com/rust-lang/rfcs/pull/3156)

## Upcoming Events

### Online

* [August 5, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [August 9, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycclbnb/)
* [August 10, 2021, Dublin, IE - Rust Dublin August Remote Meetup - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/279788945)
* [August 18, 2021, Denver, CO, US - Level up our Rust skills by building an ECS by Brooks Patton - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/278909353/)
* [August 18, 2021, Vancouver, BC, CA - Solving LeetCode Problems with Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsycclbxb/)
* [August 19, 2021, Manchester, UK - Rust Manchester - Speeding Up the Snake: Extending Python with Rust](https://www.meetup.com/rust-manchester/events/279730616/)

### North America

* [August 11, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsycclbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

**Parity Technologies**

* [Multiple Rust engineering positions available](https://www.parity.io/jobs/)

**Travala**

* [Senior Blockchain Engineer](https://arbeitnow.com/view/senior-blockchain-engineer-travala-359033)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)

# Quote of the Week

Sadly, this week saw no quote of the week nominations.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
