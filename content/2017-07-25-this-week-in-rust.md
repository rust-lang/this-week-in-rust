Title: This Week in Rust 192
Number: 192
Date: 2017-07-25
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

* 🎈🎉 [Announcing Rust 1.19](https://blog.rust-lang.org/2017/07/20/Rust-1.19.html). 🎉🎈
* [Towards a second edition of the compiler](https://internals.rust-lang.org/t/towards-a-second-edition-of-the-compiler/5582).
* [Introducing PyO3 - Rust bindings for the Python interpreter](https://www.reddit.com/r/rust/comments/6p3rjp/pyo3_python_rust_binding/).
* [Introducing Tarpaulin - a code coverage tool for Rust](https://xd009642.github.io/2017/07/20/introducing-tarpaulin.html).
* [Measuring test coverage of Rust libraries](https://jbp.io/2017/07/19/measuring-test-coverage-of-rust-programs).
* [Using Rocket + error_chain for REST APIs in Rust](https://jamesmunns.com/update/2017/07/22/rocket-plus-error-chain.html).
* [Gfx-rs - the new low-level core](https://gfx-rs.github.io/2017/07/24/low-level.html).
* [Supporting workspaces in RLS](https://xanewok.github.io/gsoc/2017/supporting-workspaces-in-rls/).
* [Making Redox self-hosting, status report 3](https://redox-os.org/news/gsoc-self-hosting-3/).
* [Making of RustDay Mexico City 2017, a RustBridge day](http://blog.community.rs/2017/07/23/rustdaymx-2017.html).
* [Rain of Rust – how we did it](http://www.rowdymehul.com/rain-of-rust-how-we-did-it/)?
* [This week in Rust docs 65](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-65).
* [This week in Redox 26](https://redox-os.org/news/this-week-in-redox-26/).

# Crate of the Week

This week's crate is [cute](https://crates.io/crates/cute), a crate containing a macro to allow Python (or Haskell) style
comprehensions (e.g. `c![x / 2, for x in 0..10, if (x & 1) == 0]`). Thanks to [Willi Kappler](https://users.rust-lang.org/u/willi_kappler)
for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Proposals is open for Rust Belt Rust 2017 until 7 August](http://cfp.rust-belt-rust.com/).
* [Get in the swing with the libz blitz contest: Win free tickets to RustFest Zürich](http://blog.rustfest.eu/libz-blitz).
* [easy] [rust-unic: Expand components/ucd/tests/category_tests.rs](https://github.com/behnam/rust-unic/issues/43).
* [easy] [rust-bindgen: Stop passing `whitelisted_items` as an argument to the codegen functions, and use ctx.codegen_items() instead](https://github.com/servo/rust-bindgen/issues/838).
* [easy] [rust-bindgen: Document bitfield usage in the users guide](https://github.com/servo/rust-bindgen/issues/818).
* [easy] [rust-bindgen: Start emitting unions in stable mode](https://github.com/servo/rust-bindgen/issues/832).
* [rust-corrode: Port Map, IntMap, Set, and IntSet methods from Haskell](https://github.com/tcr/rust-corrode/issues/2).
* [ion: Better builtin help documentation](https://github.com/redox-os/ion/issues/416).
* [ion: List of builtins to implement](https://github.com/redox-os/ion/issues/409).
* [ion: List of methods in Ion](https://github.com/redox-os/ion/issues/441).
* [ion: Possible algebraic data types / enum support](https://github.com/redox-os/ion/issues/439).
* [ion: Prompt configuration](https://github.com/redox-os/ion/issues/423).
* [ion: Builtin argument parser (ie: getopt)](https://github.com/redox-os/ion/issues/361).
* [ion: More sophisticated matching mechanisms](https://github.com/redox-os/ion/issues/358).
* [ion: Associative arrays (HashMaps)](https://github.com/redox-os/ion/issues/246).
* [ion: Syntax discussion](https://github.com/redox-os/ion/issues/329).
* [ion: Functions and variable scopes](https://github.com/redox-os/ion/issues/328).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

110 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-07-17..2017-07-24

* [cloneable `!`](https://github.com/rust-lang/rust/pull/43406)
* [LLVM 5.0 update (work in progress)](https://github.com/rust-lang/rust/pull/43387)
* [RELRO support](https://github.com/rust-lang/rust/pull/43170) (hardening technique)
* [compiler buildins abort on panic](https://github.com/rust-lang/rust/pull/43258)
* [fix macro spans](https://github.com/rust-lang/rust/pull/43352)
* [match pattern range cover check fixed](https://github.com/rust-lang/rust/pull/43266)
* [repr alignment for unions](https://github.com/rust-lang/rust/pull/43274)
* [support generic lifetime args in method calls](https://github.com/rust-lang/rust/pull/42492)
* [allow linking of Address Sanitizer to dylibs/cdylibs](https://github.com/rust-lang/rust/pull/42711)
* [`const fn` {`size_of`, `align_of`}`(_)`](https://github.com/rust-lang/rust/pull/42859)
* [reduced `#[inline(always)]` annotations](https://github.com/rust-lang/rust/pull/43367) (a good reminder not to overuse them)
* [JSON error byte positions now always start at the top of the file](https://github.com/rust-lang/rust/pull/42973)
* [rustbuild is now more eager to build our rust](https://github.com/rust-lang/rust/pull/43059)
* [some more notes changed to suggestions](https://github.com/rust-lang/rust/pull/42033)
* [improved error when mistyping `;` as `:`](https://github.com/rust-lang/rust/pull/43096)
* [more compact message for multiple unused args](https://github.com/rust-lang/rust/pull/43323)
* [cargo now uses .gitignore-like pattern matching for files/paths](https://github.com/rust-lang/cargo/pull/4270)

## New Contributors

* Bruce Mitchener
* Evan Cameron
* Jacques-Henri Jourdan
* Joe Ranweiler
* Others
* Perry Fraser

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2044: Put the RFCs repo under license terms](https://github.com/rust-lang/rfcs/pull/2044).
* [RFC 1946: Intra Rustdoc Links](https://github.com/rust-lang/rfcs/pull/1946).
* [RFC 1861: Add `extern type` declarations for declaring types from external libraries which have an unknown size/layout](https://github.com/rust-lang/rfcs/pull/1861).
* [Amend #1440: allow `const` items to contain drop types](https://github.com/rust-lang/rfcs/pull/1817).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] ["Guide-level" and "Reference-level" explanations to replace how we teach and detailed design sections](https://github.com/rust-lang/rfcs/pull/2059).
* [disposition: merge] [Add replace and swap functions to RefCell](https://github.com/rust-lang/rfcs/pull/2057).
* [disposition: merge] [Generic associated types (associated type constructors)](https://github.com/rust-lang/rfcs/pull/1598).
* [disposition: postpone] [Eager expansion of macros](https://github.com/rust-lang/rfcs/pull/1628).
* [disposition: merge] [Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [disposition: merge] [Future-proofing enums/structs with `#[non_exhaustive]` attribute](https://github.com/rust-lang/rfcs/pull/2008).
* [disposition: merge] [Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).
* [disposition: merge] [Tweak object safety rules to allow static dispatch](https://github.com/rust-lang/rfcs/pull/2027).

## New RFCs

* [stable mechanism to specify the behavior of panic! in no-std applications](https://github.com/rust-lang/rfcs/pull/2070).
* [Add impl Trait type alias and variable declarations](https://github.com/rust-lang/rfcs/pull/2071).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

Currently being discussed:

* [Define short](https://github.com/rust-lang-nursery/fmt-rfcs/issues/47)
* [Special casing some macros](https://github.com/rust-lang-nursery/fmt-rfcs/issues/86)


# Upcoming Events

* [Jul 27. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Aug  2. Rust User Group Cologne - Crate Polishing](http://rust.cologne/2017/08/02/crate-polishing.html).
* [Aug  2. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug  2. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Aug  7. Rust Belt Rust 2017 CFP deadline](http://cfp.rust-belt-rust.com/).
* [Aug  9. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug  9. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Aug  9. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywlbmb/).
* [Aug 10. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Aug 10. Columbus Rust Society - Monthly meetup](https://www.meetup.com/columbus-rs/events/czcwhlywlbnb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust developer at SparkTG India](https://twitter.com/by1x/status/887653738252451840).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
