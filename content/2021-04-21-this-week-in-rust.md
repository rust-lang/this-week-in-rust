Title: This Week in Rust 387
Number: 387
Date: 2021-04-21
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
* [Inside] [Rust Compiler April Steering Cycle](https://blog.rust-lang.org/inside-rust/2021/04/15/compiler-team-april-steering-cycle.html)
* [Inside] [Lang team April update](https://blog.rust-lang.org/inside-rust/2021/04/17/lang-team-apr-update.html)
* [Inside] [Jacob Hoffman-Andrews joins the Rustdoc team](https://blog.rust-lang.org/inside-rust/2021/04/20/jsha-rustdoc-member.html)
* [Foundation] [Introducing Jane Lusby](https://foundation.rust-lang.org/posts/2021-04-15-introducing-jane-lusby/)
* [Foundation] [Introducing Shane Miller](https://foundation.rust-lang.org/posts/2021-04-15-introducing-shane-miller/)

### Newsletters

### Project/Tooling Updates
* [rust-analyzer Changelog #73](https://rust-analyzer.github.io/thisweek/2021/04/19/changelog-73.html)
* [Knurling-rs changelog #23](https://ferrous-systems.com/blog/knurling-changelog-23/)
* [This Week in Ballista #11](https://ballistacompute.org/thisweek/2021/04/18/this-week-in-ballista-11/)
* [Welcoming Alice Ryhl as the first paid Tokio contributor](https://tokio.rs/blog/2021-04-welcome-alice)
* [Zellij: a Rusty terminal multiplexer releases a beta version](https://zellij.dev/news/beta/)
* [faux: a struct mocking library - landing v0.1](https://nrxus.github.io/faux/blog/landing-v-0-1.html)
* [Otter - a game server for arbitrary board games](https://diziet.dreamwidth.org/8121.html).  It is mainly [written in Rust](https://www.chiark.greenend.org.uk/~ianmdlvl/otter/docs/build.html)
* [audio] [What's New in Rust 1.50 and 1.51](https://rustacean-station.org/episode/033-rust-1.50-1.51/)

### Observations/Thoughts
* [Red & blue functions are actually a good thing](https://blainehansen.me/post/red-blue-functions-are-actually-good/)
* [Why Rust powers Temporal's new Core SDK](https://docs.temporal.io/blog/why-rust-powers-core-sdk/)
* [Optimizing a sudoku solver in Rust](https://www.simonclark.dev/2020/08/10/optimizing-sudoku-solver.html)
* [My journey to understand rust-lang](https://daveshawley.medium.com/my-journey-to-understand-rust-lang-28e4cf808b12)
* [How I Implemented /dev/printerfact in Rust](https://christine.website/blog/dev-printerfact-2021-04-17)
* [Why fnm was rewritten in Rust](https://gal.hagever.com/posts/why-fnm-was-rewritten-in-rust/)
* [Running GraphQL on Lambda with Rust](https://dev.to/dbanty/running-graphql-on-lambda-with-rust-1lak)
* [Runtime Alias Detection](https://myrrlyn.net/blog/bitvec/alias-detection)
* [What's in the box?](https://fasterthanli.me/articles/whats-in-the-box)
* [video] [An Overview of the Embedded Rust Ecosystem](https://youtu.be/vLYit_HHPaY)

### Rust Walkthroughs
* [Late Night Confessions — Building a Website Using Rust, Rocket, Diesel, and Askama - Part 1](https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-1-aeccade43252)
* [Tour of Rust's Standard Library Traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)
* [Learning Rust #3: crates.io & publishing your package](https://hamatti.org/posts/learning-rust-3-crates-io-publishing-your-package/)
* [Making an online multiplayer game in Rust with Nakama](https://heroiclabs.com/blog/tutorials/rust-fishgame/)
* [Creating a Sleek Masonry Gallery with React and WebAssembly](https://dev.to/rvanderlaan/creating-a-sleek-masonry-gallery-with-react-and-webassembly-17p2)
* [The GPIO war: macro bunkers for typestate explosions (2)](https://www.ecorax.net/macro-bunker-2/)
* [Using `std` in embedded Rust](https://timmmm.github.io/std-embedded-rust/index.html)
* [Rust and TUI: Building a command-line interface in Rust](https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/)
* [Rust: Serde: Working with untyped JSON values](https://youtu.be/NwYY00paiH0)
* [series] [Writing NES Emulator in Rust](https://bugzmanov.github.io/nes_ebook/index.html)

### Miscellaneous
* [rustc, iOS and an M1](https://fnordig.de/2021/04/16/rustc-ios-and-an-m1/)
* [Microsoft Previews Rust For Windows](https://www.tectalk.co/microsoft-previews-rust-for-windows/)
* [Preparing Rustls for Wider Adoption](https://www.abetterinternet.org/post/preparing-rustls-for-wider-adoption/)
* [Are We Yeet Yet?](https://areweyeetyet.rs/)
* [Run Rust RISC-V Firmware with BL602 IoT SDK](https://lupyuen.github.io/articles/rust)
* [video] [Stanford Seminar - The Soul of a New Machine: Rethinking the Computer](https://youtu.be/vvZA9n3e5pc)

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

**Grover GmbH**

* [Software Engineer - Risk & Data, Rust & Python (Berlin or Remote)](https://grnh.se/15fcbda73us)

**Massa Labs**

* [Rust Blockchain Developer (Remote)](https://massa.network/#jobs)

**Instaclustr**

* [Software Engineer (Canberra, AU)](https://www.seek.com.au/job/52021829)

**Subspace Labs**

* [Core Protocol Engineer (Remote)](https://jobs.lever.co/subspacelabs/7f6a654b-60a8-4740-aa19-36b9f7a9e624?lever-origin=applied&lever-source%5B%5D=Twitter)

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5217029002)

**Luminovo**

* [(Senior) Software Engineer - Rust (Remote, CEST timezone)](https://luminovo.jobs.personio.de/job/357453)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> We feel that Rust is now ready to join C as a practical language for implementing the \[Linux\] kernel. It can help us reduce the number of potential bugs and security vulnerabilities in privileged code while playing nicely with the core kernel and preserving its performance characteristics.

– [Wedson Almeida Filho on the Google Security Blog](https://security.googleblog.com/2021/04/rust-in-linux-kernel.html)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1040) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/mvuk1k/this_week_in_rust_387/)</small>
