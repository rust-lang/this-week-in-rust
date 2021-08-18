Title: This Week in Rust 404
Number: 404
Date: 2021-08-18
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

In the case of this newsletter, 404 is indeed found!

## Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

* [Sixtyfps weekly report for 16th of August 2021](https://sixtyfps.io/thisweek/2021-08-16.html)
* [Relm4 beta released 🎉](https://aaronerhardt.github.io/blog/posts/relm_beta/)
* [This week in Fluvio #2: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0002/)
* [This Week In TensorBase 15](https://tensorbase.io/thisweek/2021-08-11-tw_15/)

### Observations/Thoughts

* [How to deal with large Cargo workspaces](https://kerkour.com/blog/rust-large-cargo-workspace)

### Rust Walkthroughs

* [A fast port scanner in 100 lines of Rust](https://kerkour.com/blog/rust-fast-port-scanner)
* [RPC to call a Runtime API easily in Substrate](https://blog.knoldus.com/rpc-to-call-a-runtime-api-easily-in-substrate/)
* [Writing a lambda in Rust using lambda-runtime and aws-sdk-rust](https://dev.to/aviillouz/writing-a-lambda-with-rust-using-aws-lambda-rust-runtime-and-aws-sdk-rust-1aln)
* [Daily Rust: Slice Patterns](https://adventures.michaelfbryan.com/posts/daily/slice-patterns/)
* [Learning Rust by building an SSH cli tool](https://saidvandeklundert.net/learn/2021-08-06-rust-ssh-cli-tool/)
* [Getting started with Rust programming language 🦀 2021: 4. Building a CLI app in Rust](https://www.youtube.com/watch?v=4km2UijVC3M)

### Research

### Miscellaneous

* [Rust on RISC-V BL602: Simulated with WebAssembly](https://lupyuen.github.io/articles/rustsim)
* [Calling WebAssembly from Rust](https://paulbutler.org/2021/calling-webassembly-from-rust/)

## Crate of the Week

This week's crate is [ockam](https://crates.io/crates/ockam), a crate to implement transport-agnostic end-to-end encryption for the rest of us.

Thanks to [staticassert](https://users.rust-lang.org/t/crate-of-the-week/2704/943) for the self-suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Module of the Week

Continuing [Rust Module of the Week](https://motw.rs) this week is [std::fs Part 2: Dirs, Dirs, Dirs](https://motw.rs/blog/2021/08/08/stdfs-part-2-dirs-dirs-dirs/). Contribution and feedback welcome [here](https://github.com/slyons/rust-module-of-the-week).

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* GitUI has an easy to get started code base, mentoring and [good-first-issues waiting](https://github.com/extrawurst/gitui/labels/good%20first%20issue)
* [getsynth/synth - Support postgres json and jsonb types](https://github.com/getsynth/synth/issues/67)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

324 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-08-02..2021-08-09

* [fill out remaining parts of C-unwind ABI](https://github.com/rust-lang/rust/pull/86155)
* [CTFE: throw unsupported error when partially overwriting a pointer](https://github.com/rust-lang/rust/pull/87248)
* [proc macro spans: make columns 1 based](https://github.com/rust-lang/rust/pull/87712)
* [improve diagnostics for wrongly ordered keywords in function declaration](https://github.com/rust-lang/rust/pull/87235)
* [replace `HirId`s with `LocalDefId`s in `AccessLevels` tables](https://github.com/rust-lang/rust/pull/87568)
* [add `config.toml` options for enabling overflow checks in rustc and std](https://github.com/rust-lang/rust/pull/87784)
* [use zeroed allocations in the mir interpreter instead eagerly touching the memory](https://github.com/rust-lang/rust/pull/87777)
* [only compute `is_freeze` for layout-constrained ADTs](https://github.com/rust-lang/rust/pull/87737)
* [allow generic SIMD array element type](https://github.com/rust-lang/rust/pull/87716)
* [properly find owner of closure in THIR unsafeck](https://github.com/rust-lang/rust/pull/87645)
* [make `wrapping_neg()` use `wrapping_sub()`, `#[inline(always)]`](https://github.com/rust-lang/rust/pull/87150)
* [stabilize `Vec<T>::shrink_to`](https://github.com/rust-lang/rust/pull/86879)
* [`impl Default, Copy, Clone for std::io::Sink` and `Empty`](https://github.com/rust-lang/rust/pull/86744)
* [change environment variable getters to error recoverably](https://github.com/rust-lang/rust/pull/86183)
* [add `core::stream::from_iter`](https://github.com/rust-lang/rust/pull/81797)
* [futures: implement `Default` for `OptionFuture`](https://github.com/rust-lang/futures-rs/pull/2471)
* [clippy: don't emit `too_many_lines` for closures](https://github.com/rust-lang/rust-clippy/pull/7534)
* [clippy: add xor case to manual swap lint](https://github.com/rust-lang/rust-clippy/pull/7506)

### Rust Compiler Performance Triage

Quiet week for performance, with no large changes. Regressions are limited to just a few benchmarks.

Triage done by **@simulacrum**.
Revision range: [998cfe5..3354a44](https://perf.rust-lang.org/?start=998cfe5aad7c21eb19a4bca50f05a13354706970&end=3354a44d2fa8d5ba6b8d6b40d2596de2c8292ec1&absolute=false&stat=instructions%3Au)

2 Regressions, 0 Improvements, 0 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-08-03.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize "force warn" option](https://github.com/rust-lang/rust/issues/86516)

### New RFCs

* [RFC: let-expression](https://github.com/rust-lang/rfcs/pull/3159)

## Upcoming Events

### Online

* [August 18, 2021, Denver, CO, US - Level up our Rust skills by building an ECS by Brooks Patton - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/278909353/)
* [August 18, 2021, Vancouver, BC, CA - Solving LeetCode Problems with Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsycclbxb/)
* [August 19, 2021, Manchester, UK - Rust Manchester - Speeding Up the Snake: Extending Python with Rust](https://www.meetup.com/rust-manchester/events/279730616/)
* [August 19, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [August 31, 2021, Dallas, TX - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Apple**

* [Senior Rust Engineer (Seattle)](https://jobs.apple.com/en-us/details/200256576/ai-ml-software-engineer-information-intelligence?team=MLAI) 
* [Software Engineer, Distributed Block Storage (Seattle, WA, USA)](https://jobs.apple.com/en-us/details/200275619/software-engineer?team=SFTWR)

**Nymi**

* [Senior Software Developer](https://www.nymi.com/careers)

**Dialectic AG**

* [Software Development Intern - Rust (EU / CH)](https://swissdevjobs.ch/de/jobs/Dialectic-AG-Software-Development-Intern)

**PolarFox Network**

* [Senior Rust Engineer (Remote)](https://jro-ventures.breezy.hr/p/0c1d3630d39d)

**ChainSafe**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Stader**

* [Senior & Junior Rust Developer (Remote)](https://www.notion.so/Job-Openings-at-Stader-a0faadf6454c4f29a68f23851d901619?p=519b05172ea04263ae09a2d54b004c15)

**Subspace Labs**

* [Several positions available (Remote)](https://jobs.lever.co/subspacelabs)

**Headline**

* [Rust Developer (Remote)](https://old.reddit.com/r/rust/comments/ou5szr/official_rrust_whos_hiring_thread_for_jobseekers/h9f4r52/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

We regrettably lack nominations,  
so as I can't choose fresh quotations,  
at last nor this time,  
I'll offer this rhyme  
to quell all discombombulations.

– a very sorry llogiq

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
