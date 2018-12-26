Title: This Week in Rust 266
Number: 266
Date: 2018-12-25
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

* 🎈🎉 [Announcing Rust 1.31.1](https://blog.rust-lang.org/2018/12/20/Rust-1.31.1.html). 🎉🎈
* [Procedural macros in Rust 2018](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html).
* [Tokio: A great 2018, an even better 2019](https://tokio.rs/blog/2018-12-recap-2018/).
* [Using C libraries un Rust: making a `*-sys` crate](https://kornel.ski/rust-sys-crate).
* [Rust asynchronous IO: from mio to coroutine](https://github.com/Hexilee/async-io-demo).
* [Methods for array initialization in Rust](https://www.joshmcguigan.com/blog/array-initialization-rust/).
* [Currying in rust Part 3 (The circle of life... aka why borrowchecker... why)](https://hashnode.com/post/currying-in-rust-part-3-the-circle-of-life-aka-why-borrowchecker-why-cjq3z1dd800dknds1sls4dqav)!?
* [How to get better at Rust: For beginners](https://hashnode.com/post/how-to-become-a-rust-super-developer-cjpv1ee7e000buhs2aqrdw2ym).

### #Rust2019

Find all #Rust2019 posts at [Read Rust](https://readrust.net/rust-2019/).

# Crate of the Week

This week's crate is [sandspiel](https://sandspiel.info), a WASM-powered online sandbox automaton game. Thanks to [Vikrant Chaudhary](https://users.rust-lang.org/t/crate-of-the-week/2704/473) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [A call for Rust 2019 Roadmap blog posts](https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html).
* [PEACE: Implement loading functions from static linked libraries](https://github.com/playXE/PEACE/issues/1). PEACE is a simple JIT library.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

214 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-12-17..2018-12-24

* [add targets thumbv7neon-linux-androideabi and thumbv7neon-unknown-linux-gnueabihf](https://github.com/rust-lang/rust/pull/56947)
* [less conservative uninhabitedness check](https://github.com/rust-lang/rust/pull/54125)
* [remove "visited" set from inhabitedness checking](https://github.com/rust-lang/rust/pull/57033)
* [short-circuit DefIdForest::intersection()](https://github.com/rust-lang/rust/pull/57060)
* [make the 'a lifetime on TyCtxt useless](https://github.com/rust-lang/rust/pull/56601)
* [trigger unsized coercions keyed on Sized bounds](https://github.com/rust-lang/rust/pull/56219)
* [fix various aspects around `let` bindings inside const functions](https://github.com/rust-lang/rust/pull/56160)
* [deny intra-doc link resolution failures in libstd](https://github.com/rust-lang/rust/pull/56941)
* [process nested obligations in autoderef](https://github.com/rust-lang/rust/pull/54252)
* [mark tuple structs as live if their constructors are used](https://github.com/rust-lang/rust/pull/56953)
* [fix alignment for array indexing](https://github.com/rust-lang/rust/pull/57053)
* [enable emission of alignment attrs for pointer params](https://github.com/rust-lang/rust/pull/57021)
* [enum type instead of variant suggestion unification](https://github.com/rust-lang/rust/pull/56188)
* [make basic CTFE tracing available on release builds](https://github.com/rust-lang/rust/pull/56973)
* [remove `TokenStream::JointTree`](https://github.com/rust-lang/rust/pull/56964)
* [miri: allocation is infallible](https://github.com/rust-lang/rust/pull/56981)
* [fix mutable references in `static mut`](https://github.com/rust-lang/rust/pull/56916)
* [simplify MIR generation for logical operations](https://github.com/rust-lang/rust/pull/56917)
* [static eval: do not ICE on layout size overflow](https://github.com/rust-lang/rust/pull/56909)
* [disable field reordering for repr(int)](https://github.com/rust-lang/rust/pull/56887)
* [always run rustc in a thread](https://github.com/rust-lang/rust/pull/56813)
* [version-gate the trailing semicolon change of return statements inside a match arm](https://github.com/rust-lang/rustfmt/pull/3250)
* [add `DoubleEndedIterator::nth_back`](https://github.com/rust-lang/rust/pull/56802)
* [mir-opt: make `SimplifyCfg` collapse goto chains starting from `bb0`](https://github.com/rust-lang/rust/pull/56764)
* [treat ref-to-raw cast like a reborrow: do a special kind of retag](https://github.com/rust-lang/rust/pull/56741)
* [MIR borrowck doesn't accept the example of iterating and updating a mutable reference](https://github.com/rust-lang/rust/pull/56649)
* [rework treatment of `$crate` in procedural macros](https://github.com/rust-lang/rust/pull/56647)
* [tweak query code for performance](https://github.com/rust-lang/rust/pull/56613)
* [implement `Eq`, `PartialEq` and `Hash` for `atomic::Ordering`](https://github.com/rust-lang/rust/pull/56881)
* [add unstable `VecDeque::rotate_`{`left`, `right`}](https://github.com/rust-lang/rust/pull/56842)
* [remove Cycle::try_fold override](https://github.com/rust-lang/rust/pull/56904)
* [short-circuit `Rc`/`Arc` equality checking on equal pointers where `T: Eq`](https://github.com/rust-lang/rust/pull/56550)
* [stabilize `Rc`, `Arc` and `Pin` as method receivers](https://github.com/rust-lang/rust/pull/56805)
* [stabilize `min_const_unsafe_fn` in 1.33](https://github.com/rust-lang/rust/pull/57067)
* [stabilize `Vec(Deque)::resize_with`](https://github.com/rust-lang/rust/pull/57002)
* [stabilize `Pin`](https://github.com/rust-lang/rust/pull/56939)
* [stabilize `underscore_imports`](https://github.com/rust-lang/rust/pull/56303)
* [bootstrap: Link LLVM as a dylib with ThinLTO](https://github.com/rust-lang/rust/pull/56944)
* [profiler: simplify total_duration, improve readability](https://github.com/rust-lang/rust/pull/56918)
* [cargo: warn on unused patches](https://github.com/rust-lang/cargo/pull/6470)
* [rustdoc: add new CLI flag to load static files from a different location](https://github.com/rust-lang/rust/pull/57011)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Make the getter for NonZero types into a const fn](https://github.com/rust-lang/rust/pull/56739).

## New RFCs

* [Using enums like traits](https://github.com/rust-lang/rfcs/pull/2618).
* [Local `loop` bindings](https://github.com/rust-lang/rfcs/pull/2617).

# Upcoming Events

### Online

* [Jan 2. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jan 9. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Europe

* [Dec 27 - 30. Leipzig, DE - Rust assembly at 35c3](https://users.rust-lang.org/t/35c3-rust-assembly-at-ccc-leipzig/22288).
* [Jan 8. Rapperswil-Jona, CH - Rust Zürichsee meetup at Coredump - Looking for a speaker](https://www.meetup.com/Rust-Zurich/events/253608548/).
* [Jan 9. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzcbmb/).
* [Jan 10. Brno, CZ - Rust meetup at Masaryk University](https://rust-brno.github.io/).

### North America

* [Dec 30. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbnc/).
* [Jan  2. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/246726699/).
* [Jan  2. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyzcbdb/).
* [Jan  6. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbjb/).
* [Jan  9. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyzcbmb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at XAIN, Berlin, DE](https://www.linkedin.com/jobs/cap/view/948142464/?pathWildcard=948142464&trk=mcm).
* [Networking Engineer at MaidSafe, Ayr, GB (Remote)](https://maidsafe.net/careers/#networking_engineer).
* [Senior Backend/Blockchain Developer with Rust at BitFinex, Remote](https://bitfinex.recruitee.com/o/senior-backendblockchain-developer-with-rust-remote).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Sr. Software Engineer - Rust at Mersive, Denver, US](https://www.mersive.com/company/join-mersive-team/?gh_jid=4136286002).
* [Embedded operating system developer, Karlsruhe, DE](https://www.pse.kit.edu/karriere/joboffer.php?id=2093&language=en).
* [Student research assistant (embedded), Karlsruhe, DE](https://twitter.com/oli_obk/status/1064856324071178240).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Using (traits) for Inheritance was like putting car wheels on a boat because I am used to driving a vehicle with wheels.

– Marco Alka [on Hashnode](https://hashnode.com/post/how-to-become-a-rust-super-developer-cjpv1ee7e000buhs2aqrdw2ym)

Thanks to [oberien](https://users.rust-lang.org/t/twir-quote-of-the-week/328/590) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/a9nw7t/this_week_in_rust_266/).</small>
