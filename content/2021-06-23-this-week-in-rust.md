Title: This Week in Rust 396
Number: 396
Date: 2021-06-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

No newsletters or research articles this week.

### Official
* [Announcing Rust 1.53.0](https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html)
* [Inside] [Rust Compiler June Steering Cycle](https://blog.rust-lang.org/inside-rust/2021/06/23/compiler-team-june-steering-cycle.html)

### Project/Tooling Updates
* [rust-analyzer Changelog #82](https://rust-analyzer.github.io/thisweek/2021/06/21/changelog-82.html)
* [IntelliJ Rust Changelog #149](https://intellij-rust.github.io/2021/06/21/changelog-149.html)
* [rustymind - Parse and visualize brainwaves with Rust](https://github.com/junjunjd/rustymind)
* [This Week In TensorBase 8](https://tensorbase.io/thisweek/2021-06-23-tw_8/)
* [Supporting Miguel Ojeda's Work on Rust in the Linux Kernel](https://www.memorysafety.org/blog/supporting-miguel-ojeda-rust-in-linux/)
* [rustc_codegen_gcc: Progress Report #1](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-1)

### Observations/Thoughts
* [Walking through "The Java Tutorials" with Rust - boxed trait objects and the search for inheritance](https://rust-java-tutorials.netlify.app/blog/5-trait-objects-2/)
* [WABT: A wonderful CLI for manipulating Wasm](https://blog.knoldus.com/wabt-a-wonderful-cli-for-manipulating-wasm/)
* [wasm-bindgen making Rust and JavaScript interoperability easy](https://blog.knoldus.com/wasm-bindgen-making-rust-and-javascript-interoperability-easy/)
* [Type-checked keypaths in Rust](https://www.cmyr.net/blog/keypaths.html)
* [Exploring ways to make async Rust easier](https://carllerche.com/2021/06/17/six-ways-to-make-async-rust-easier/)
* [Reading Back What You Wrote in Rust](https://www.morsecodist.io/blog/rust-writer-ownership)
* [First Rust Project: A Chess Engine](https://www.reddit.com/r/rust/comments/o3k6yu/first_rust_project_a_chess_engine/)
* [Rust vs C++ for game development](https://blog.logrocket.com/rust-vs-c-for-game-development/)
* [Skipping Tests in Rust](https://plume.benboeckel.net/~/JustAnotherBlog/skipping-tests-in-rust)
* [ZH] [Build a Gameboy emulator in Rust](https://yodalee.me/2020/12/2020_rust_gameboy/)
* [video] [Why the future of the cloud will be built on Rust - Oliver Gould, Buoyant](https://youtu.be/BWL4889RKhU)

### Rust Walkthroughs
* [Rust and AWS Lambda](https://mitchgollub.com/rust-and-aws-lambda/)
* [Deserializing Binary Data Files in Rust](https://adventures.michaelfbryan.com/posts/deserializing-binary-data-files/)
* [Yet Another Snazzy Rust CLI](https://dev.to/jeikabu/yet-another-snazzy-rust-cli-k4i)
* [Build an API in Rust (Part 2)](https://dev.to/naruhodo/build-an-api-in-rust-part-2-f11)
* [Polymorphism in Rust](https://oswalt.dev/2021/06/polymorphism-in-rust/)
* [Getting started with MongoDB and Redis in Rust](https://romankudryashov.com/blog/2021/06/mongodb-redis-rust/)
* [Making My Website Part 1: Monitoring On A Raspberry Pi](https://www.rotoclone.zone/blog/posts/raspberry-pi-monitoring)
* [ZH] [Develop WebAssembly Program in Rust](https://yodalee.me/2021/05/1helloworld/)
* [video] [Rust Fuzzing #3: How to write (better) Rust fuzz targets?](https://youtu.be/MiDFvrqjM2E)
* [video] [Building a Web Application with Rust - Part III - Database Manager](https://youtu.be/u-bjMHQ22TI)
* [video] [Building a Web Application with Rust - Part IV - HTTP Server](https://youtu.be/rJB0PLwipRI)

### Miscellaneous
* [Rust is not a Company](https://blog.m-ou.se/rust-is-not-a-company/)
* [Google Wants To See Rust Code In The Linux Kernel, Contracts The Main Developer](https://www.phoronix.com/scan.php?page=news_item&px=Google-Wants-Rust-In-Kernel)
* [Rust is the most wanted language by Godot Engine users](https://www.reddit.com/r/rust/comments/o5p267/rust_is_the_most_wanted_language_by_godot_engine/)

## Crate of the Week

This week's crate is [serde-encrypt](https://github.com/laysakura/serde-encrypt), a library that adds encryption to all `Serialize` impls.

Thanks to [Sho Nakatani](https://users.rust-lang.org/t/crate-of-the-week/2704/926) for the nomination.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [cargo - SearchIndexer takes time indexing \target on windows](https://github.com/rust-lang/cargo/issues/8694)
* [cargo - Ability to specify the output name for a bin target different from the crate name](https://github.com/rust-lang/cargo/issues/1706)
* [cargo - Using alternative registries names in text output](https://github.com/rust-lang/cargo/issues/6691)
* [cargo - A dependency on path = "." should have a good error message](https://github.com/rust-lang/cargo/issues/9518)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

301 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-06-07..2021-06-14

* [use `AttrVec` for `Arm`, `FieldDef`, and `Variant`](https://github.com/rust-lang/rust/pull/86385)
* [prefer `partition_point` to look up assoc items](https://github.com/rust-lang/rust/pull/86392)
* [lint for unused borrows as part of `UNUSED_MUST_USE`](https://github.com/rust-lang/rust/pull/86426)
* [miri: report an error if a `#[no_mangle]`/`#[export_name = ...]` function has the same symbol name as a built-in shim](https://github.com/rust-lang/miri/pull/1832)
* [fix span calculation in format strings](https://github.com/rust-lang/rust/pull/86104)
* [stabilize `span_open()` and `span_close()`](https://github.com/rust-lang/rust/pull/86136)
* [stabilize `ops::ControlFlow` (just the type)](https://github.com/rust-lang/rust/pull/85608)
* [linear interpolation](https://github.com/rust-lang/rust/pull/85925)
* [add functions `Duration::try_from_secs_`{`f32`, `f64'}](https://github.com/rust-lang/rust/pull/82179)
* [specialize `io::Bytes::size_hint` for more types](https://github.com/rust-lang/rust/pull/86202)
* [optimize `Eq` implementation for paths](https://github.com/rust-lang/rust/pull/86179)
* [integrate binary search codes of `binary_search_by` and `partition_point`](https://github.com/rust-lang/rust/pull/85406)
* [futures: introduce `stream::select_with_strategy`](https://github.com/rust-lang/futures-rs/pull/2450)
* [cargo: don't allow config env to modify vars set by cargo](https://github.com/rust-lang/cargo/pull/9579)
* [cargo: avoid quadratic complexity when splitting output into lines](https://github.com/rust-lang/cargo/pull/9586)
* [rustdoc: fix ICE when using `#[doc(keyword = "...")]` on non-items](https://github.com/rust-lang/rust/pull/86401)
* [rustdoc: account for const-unstable functions](https://github.com/rust-lang/rust/pull/86473)
* [clippy: fix wrong config option being suggested for deprecated `wrong_pub_self_convention` lint](https://github.com/rust-lang/rust-clippy/pull/7382)
* [clippy: improve panic message on "Found multiple rlibs" error in compile-test](https://github.com/rust-lang/rust-clippy/pull/7380)
* [clippy: add `macro_braces` lint to check for irregular brace use in certain macros](https://github.com/rust-lang/rust-clippy/pull/7299)
* [clippy: check for unbalanced tick pairs in `doc_markdown` lint](https://github.com/rust-lang/rust-clippy/pull/7357)
* [clippy: move `from-iter-instead-of-collect` to pedantic](https://github.com/rust-lang/rust-clippy/pull/7375)

### Rust Compiler Performance Triage

A few small regressions on smaller benchmarks (e.g., helloworld), likely
centered around more IR being generated in a few cases.

Triage done by **@simulacrum**.
Revision range: [d192c80..3912083](https://perf.rust-lang.org/?start=d192c80d2284ba6b5146bb3da586354c3762c72b&end=3912083821c5072f700a75589c8af6a9d3e20a21&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 0 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-22.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [RFC: Add delete and delete_by methods to Iterator](https://github.com/rust-lang/rfcs/pull/2475)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Redefine ErrorKind::Other and stop using it in std.](https://github.com/rust-lang/rust/pull/85746)
* [disposition: merge] [When using process::Command on Windows, environment variable names must be case-preserving but case-insensitive](https://github.com/rust-lang/rust/pull/85270)
* [disposition: merge] [Tracking Issue for std::io::Seek::rewind()](https://github.com/rust-lang/rust/issues/85149)
* [disposition: merge] [Support forwarding caller location through trait object method call](https://github.com/rust-lang/rust/pull/81360)
* [disposition: merge] [Tracking issue for ops::Bound::cloned()](https://github.com/rust-lang/rust/issues/61356)

### New RFCs

* [Stabilize Cargo's weak-dep-features and namespaced-features.](https://github.com/rust-lang/rfcs/pull/3143)

## Upcoming Events

### Online

* [June 24, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [June 29, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccjbmc/)
* [July 6, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/jxfdjsycckbjb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**ChainSafe Systems**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> At last, I can name my unsafe functions appropriately.
>
> `unsafe fn e͙̤͎̪͒x̲͓̞̤͍̻̺̂͗͛͆͡t̜̣͊̓ͩ̍̑e̩͖͙͎̼̖͉ͮṇ̨͖̎̓ͅd̗̼͕ͫ̅_̲̦̥̙̙͍͂́l͙͙̦̞̠̃͌͒i̹̘͍̳̊ͪͦͤ͒̊͋f̨ͥ̄̌ḛ̜͗̉̃̎̂̔̐t̩̲̘͕͉̺̫̓͗́i̹̤̭ͭ͆̔ͪͤ͢m̹̤̜̗̫̩͍ͨe̝͒ͣ<'b>(r: R<'b>) -> R<'static>`

– [Freeky on r/rust](https://www.reddit.com/r/rust/comments/o1yy1p/announcing_rust_1530/h2488f5)

Thanks to [Vincent de Phily](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1063) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/o6q7rw/this_week_in_rust_396/)</small>
