Title: This Week in Rust 334
Number: 334
Date: 2020-04-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [How often does Rust change](https://words.steveklabnik.com/how-often-does-rust-change)?
* [The differences between Ok-wrapping, try blocks, and function level try](https://yaah.dev/try-blocks).
* [Mental models around Ok-wrapping](https://vorner.github.io/2020/04/09/wrapping-mental-models.html).
* [The problem of effects in Rust](https://boats.gitlab.io/blog/post/the-problem-of-effects/).
* [Library-ification and analyzing Rust](https://smallcultfollowing.com/babysteps/blog/2020/04/09/libraryification/).
* [A possible new backend for Rust](https://jason-williams.co.uk/a-possible-new-backend-for-rust).
* [Simple but powerful Pratt parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).
* [Ruma is dead, long live Ruma](https://www.ruma.io/news/ruma-is-dead-long-live-ruma-2020-04-10/).
* [Programming Servo: My own private runtime](https://medium.com/programming-servo/programming-servo-my-own-private-runtime-8a5ba74c63c8).
* [Hyper traps](https://vorner.github.io/2020/04/13/hyper-traps.html).
* [Downloading all the crates on crates.io](https://www.pietroalbini.org/blog/downloading-crates-io/).
* [Kubernetes: A Rusty Friendship](https://deislabs.io/posts/kubernetes-a-rusty-friendship/).
* [Fallible Iterator Adapters](https://blog.yoshuawuyts.com/fallible-iterator-adapters/).
* [Types over strings: Extensible architectures in Rust](http://willcrichton.net/notes/types-over-strings/).
* [pdf] [LLHD: Rust is used to drive research in Hardware Design Languages](https://arxiv.org/pdf/2004.03494).
* [rust-analyzer changelog 20](https://rust-analyzer.github.io/thisweek/2020/04/13/changelog-20.html).
* [IntelliJ Rust changelog 120](https://intellij-rust.github.io/2020/04/13/changelog-120.html).
* [New sysinfo release: processes disk usage](https://blog.guillaume-gomez.fr/articles/2020-04-09+New+sysinfo+release%3A+processes+disk+usage).
* [April lang team design meetings](https://blog.rust-lang.org/inside-rust/2020/04/10/lang-team-design-meetings.html).

# Crate of the Week

This week's crate is [sudo](https://crates.io/crates/sudo), a library to let your program run as root.

Thanks to [Stefan Schindler](https://users.rust-lang.org/t/crate-of-the-week/2704/751) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [tree-buf: Floating point compression](https://github.com/That3Percent/tree-buf/issues/1). Tree-buf is an experimental serialization system written in Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

367 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-04-06..2020-04-13

* [support `#[track_caller]` on functions in `extern "Rust" { ... }`](https://github.com/rust-lang/rust/pull/70916)
* [handle `impl Trait` where `Trait` has an assoc type with missing bounds](https://github.com/rust-lang/rust/pull/69707)
* [normalize function signature in function casting check procedure](https://github.com/rust-lang/rust/pull/70982)
* [do not lose or reorder user-provided linker arguments](https://github.com/rust-lang/rust/pull/70665)
* [suggest move for closures and async blocks in more cases](https://github.com/rust-lang/rust/pull/70906)
* [remove false positives of `unused_braces`](https://github.com/rust-lang/rust/pull/70789)
* [use a `SmallVec` for `Cache::predecessors`](https://github.com/rust-lang/rust/pull/70876)
* [speed up path searching with `find_library_crate`](https://github.com/rust-lang/rust/pull/70837)
* [allocate some query results on an arena](https://github.com/rust-lang/rust/pull/70161)
* [add `io::Write::write_all_vectored`](https://github.com/rust-lang/rust/pull/70612)
* [detailed panic messages for `Vec` functions](https://github.com/rust-lang/rust/pull/70573)
* [small tweaks in `ToOwned::clone_into`](https://github.com/rust-lang/rust/pull/70201)
* [remove the `Ord` bound that was plaguing `drain_filter`](https://github.com/rust-lang/rust/pull/70843)
* [match options directly in the `Fuse` implementation](https://github.com/rust-lang/rust/pull/70750)
* [implement `Chain` with `Option` fuses](https://github.com/rust-lang/rust/pull/70896)
* [rearrange `BTreeMap::into_iter` to match `range_mut`](https://github.com/rust-lang/rust/pull/70981)
* [`BTreeMap` first last proposal tweaks](https://github.com/rust-lang/rust/pull/70850)
* [add `or_insert_with_key` to `Entry` of `HashMap`/`BTreeMap`](https://github.com/rust-lang/rust/pull/70996)
* [hashbrown: add or_insert_with_key to Entry of HashMap](https://github.com/rust-lang/hashbrown/pull/152)
* [arch: add more ARM SIMD intrinsics](https://github.com/rust-lang/stdarch/pull/792)
* [cargo: add `cargo tree` command](https://github.com/rust-lang/cargo/pull/8062)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Unsafe blocks in unsafe fn](https://github.com/rust-lang/rfcs/pull/2585).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Resolving `Ok`-wrapping for `try` blocks](https://github.com/rust-lang/rust/issues/70941).
* [disposition: merge] [Stabilize `Span::mixed_site`](https://github.com/rust-lang/rust/pull/68716).
* [disposition: merge] [Stabilize most common subset of alloc_layout_extras](https://github.com/rust-lang/rust/pull/69362).
* [disposition: merge] [Tracking issue for PathBuf capacity methods](https://github.com/rust-lang/rust/issues/58234).
* [disposition: merge] [Add Option to Force Unwind Tables](https://github.com/rust-lang/rust/pull/69984).
* [disposition: merge] [Move LLVM bitcode destination](https://github.com/rust-lang/rust/pull/70458).
* [disposition: merge] [A big options clean-up](https://github.com/rust-lang/rust/pull/70729).
* [disposition: merge] [Stabilize UNICODE_VERSION (feature unicode_version)](https://github.com/rust-lang/rust/pull/71068).

## New RFCs

* [Deduplicate Cargo workspace information](https://github.com/rust-lang/rfcs/pull/2906).
* [Major change proposal process for compiler team](https://github.com/rust-lang/rfcs/pull/2904).

# Upcoming Events

### Online

* [Apr 23. Turin, IT - Rust Turin online meetup](http://www.toolboxoffice.it/eventi/rust-meetup-15/).

### Europe

* [Apr 30. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcgbnc/).

### North America

* [Apr 22. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscrybcgbdc/).
* [Apr 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcgbkc/).
* [Apr 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcgblc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Backend Engineer, Data Processing – Rust (Remote) at Kraken](https://jobs.lever.co/kraken/246f7fd2-000a-4f61-8f53-b1cc783d51cb).
* [Rust Developer (US & Canada) at 1Password](https://1password.com/jobs/rust-developer/).
* [Infrastructure Engineer at Aleph Alpha, Heidelberg, Germany](https://aleph-alpha.de/sw_engineer.html?language=de).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This viewpoint is very controversial, and I have no capacity to debate it with anyone who disagrees with me. But Rust has a very powerful macro system, so I don’t have to.

– [withoutboats blogging about failure/fehler](https://boats.gitlab.io/blog/post/failure-to-fehler)

Thanks to [lxrec](https://users.rust-lang.org/t/twir-quote-of-the-week/328/849) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
