Title: This Week in Rust 373
Number: 373
Date: 2021-01-13
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
* [This Month in Rust GameDev #17 - December 2020](https://rust-gamedev.github.io/posts/newsletter-017/)
* [This Month in Rust OSDev (December 2020)](https://rust-osdev.com/this-month/2020-12/)

### Project/Tooling Updates
* [rust-analyzer Changelog #59](https://rust-analyzer.github.io/thisweek/2021/01/11/changelog-59.html)
* [Knurling-rs Changelog #12](https://ferrous-systems.com/blog/knurling-changelog-12/)
* [GCC Rust – How it can be achieved](https://www.embecosm.com/2021/01/12/gcc-rust-how-it-can-be-achieved/)
* [Open Source Security, Inc. Announces Funding of GCC Front-End for Rust](https://opensrcsec.com/open_source_security_announces_rust_gcc_funding)
* [Progress report on rustc_codegen_cranelift (Dec 2020)](https://bjorn3.github.io/2021/01/07/progress-report-dec-2020.html)

### Observations/Thoughts

* [XML parsing in Rust](https://simplabs.com/blog/2020/12/31/xml-and-rust/)
* [parsing baseball files in Rust instead of Python for an 8x speedup!](https://gregstoll.wordpress.com/2021/01/10/parsing-baseball-files-in-rust-instead-of-python-for-an-8x-speedup/)
* [Philosophies of Rust and Haskell](https://www.fpcomplete.com/blog/philosophies-rust-haskell/)
* [Maybe you don't need Rust and WASM to speed up your JS](https://mrale.ph/blog/2018/02/03/maybe-you-dont-need-rust-to-speed-up-your-js.html)
* [Simplifying Endian-Specific file parsers in Rust with Omnom and type inference](https://www.parsed.uk/articles/Simplifying_EndianSpecific_file_parsers_in_Rust_with_Omnom_and_type_inference)
* [Rust's SemVer Snares: repr(transparent) Super-Cut](https://jack.wrenn.fyi/blog/semver-snares-transparent/)
* [Exploiting Mixed Binaries](https://www.cs.ucy.ac.cy/~eliasathan/papers/tops20.pdf)
* [audio] [The Virtual World: Embedded Rust part 2 - James Munns](https://anchor.fm/the-virtual-world/episodes/Embedded-Rust-part-2---James-Munns-eooog5)

### Rust Walkthroughs

* [Setting a Rust Executable's Icon in Windows](https://www.anthropicstudios.com/2021/01/05/setting-a-rust-windows-exe-icon/)
* [The Little Book of Rust Books](https://lborb.github.io/book/title-page.html)
* [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
* [Hecto: Build your own text editor in Rust](https://www.philippflenker.com/hecto/)
* [CLI Shell Completions in Rust](https://dev.to/kbknapp/cli-shell-completions-in-rust-37g1)
* [Dynamically Aligned Types?](https://dev.to/cad97/dynamically-aligned-types-360i)
* [Polishing Rust - Temporary Value Dropped While Borrowed](https://mrtact.medium.com/polishing-rust-42d9131d1c3)
* [Testing Hardware Using Rust Firmware and Rust Based CLI](https://www.jaredwolff.com/testing-hardware-using-rust/)
* [Exposing a Rust Library to Other Languages](https://sixtyfps.io/blog/expose-rust-library-to-other-languages.html)
* [Replacing FastAPI with Rust: Part 3 - Trying Actix](https://dev.to/dbanty/replacing-fastapi-with-rust-part-3-trying-actix-32lp)
* [video] [Learning Rust by following the "Too Many Linked Lists" tutorial (Episode 1)](https://youtu.be/aNv_oqveuCE)
* [video] [Rust Parser and State Machine](https://youtu.be/2aI6bJk76xM)

### Miscellaneous
* [Rust Language Cheat Sheet](https://cheats.rs/)
* [Another Rust-y OS: Theseus joins Redox in pursuit of safer, more resilient systems](https://www.theregister.com/2021/01/14/rust_os_theseus/)
* [video] [OSDI '20 - Redleaf: Isolation and Communication in a Safe Operating System](https://youtu.be/MKjliJWzs6w)
* [video] [Read a paper: Theseus--An OS written in Rust](https://youtu.be/NzmbDU13Ki8)

# Crate of the Week

This week's crate is [fast-float](https://github.com/aldanor/fast-float-rust), a crate providing methods to parse floats *really* fast.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/868) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ZcashFoundation/zebra - Create test to catch duplicate dependencies](https://github.com/ZcashFoundation/zebra/issues/1582)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

320 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-01-04..2021-01-11

* [use correct span for structured suggestion](https://github.com/rust-lang/rust/pull/80801)
* [rustc_parse: better spans for synthesized token streams](https://github.com/rust-lang/rust/pull/80784)
* [ast: remove some indirection layers from values in key-value attributes](https://github.com/rust-lang/rust/pull/80441)
* [resolve: scope visiting doesn't need an `Ident`](https://github.com/rust-lang/rust/pull/80782)
* [resolve/expand: improve attribute expansion on macro definitions and calls](https://github.com/rust-lang/rust/pull/80563)
* [optimize DST field access](https://github.com/rust-lang/rust/pull/80200)
* [allow references to interior mutable data behind a feature gate](https://github.com/rust-lang/rust/pull/80418)
* [fixed const_generics error help](https://github.com/rust-lang/rust/pull/80714)
* [use an empty `TokenCursorFrame` stack when capturing tokens](https://github.com/rust-lang/rust/pull/80830)
* [deduplicate solution enum in chalk-recursive](https://github.com/rust-lang/chalk/pull/674)
* [optimize away some `fs::metadata` calls](https://github.com/rust-lang/rust/pull/80756)
* [optimize away some path lookups in the generic `fs::copy` implementation](https://github.com/rust-lang/rust/pull/80755)
* [implement `From<char>` for `u64` and `u128`](https://github.com/rust-lang/rust/pull/79502)
* [stabilize `slice::strip_prefix` and `slice::strip_suffix`](https://github.com/rust-lang/rust/pull/77853)
* [add `[T; N]::each_ref` and `[T; N]::each_mut`](https://github.com/rust-lang/rust/pull/75490)
* [futures: perf: avoid an Option in the `Map*` futures](https://github.com/rust-lang/futures-rs/pull/2306)
* [backtrace: use the symbol table if the DWARF only has line numbers](https://github.com/rust-lang/backtrace-rs/pull/401)
* [cargo: stabilize -Zfeatures and -Zpackage-features](https://github.com/rust-lang/cargo/pull/8997)
* [rustdoc: fix macros 2.0 and built-in derives being shown at the wrong path](https://github.com/rust-lang/rust/pull/77862)
* [docs.rs: fix N+1 queries when fetching crate details](https://github.com/rust-lang/docs.rs/pull/1239)
* [docs.rs: fix performance regression in all releases-views](https://github.com/rust-lang/docs.rs/pull/1237)
* [clippy: new lint: vec_init_then_push](https://github.com/rust-lang/rust-clippy/pull/6538)

## Rust Compiler Performance Triage

* [2020-01-12](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-01-12.md):
1 Regressions, 2 Improvements, 3 Mixed
Overall, a positive albeit quiet week. The largest change came from the incremental compilation working group which delivered large gains in performance caused by [changes](https://github.com/rust-lang/rust/issues/76896) in how inlining is handled in debug mode. Unfortunately, these changes may be reversed due to concerns

Triage done by @rylev.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-01-12.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Serve crates-io registry over HTTP as static files](https://github.com/rust-lang/rfcs/pull/2789)
* [Infallible promotion](https://github.com/rust-lang/rfcs/pull/3027)
* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [RFC: Plan to make core and std's panic identical](https://github.com/rust-lang/rfcs/pull/3007)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Allow "artifact dependencies" on bin, cdylib, and staticlib crates](https://github.com/rust-lang/rfcs/pull/3028)
* [RFC: Pointer metadata & VTable](https://github.com/rust-lang/rfcs/pull/2580)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking Issue for `panic_any`](https://github.com/rust-lang/rust/issues/78500)
* [Tracking issue for stable SIMD in Rust](https://github.com/rust-lang/rust/issues/48556)

## New RFCs

* [Add the boxed!() macro to "de-magic" box syntax](https://github.com/rust-lang/rfcs/pull/3057)
* [try_trait_v2: A new design for the ? desugaring](https://github.com/rust-lang/rfcs/pull/3058)
* [Add language support for C-compatible bit-fields](https://github.com/rust-lang/rfcs/pull/3064)

# Upcoming Events

### Online
* [January 14, San Diego, CA, US - San Diego Rust January 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/275547915/)
* [January 20, Vancouver, BC, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/npqfbsycccbbc/)
* [January 21, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrycccbcc/)
* [January 26, Dallas, TX, US - Last Tuesay - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrycccbjc/)

### North America
* [January 14, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrycccbsb/)
* [January 14, Provo, UT, US - The Blue Pill: Rust on Microcontrollers (Jan / Third Round) - Utah Rust](https://www.meetup.com/utah-rust/events/268567961/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

- [Rust Engineer at The Graph (Remote)](https://thegraph.com/jobs/rust-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust favours security over convenience. Rust does not want you to make silly little mistakes than can waste so much of your time debugging, which in the end makes it more convenient.

– [@Joe232 on rust-users](https://users.rust-lang.org/t/rust-does-not-support-and-operator/53851/7)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/986) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://this-week-in-rust.org/blog/2021/01/13/this-week-in-rust-373/)</small>
