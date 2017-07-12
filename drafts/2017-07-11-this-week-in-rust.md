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

* [Rust's 2017 roadmap, six months in](https://blog.rust-lang.org/2017/07/05/Rust-Roadmap-Update.html).
* [How Rust is tested](https://brson.github.io/2017/07/10/how-rust-is-tested).
* [Shipping specialization: a story of soundness](https://aturon.github.io/blog/2017/07/08/lifetime-dispatch/).
* [RustBelt: Securing the foundations of the Rust programming language](https://www.ralfj.de/blog/2017/07/08/rustbelt.html).
* [Why you should, actually, rewrite it in Rust](https://unhandledexpression.com/2017/07/10/why-you-should-actually-rewrite-it-in-rust/).
* [Oracle releases an OCI-Based container runtime in Rust](https://thenewstack.io/oracle-opens-oci-container-runtime).
* [Speeding up crypto on Wire desktop apps](https://medium.com/@wireapp/speeding-up-crypto-on-wire-desktop-apps-3ff37fc98c3f).
* [Iterators and Streams in Rust and Haskell](https://www.fpcomplete.com/blog/2017/07/iterators-streams-rust-haskell).
* [Diesel 0.14.0 released - now supporting arbitrary number of joints and new data types](https://github.com/diesel-rs/diesel/releases/tag/v0.14.0).
* [My experience with the libz blitz](https://www.reddit.com/r/rust/comments/6lkuyd/my_experience_with_the_libz_blitz/).
* [Working on Rust Language Server for GSoC 2017](https://xanewok.github.io/gsoc/2017/working-on-rust-language-server/).
* [GSoC project: Making Redox self-hosting, status report 2](https://redox-os.org/news/gsoc-self-hosting-2/).
* [The semver trick: how to avoid complicated coordinated upgrades](https://github.com/dtolnay/semver-trick).
* [A collection of notable Rust blog posts](https://gist.github.com/brson/a324c83a6af6a8a78dfaa9d33eb9b48e).
* [ggez: A Rust library to create good games easily](http://ggez.rs/).
* [Monochord: implementing a musical tunings library](http://draft.sx/317fb399adc3c65ac0779df468aa6aa7).
* [This week in Redox 24](https://redox-os.org/news/this-week-in-redox-24/).
* [podcast] [Request for Explanation #2 - Stealing chickens on the Internet](https://request-for-explanation.github.io/podcast/ep2-stealing-chickens-on-the-internet/index.html). Discussing the [Evolving Rust through Epochs RFC](https://github.com/rust-lang/rfcs/pull/2052).

# Friends of the Forest

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. These people are
'friends of the forest'. The community team has been lax in making nominations for
this on a regular basis, but we hope to get back on track!

Our this week's friend of the forest is [Guillaume Gomez](https://github.com/GuillaumeGomez), whose [influence is evident everywhere you look in Rust](https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/23).

# Crate of the Week

This week's crate is [strum](https://crates.io/crates/cargo-make), a crate that helps you automate your build workflow beyond what cargo already offers. Thanks to [Sagie Gur Ari](https://users.rust-lang.org/u/sagiegurari) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust is in great need of volunteers interested in making a dent in some of the libz blitz crates](https://users.rust-lang.org/t/twir-call-for-participation/4821/81).
* [Rust libz blitz status update 2017/07/05](https://internals.rust-lang.org/t/rust-libz-blitz/5184/104).
* [Out of band crate evaluation for 2017-07-06: env_logger](https://internals.rust-lang.org/t/out-of-band-crate-evaluation-for-2017-07-06-env-logger/5488).
* [Out of band crate evaluation for 2017-07-06: threadpool](https://internals.rust-lang.org/t/out-of-band-crate-evaluation-for-2017-07-06-threadpool/5495).
* [Crate evaluation for 2017-07-11: gcc](https://internals.rust-lang.org/t/crate-evaluation-for-2017-07-11-gcc/5450).
* [Help wanted: parser-c (and parser-haskell, corollary, and rust-corrode)](https://users.rust-lang.org/t/help-wanted-parser-c-and-parser-haskell-corollary-and-rust-corrode/11729).
* [Get in the swing with the libz blitz contest: Win free tickets to RustFest Zürich](http://blog.rustfest.eu/libz-blitz).
* [Call for Proposals is open for RustFest Zürich](https://cfp.rustfest.eu/events/rustfest-ch).
* [rust-api-guidelines: Mention HashMap::insert and HashSet::insert under c-intermediate](https://github.com/brson/rust-api-guidelines/issues/55).
* [rust-api-guidelines: Provide easier navigation/multipage structure](https://github.com/brson/rust-api-guidelines/issues/52).
* [easy] [stdx: Add semver crate](https://github.com/brson/stdx/issues/50).
* [easy] [stdx: Add encoding_rs crate](https://github.com/brson/stdx/issues/51).
* [hard] [maud: Port to new proc-macro interface](https://github.com/lfairy/maud/issues/92).
* [sass-rs: Fix build on OSX and Windows](https://github.com/compass-rs/sass-rs/issues/12).
* [walkdir: Correct errors in WalkDir type docs](https://github.com/BurntSushi/walkdir/issues/32).
* [walkdir: Add links to other walkdir items in WalkDirIterator docs](https://github.com/BurntSushi/walkdir/issues/30).
* [walkdir: Add Error docs to methods that return Result](https://github.com/BurntSushi/walkdir/issues/24).
* [easy] [rust-url: Modify docs to put error conditions into `Errors` sections](https://github.com/servo/rust-url/issues/314).
* [easy] [flate2-rs: Rename internal types to match the public types](https://github.com/alexcrichton/flate2-rs/issues/75).
* [PumpkinDB: "builtins" files don't allow for computed constants](https://github.com/PumpkinDB/PumpkinDB/issues/318).
* [PumpkinDB: different users would use different naming conventions](https://github.com/PumpkinDB/PumpkinDB/issues/317).
* [PumpkinDB: numerous mio deprecation notices](https://github.com/PumpkinDB/PumpkinDB/issues/294).
* [PumpkinDB: lack of synchronization primitives](https://github.com/PumpkinDB/PumpkinDB/issues/115).
* [PumpkinDB: non-trivial to detect if JSON's value is an integer or a float](https://github.com/PumpkinDB/PumpkinDB/issues/296).
* [PumpkinDB: integer constants in builtins get interpreted as instructions](https://github.com/PumpkinDB/PumpkinDB/issues/314).
* [rustup: Fix 'show' displaying UNC paths on windows](https://github.com/rust-lang-nursery/rustup.rs/issues/1177).
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

* boreeas
* Kornel
* oyvindln

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2033: Experimentally add coroutines to Rust](https://github.com/rust-lang/rfcs/pull/2033).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Tweak object safety rules to allow static dispatch](https://github.com/rust-lang/rfcs/pull/2027).
* [disposition: merge] [Allow the usage of `use` inside `impl` blocks and `match` blocks](https://github.com/rust-lang/rfcs/pull/1976).
* [disposition: postpone] [Warn by default when casting a pointer to an integer smaller than usize](https://github.com/rust-lang/rfcs/pull/1782).
* [disposition: merge] [Amend #1440: allow `const` items to contain drop types](https://github.com/rust-lang/rfcs/pull/1817).
* [disposition: merge] [Add `extern type` declarations for declaring types from external libraries which have an unknown size/layout](https://github.com/rust-lang/rfcs/pull/1861).
* [disposition: merge] [Prepublication dependencies for Cargo](https://github.com/rust-lang/rfcs/pull/1969).
* [disposition: merge] [Support the `#[must_use]` attribute on arbitrary functions](https://github.com/rust-lang/rfcs/pull/1940).
* [disposition: merge] [Allow the `?` operator to be used in `main`, and in `#[test]` functions and doctests](https://github.com/rust-lang/rfcs/pull/1937).

## New RFCs

* [Make unreachable patterns error, not warning](https://github.com/rust-lang/rfcs/pull/2058).
* [Move "How do we teach this?" before "Detailed design"](https://github.com/rust-lang/rfcs/pull/2059).
* [Allow destructuring of structs that implement Drop](https://github.com/rust-lang/rfcs/pull/2061).
* [Add replace and swap functions to RefCell](https://github.com/rust-lang/rfcs/pull/2057).
* [Allow trivial constraints to appear in where clauses](https://github.com/rust-lang/rfcs/pull/2056).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

Issues in final comment period:

* [Combining openings and closings](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)

An interesting issue:

* [Define short](https://github.com/rust-lang-nursery/fmt-rfcs/issues/47)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [paths](https://github.com/rust-lang-nursery/fmt-rfcs/issues/69)
* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)
* [control flow](https://github.com/rust-lang-nursery/fmt-rfcs/issues/62)

# Upcoming Events

* [Jul 13. Columbus Rust Society - Monthly Meetup](https://www.meetup.com/columbus-rs/events/240698982/).
* [Jul 13. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jul 15. Rust Bangalore - Rust 'ORBL' workshop](https://www.meetup.com/rustox/events/240305692/).
* [Jul 19. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul 19. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jul 20. Rust DC: Lessons learned integrating Rust with Ruby](https://www.meetup.com/RustDC/events/241110467/).
* [Jul 20. Mozilla Community Dresden Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/241302860/).
* [Jul 26. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jul 26. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jul 26. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/241512566/).
* [Jul 27. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

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

> Unsafe is your friend! It's maybe not a friend like you would invite to your sister's wedding, or the christening of her first-born child. But it's sort of the friend who lives in the country and has a pick-up truck and 37 guns. And so you might not want to hang out with them all the time, but if you need something blown up he is there for you.

— [Simon Heath on game development in Rust (at 38:35 in video)](https://air.mozilla.org/game-developement-in-rust/).

Thanks to [G2P](https://users.rust-lang.org/t/twir-quote-of-the-week/328/416) and [David Tolnay](https://users.rust-lang.org/t/twir-quote-of-the-week/328/418) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
