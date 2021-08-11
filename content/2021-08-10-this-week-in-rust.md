Title: This Week in Rust 403
Number: 403
Date: 2021-08-10
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
* [The push for GATs stabilization](https://blog.rust-lang.org/2021/08/03/GATs-stabilization-push.html)
* [Inside] [Lang team August update](https://blog.rust-lang.org/inside-rust/2021/08/04/lang-team-aug-update.html)

### Newsletters
* [This Month in Rust GameDev #24 - July 2021](https://gamedev.rs/news/024/)
* [This Month in Rust OSDev (July 2021)](https://rust-osdev.com/this-month/2021-07/)
* [The Monthly Oxide #5](https://mgattozzi.substack.com/p/the-monthly-oxide-5)
* [Rust Module of the Week: std::fs (Part 2): Dirs, Dirs, Dirs](https://motw.rs/blog/2021/08/08/stdfs-part-2-dirs-dirs-dirs/)

### Project/Tooling Updates
* [Rustdoc will now add Jump to Definition links in source code pages!](https://www.reddit.com/r/rust/comments/oz50qk/rustdoc_will_now_add_jump_to_definition_links_in/)
* [rust-analyzer Changelog #89](https://rust-analyzer.github.io/thisweek/2021/08/09/changelog-89.html)
* [rust-analyzer Financial Report #3](https://rust-analyzer.github.io/blog/2021/08/03/financial-report-3.html)
* [Whats New in IntelliJ Rust for the 2021.2 Release Cycle](https://blog.jetbrains.com/rust/2021/08/04/what-s-new-in-intellij-rust-for-the-2021-2-release-cycle/)
* [SixtyFPS weekly report for 8th of August 2021](https://sixtyfps.io/thisweek/2021-08-09.html)
* [How do Mina nodes communicate? Implementing bin_prot in Rust](https://medium.com/chainsafe-systems/how-do-mina-nodes-communicate-5a10b80fa253)
* [Trunk v0.10 – v0.13](https://trunkrs.dev/blog/v10-v13/)
* [Bevy's First Birthday](https://bevyengine.org/news/bevys-first-birthday/)
* [Progress report on rustc_codegen_cranelift (July 2021)](https://bjorn3.github.io/2021/08/05/progress-report-july-2021.html)
* [[upcoming] Volt - A rust-based package manager that's up to 12x faster than Yarn](https://www.reddit.com/r/rust/comments/oymdoj/upcoming_volt_a_rustbased_package_manager_thats/)
* [This week in Datafuse #2](https://datafuselabs.github.io/weekly/2021-08-11-datafuse-weekly/)

### Observations/Thoughts
* [On Collecting Result Types in Rust](https://diaries.vercel.app/posts/collecting-result/)
* [Rust in Production: Qovery](https://serokell.io/blog/rust-in-production-qovery)
* [How to write really slow Rust code - Part 2](https://renato.athaydes.com/posts/how-to-write-slow-rust-code-part-2.html)
* [Slitter: a slab allocator that trusts but verifies (in Rust, for C)](https://engineering.backtrace.io/2021-08-04-slitter-a-slab-allocator-that-trusts-but-verifies/)
* [When Zero Cost Abstractions Aren't Zero Cost](https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html)

### Rust Walkthroughs
* [Rust cli example: Ferris fetches Go gopher postcards](https://dev.to/uggla/rust-cli-example-ferris-fetches-go-gopher-postcards-3gb5)
* [A Little Bit About PRNG Stuff](https://lokathor.github.io/prng/)
* [Rust's Vector](https://blog.frankel.ch/start-rust/9/)
* [Interacting with data from FFI in Rust](https://blog.guillaume-gomez.fr/articles/2021-07-29+Interacting+with+data+from+FFI+in+Rust)
* [Loading a Rust library as a Lua module in Neovim](https://blog.kdheepak.com/loading-a-rust-library-as-a-lua-module-in-neovim.html)
* [Adding Rust-Stable libstd Support for Xous](https://www.crowdsupply.com/sutajio-kosagi/precursor/updates/adding-rust-stable-libstd-support-for-xous)
* [Rust BDD tests with Cucumber](https://dev.to/rogertorres/rust-bdd-with-cucumber-4p68)
* [Rust cli example: Ferris fetches Go gopher postcards](https://dev.to/uggla/rust-cli-example-ferris-fetches-go-gopher-postcards-3gb5)
* [So you want to write a GUI framework](https://www.cmyr.net/blog/gui-framework-ingredients.html)
* [Tauri with Standard Svelte or SvelteKit](https://medium.com/@cazanator/tauri-with-standard-svelte-or-sveltekit-ad7f103c37e7)
* [JP] [Rust で Web バックエンド開発をはじめる](https://developers.cyberagent.co.jp/blog/archives/31110/)
* [video] [Explaining Rust Analyzer 08: Mini Rowan, Green and Red Trees](https://youtu.be/n5LDjWIAByM)

### Research
* [CVE-2021-29922 – rust standard library “net” – Improper Input Validation of octal literals in rust 1.52.0 std::net and below results in indeterminate SSRF & RFI vulnerabilities.](https://sick.codes/sick-2021-015)
* [Retrofitting Typestates into Rust](https://github.com/rustype/typestate-rs/blob/main/paper/sblp21.pdf)

### Miscellaneous
* [SIMD usage in C++, C# and RUST](https://vksegfault.github.io/posts/simd-usage-cpp-csharp-rust/)
* [RIP 16-bit crate IDs](https://www.reddit.com/r/rust/comments/p1t32e/media_rip_16bit_crate_ids/)
* [Your favorite Rust CLI utility?](https://www.reddit.com/r/rust/comments/oygrp1/your_favorite_rust_cli_utility_i_have_my_top_10/)
* [Go, Rust "net" library affected by critical IP address validation vulnerability](https://www.bleepingcomputer.com/news/security/go-rust-net-library-affected-by-critical-ip-address-validation-vulnerability/)
* [Microsoft Rust intro says "Rust is known to leak memory"](https://www.reddit.com/r/rust/comments/p0bu4a/microsoft_rust_intro_says_rust_is_known_to_leak/)

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

* [rust-lang/this-week-in-rust - Feature request: Dark theme for the website](https://github.com/rust-lang/this-week-in-rust/issues/2274)
* [Stranger6667/jsonschema-rs - Implement the uuid format validator](https://github.com/Stranger6667/jsonschema-rs/issues/266)
* [Stranger6667/jsonschema-rs - Implement the duration format validator](https://github.com/Stranger6667/jsonschema-rs/issues/265)
* [Stranger6667/jsonschema-rs - Option to turn off processing of the format keyword](https://github.com/Stranger6667/jsonschema-rs/issues/261)

Some of these tasks may also have mentors available, visit the task page for more information.

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

* [RFC: Overconstraining and omitting unsafe in impls of unsafe trait methods](https://github.com/rust-lang/rfcs/pull/2316)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for UnsafeCell::raw_get](https://github.com/rust-lang/rust/issues/66358)

### New RFCs

* [RFC: let-expression](https://github.com/rust-lang/rfcs/pull/3159)

## Upcoming Events

### Online

* [August 10, 2021, Dallas, TX, US - Second Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/vqtjcsycclbnb/)
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

**TrueLayer**

* [Associate Software Engineer - PayDirect (Milan, IT)](https://apply.workable.com/truelayer/j/42914EDDEC/)

**Xayn**

* [(Senior) Rust Engineer (Berlin, Germany)](https://xayn.jobs.personio.de/job/288899)

**ChainSafe**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Kraken**

* [Senior Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105)
* [Senior Banking Engineer - Rust (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)

**Tempus Ex**

* [Several positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US and Remote)](https://tempus-ex.com/careers?r=twir)

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/p2701i/this_week_in_rust_403/)</small>
