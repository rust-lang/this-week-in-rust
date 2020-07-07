Title: This Week in Rust 346
Number: 346
Date: 2020-07-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*]()

# Updates from Rust Community

## News & Blog Posts

* [Ownership of the standard library implementation](https://blog.rust-lang.org/inside-rust/2020/07/02/Ownership-Std-Implementation.html)
* [Announcing Rustup 1.22.0](https://blog.rust-lang.org/2020/07/06/Rustup-1.22.0.html)
* [Back to old tricks ..(or, baby steps in Rust)](https://donsbot.wordpress.com/2020/07/04/back-to-old-tricks-or-baby-steps-in-rust/)
* [Small strings in Rust](https://fasterthanli.me/articles/small-strings-in-rust)
* [Choosing a Rust web framework, 2020 edition](https://www.lpalmieri.com/posts/2020-07-04-choosing-a-rust-web-framework-2020-edition/)
* [Writing Interpreters in Rust: a Guide](https://pliniker.github.io/post/rust-hosted-langs/)
* [Transpiling A Kernel Module to Rust: The Good, the Bad and the Ugly](https://immunant.com/blog/2020/06/kernel_modules/)
* [Bad Apple!! and how I wrote a Rust video player for Task Manager!!](https://www.azabani.com/2020/06/29/bad-apple-for-taskmgr.html)
* [Boa release v0.9 and make use of Rust's measureme](https://boa-dev.github.io/2020/07/03/boa-release-09.html)
* [RiB (Rust in Blockchain) Newsletter #13](https://rustinblockchain.org/newsletters/2020-07-01-stuck-inside-hacking-away/)
* [7 Things I learned from Porting a C Crypto Library to Rust](https://sharpend.io/7-things-I-learned-from-porting-a-c-crypto-library-to-rust/)
* [AWS Lambda with Rust](https://blog.knoldus.com/aws-lambda-with-rust/)
* [Writing a winning 4K intro in Rust](https://www.codeslow.com/2020/07/writing-winning-4k-intro-in-rust.html)
* [Ringbahn II: the central state machine](https://without.boats/blog/ringbahn-ii/)
* [Bastion floating on Tide - Part 2](https://blog.bastion.rs/2020/06/14/bastion-floating-on-tide-part-2.html)
* [Porting Godot Games To Rust (Part 1)](https://paytonrules.com/post/games-in-rust-with-godot-part-one/)
* [Image decay as a service](https://fasterthanli.me/articles/image-decay-as-a-service)
* [IntelliJ Rust Changelog #125](https://intellij-rust.github.io/2020/06/29/changelog-125.html)
* [Abstracting away correctness](https://fasterthanli.me/articles/abstracting-away-correctness)
* [Rendering in Rust](https://www.zerotoga.me/dev/renderinginrust)
* [Super hero Rust fuzzing](https://blog.firosolutions.com/2020/07/superhero-rust-fuzzing/)
* [What Is a Dangling Pointer?](https://medium.com/swlh/what-is-a-dangling-pointer-2773d49cf86c)
* [Simple Rocket Web Framework Tutorial | POST Request](https://frogtok.com/simple-rocket-web-framework-tutorial-part-2in/)

# Crate of the Week

This week's crate is [print_bytes](https://crates.io/crates/print_bytes), a library to print arbitrary bytes to a stream as losslessly as possible.

Thanks to [dylni](https://users.rust-lang.org/t/crate-of-the-week/2704/784) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

339 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-22..2020-06-29

* [move leak-check to during coherence, candidate eval](https://github.com/rust-lang/rust/pull/72493)
* [account for multiple impl/dyn Trait in return type when suggesting `'_`](https://github.com/rust-lang/rust/pull/73496)
* [tweak binop errors](https://github.com/rust-lang/rust/pull/73674)
* [adds a clearer message for when the async keyword is missing from a function](https://github.com/rust-lang/rust/pull/73672)
* [allow dynamic linking for iOS/tvOS targets](https://github.com/rust-lang/rust/pull/73516)
* [always capture tokens for `macro_rules!` arguments](https://github.com/rust-lang/rust/pull/73293)
* [change heuristic for determining range literal](https://github.com/rust-lang/rust/pull/73639)
* [check for assignments between non-conflicting generator saved locals](https://github.com/rust-lang/rust/pull/73244)
* [const prop: erase all block-only locals at the end of every block](https://github.com/rust-lang/rust/pull/73757)
* [emit line info for generator variants](https://github.com/rust-lang/rust/pull/73460)
* [explain move errors that occur due to method calls involving `self`](https://github.com/rust-lang/rust/pull/73708)
* [fix handling of reserved registers for ARM inline asm](https://github.com/rust-lang/rust/pull/73588)
* [improve compiler error message for wrong generic parameter order](https://github.com/rust-lang/rust/pull/72271)
* [point at the call span when overflow occurs during monomorphization](https://github.com/rust-lang/rust/pull/73601)
* [provide suggestions for some moved value errors](https://github.com/rust-lang/rust/pull/73534)
* [self contained linking option](https://github.com/rust-lang/rust/pull/72738)
* [perform obligation deduplication to avoid buggy `ExistentialMismatch`](https://github.com/rust-lang/rust/pull/73485)
* [show the values and computation that would overflow a const evaluation or propagation](https://github.com/rust-lang/rust/pull/73513)
* [stabilize `#![feature(const_if_match)]` and `#![feature(const_loop)]`](https://github.com/rust-lang/rust/pull/72437)
* [A way forward for pointer equality in const eval](https://github.com/rust-lang/rust/pull/73398)
* [the const propagator cannot trace references](https://github.com/rust-lang/rust/pull/73613)
* [warn if linking to a private item](https://github.com/rust-lang/rust/pull/72771)
* [`improper_ctypes_definitions` lint](https://github.com/rust-lang/rust/pull/72700)
* [add Windows system error codes that should map to io::ErrorKind::TimedOut](https://github.com/rust-lang/rust/pull/71756)
* [errors: use `-Z terminal-width` in JSON emitter](https://github.com/rust-lang/rust/pull/73763)
* [proc_macro: stop flattening groups with dummy spans](https://github.com/rust-lang/rust/pull/73102)
* [rustc_lint: only query `typeck_tables_of` when a lint needs it](https://github.com/rust-lang/rust/pull/73743)
* [rustdoc: fix doc aliases with crate filtering](https://github.com/rust-lang/rust/pull/73644)
* [chalk: .chalk file syntax writer](https://github.com/rust-lang/chalk/pull/430)
* [chalk: add method to get repr data of an ADT to ChalkDatabase](https://github.com/rust-lang/chalk/pull/523)
* [chalk: fix built-in `Fn` impls when generics are involved](https://github.com/rust-lang/chalk/pull/541)
* [chalk: fix coherence issue with associated types in generic bound](https://github.com/rust-lang/chalk/pull/538)
* [miri: implement rwlocks on Windows](https://github.com/rust-lang/miri/pull/1461)
* [miri: supply our own implementation of the CTFE pointer comparison intrinsics](https://github.com/rust-lang/miri/pull/1459)
* [shortcuts for min/max on ordinary BTreeMap/BTreeSet iterators](https://github.com/rust-lang/rust/pull/73627)
* [add `TryFrom<{int}>` for `NonZero{int}`](https://github.com/rust-lang/rust/pull/72717)
* [add a fast path for `std::thread::panicking`.](https://github.com/rust-lang/rust/pull/72617)
* [add `[T]::partition_point`](https://github.com/rust-lang/rust/pull/73577)
* [add unstable `core::mem::variant_count` intrinsic](https://github.com/rust-lang/rust/pull/73418)
* [added io forwarding methods to the stdio structs](https://github.com/rust-lang/rust/pull/72705)
* [stabilize `leading_trailing_ones`](https://github.com/rust-lang/rust/pull/73032)
* [`impl PartialEq<Vec<B>> for &[A], &mut [A]`](https://github.com/rust-lang/rust/pull/71660)
* [forward `Hash::write_iN` to `Hash::write_uN`](https://github.com/rust-lang/rust/pull/73800)
* [libc: add ancillary socket data accessor functions for solarish OSes](https://github.com/rust-lang/libc/pull/1792)
* [libc: FreeBSD: machine register structs](https://github.com/rust-lang/libc/pull/1791)
* [libc: add wexecv, wexecve, wexecvp, wexecvpe](https://github.com/rust-lang/libc/pull/1796)
* [cargo: add support for `workspace.metadata` table](https://github.com/rust-lang/cargo/pull/8323)
* [cargo: adding environment variable CARGO_PKG_LICENSE_FILE](https://github.com/rust-lang/cargo/pull/8387)
* [cargo: enable "--target-dir" in "cargo install"](https://github.com/rust-lang/cargo/pull/8391)
* [cargo: expose built cdylib artifacts in the Compilation structure](https://github.com/rust-lang/cargo/pull/8418)
* [cargo: improve support for non-`master` main branches ](https://github.com/rust-lang/cargo/pull/8364)
* [docs.rs: don't panic when a crate doesn't exist for target-redirect](https://github.com/rust-lang/docs.rs/pull/859)
* [docs.rs: improve executing tests](https://github.com/rust-lang/docs.rs/pull/861)
* [clippy: lint iterator.map(|x| x)](https://github.com/rust-lang/rust-clippy/pull/5694)
* [clippy: new lint: suggest `ptr::read` instead of `mem::replace(..., uninitialized())`](https://github.com/rust-lang/rust-clippy/pull/5695)
* [clippy: clippy-driver: pass all args to rustc if --rustc is present](https://github.com/rust-lang/rust-clippy/pull/5178)
* [clippy: cmp_owned: handle when PartialEq is not implemented symmetrically](https://github.com/rust-lang/rust-clippy/pull/5701)
* [rustfmt: do not reorder module declaration with #![macro_use]](https://github.com/rust-lang/rustfmt/pull/4284)
* [rustfmt: don't reformat with errors unless --force flag supplied](https://github.com/rust-lang/rustfmt/pull/4256)

## Rust Compiler Performance Triage

* [2020-07-07](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-07-07). One unimportant regression on a rollup; six improvements, two on rollups.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Add a new `#[instruction_set(...)]` attribute for supporting per-function instruction set changes](https://github.com/rust-lang/rfcs/pull/2867)
* [Inline `const` expressions and patterns](https://github.com/rust-lang/rfcs/pull/2920)
* [Inline assembly](https://github.com/rust-lang/rfcs/pull/2873)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize const mem::forget](https://github.com/rust-lang/rust/pull/73887)
* [disposition: merge] [Stabilize casts and coercions to `&[T]` in const fn](https://github.com/rust-lang/rust/pull/73862)
* [disposition: merge] [mv std libs to std/](https://github.com/rust-lang/rust/pull/73265)
* [disposition: merge] [Stabilize `transmute` in constants and statics but not const fn](https://github.com/rust-lang/rust/pull/72920)
* [disposition: merge] [Stabilize const_type_id feature](https://github.com/rust-lang/rust/pull/72488)
* [disposition: merge] [Accept tuple.0.0 as tuple indexing (take 2)](https://github.com/rust-lang/rust/pull/71322)

## New RFCs

* [RFC: IndexGet and IndexSet](https://github.com/rust-lang/rfcs/pull/2953)

# Upcoming Events

### Online
* [July 9. Berlin, DE - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybckbmb/)
* [July 9. San Diego, CA, US - July 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/271680644/)
* [July 13. Seattle, WA, US - Seattle Rust Meetup - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybckbsb/)
* [July 16. Turin, IT - Rust Italia - Gruppo di studio di Rust](https://community.mozilla.org/events/gruppo-di-studio-di-rust-3/)

### North America
* [July 8. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybckblb/)
* [July 9. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/)
* [July 15. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybckbtb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at 1Password, Remote (US or Canada)](https://jobs.lever.co/1password/0623888f-0125-41b9-9902-eae8cfeae0c3)
* [Security Engineer at 1Password, Remote (US or Canada)](https://jobs.lever.co/1password/23444f56-c83b-4c75-85cf-64305c335e78)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> References are a sharp tool and there are roughly three different approaches to sharp tools.
>
> 1. Don't give programmers sharp tools. They may make mistakes and cut their fingers off. *This is the Java/Python/Perl/Ruby/PHP... approach.*
> 2. Give programmers all the sharp tools they want. They are professionals and if they cut their fingers off it's their own fault. *This is the C/C++ approach.*
> 3. Give programmers sharp tools, but put guards on them so they can't accidentally cut their fingers off. *This is Rust's approach.*
>
> Lifetime annotations are a safety guard on references. Rust's references have no sychronization and no reference counting -- that's what makes them sharp. References in category-1 languages (which typically *do* have synchronization and reference counting) are "blunted": they're not really *quite* as effective as category-2 and -3 references, but they don't cut you, and they still work; they might just slow you down a bit.
>
> So, frankly, I like lifetime annotations because they prevent me from cutting my fingers off.

– [trentj on rust-users](https://users.rust-lang.org/t/when-do-you-find-lifetime-annotations-helpful/44434/6)

Thanks to [Ivan Tham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/897) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hisn3e/this_week_in_rust_345/)</small>
