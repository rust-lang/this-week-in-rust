Title: This Week in Rust 413
Number: 413
Date: 2021-10-20
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Foundation

* [Adios Pagers: New Developments on Crates.io](https://foundation.rust-lang.org/posts/2021-10-18-crates-io-oncall-ferrous-systems/)

### Project/Tooling Updates

* [This week in Fluvio #9: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0009/)
* [SixtyFPS (GUI crate): Changelog for 17th of October 2021](https://sixtyfps.io/thisweek/2021-10-18.html)
* [What's new in SeaORM 0.3.0](https://www.sea-ql.org/SeaORM/blog/2021-10-15-whats-new-in-0.3.0/)
* [Annoucing the ogma project v0.1: a shell-like scripting language for tabular data](https://users.rust-lang.org/t/ogma-project-0-1-release/65848)
* [Rust on Espressif chips - 18-10-2021](https://mabez.dev/blog/posts/esp-rust-18-10-2021/)
* [Rust Analyzer Changelog #99](https://rust-analyzer.github.io/thisweek/2021/10/18/changelog-99.html)
* [This week in Databend #12: an elastic and reliable cloud warehouse](https://datafuselabs.github.io/weekly/2021-10-20-databend-weekly/)

### Observations/Thoughts

* [How Rust developers are making the web safer](https://github.com/readme/featured/rust-programming)
* [Overhead of Returning Optional Values in Java and Rust](https://pkolaczk.github.io/overhead-of-optional/)
* [Designing an API Client in Rust: New Rspotify Version a Year Later](https://nullderef.com/blog/web-api-client/)
* [video] [[Rust Working Group] Rust for Visual Effects - Anders Langlands, Owen Nelson, Luke Titley](https://odysee.com/@Pipeliner:f/Rust-VFX:a?r=6Ac8ttKMEn1Airp7gL6QvZpi2tcV9DCX)

### Rust Walkthroughs

* [Structuring, testing and debugging procedural macro crates](https://ferrous-systems.com/blog/testing-proc-macros/)
* [Custom Logging in Rust using tracing and tracing-subscriber](https://burgers.io/custom-logging-in-rust-using-tracing)
* [WSL2 and Embedded Rust](https://pfesenmeier.github.io/wsl2-and-embedded-development/)
* [Rust Guide: Generics Part 2](https://jeffa.io/rust_guide_generics_demystified_part_2)
* [Making slow Rust code fast - performance tuning using Criterion.rs and flamegraphs](https://patrickfreed.github.io/rust/2021/10/15/making-slow-rust-code-fast.html)
* [Builder pattern in Rust](https://www.greyblake.com/blog/2021-10-19-builder-pattern-in-rust/)
* [Solana: How to send custom instructions via instruction data](https://dev.to/cogoo/solana-how-to-send-custom-instructions-via-instruction-data-4g9g)
* [Hexagonal architecture in Rust #7 - Long-lived repositories](https://alexis-lozano.com/hexagonal-architecture-in-rust-7/)
* [JA] [Rust 関数オーバーロード、引数の数が異なる場合](https://dev.to/fmtweisszwerg/rust-guan-shu-obarodo-yin-shu-noshu-gayi-naruchang-he-22fb)
* [video] [Getting started with Rust 🦀 2021: 7b. Building a GUI app in Rust [Part B]](https://www.youtube.com/watch?v=SvFPdgGwzTQ)
* [video] [Processing Shaders with Rust](https://vimeo.com/632377558)
* [video] ["Streaming video analysis in Rust using Pravega" by Tom Kaitchuck](https://www.youtube.com/watch?v=AI1M7Wr4_6w)
* [video] [Rust for Java Developers 3/3 - Understanding Ownership](https://www.youtube.com/watch?v=Vg1LGHuAPP8)

### Miscellaneous

* [Results from the OpenSUSE 2021 Rust Survey](https://fy.blackhats.net.au/blog/html/2021/10/08/results_from_the_opensuse_2021_rust_survey.html)
* [Academy Software Foundation Announces Formation of Rust Working Group, Initial Release of OpenEXR Rust Binding](https://www.aswf.io/news/academy-software-foundation-announces-formation-of-rust-working-group-initial-release-of-openexr-rust-binding/)
* [ROAPI: An API Server for Static Datasets](https://tech.marksblogg.com/roapi-rust-data-api.html)

## Crate of the Week

This week's crate is [serde\_with](https://docs.rs/serde_with), a crate of helper macros to ease implementing serde traits for your types.

Thanks to [piegames](https://users.rust-lang.org/t/crate-of-the-week/2704/971) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [Paccat is looking for contributors](https://users.rust-lang.org/t/twir-call-for-participation/4821/395)
* [ockam - Use Zeroize for temporary sensitive data](https://github.com/ockam-network/ockam/issues/2051)
* [ockam - Remove None errors from our error enums](https://github.com/ockam-network/ockam/issues/2055)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

353 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-10-04..2021-10-11

* [fix ICE with `let...else` and `ref mut`](https://github.com/rust-lang/rust/pull/89965)
* [fix an ICE with TAITs and Future](https://github.com/rust-lang/rust/pull/89946)
* [suggest  Box::pin` when `Pin::new` is used instead](https://github.com/rust-lang/rust/pull/89870)
* [fix incorrect `Box::pin` suggestion](https://github.com/rust-lang/rust/pull/89390)
* [nicer error message if the user attempts to do `let`...`else if`](https://github.com/rust-lang/rust/pull/89974)
* [index and hash HIR as part of lowering](https://github.com/rust-lang/rust/pull/89124)
* [polymorphization: shims and predicates](https://github.com/rust-lang/rust/pull/89514)
* [optimize `VecDeque::append`](https://github.com/rust-lang/rust/pull/88717)
* [speedup int `log10` branchless](https://github.com/rust-lang/rust/pull/88788)
* [stabilize `unreachable_unchecked` as `const fn`](https://github.com/rust-lang/rust/pull/89509)
* [use `BCryptGenRandom` instead of `RtlGenRandom` on Windows.](https://github.com/rust-lang/rust/pull/84096)
* [make `Option::as_mut` const](https://github.com/rust-lang/rust/pull/89953)
* [make `Result::as_mut` const](https://github.com/rust-lang/rust/pull/89977)
* [add `slice::swap_unchecked`](https://github.com/rust-lang/rust/pull/88540)
* [add `#[repr(i8)]` to `Ordering`](https://github.com/rust-lang/rust/pull/89507)
* [add `Poll::ready` and revert stabilization of `task::ready!`](https://github.com/rust-lang/rust/pull/89651)
* [avoid allocations and copying in `Vec::leak`](https://github.com/rust-lang/rust/pull/89337)
* [rustdoc: associated consts sidebar](https://github.com/rust-lang/rust/pull/89815)
* [rustfmt: stabilize `disable_all_formatting`](https://github.com/rust-lang/rustfmt/pull/5026)
* [clippy: fix false positive of `implicit_saturating_sub` with `else` clause](https://github.com/rust-lang/rust-clippy/pull/7832)
* [clippy: do not expand macros in `equatable_if_let` suggestion](https://github.com/rust-lang/rust-clippy/pull/7788)
* [clippy: `unnecessary_sort_by` checks if argument implements `Ord` trait](https://github.com/rust-lang/rust-clippy/pull/7824)
* [clippy: allow giving reasons for `disallowed_types`](https://github.com/rust-lang/rust-clippy/pull/7791)
* [clippy: implement `uninit_vec` lint](https://github.com/rust-lang/rust-clippy/pull/7682)
* [clippy: add `format_in_format_args` and `to_string_in_format_args` lints](https://github.com/rust-lang/rust-clippy/pull/7743)
* [clippy: add lint `transmute_num_to_bytes`](https://github.com/rust-lang/rust-clippy/pull/7805)
* [clippy: add `match_str_case_mismatch` lint](https://github.com/rust-lang/rust-clippy/pull/7806)

### Rust Compiler Performance Triage

A week where improvements outweigh regressions. The highlight of the week is the change to split out LLVM profile guided optimization (PGO) and using clang 13 to compile LLVM which led to improvements in many real world crates (e.g., cargo) in the range of 10%. Most regressions were limited and at most in the less than 1% range. We are seeing more performance changes in rollups which are supposed to be performance neutral. We'll have to decide how to best address this.

Triage done by **@rylev**.
Revision range: [9475e609..d45ed750](https://perf.rust-lang.org/?start=9475e609b8458fff9e444934a6017d2e590642cf&end=d45ed7502ad225739270a368528725930f54b7b6&absolute=false&stat=instructions%3Au)

3 Regressions, 4 Improvements, 2 Mixed; 2 of them in rollups;
34 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-10-19.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Multiple artifact deps on the same crate with different names, for different targets](https://github.com/rust-lang/rfcs/pull/3176)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize is_symlink() for Metadata and Path](https://github.com/rust-lang/rust/pull/89677)
* [disposition: merge] [Implement `Termination` for `Result<Infallible, E>`](https://github.com/rust-lang/rust/pull/88601)
* [disposition: close] [Port clippy lint `redundant_field_names` to compiler](https://github.com/rust-lang/rust/pull/87512)
* [disposition: merge] [Make two Paths unequal if they differ in trailing slash](https://github.com/rust-lang/rust/pull/87339)
* [disposition: merge] [Stabilize `File::options()`](https://github.com/rust-lang/rust/pull/85766)
* [disposition: merge] [Tracking Issue for relaxed struct unsizing rules](https://github.com/rust-lang/rust/issues/81793)
* [disposition: merge] [Tracking Issue for `option_result_unwrap_unchecked`](https://github.com/rust-lang/rust/issues/81383)
* [disposition: merge] [Tracking Issue for inherent_ascii_escape](https://github.com/rust-lang/rust/issues/77174)
* [disposition: merge] [Tracking Issue for `destructuring_assignment`](https://github.com/rust-lang/rust/issues/71126)
* [disposition: close] [Tracking issue for `slice_concat_ext` stabilization](https://github.com/rust-lang/rust/issues/27747)

### New RFCs

* [Thread local Cell methods.](https://github.com/rust-lang/rfcs/pull/3184)
* [RFC: Console Input Simplified](https://github.com/rust-lang/rfcs/pull/3183)

## Upcoming Events

### Online

* [October 20, 2021, Buffalo, NY, US - Buffalo Rust User Group, Alternate Day - Buffalo Rust](https://www.meetup.com/Buffalo-Rust-Meetup/events/281236385/)
* [October 20, 2021, Vancouver, BC, CA - WASM plugin for Istio - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccnbbc/)
* [October 22, 2021, Iran - The First Rust Iran online meetup - Rust Iran Meetup](https://rust-meetup.ir/)
* [October 26, 2021, Dublin, IE - Rust Dublin October Remote Meetup 🎃 - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/281406298)
* [October 26, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [October 26, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccnbjc/)
* [October 27, 2021, London, UK - Rust London Ockam Takeover - Rust London User Group](https://skillsmatter.com/meetups/13606-rust-london-october2021#community)
* [October 27, 2021, Phoenix, AZ - Desert Rust Halloween - Desert Rust](https://www.meetup.com/Desert-Rustaceans/events/281215858/)
* [October 28, 2021, Copenhagen, DK - Hack Night #22 - Copenhagen Rust Group](https://cph.rs/)
* [November 2, 2021, Buffalo, NY, US - First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/281558952/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Modeldrive**

* [Senior Rust Engineer (London / Remote)](https://www.modeldrive.com/jobs)

**Connected Cars**

* [Software Engineering Lead (Denmark)](https://connectedcars.io/jobs/embedded-software-engineer)

**Bytewax**

* [Senior Software Engineer (Remote)](https://bytewax.notion.site/Senior-Software-Engineer-9d83531eb0704afd8f323e4080e0d620)

**Timescale**

* [Senior Rust Engineer (Remote: UTC-8 to UTC-5)](https://boards.greenhouse.io/timescale/jobs/5542785002)

**Immunant**

* [Systems Programmer/Rustacean (Optionally Remote)](https://immunant.com/jobs/)

**Nexthink**

* [Site Reliability Engineer (Madrid, ES)](https://www.smartrecruiters.com/Nexthink/743999779427018)

**Kraken**

* [Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Senior Rust Engineer - Banking (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The biggest failure in Rust‘s communication strategy has been the inability to explain to non-experts that unsafe abstractions are the point, not a sign of failure.

– [withoutboats on twitter](https://mobile.twitter.com/withoutboats/status/1447512045558149122)

Thanks to [Alice Ryhl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1124) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/qcgk2e/this_week_in_rust_413/)</small>
