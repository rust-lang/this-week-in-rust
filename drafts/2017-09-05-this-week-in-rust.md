Title: This Week in Rust 198
Number: 198
Date: 2017-09-05
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

* &#127880;&#127881; [Announcing Rust 1.20](https://blog.rust-lang.org/2017/08/31/Rust-1.20.html). &#127881;&#127880;
* [Rust 2017 survey results](https://blog.rust-lang.org/2017/09/05/Rust-2017-Survey-Results.html).
* [Welcome cramertj to the lang team](https://internals.rust-lang.org/t/welcome-cramertj-to-the-lang-team/5844)!
* [elfmalloc performance evaluation - a pure Rust allocator that's competitive on performance with jemalloc](https://github.com/ezrosent/allocators-rs/blob/master/info/elfmalloc-performance.md).
* [Announcing Rust Qt binding generator](https://www.vandenoever.info/blog/2017/09/04/rust_qt_binding_generator.html).
* [5 tips for writing small CLI tools in Rust](https://deterministic.space/rust-cli-tips.html).
* [Generating C bindings for Rust crates with cbindgen](http://dreamingofbits.com/post/generating-c-bindings-for-rust-crates-with-cbindgen/).
* [Librsvg's build infrastructure: Autotools and Rust](https://people.gnome.org/~federico/blog/librsvg-build-infrastructure.html).
* [Replacing a shells script with strongly typed Rust code](https://www.worthe-it.co.za/programming/2017/08/29/writing-git-hooks-using-rust.html).
* [Getting started with artifact - a design documentation tool](https://github.com/vitiral/artifact/blob/master/docs/QuickStart.md).
* [Rust Belt Rust 2017 sessions announced](http://rust-belt-rust.com/sessions.html?year=2017).
* [This week in Rust docs 71](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-71).
* [podcast] [Request for explanation #10 - Two paths diverged in a yellow wood](https://request-for-explanation.github.io/podcast/ep10-two-paths-diverged-in-a-yellow-wood/index.html). This week's RFC is [RFC 2126](https://github.com/rust-lang/rfcs/pull/2126) "Clarify and streamline paths and visibility" (aka "The modules RFC").
* [videos] [Talks from RustConf 2017](https://www.youtube.com/watch?v=COrl851gMTY&list=PL85XCvVPmGQhUSX_QBkxb4g1-o56cCqI9).

# Crate of the Week

Sadly, we had no nomination for the crate of the week.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help test async/await/generators/coroutines](https://internals.rust-lang.org/t/help-test-async-await-generators-coroutines/5835).
* [medium] [allocators-rs: mmap-alloc: Support full Alloc API](https://github.com/ezrosent/allocators-rs/issues/9).
* [medium] [allocators-rs: bsalloc: Support allocation failures](https://github.com/ezrosent/allocators-rs/issues/4).
* [easy] [allocators-rs: Add links in documentation](https://github.com/ezrosent/allocators-rs/issues/1).
* [medium] [allocators-rs: Travis: Spurious test failures on Mac](https://github.com/ezrosent/allocators-rs/issues/36).
* [easy] [bindgen: Add `bindgen::Builder::derive_copy` to control whether we emit `derive(Copy)` on type definitions](https://github.com/rust-lang-nursery/rust-bindgen/issues/948).
* [medium] [sysconf: page: Add default hugepage support on Linux](https://github.com/ZeroCostGoods/sysconf.rs/issues/7).
* [rust-machine-id: windows: add implementation](https://github.com/mathstuf/rust-machine-id/pull/2).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

120 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-08-21..2017-08-28

* [Generator support](https://github.com/rust-lang/rust/pull/43076)
* [warn by default on unused extern crates](https://github.com/rust-lang/rust/pull/42588)
* [speed up APFloat divisions for small divisors](https://github.com/rust-lang/rust/pull/44051)
* [generate builtin `Clone` impls](https://github.com/rust-lang/rust/pull/43690)
* [no needless `min_stack` on thread spawn if user has set stack size](https://github.com/rust-lang/rust/pull/44054)
* [`[T]::swap_with_slice(_)`](https://github.com/rust-lang/rust/pull/44031)
* [handle OS errors on joining threads](https://github.com/rust-lang/rust/pull/44112)
* [relax syntax path grammar](https://github.com/rust-lang/rust/pull/43540)
* [desugar parenthesized generic arguments in HIR](https://github.com/rust-lang/rust/pull/43532)
* [elaborate trait obligations while type checking impls](https://github.com/rust-lang/rust/pull/43786)
* [fix length of arrays with zero-sized-typed components](https://github.com/rust-lang/rust/pull/44060)
* [feature gate `fn_must_use`](https://github.com/rust-lang/rust/pull/43776) (sorry, no insta-stable)
* [respect formatting flags for OSStr](https://github.com/rust-lang/rust/pull/43830)
* [keep generic arguments out of attribute paths](https://github.com/rust-lang/rust/pull/43948)
* [improve placement of `use` suggestions](https://github.com/rust-lang/rust/pull/43929)
* [improve message on missing condition after `else if`](https://github.com/rust-lang/rust/pull/43854)
* [the error emitter no longer confuses bytes with chars](https://github.com/rust-lang/rust/pull/44081)
* [add let-bindings to the HIR parent map](https://github.com/rust-lang/rust/pull/43971)
* [borrowck: name the correct type in error message](https://github.com/rust-lang/rust/pull/43993)
* [querified MIR borrowck](https://github.com/rust-lang/rust/pull/44009)
* [incr. comp.: Cache HIR-DepNodeIndices in HIR map](https://github.com/rust-lang/rust/pull/44012)
* [fix missing `EndRegion`s because of faulty lookup](https://github.com/rust-lang/rust/pull/44082)
* [fix trait constraint cycle detection](https://github.com/rust-lang/rust/pull/44071)
* [avoid duplication in rustdoc](https://github.com/rust-lang/rust/pull/43966)
* [rustdoc: Add links to impls](https://github.com/rust-lang/rust/pull/43979)

## New Contributors

* Andrew Gauger
* Andy Gauge
* Jeremy Sorensen
* Lukas H
* Phlosioneer

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Clamp functions](https://github.com/rust-lang/rfcs/pull/1961).
* [Generic associated types (associated type constructors)](https://github.com/rust-lang/rfcs/pull/1598).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Clarify and streamline paths and visibility](https://github.com/rust-lang/rfcs/pull/2126).
* [disposition: merge] [Nested groups in imports](https://github.com/rust-lang/rfcs/pull/2128).
* [disposition: postpone] [Add the `()` → `Result<(), _>` coercion rule, for removing `Ok(())` everywhere](https://github.com/rust-lang/rfcs/pull/2120).
* [disposition: merge] [In-band lifetime bindings](https://github.com/rust-lang/rfcs/pull/2115).
* [disposition: merge] [Autoreferencing `Copy` types](https://github.com/rust-lang/rfcs/pull/2111).
* [disposition: postpone] [Ok wrapping: Improved support for writing code from an error handling mindset](https://github.com/rust-lang/rfcs/pull/2107).
* [disposition: merge] [Attributes for tools, 2.0](https://github.com/rust-lang/rfcs/pull/2103).
* [disposition: merge] [Infer `T: 'x` outlives requirements on structs](https://github.com/rust-lang/rfcs/pull/2093).
* [disposition: merge] [Implied bounds](https://github.com/rust-lang/rfcs/pull/2089).
* [disposition: merge] [Allow Irrefutable Patterns in if-let and while-let statements](https://github.com/rust-lang/rfcs/pull/2086).
* [disposition: merge] [Add impl Trait type alias and variable declarations](https://github.com/rust-lang/rfcs/pull/2071).
* [disposition: merge] [stable mechanism to specify the behavior of panic! in no-std applications](https://github.com/rust-lang/rfcs/pull/2070).
* [disposition: merge] [Evolving Rust through Epochs](https://github.com/rust-lang/rfcs/pull/2052).
* [disposition: merge] [Add `align_offset` intrinsic and `[T]::align_to` function](https://github.com/rust-lang/rfcs/pull/2043).
* [disposition: postpone] [Const/static type annotation elision](https://github.com/rust-lang/rfcs/pull/2010).
* [disposition: merge] [Const generics](https://github.com/rust-lang/rfcs/pull/2000).
* [disposition: merge] [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [disposition: close] [Allow crates to specify the version of Rust in which they are written](https://github.com/rust-lang/rfcs/pull/1709).
* [disposition: merge] [Copy/Clone closures](https://github.com/rust-lang/rfcs/pull/2132). Implement `Clone` and `Copy` for closures where possible.
* [disposition: merge] [Compiler-generated Clone impls for arrays and tuples](https://github.com/rust-lang/rfcs/pull/2133).

## New RFCs

* [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [Support defining C-compatible variadic functions in Rust](https://github.com/rust-lang/rfcs/pull/2137).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

We're currently writing up the discussions, we'd love some help. Check out [the tracking issue](https://github.com/rust-lang-nursery/fmt-rfcs/issues/89) for details.

PRs:

* [ranges and blocks](https://github.com/rust-lang-nursery/fmt-rfcs/pull/91)

# Upcoming Events

* [Sep  7. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Sep 11. Seattle Rust Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/242563613/).
* [Sep 12. Rust Berlin Meetup - September 2017](https://www.meetup.com/Rust-Berlin/events/242564404/).
* [Sep 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep 13. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep 14. Rust Washington, DC - Hacktember](https://www.meetup.com/RustDC/events/242847065/).
* [Sep 14. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/242459785/).
* [Sep 20. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep 20. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep 20. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/242793549/).
* [Sep 21. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Systems Engineer at Immunant](http://immunant.com/page/002_rustacean/).
* [Sr. Software Development Engineer at Amazon](https://www.amazon.jobs/en/jobs/559813/sr-software-development-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> you can ask a Future “are we there yet”, to which it can answer “yes”, “no”, or "don’t make me come back there"
> an Iterator is something you can keep asking “more?” until it gets fed up and stops listening
> Display is just a way to say “show me your moves”, with the other formatting traits being other dance moves
> if something isn’t Send, then it’s a cursed item you can’t give away, it’s yours to deal with
> if something isn’t Sync, then it won’t even appear for other people, it’s possibly an apparition inside your head
> things that are Clone can reproduce asexually, but only on command. things that are Copy won’t bother waiting for you

— [@QuietMisdreavus on Twitter](https://twitter.com/QuietMisdreavus/status/903071042834489344).

Thanks to [Havvy](https://users.rust-lang.org/t/twir-quote-of-the-week/328/441) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
