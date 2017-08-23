Title: This Week in Rust 196
Number: 196
Date: 2017-08-22
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

* [Announcing Gotham - a flexible web framework that does not sacrifice safety, security or speed.](https://gotham.rs/blog).
* [What the RLS can do for Rust support in IDEs](https://www.ncameron.org/blog/what-the-rls-can-do/).
* [Optimizing Rust](https://blogs.gentoo.org/lu_zero/2017/08/12/optimizing-rust/).
* [Rust for the web](https://thefullsnack.com/en/rust-for-the-web.html).
* [Setting up a Rust environment on Windows](https://fungos.github.io/blog/2017/08/12/setting-up-a-rust-environment-on-windows/).
* [Of boxes and trees - smart pointers in Rust](https://matthias-endler.de/2017/boxes-and-trees/).
* [Rustls and Servo](https://simrangujral.github.io/Rustls&Servo).
* [My experience contributing to Servo](http://brainlessdeveloper.com/2017/08/12/my-experience-contributing-to-servo/).
* [Debugging a race condition in a release target](http://blog.boxofrox.me/2017/08/debugging-a-rust-release-build.html).
* [Designing a channel](https://stjepang.github.io/2017/08/13/designing-a-channel.html).
* [Types as contracts: Implementation and evaluation](https://www.ralfj.de/blog/2017/08/11/types-as-contracts-evaluation.html).
* [Exposing a Rust library to C](http://greyblake.com/blog/2017/08/10/exposing-rust-library-to-c/).
* [Think local, act local in Rust](https://llogiq.github.io/2017/08/14/local.html).
* [Announcing Rusty Object Notation](https://kvark.github.io/format/data/json/2017/08/09/rusty-object-notation.html).
* [Implementing a bot for Slack in Rust, Rocket and Anterofit - Part 2](https://abishov.com/2017/08/08/hexocat-bot-part-2.html).
* [Evolution of a simple `du -s` clone](https://durka.github.io/blog/2017/08/06/du-evolution.html).
* [REST calls made rustic: RS-ES in idiomatic Rust tutorial](https://qbox.io/blog/elasticsearch-rest-client-idiomatic-rust-tutorial).
* [User-friendly Elasticsearch queries with Rust and Elastic](https://qbox.io/blog/user-friendly-elasticsearch-queries-with-rust-and-elastic).
* [This week in Rust docs 68](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-68).
* [These weeks in dev-tools 1](https://www.ncameron.org/blog/these-weeks-in-dev-tools-issue-1/).
* [This week in Redox 28](https://redox-os.org/news/this-week-in-redox-28/).
* [Annoucnement - try out Rust IDE support in Visual Studio Code](https://users.rust-lang.org/t/try-out-rust-ide-support-in-visual-studio-code/12407)

# Crate of the Week

This week's crate is [pest](https://crates.io/crates/pest), a PEG-based parsing library. Thanks to [Laurent Wandrebeck](https://users.rust-lang.org/u/lwandrebeck) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Pleco.rs - a chess engine in Rust is looking for contributors](https://www.reddit.com/r/rust/comments/6tpc3b/lets_create_a_rustbased_chess_engine_a_call_for/).
* [easy] [bindgen: Rename `TypeKind::Named` to `TypeKind::TypeParam`](https://github.com/rust-lang-nursery/rust-bindgen/issues/914).
* [less easy] [bindgen: Emitting or deriving trait implementations](https://github.com/rust-lang-nursery/rust-bindgen/issues/886).
* [less easy] [bindgen: Emit a "manual" implementation of Debug when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/875).
* [less easy] [bindgen: "manually" implement Hash when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/877).
* [less easy] [bindgen: "manually" implement PartialEq when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/879).
* [less easy] [bindgen: Derive `Eq` when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/880).
* [less easy] [bindgen: "manually" implement Eq when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/881).
* [less easy] [bindgen: Derive PartialOrd when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/882).
* [less easy] [bindgen: "manually" implement PartialOrd when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/883).
* [less easy] [bindgen: Derive Ord when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/884).
* [less easy] [bindgen: "manually" implement Ord when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/885).
* [PumpkinDB: Rust nightly after 2017-06-20 affects benchmarks negatively](https://github.com/PumpkinDB/PumpkinDB/issues/345). ([Discuss here](https://gitter.im/PumpkinDB/Lobby)).
* [wayland-window: Add control buttons](https://github.com/Smithay/wayland-window/issues/4).
* [wayland-window: Make borders prettier](https://github.com/Smithay/wayland-window/issues/19).
* [doc] [lyon: API guidelines: methods on collections that produce iterators follow the iter, iter_mut, into_iter conventions](https://github.com/nical/lyon/issues/86). Lyon is a GPU-based 2D graphics rendering engine in Rust.
* [doc] [lyon: API guidelines: ad-hoc conversions follow as_, to_, into_ conventions](https://github.com/nical/lyon/issues/85).
* [doc] [lyon: API guidelines: iterator type names should match the methods that produce them](https://github.com/nical/lyon/issues/87).
* [medium] [lyon: Implement clipping line joins at the miter limit](https://github.com/nical/lyon/issues/35).
* [easy] [ggez: Input doesn't work on mac using tmux and iterm2](https://github.com/ggez/ggez/issues/30). ggez is a Rust library to create good games easily.
* [easy] [ggez: SDL controller input doesn't work](https://github.com/ggez/ggez/issues/35).
* [doc] [ggez: Finish full building-and-install docs for each platform](https://github.com/ggez/ggez/issues/118).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

99 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-08-14..2017-08-21

* [forbid non-standard literal patterns](https://github.com/rust-lang/rust/pull/43842)
* [cleanup for LLVM-less build, second attempt](https://github.com/rust-lang/rust/pull/43842)
* [stabilize rvalue promotion to `'static`](https://github.com/rust-lang/rust/pull/43838)
* [implement `CompilerDesugaringKind`](https://github.com/rust-lang/rust/pull/43832) (was stringly typed before)
* [fix span miscalculation in `save-analysis`](https://github.com/rust-lang/rust/pull/43826)
* [fix ICE with elided lifetimes in foreign function return types](https://github.com/rust-lang/rust/pull/43651)
* [`RefCell::`{`swap`, `replace`}](https://github.com/rust-lang/rust/pull/43574)
* [`String::retain`](https://github.com/rust-lang/rust/pull/43500)
* [`Vec::drain_filter`](https://github.com/rust-lang/rust/pull/43245)
* [MIR borrowck](https://github.com/rust-lang/rust/pull/43108)
* [rerun MIR passes on promoted temporaries](https://github.com/rust-lang/rust/pull/43902)
* [everybody loops🎶 but `impl Trait`](https://github.com/rust-lang/rust/pull/43878)
* [redox now has unwinding panics](https://github.com/rust-lang/rust/pull/43917)
* [ship the rustdoc book](https://github.com/rust-lang/rust/pull/43863)
* [crates.io now shows the README.md on crate pages](https://github.com/rust-lang/crates.io/pull/869)

## New Contributors

* Alexey Tarasov
* arshiamufti
* Foucher
* Justin Browne
* Natalie Boehm
* nicole mazzuca
* Owen Sanchez
* Ryan Leckey
* Tej Chajed
* Thomas Levy

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Evolving Rust through checkpoints](https://github.com/rust-lang/rfcs/pull/2052).
* [disposition: close] [Zero-Sized References](https://github.com/rust-lang/rfcs/pull/2040).
* [disposition: merge] [Generic associated types (associated type constructors)](https://github.com/rust-lang/rfcs/pull/1598).
* [disposition: close] [Allow use of pipe operator in patterns](https://github.com/rust-lang/rfcs/pull/1882).
* [disposition: merge] [Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [disposition: merge] [Unsafe pointer methods](https://github.com/rust-lang/rfcs/pull/1966).
* [disposition: merge] [Amend RFC 1242 to require an RFC for deprecation of crates from the nursery](https://github.com/rust-lang/rfcs/pull/1983).
* [disposition: merge] [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [disposition: close] [Match branch semicolon](https://github.com/rust-lang/rfcs/pull/1994).
* [disposition: merge] [Future-proofing enums/structs with `#[non_exhaustive]` attribute](https://github.com/rust-lang/rfcs/pull/2008).
* [disposition: merge] [Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).

## New RFCs

* [Module redesign](https://github.com/rust-lang/rfcs/pull/2108).
* [Ok wrapping: Improved support for writing code from an error handling mindset](https://github.com/rust-lang/rfcs/pull/2107).
* [Introduce more potentially useful environment-related functions to `std::env`](https://github.com/rust-lang/rfcs/pull/2101).
* [Unnamed fields of struct and union type](https://github.com/rust-lang/rfcs/pull/2102).
* [Attributes for tools, 2.0](https://github.com/rust-lang/rfcs/pull/2103).
* [Rand crate revision (pre-stabilisation)](https://github.com/rust-lang/rfcs/pull/2106). Evaluate options for the future of `rand` regarding both cryptographic and non-cryptographic uses.

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

Nothing of note this week.


# Upcoming Events

* **[Aug 18-19. RustConf 2017](http://rustconf.com/)**.
* [Aug 19. Rust Bangalore - Rust "Traits" Workshop](https://www.meetup.com/rustox/events/240321748/).
* [Aug 23. GOTO Night Berlin: Modern low level - Rust in 2017](https://www.meetup.com/GOTO-Nights-Berlin/events/241544422/).
* [Aug 23. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/242277432/).
* [Aug 23. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug 23. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Aug 24. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Aug 30. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug 30. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Aug 31. Rust NYC - Come learn about Rust](https://www.meetup.com/Rust-NYC/events/241866546/).
* [Aug 31. Rust London - Rust learning and hacking evening #2](https://www.meetup.com/Rust-London-User-Group/events/242378000/).
* [Aug 31. Cambridge Rust Meetup - Rust Study Group](https://www.meetup.com/Cambridge-Rust-Meetup/events/242409356/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> once you can walk barefoot (C), it’s easy to learn to walk with shoes (go) but it will take time to learn to ride a bike (rust)

— [/u/freakhill on Reddit](https://www.reddit.com/r/rust/comments/6srf8h/thoughts_from_a_dumb_person_notes_on_my_threeweek/dlf58jt/).

Thanks to [Rushmore](https://users.rust-lang.org/t/twir-quote-of-the-week/328/431) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
