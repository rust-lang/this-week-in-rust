Title: These Weeks in Rust 127
Number: 127
Date: 2016-04-25
Category: This Week in Rust

Hello and welcome to another multi-week issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts

* 🎈🎉 [Announcing Rust 1.8](http://blog.rust-lang.org/2016/04/14/Rust-1.8.html). 🎉🎈
* [Writing an OS in Rust: Kernel heap](http://os.phil-opp.com/kernel-heap.html). Part of the series [Writing an OS in Rust](http://os.phil-opp.com/).
* [Learn you a Rust III - Lifetimes 101](http://pro.theta.eu.org/2016/04/16/lyar-lifetimes.html). Part of the series [Learn you a Rust for great good!](http://pro.theta.eu.org/2016/03/12/learn-you-a-rust-for-great-good.html).
* [PoC: using LLVM’s profile guided optimization in Rust](https://unhandledexpression.com/2016/04/14/using-llvm-pgo-in-rust/).
* [From &str to Cow](http://blog.jwilm.io/from-str-to-cow/).
* [The basics of Rust structs](https://facility9.com/2016/04/the-basics-of-rust-structs/).
* [Rust community == Awesome!](https://llogiq.github.io/2016/04/23/awesome.html). How the Rust community wins despite its small size and incomplete ecosystem.
* [This week in Rust docs 1](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-1).
* [This week in Servo 59](http://blog.servo.org/2016/04/11/twis-59/) and [This week in Servo 60](http://blog.servo.org/2016/04/18/twis-60/).
* [This week in Ruma 2016-04-17](https://www.ruma.io/news/this-week-in-ruma-2016-04-17/). Ruma is a Matrix client-server API written in Rust.

## Notable New Crates & Project Updates

* Reminder: [Rust Belt Rust Conference CFP](http://cfp.rust-belt-rust.com/) is open until April 30!
* [rustc/cargo can now be installed on ARM Linux, NetBSD and FreeBSD using rustup/multirust](https://users.rust-lang.org/t/psa-rustc-cargo-can-now-be-installed-on-arm-linux-netbsd-and-freebsd-using-rustup-multirust/5383).
* [Announcing cargo-apk](https://users.rust-lang.org/t/announcing-cargo-apk/5501).
* Leaf now [has a book](http://autumnai.com/leaf/book/leaf.html) that teaches you how you can build machine learning applications.
* [nix](http://kamalmarhubi.com/blog/2016/04/13/rust-nix-easier-unix-systems-programming-3/). Rust friendly bindings to \*nix APIs.
* [winapi-kmd](https://github.com/pravic/winapi-kmd-rs). Windows Kernel-Mode Drivers written in Rust.
* [rust-musl-builder](https://github.com/emk/rust-musl-builder). Docker container for easily building static Rust binaries.
* [rustw](https://github.com/nrc/rustw). A web frontend for the Rust compiler.
* [Cake](https://github.com/ticki/cake). A simple, Rustic build tool.
* [metacollect](https://llogiq.github.io/2016/04/24/nsa.html). A lint to collect some crate metadata.
* [Anima Engine](https://github.com/anima-engine/anima-engine). The quirky game engine.
* [Tera](https://blog.wearewizards.io/introducing-tera-a-template-engine-in-rust). A template engine in Rust.
* [rgo](https://github.com/yberreby/rgo). A Go compiler toolchain, written in Rust.
* [specs](https://github.com/slide-rs/specs). A rusty parallel ECS with breaking performance ([ecs_bench](https://github.com/lschmierer/ecs_bench))
* [Epaste](https://github.com/zetok/epaste). Tool to encrypt data and encode it as base64, so that it could be pasted on pastebin.

# Crate of the Week

This week's Crate of the Week is [owning_ref](https://crates.io/crates/owning_ref), which contains a reference type that can carry its owner with it. Thanks to [Diwic](https://users.rust-lang.org/users/diwic) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [easy] [servo/saltfs: buildbot steps need description and descriptionDone](https://github.com/servo/saltfs/issues/337).
* [easy] [rust: rustbuild: Add a tidy check to ensure Cargo.lock updates are checked in](https://github.com/rust-lang/rust/issues/32901).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

186 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-04-11..2016-04-25

## Notable changes

* [`pub(restricted)` (RFC 1422) implemented](https://github.com/rust-lang/rust/pull/32875)
* [Warn on type parameter defaults](https://github.com/rust-lang/rust/pull/32817), this will become an error in the future
* [RFC #1494 (amendment to #550) implemented](https://github.com/rust-lang/rust/pull/32945), allows blocks to follow types/paths in macro patterns
* [`de-`/`encode()` methods no longer break Serialization deriving](https://github.com/rust-lang/rust/pull/32908)
* [Macro hygiene bugs fixed](https://github.com/rust-lang/rust/pull/32923)
* [Avoid crashing due to duplicate external items](https://github.com/rust-lang/rust/pull/32970)
* [Fix multiple glob import](https://github.com/rust-lang/rust/pull/32814)
* [MIR debuginfo mostly works](https://github.com/rust-lang/rust/pull/32952) ([etc.](https://github.com/rust-lang/rust/pull/32803))
* [MIR Blocks no longer require END_BLOCK](https://github.com/rust-lang/rust/pull/33030)
* [MIR now has LLVM-agnostic type layout](https://github.com/rust-lang/rust/pull/32939)
* [Register duplicate item symbols anyway](https://github.com/rust-lang/rust/pull/32946)
* [Resolve compiler performance regression fixed](https://github.com/rust-lang/rust/pull/33064)
* [Syntax: Import prefixes are now paths](https://github.com/rust-lang/rust/pull/33044)
* [Don't report errors in constants at every use site](https://github.com/rust-lang/rust/pull/32877)
* [Handle over-aligned realloc failures on UNIX](https://github.com/rust-lang/rust/pull/32997)
* [String::truncate goes to greater lengths to not panic](https://github.com/rust-lang/rust/pull/32977)
* [`BinaryHeap::append(..)`](https://github.com/rust-lang/rust/pull/32987)
* [Faster `is_char_boundary()` with bit twiddling](https://github.com/rust-lang/rust/pull/32862)
* [Fixed `BufRead` overrun on `Take`](https://github.com/rust-lang/rust/pull/32855)
* [`Default` for `RwLock`, `Mutex`, `CondVar`, `CStr`, `Path`](https://github.com/rust-lang/rust/pull/32785)
* [Cargo can now use multiple git user names](https://github.com/rust-lang/cargo/pull/2584)
* [Removed the (apparently broken) `std::net::IPV6_V6ONLY` feature](https://github.com/rust-lang/rust/pull/33124)
* [Handle `DefId`s and extern crates before lowering the AST to HIR](https://github.com/rust-lang/rust/pull/33089)
* [Compiletest now uses JSON output](https://github.com/rust-lang/rust/pull/33020)
* [`VecDeque::contains(_)` and `LinkedList::contains(_)` implemented](https://github.com/rust-lang/rust/pull/32951)
* [Rust now bootstraps from previous stable instead of snapshots](https://github.com/rust-lang/rust/pull/32942)
* [`impl From<Vec<T>>` and `Into<Vec<T>>` for `VecDeque<T>`](https://github.com/rust-lang/rust/pull/32866)
* [`BTree::append(_)` implemented](https://github.com/rust-lang/rust/pull/32466)

## New Contributors

* Alec S
* Andrey Tonkih
* c4rlo
* David Hewitt
* David Tolnay
* Deepak Kannan
* Gigih Aji Ibrahim
* jocki84
* Jonathan Turner
* Kaiyin Zhong
* Lukas Kalbertodt
* Lukas Pustina
* Maxim Samburskiy
* Raph Levien
* rkjnsn
* Sander Maijers
* Szabolcs Berecz

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1543: Add more integer atomic types](https://github.com/rust-lang/rfcs/pull/1543).
* [RFC 1510: Add a new crate-type, cdylib](https://github.com/rust-lang/rfcs/pull/1510).
* [RFC 1535: Stabilize the `-C overflow-checks` command line argument](https://github.com/rust-lang/rfcs/pull/1535).
* [RFC 1440: Allow Drop types in statics/const functions](https://github.com/rust-lang/rfcs/pull/1440).
* [RFC 1399: Add `#[repr(pack = "N")]`](https://github.com/rust-lang/rfcs/pull/1399).
* [Amend RFC 1228 with operator fixity and precedence](https://github.com/rust-lang/rfcs/pull/1319).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add `#[repr(align = "N")]`](https://github.com/rust-lang/rfcs/pull/1358).
* [Proposal for thread affinity](https://github.com/rust-lang/rfcs/pull/1480).
* [Specifying that `<T as Clone>::clone(&t)` where `T: Copy` should be equivalent to `ptr::read(&t)`](https://github.com/rust-lang/rfcs/pull/1521).
* [Add workspaces to Cargo](https://github.com/rust-lang/rfcs/pull/1525).
* [Add `TryFrom` and `TryInto` traits](https://github.com/rust-lang/rfcs/pull/1542).
* [`as_millis` function on `std::time::Duration`](https://github.com/rust-lang/rfcs/pull/1547).

## New RFCs

* [`FusedIterator` marker trait and `iter::Fuse` specialization](https://github.com/rust-lang/rfcs/pull/1581).
* [Add `parse_generics!` and `parse_where!` macros](https://github.com/rust-lang/rfcs/pull/1583).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [Add a `lifetime` specifier to `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1590/files).

# Upcoming Events

* [4/26. Rust Bay Area: Securing OSes and Apps with Rust, seL4, and SGX](http://www.meetup.com/Rust-Bay-Area/events/230271083/).
* 4/27. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/4. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [5/9. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [5/13. Rust Meetup Darmstadt](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/230396961/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.
* [Software Engineer](http://www.coturnix.fr/en/#join) at Coturnix.
* [Senior full stack developer](http://onesignal.applytojob.com/apply/gpSzt4/Senior-Full-Stack-Developer) at OneSignal.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

>> Cow is still criminally underused in a lot of code bases
>
> I suggest we make a new slogan to remedy this:
> "To err is human, to moo bovine."
> (I may or may not have shamelessly stolen this from [this bug report](https://bugs.launchpad.net/ubuntu/+source/apt/+bug/56125/comments/63))

[so_you_like_donuts on reddit](https://www.reddit.com/r/rust/comments/4fc6m4/from_str_to_cow/d27rv0f).

Thanks to [killercup](https://users.rust-lang.org/users/killercup) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
