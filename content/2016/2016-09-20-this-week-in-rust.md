Title: This Week in Rust 148
Number: 148
Date: 2016-09-20
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

* [My experience rewriting Enjarify in Rust](https://medium.com/@robertgrosse/my-experience-rewriting-enjarify-in-rust-723089b406ad). Enjarify is a tool (written in Python) for translating Dalvik bytecode to equivalent Java bytecode.
* [The PlayRust Classifier](http://www.suchin.co/2016/09/13/The-PlayRust-Classifier/). Synopsis of a RustConf talk on a classifier to detect posts that were intended for [/r/playrust](https://www.reddit.com/r/playrust/) but were mistakenly posted on [/r/rust](https://www.reddit.com/r/rust/).
* [Why Rust's `std::collections` is absolutely fantastic](https://ticki.github.io/blog/fantastic/). Follow-up to - [a critique of Rust's `std::collections`](https://ticki.github.io/blog/horrible/).
* [Using `and_then` and `map` combinators on the Rust `Result` Type](http://hermanradtke.com/2016/09/12/rust-using-and_then-and-map-combinators-on-result-type.html).
* [GFX Programming Model](http://gfx-rs.github.io/2016/09/14/programming-model.html). A deep dive into what makes gfx-rs complex and awesome.
* [Building a scalable MySQL Proxy in Rust](http://www.agildata.com/building-scalable-mysql-proxy-rust/).
* [Using unsafe tricks to examine Rust data structure layout](http://pramode.in/2016/09/13/using-unsafe-tricks-in-rust/).
* [Tools for profiling Rust](https://athemathmo.github.io/2016/09/14/tools-for-profiling-rust.html).
* [Generating Rustdoc with a custom style](https://blog.guillaume-gomez.fr/articles/2016-09-16+Generating+doc+with+rustdoc+and+a+custom+theme).
* [Understanding where clauses and trait constraints](https://mgattozzi.github.io/2016/09/13/understanding-where-clauses.html).
* [Let's Build a REPL/Parser with Rust & LALRPOP](https://dfockler.github.io/2016/09/15/lalrpop.html).
* [video] [Videos from Rust Meetup Cologne/Bonn](https://media.ccc.de/c/rustmcb).

### RustConf Experiences

* [My RustConf travelogue](http://zackmdavis.net/blog/2016/09/rustconf-2016-travelogue/) by Zack M. Davis.
* [Rustconf 2016 – What was cool and what surprised me](http://www.agildata.com/rustconf-2016-what-was-cool-and-what-surprised-me/) by Andy Grove.
* [Notes from RustConf 2016 talks](http://alwayscoding.ca/momentos/2016/09/10/rustconf-2016-talks/) by Brian Pearce.

## New Crates & Project Updates

* [Announcing the code style RFC process and style team](https://internals.rust-lang.org/t/announcing-the-code-style-rfc-process-and-style-team/4079).
* [Redox is now listed in Github's Open Source Operating Systems Showcase](https://github.com/showcases/open-source-operating-systems).
* [rustcxx](https://github.com/google/rustcxx) is a tool allowing C++ to be used from a Rust project easily. It works by allowing snippets of C++ to be included within a Rust function, and vice-versa.
* [Kinder](https://github.com/KitFreddura/Kinder). Algebraic structure and emulation of higher kinded types for Rust.
* [rdedup](https://github.com/dpc/rdedup). Data deduplication with compression and public key encryption.
* [bit_reverse](https://crates.io/crates/bit_reverse). A Rust library to compute the bit reversal of primitive integers.
* [This Week in Rust Docs 22](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-22).
* [These days in Piston 3](http://blog.piston.rs/2016/09/19/what-is-happening-3/).
* [This week in Ruru 1](http://this-week-in-ruru.org/2016/09/18/this-week-in-ruru-1/). Ruru lets you write native Ruby extensions in Rust.
* [This week in TiKV 2016-09-19](http://www.pingcap.com/tikv/2016/09/19/tikv-weekly/).

# Crate of the Week

This week's crate of the week is (the in best TWiR-tradition shamelessly self-promoted) [mysql-proxy](https://crates.io/crates/mysql-proxy), a flexible, lightweight and scalable proxy for MySQL databases. Thanks to [andygrove](https://users.rust-lang.org/users/andygrove) for the suggestion!

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

98 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-09-12..2016-09-19

* [Macro invocations now fold/visit in the same order](https://github.com/rust-lang/rust/pull/36555) (makes it easier to reason about them)
* [Optimized parser's last token handling](https://github.com/rust-lang/rust/pull/36527) (who'd have thought it could be optimized further?)
* [Some dependency graph improvements](https://github.com/rust-lang/rust/pull/35960)
* [Rustbuild now supports python3](https://github.com/rust-lang/rust/pull/36509)
* [LLVM updated](https://github.com/rust-lang/rust/pull/36508)
* [Don't lose padding for constant closures and tuples](https://github.com/rust-lang/rust/pull/36406) (fixes [#36401](https://github.com/rust-lang/rust/issues/36401) segfault)
* [Improved move checker accuracy and error reports](https://github.com/rust-lang/rust/pull/36353)
* [Better error message when shadowing type with generics](https://github.com/rust-lang/rust/pull/36338)
* [Improve Macro-1.1 errors labelling](https://github.com/rust-lang/rust/pull/36308)
* [`SyntaxExtension::MacroRulesTT` is no more](https://github.com/rust-lang/rust/pull/36444)
* [`#[derive(Clone, Eq)]` produces less code](https://github.com/rust-lang/rust/pull/36384) (Yay! faster builds!)
* [Default stack size upped to 16MiB](https://github.com/rust-lang/rust/pull/36505) (temporary measure against stack overflows)
* [Change in invoking drop glue for boxed dynamically-sized values](https://github.com/rust-lang/rust/pull/36459) (fixes LLVM assertion failure)
* [`private_in_public` error demoted to warning](https://github.com/rust-lang/rust/pull/36270) (until remaining regressions are fixed, also in beta)
* [MIR optimization: Remove reborrows for references](https://github.com/rust-lang/rust/pull/36504) (and already pass dependencies seem to become subtle...)
* [Better parent info for `-Z save-analysis`](https://github.com/rust-lang/rust/pull/36487)
* [Avoid loading/parsing unused modules](https://github.com/rust-lang/rust/pull/36482) (e.g. `#[cfg(any())] mod foo`)
* [Fix closure-as-trait-object dropping](https://github.com/rust-lang/rust/pull/36468)
* [`Duration::checked_`{`add`, `sub`, `mul`, `div`}](https://github.com/rust-lang/rust/pull/36463)
* [`ty::TraitObject`'s projection bounds are now stably sorted](https://github.com/rust-lang/rust/pull/36425) (also unifies/removes diverse hashing implementations)
* [De-specialized `Zip` data](https://github.com/rust-lang/rust/pull/36490) (some ongoing optimization work)
* [`Iterator::sum()` and `product()` no longer check for overflow in release mode](https://github.com/rust-lang/rust/pull/36372)
* [Fix poor performance in `Vec::`{`extend_from_slice`,`extend_with_element`}`()`](https://github.com/rust-lang/rust/pull/36355)
* [`std::str::replacen(..)`](https://github.com/rust-lang/rust/pull/36347)
* [Zero the first byte of `CString`s on drop](https://github.com/rust-lang/rust/pull/36264)
* [`likely(_)`/`unlikely(_)` intrinsics added](https://github.com/rust-lang/rust/pull/36181) (help the CPU with branch prediction)
* [`std::io::Take::into_inner()`](https://github.com/rust-lang/rust/pull/36019)
* [`std::alloc::`{`Rc`, `Arc`}`::ptr_eq(..)`](https://github.com/rust-lang/rust/pull/35992)
* [`compiler-rt` is dead, long live `compiler-builtins`!](https://github.com/rust-lang/rust/pull/35021)
* [dist tarball now contains version info](https://github.com/rust-lang/rust/pull/36213)
* [sublime-rust now works with the new error format](https://github.com/rust-lang/sublime-rust/pull/87)

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

*No quote was selected for QotW.*

_(Full disclosure: we removed QotW for this issue because selected QotW was deemed inappropriate and against the core values of Rust community. Here is the relevant [discussion on reddit](https://www.reddit.com/r/rust/comments/53oocf/this_week_in_rust_148/d7v4tnd). If you are curious, you can find the quote in [git history](https://github.com/cmr/this-week-in-rust/commits/master))._

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
