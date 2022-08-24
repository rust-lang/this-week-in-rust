Title: This Week in Rust 448
Number: 448
Date: 2022-06-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

* [2021 Annual Survey Report](https://blog.rust-lang.org/inside-rust/2022/06/21/survey-2021-report.html)

### Project/Tooling Updates

* [rust-analyzer changelog #134](https://rust-analyzer.github.io/thisweek/2022/06/20/changelog-134.html)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W24](https://www.fornjot.app/blog/weekly-dev-log/2022-w24/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-06-20.html)
* [This week in Databend #47: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-06-22-databend-weekly/)
* [Postcard 1.0.0 Release](https://jamesmunns.com/blog/postcard-1-0/)
* [mold 1.3.0](https://github.com/rui314/mold/releases/tag/v1.3.0)

### Observations/Thoughts

* [That vendored OpenSSL 🦀 most of us rely on probably needs a patch ..](https://medium.com/rust-secure-code/that-vendored-openssl-most-of-us-rely-on-probably-needs-a-patch-aae8fea5160f)
* [GUI in Rust with iced #2: Composable Layout](https://nikolish.in/gs-with-iced-2)
* [Remote development with Rust on fly.io](https://fasterthanli.me/articles/remote-development-with-rust-on-fly-io)
* [NVIDIA GPU profiling with Rust](https://simbleau.github.io/blog/gpu-profiling-with-rust/)
* [Unsafe code highlighting with rust-analyzer](https://veykril.github.io/posts/semantic-unsafe/)
* [When serde_json::to_string() fails](https://www.greyblake.com/blog/when-serde-json-to-string-fails/)
* [Aeneas: Rust Verification by Functional Translation](https://arxiv.org/abs/2206.07185)
* [Rust: Turtles all the way down](https://www.youtube.com/watch?v=PuMXWc0xrK0)
* [What is `Box<str>` and how is it different from `String` in Rust?](https://mahdi.blog/rust-box-str-vs-string/)
* [What it feels like when Rust saves your bacon](https://smallcultfollowing.com/babysteps/blog/2022/06/15/what-it-feels-like-when-rust-saves-your-bacon/)
* [Serde by Example 1: JSON-RPC](https://blog.dzejkop.space/serde-by-example-1/)
* [video] [Adding a Rust compiler front end to GCC Philip Herron & Arthur Cohen, Embecosm](https://m.youtube.com/watch?v=R8Pr21nlhig)
* [audio] [This Week in Rust - Issue 445](https://rustacean-station.org/episode/twir-445/)
* [audio] [Fig with Grant Gurvis](https://rustacean-station.org/episode/grant-gurvis/)
* [audio] [Building with Rust: Ernest Kissiedu on the DevX Initiative](https://anchor.fm/building-with-rust/episodes/Ernest-Kissiedu-on-the-DevX-Initiative-e1k3cs4)

### Rust Walkthroughs

* [Build a simple template engine in <100 lines of Rust code](https://blog.spike.codes/build-a-template-engine)
* [A short introduction to async Rust](https://www.shuttle.rs/blog/2022/06/16/a-short-introduction-to-async-rust)
* [Nine rules for elegant Rust library APIs](https://towardsdatascience.com/nine-rules-for-elegant-rust-library-apis-9b986a465247)
* [Automated maintenance and dependencies security for Rust projects](https://kerkour.com/rust-projects-maintenance-and-supply-chain-security)
* [video] [Async I/O in Depth (Part 3): Implementing an Async Runtime](https://www.youtube.com/watch?v=yfcJGEISsLc)
* [video] [Rust for Beginners -- Learning Rust Syntax faster with these Rustlings Tips](https://www.youtube.com/watch?v=kvmMGjF-_zo)

## Crate of the Week

This week's crate is [error-stack](https://docs.rs/error-stack/latest/error_stack/), a currently nightly-only error handling library that optimizes for ease of use while allowing to add arbitrary context data to errors.

Thanks to [Alfred Mountfield](https://users.rust-lang.org/t/crate-of-the-week/2704/1070) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

374 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-06-13..2022-06-20

* [add Apple WatchOS compile targets](https://github.com/rust-lang/rust/pull/95243)
* [std support for the Nintendo 3DS](https://github.com/rust-lang/rust/pull/95897)
* [make missing argument placeholder more obvious that it's a placeholder](https://github.com/rust-lang/rust/pull/98264)
* [improve `lifetime arguments are not allowed on` error message](https://github.com/rust-lang/rust/pull/98268)
* [improve parser diagnostics](https://github.com/rust-lang/rust/pull/95211)
* [improve parsing errors and suggestions for bad `if` statements](https://github.com/rust-lang/rust/pull/97474)
* [suggest adding a `#[macro_export]` to a private macro](https://github.com/rust-lang/rust/pull/98087)
* [hide irrelevant lines in suggestions to allow for suggestions that are far from each other to be shown](https://github.com/rust-lang/rust/pull/97798)
* [do not suggest adding semicolon/changing delimiters for macros in item position that originates in macros](https://github.com/rust-lang/rust/pull/97377)
* [fix suggestions for `&a: T` parameters](https://github.com/rust-lang/rust/pull/97964)
* [miri: prevent futex_wait from actually waiting if a concurrent waker was executed before us](https://github.com/rust-lang/miri/pull/2228)
* [miri: add ICE error level](https://github.com/rust-lang/miri/pull/2237)
* [fix `MissingDoc` quadratic behaviour](https://github.com/rust-lang/rust/pull/98153)
* [fix `SourceScope` for `if let` bindings](https://github.com/rust-lang/rust/pull/97931)
* [make "Assemble stage1 compiler" orders of magnitude faster (take 2)](https://github.com/rust-lang/rust/pull/97268)
* [`BitSet` related perf improvements](https://github.com/rust-lang/rust/pull/97863)
* [obligation forest tweaks](https://github.com/rust-lang/rust/pull/97674)
* [batch `proc_macro` RPC for `TokenStream` iteration and combination operations](https://github.com/rust-lang/rust/pull/98186)
* [compile `unicode-normalization` faster](https://github.com/rust-lang/rust/pull/97936)
* [use valtrees as the type-system representation for constant values](https://github.com/rust-lang/rust/pull/96591)
* [make some lints incremental](https://github.com/rust-lang/rust/pull/98238)
* [optimize heapsort](https://github.com/rust-lang/rust/pull/93765)
* [`impl Termination for Infallible` and then make the `Result` impls of `Termination` more generic](https://github.com/rust-lang/rust/pull/97803)
* [`Stdio::makes_pipe`](https://github.com/rust-lang/rust/pull/97150)
* [implement `core::slice::IterMut::as_mut_slice` and `impl<T> AsMut<[T]> for IterMut<'_, T>`](https://github.com/rust-lang/rust/pull/93080)
* [add `core::mem::copy` to complement `core::mem::drop`](https://github.com/rust-lang/rust/pull/95534)
* [avoid `thread::panicking()` in non-poisoning methods of `Mutex` and `RwLock`](https://github.com/rust-lang/rust/pull/97924)
* [make {`Mutex`, `Condvar`, `RwLock`}`::new()` const](https://github.com/rust-lang/rust/pull/97791)
* [add `#[inline]` to small fns of futex `RwLock`](https://github.com/rust-lang/rust/pull/98143)
* [add {`Arc`, `Rc`}`::downcast_unchecked`](https://github.com/rust-lang/rust/pull/96609)
* [add `VecDeque::extend` from `TrustedLen` specialization](https://github.com/rust-lang/rust/pull/98004)
* [`BTreeMap`: Support custom allocators (v1.5)](https://github.com/rust-lang/rust/pull/98103)
* [use unchecked mul to compute slice sizes](https://github.com/rust-lang/rust/pull/98078)
* [stabilize `try_reserve_2`](https://github.com/rust-lang/rust/pull/95392)
* [stabilize `io_safety`](https://github.com/rust-lang/rust/pull/95118)
* [stabilize `Path::try_exists()` and improve doc](https://github.com/rust-lang/rust/pull/97912)
* [const-stabilize checked slice → `str` conversion functions](https://github.com/rust-lang/rust/pull/97367)
* [hashbrown: Add an `Entry` API for `HashSet`](https://github.com/rust-lang/hashbrown/pull/342)
* [hashbrown: allow compiling on 1.56.0](https://github.com/rust-lang/hashbrown/pull/343)
* [codegen\_gcc: more intrinsics](https://github.com/rust-lang/rustc_codegen_gcc/pull/181)
* [clippy: add lint output to lint list](https://github.com/rust-lang/rust-clippy/pull/8947)
* [clippy: fix false positive for `never_loop` struct expression fields](https://github.com/rust-lang/rust-clippy/pull/9002)
* [clippy: rework `branches_sharing_code`](https://github.com/rust-lang/rust-clippy/pull/8901)
* [clippy: warn about read into zero-length `Vec`](https://github.com/rust-lang/rust-clippy/pull/8964)
* [clippy: ignore `todo!` and `unimplemented!` in `if_same_then_else`](https://github.com/rust-lang/rust-clippy/pull/9006)
* [clippy: add `default_iter_empty`](https://github.com/rust-lang/rust-clippy/pull/8989)
* [clippy: unused_async: lint async methods](https://github.com/rust-lang/rust-clippy/pull/9025)
* [rustfmt: add width for codeblocks in comments](https://github.com/rust-lang/rustfmt/pull/5372)
* [rust-analyzer: show proc-macro loading errors in unresolved-proc-macro diagnostics](https://github.com/rust-lang/rust-analyzer/pull/12544)
* [rust-analyzer: add fold range for multi line match arm list](https://github.com/rust-lang/rust-analyzer/pull/12576)
* [rust-analyzer: fix methods in pub trait generated by macro cannot be completed](https://github.com/rust-lang/rust-analyzer/pull/12517)
* [rust-analyzer: ask the user to reload the vscode window when changing server settings](https://github.com/rust-lang/rust-analyzer/pull/12529)
* [rust-analyzer: check for the correct proc-macro settings in missing proc-macro diagnostics](https://github.com/rust-lang/rust-analyzer/pull/12528)
* [rust-analyzer: clear proc-macro changed flag when reloading workspace](https://github.com/rust-lang/rust-analyzer/pull/12541)
* [rust-analyzer: don't trigger pattern completions when typing a wildcard pattern](https://github.com/rust-lang/rust-analyzer/pull/12596)

### Rust Compiler Performance Triage

A pretty busy week for performance, with quite a few PRs landing with
significant improvements to specific benchmarks. Overall the week was positive,
with many benchmarks improving at least a little. No significant changes in
memory usage this week.

Triage done by **@simulacrum**.
Revision range: [edab34a..abace0a](https://perf.rust-lang.org/?start=edab34ab2abbafc16a78daedf71dbacd2eb0b7bf&end=abace0a1f17986d89aedf610819deab2b4afee56&absolute=false&stat=instructions%3Au)

5 Regressions, 6 Improvements, 3 Mixed; 1 of them in rollups
54 artifact comparisons made in total

See [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-06-21.md) for details.

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Mention de-approval of `cfg(target = "...")`](https://github.com/rust-lang/rfcs/pull/3276)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Refined trait implementations](https://github.com/rust-lang/rfcs/pull/3245)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Remove a back-compat hack on lazy TAIT](https://github.com/rust-lang/rust/pull/97346)
* [disposition: merge] [allow unions with mutable references and tuples of allowed types](https://github.com/rust-lang/rust/pull/97995)
* [disposition: merge] [make const_err show up in future breakage reports ](https://github.com/rust-lang/rust/pull/97743)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-06-22 - 2022-07-20 🦀

### Virtual

* 2022-06-22 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Study Session - Procedural Macros**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/286594851/)
* 2022-06-24 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286688186/)
* 2022-06-28 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydcjblc/)
* 2022-06-29 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcjbmc/)
* 2022-06-30 | Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 23th Edition**](https://www.meetup.com/Rust-Linz/events/286029968/)
* 2022-07-05 | Austin, TX, US | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting #10**](https://www.meetup.com/webassembly-and-wasmedge/events/zzdnrsydckbhb/)
* 2022-07-05 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/jbfnrsydckbhb/)
* 2022-07-05 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydckbhb/)
* 2022-07-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydckbjb/)
* 2022-07-07 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust, nalgebra, and Fourier Optics**](https://www.meetup.com/charlottesville-rust-meetup/events/285818136/)
* 2022-07-09 | Virtual | [Rust Game Dev](https://github.com/rust-gamedev/wg)
    * [**Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-14 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/swgrssydckbsb/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)

### Asia

* 2022-07-04 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**Rust Interop, Rewrites and fun**](https://www.meetup.com/rust-tlv/events/286610368/)

### Europe

* 2022-06-22 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/286305083/)
* 2022-06-23 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #26**](https://www.meetup.com/rust-wroclaw/events/286415834/)
* 2022-06-28 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks June 2022: Community Showcase**](https://www.meetup.com/rust-london-user-group/events/286489185/)
* 2022-06-29 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Get started with Rust and stories from the frontlines**](https://www.meetup.com/rust-nederland/events/286582960/)
* 2022-06-30 | Copenhagen, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #26**](https://cph.rs/)

### North America

* 2022-06-29 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - June 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)
* 2022-06-29 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/286596997/)
* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)

### Oceania

* 2022-06-23 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**June Meetup**](https://www.meetup.com/rust-brisbane/events/286385515/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

**Mutate**

* [Rust Backend Software Engineer (Remote Europe)](https://rustjobs.dev/featured-jobs/Mutate-Rust-Backend-Software-Engineer-7kfTlQFSagzwHhugw1p0)

**Matician**

* [Robotics Systems Engineer - FT and intern roles (Mountain View, CA, US)](https://jobs.lever.co/matician/3d2a49ae-43c9-41c0-b9e8-7b87e7429b8f)

**Lithic**

* [Senior Infrastructure Engineer (New York, NY, US, Austin, TX, US, San Francisco, CA, US, or Remote)](https://boards.greenhouse.io/lithic/jobs/4252422004)

**Spire Global**

* [Software Engineers + Engineering Managers (Glasgow, UK, Luxembourg - Relocation Available)](https://spire.com/careers/job-openings/job/?gh_jid=4226230)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rwlock vs Mutex? Please, tell me like I'm 5

<!-- -->
> Mutex: "Mom says it's my turn on the synchronization primitive."  
> vs.  
> Write lock: "Hey! You all are not allowed to look until I'm done writing!"
> Read lock: "Hey! You are not allowed to edit what you wrote until we're done reading it!"

<!-- -->
> Thanks for an actual 5 year old reply, made me laugh

– [/u/LyonSyonII and /u/everything-narrative on /r/rust](https://www.reddit.com/r/rust/comments/vcaabk/rwlock_vs_mutex_please_tell_me_like_im_5/)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1254) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/vioiwj/this_week_in_rust_448/)</small>
