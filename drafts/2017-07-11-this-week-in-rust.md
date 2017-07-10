Title: This Week in Rust 190
Number: 190
Date: 2017-07-11
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


# Crate of the Week

This week's crate is [strum](https://crates.io/crates/cargo-make), a crate that helps you automate your build workflow beyond what cargo already offers. Thanks to [Sagie Gur Ari](https://users.rust-lang.org/u/sagiegurari) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Proposals is open for RustFest Zürich](https://cfp.rustfest.eu/events/rustfest-ch).
* [rust-api-guidelines: Mention HashMap::insert and HashSet::insert under c-intermediate](https://github.com/brson/rust-api-guidelines/issues/55).
* [rust-api-guidelines: Provide easier navigation/multipage structure](https://github.com/brson/rust-api-guidelines/issues/52).
* [PumpkinDB: "builtins" files don't allow for computed constants](https://github.com/PumpkinDB/PumpkinDB/issues/318).
* [PumpkinDB: different users would use different naming conventions](https://github.com/PumpkinDB/PumpkinDB/issues/317).
* [PumpkinDB: numerous mio deprecation notices](https://github.com/PumpkinDB/PumpkinDB/issues/294).
* [PumpkinDB: lack of synchronization primitives](https://github.com/PumpkinDB/PumpkinDB/issues/115).
* [PumpkinDB: non-trivial to detect if JSON's value is an integer or a float](https://github.com/PumpkinDB/PumpkinDB/issues/296).
* [PumpkinDB: integer constants in builtins get interpreted as instructions](https://github.com/PumpkinDB/PumpkinDB/issues/314).
* [rustup: Fix 'show' displaying UNC paths on windows](https://github.com/rust-lang-nursery/rustup.rs/issues/1177).
* [walkdir: Add Error docs to methods that return Result](https://github.com/BurntSushi/walkdir/issues/24).
* [walkdir: Link references to std in docs](https://github.com/BurntSushi/walkdir/issues/39).
* [walkdir: Correct errors in WalkDir type docs](https://github.com/BurntSushi/walkdir/issues/32).
* [walkdir: Document that `Iter` and `IterFilterEntry` are the result of trait methods](https://github.com/BurntSushi/walkdir/issues/31).
* [walkdir: Add links to other walkdir items in WalkDirIterator docs](https://github.com/BurntSushi/walkdir/issues/30).
* [easy] [rust-bindgen: Default to generating constified enums, rather than generating Rust enums](https://github.com/servo/rust-bindgen/issues/758).
* [less-easy] [rust-bindgen: Rewrite `is_unsized` as either a graph traversal or fix-point analysis](https://github.com/servo/rust-bindgen/issues/768).
* [less-easy] [rust-bindgen: Rewrite `can_derive_debug` as either a graph traversal or fix-point analysis](https://github.com/servo/rust-bindgen/issues/767).
* [less-easy] [rust-bindgen: Rewrite `can_derive_copy[_in_array]` as either a graph traversal or fix-point analysis](https://github.com/servo/rust-bindgen/issues/766).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

109 pull requests were [merged in the last week][merged]

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-06-26..2017-07-03

* [1.19 stabilizations](https://github.com/rust-lang/rust/pull/42745)
* [stabilize `sort_unstable`](https://github.com/rust-lang/rust/pull/43010)
* [stabilize some IO `into_inner()` methods](https://github.com/rust-lang/rust/pull/43002)
* [update LLVM to 4.0.1](https://github.com/rust-lang/rust/pull/42930)
* [LLVM code now contains demangled `fn` names as comments](https://github.com/rust-lang/rust/pull/42971) (hooray for readability!)
* [fix windows32 stack probes](https://github.com/rust-lang/llvm/pull/89)
* [split signatures off function items](https://github.com/rust-lang/rust/pull/42417) (plugin-breaking change)
* [issue lint-by-default/by-setting notes only once per lint](https://github.com/rust-lang/rust/pull/42919) (yay for reduced clutter!)
* [MIR dataflow](https://github.com/rust-lang/rust/pull/42924) (another step to MIR-borrowck)
* [`$crate` is a keyword](https://github.com/rust-lang/rust/pull/42902)
* [fix `alloc::alloc_one`](https://github.com/rust-lang/rust/pull/42901)
* [activate jemalloc fill](https://github.com/rust-lang/rust/pull/42900)
* [speed up `slice::rotate`](https://github.com/rust-lang/rust/pull/42819) (same trick as `mem::swap`)
* [`iterator::for_each`](https://github.com/rust-lang/rust/pull/42782) (faster than a `for` loop for complex iterators)
* [detect missing `;` on unit-returning methods](https://github.com/rust-lang/rust/pull/42850) (huzzah for better error messages!)
* [report the total number of errors on compilation failure](https://github.com/rust-lang/rust/pull/43015) (about time)
* [coerce fields to the correct type](https://github.com/rust-lang/rust/pull/42807)
* [don't hash single-variant enum discriminant](https://github.com/rust-lang/rust/pull/42709)
* [correct sign handling for NaNs](https://github.com/rust-lang/rust/pull/42431)
* [`#[allow_fail]` attributes for tests that run, but may fail](https://github.com/rust-lang/rust/pull/42219)
* [cargo now infers multi-file binaries by convention](https://github.com/rust-lang/cargo/pull/4214)
* [cargo can now install specific versions](https://github.com/rust-lang/cargo/pull/4229)
* [crates.io now allows multiple API tokens per user](https://github.com/rust-lang/crates.io/pull/697)
* [crates.io images are now SVG](https://github.com/rust-lang/crates.io/pull/826)

## New Contributors

* Lee Bousfield
* Milton Mazzarri

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1857: Stabilize drop order](https://github.com/rust-lang/rfcs/pull/1857).
* [RFC 1985: Tiered browser support policy for Rust's web content](https://github.com/rust-lang/rfcs/pull/1985).
* [RFC 1758: Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [RFC 1789: Conversions from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rfcs/pull/1789).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Experimentally add coroutines to Rust](https://github.com/rust-lang/rfcs/pull/2033).
* [disposition: merge] [Amend #1440: allow `const` items to contain drop types](https://github.com/rust-lang/rfcs/pull/1817).
* [disposition: merge] [Prepublication dependencies for Cargo](https://github.com/rust-lang/rfcs/pull/1969).

## New RFCs

* [Evolving Rust through Epochs](https://github.com/rust-lang/rfcs/pull/2052).
* [Add alternative to Conservative Impl Trait (RFC 1522)](https://github.com/rust-lang/rfcs/pull/2051).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

An interesting issue:

* [Define short](https://github.com/rust-lang-nursery/fmt-rfcs/issues/47)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [paths](https://github.com/rust-lang-nursery/fmt-rfcs/issues/69)
* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)
* [control flow](https://github.com/rust-lang-nursery/fmt-rfcs/issues/62)

# Upcoming Events

* [Jul  7. Rust Toronto - Game Development in Rust](https://www.meetup.com/Rust-Toronto/events/240585994/).
* [Jul 10. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/240751286/).
* [Jul 12. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/241046172/).
* [Jul 12. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul 12. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jul 13. Columbus Rust Society - Monthly Meetup](https://www.meetup.com/columbus-rs/events/240698982/).
* [Jul 13. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jul 19. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul 19. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jul 20. Lessons learned integrating Rust with Ruby](https://www.meetup.com/RustDC/events/241110467/).
* [Jul 20. Mozilla Community Dresden Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/241302860/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer, Systems - Buoyant](https://jobs.lever.co/buoyant/7a64f7d1-6fea-40b1-ba52-5ab44802c5f6).
* [Rust Software Engineer - Remote working available](http://www.headresourcing.com/software-engineer-c-c-rust-remote-working-available-bbbh28438-1496919540).
* [Senior Research Engineer - Servo at Mozilla](https://careers.mozilla.org/position/gh/727971).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I have rewritten
> the code
> that was formerly
> in c
>
> And which
> you probably had
> written very well
>
> Forgive me
> it was unsafe

— [@horse_rust on Twitter](https://twitter.com/horse_rust/status/881936300261101568).

Thanks to [@balrogboogie](https://twitter.com/balrogboogie/status/882061844436963329) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
