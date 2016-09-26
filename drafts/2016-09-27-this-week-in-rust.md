Title: This Week in Rust 149
Number: 149
Date: 2016-09-27
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

* Llogiq goes [from Tweet to Rust feature](https://llogiq.github.io/2016/09/14/feature.html)
* Also there's some [Faster line counting](https://llogiq.github.io/2016/09/24/newline.html)

## New Crates & Project Updates

* [What's coming up in imag 16](http://beyermatthias.de/blog/2016/09/23/what-s-coming-up-in-imag-16/)

# Crate of the Week

Somewhat unsurprisingly, this week's crate of the week is [ripgrep](https://crates.io/crates/ripgrep). In case you've missed it, this is a grep/ag/pt/whatever search tool you use replacement that absolutely smokes the competition in most performance tests. Thanks to [DanielKeep](https://users.rust-lang.org/users/DanielKeep) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Specialisation error 502 is misleading](https://github.com/rust-lang/rust/issues/36553).
* [easy] [rust: Bootstrap key logic is too strict](https://github.com/rust-lang/rust/issues/36548).
* [easy] [rust: rustc should emit an error when there's a bootstrap key mismatch](https://github.com/rust-lang/rust/issues/36544).
* [easy] [rust: Lint against using generic conversion traits when concrete methods are available](https://github.com/rust-lang/rust/issues/36443).
* [hard] [rust: Fix unwinding on emscripten](https://github.com/rust-lang/rust/issues/36514).
* [moderate] [rust: Create official .deb packages](https://github.com/rust-lang/rust/issues/28307).
* [easy] [rust-www: Better front-page example](https://github.com/rust-lang/rust-www/issues/180).
  The front page example on the website isn't so special. Make it shine.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

77 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-09-19..2016-09-26

* [`Self` can no longer be a type parameter](https://github.com/rust-lang/rust/pull/36649)
* [MIR: Trivial copy propagation](https://github.com/rust-lang/rust/pull/36388)
* [MIR: Constant propagation](https://github.com/rust-lang/rust/pull/36639)
* [Attribute invocation at crate root level allowed again](https://github.com/rust-lang/rust/pull/36618) (were inadvertently disallowed two weeks ago)
* [`TypedArena` now allocates lazily](https://github.com/rust-lang/rust/pull/36618), [loses `.with_capacity(_)`](https://github.com/rust-lang/rust/pull/36657) (the latter is a breaking change)
* [`syntax::codemap::Span`s can now be merged if adjacent](https://github.com/rust-lang/rust/pull/36585)
* [RBML is gone](https://github.com/rust-lang/rust/pull/36585) (epic PR)
* [`#[inline]`d functions are now only instantiated on use site](https://github.com/rust-lang/rust/pull/36524) (epic speedup)
* [Better `parent` info for `save-analysis](https://github.com/rust-lang/rust/pull/36487)
* [`trans::adt` is superceded by `rustc::ty::layout`](https://github.com/rust-lang/rust/pull/36151)
* [Rustc metadata diagnostics](https://github.com/rust-lang/rust/pull/36102)
* [`assert_ne!(..)` and `debug_assert_ne!(..)`](https://github.com/rust-lang/rust/pull/35074)
* [`2u64.pow(99)` now panics instead of silently overflowing](https://github.com/rust-lang/rust/pull/34942)
* [`String` no longer `impl`s `From<Vec<char>>` nor `From<&'a [char]>`](https://github.com/rust-lang/rust/pull/36685) (for now, until the regressions are sorted out)
* [ARM LLVM bug workaround: Setting discriminant via `memset`](https://github.com/rust-lang/rust/pull/36496)
* [Preparations for macros 2.0](https://github.com/rust-lang/rust/pull/36154)

## New Contributors

* Caleb Jones
* dangcheng
* Eugene Bulkin
* knight42
* Liigo

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1696: `mem::discriminant()`](https://github.com/rust-lang/rfcs/pull/1696). Add a function that extracts the discriminant from an enum variant as a comparable, hashable, printable, but (for now) opaque and unorderable type.

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [Generalize the delayed resolution of language items to arbitrary items](https://github.com/rust-lang/rfcs/pull/1408).

## New RFCs

* [Crates.io should offer an API to release security advisories for crates](https://github.com/rust-lang/rfcs/pull/1752).

# Upcoming Events

* [9/21. Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/events/233463725/).
* 9/21. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 9/21. Rust Dcoumentation Team Meeting at #rust-docs on irc.mozilla.org.
* [9/22. RustPH Mentors Meeting](http://www.rustph.tech/).
* 9/22. Rust release triage at #rust-triage on irc.mozilla.org.
* [9/26. São Paulo Meetup](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/233713814/).
* [9/28. Boston Rust Meetup](https://www.meetup.com/BostonRust/events/234241654/).
* 9/28. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 9/28. Rust Dcoumentation Team Meeting at #rust-docs on irc.mozilla.org.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> "Why I’m dropping Rust" was so meh and it got into TWiR? Rdedup never got to TWiR, while it is way cooler than a guy that just couldn't force Rust to be Java. :P

— [/u/dpc_pw on reddit](https://www.reddit.com/r/rust/comments/52ramk/this_week_in_rust_147/d7n5p9o).

Thanks to [/u/llogiq](https://www.reddit.com/r/rust/comments/52ramk/this_week_in_rust_147/d7nfvfw) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
