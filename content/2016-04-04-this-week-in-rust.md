Title: This Week in Rust 125
Number: 125
Date: 2016-04-04
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
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

* [Carol Nichols joins the Community team](https://users.rust-lang.org/t/carol-nichols-is-joining-the-community-team/5190)!
* Steve Klabnik is giving a [2 day course introducing Rust at OSCON](http://conferences.oreilly.com/oscon/open-source-us/public/schedule/detail/49862).
* [Call for Proposals](http://cfp.rust-belt-rust.com/) for Rust Belt Rust Conference are now open.
* [Deploying a Rust app to Google App Engine](http://blog.jecrooks.com/posts/rust-on-appengine.html).
* [Native JavaScript classes in Neon](http://calculist.org/blog/2016/04/01/native-js-classes-in-neon/). Neon is a safe Rust abstraction layer for native Node.js modules.
* [Rust via its Core Values](http://designisrefactoring.com/2016/04/01/rust-via-its-core-values/). Couple of ideas about learning programming languages.
* [Error handling in Rust](https://facility9.com/2016/03/error-handling-in-rust/).
* [Rust borrow checker woes](http://osa1.net/posts/2016-03-28-rust-brwchk-woes.html).
* [Linear algebra in Rust](https://bluss.github.io/rust/2016/03/28/a-gemmed-rabbit-hole/).
* [This week in Redox 13](http://www.redox-os.org/news/this-week-in-redox-13/).
* [This week in Servo 57](https://blog.servo.org/2016/03/28/twis-57/).
* [Announcing the official release of the Visual C++ Build Tools 2015](https://blogs.msdn.microsoft.com/vcblog/2016/03/31/announcing-the-official-release-of-the-visual-c-build-tools-2015/). Visual C++ Build Tools are required for MSVC builds of Rust.

## Notable New Crates & Project Updates

* `rustup`, successor to `multirust` and `multirust-rs` is [now in beta](https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/50).
* [TiKV](https://github.com/pingcap/tikv). Distributed key-value database powered by Rust.
* [Steven](https://github.com/Thinkofname/steven-rust). Rust Minecraft client.
* [semantic-rs](https://fnordig.de/2016/03/29/releasing-rust-projects-the-automatic-way/). Automate your crate publishing routine.
* [EGLI](https://github.com/Nercury/egli-rs). Easy to use EGL Interface for Rust.

# Crate of the Week

This week's Crate of the Week is [rustful](https://crates.io/crates/rustful), a simple, modular REST-like HTTP framework. Thanks to [Austin B](https://users.rust-lang.org/users/DroidLogician) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rustup: Add prelimenary telemetry](https://github.com/rust-lang-nursery/multirust-rs/issues/254).
* [difficult] [rustup: Add Windows GUI / MSI installer](https://github.com/rust-lang-nursery/multirust-rs/issues/253).
* [easy] [rexiv2: Results should likely use our own aliased Error (and Result?) type](https://github.com/felixc/rexiv2/issues/16).
* [easy] [rexiv2: Provide access to full XML XMP packet](https://github.com/felixc/rexiv2/issues/14).
* [medium] [buildengine5: Test `net::test::client_server_send` fails](https://github.com/Ameliorate/buildengine5/issues/10).
* [less easy] [Vulkano: Add a memory pool](https://github.com/tomaka/vulkano/issues/14).
* [easy] [`cargo add`: Target specifications](https://github.com/killercup/cargo-edit/issues/13).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

65 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-28..2016-04-04

## Notable changes

* [specialization of `str::to_string()`](https://github.com/rust-lang/rust/pull/32586)
* [weed out unneeded dependencies within rustc](https://github.com/rust-lang/rust/pull/32571)
* [lldb breakpoints get source file names](https://github.com/rust-lang/rust/pull/32522)
* [Parser recovery is behind debug flag until kinks worked out](https://github.com/rust-lang/rust/pull/32494)
* [Parsing after EOF is now ICE](https://github.com/rust-lang/rust/pull/32479) (better than possible infinite loops)
* [melt the ICE on lowering impossible range](https://github.com/rust-lang/rust/pull/32267)
* [`const_eval` and `check_match` now live in their own crate](https://github.com/rust-lang/rust/pull/32259)
* [MIR traversals, orbit bootstraps](https://github.com/rust-lang/rust/pull/32210)
* [private fields/methods no longer interfere with selection](https://github.com/rust-lang/rust/pull/31938)
* [RefCell/RefMut coercible to unsized](https://github.com/rust-lang/rust/pull/32652)
* [Arc now `compare_exchange`s instead of `compare_and_swap`](https://github.com/rust-lang/rust/pull/32643) (should be faster on ARM)
* [`HashMap`/`HashSet` and their iterators are now covariant](https://github.com/rust-lang/rust/pull/32635)
* [`BTree`/`HashMap::values_mut()`](https://github.com/rust-lang/rust/pull/32633)

## New Contributors

* Alan Somers
* Andreas Linz
* Christian Wesselhoeft
* Christopher Serr
* David AO Lozano
* Florian Berger
* Tobias Müller
* Valentin Lorentz
* Валерий Лашманов

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1552: Add a contains method to `VecDeque` and `LinkedList`](https://github.com/rust-lang/rfcs/pull/1552).
* [Amendment to RFC 1291: Add libutil to scope of libc crate on Linux](https://github.com/rust-lang/rfcs/pull/1529).


## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add more integer atomic types](https://github.com/rust-lang/rfcs/pull/1543).
* [Stabilize implementing panics as aborts](https://github.com/rust-lang/rfcs/pull/1513).
* [Add a generic `Atomic<T>` type](https://github.com/rust-lang/rfcs/pull/1505).
* [Amend RFC 550 with misc. follow set corrections](https://github.com/rust-lang/rfcs/pull/1494).
* [Provide native support for C-compatible unions, defined via a new keyword `untagged_union`](https://github.com/rust-lang/rfcs/pull/1444).
* [Add a standard allocator interface and support for user-defined allocators](https://github.com/rust-lang/rfcs/pull/1398).
* [Remove some kinds of doc comments](https://github.com/rust-lang/rfcs/pull/1373).
* [Amend RFC 1228 with operator fixity and precedence](https://github.com/rust-lang/rfcs/pull/1319).

## New RFCs

* [😱 ⛔️ ❗️ 🙅 ✅ 😎](https://github.com/rust-lang/rfcs/pull/1565).
* [Amendment to RFC 1291: Add `errno()` and `set_errno()` to `libc` scope](https://github.com/rust-lang/rfcs/pull/1571).
* [Standardise stream wrappers like compression, encryption and etc](https://github.com/rust-lang/rfcs/pull/1568).
* [Normalization for long error codes explanations](https://github.com/rust-lang/rfcs/pull/1567).
* [Procedural macros](https://github.com/rust-lang/rfcs/pull/1566).
* [Add function overloading by using pattern matching](https://github.com/rust-lang/rfcs/pull/1564).
* [Add `#![cfg_assert]`](https://github.com/rust-lang/rfcs/pull/1563).
* [Macro naming and modularisation](https://github.com/rust-lang/rfcs/pull/1561).
* [Some internal and language-level changes to name resolution](https://github.com/rust-lang/rfcs/pull/1560).

# Upcoming Events

* [4/6. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [4/6. Germany/Cologne - Hack and Learn](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/229919455/).
* [4/6. Rust São Paulo Meetup](http://www.meetup.com/Rust-Sao-Paulo-Meetup/events/229377422/).
* [4/11. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [4/13. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [4/14. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [4/15. Frankfurt/Main Rust Lint Workshop](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/229564640/?eventId=229564640)
* [4/18. Rust Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Senior full stack developer](http://onesignal.applytojob.com/apply/gpSzt4/Senior-Full-Stack-Developer) at OneSignal.
* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.
* [Rust developer](http://rust.jobboard.io/jobs/125594-rust-developer-at-the-blackbird) at The Blackbird.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Explicitness is the fourth core value of Rust. Ironically, I don’t see that “Explicitness” is ever explicitly stated as a goal of Rust.

— [Ian Whitney in a blog post](http://designisrefactoring.com/2016/04/01/rust-via-its-core-values/).

Thanks to [nayru25](https://users.rust-lang.org/users/nayru25) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
