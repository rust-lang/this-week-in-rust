Title: This Week in Rust 65
Date: 2015-01-12
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the
trifecta: safe, concurrent, and fast. This is a weekly summary of its
progress and community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on
Github](https://github.com/cmr/this-week-in-rust).  If you find any
errors or omissions in this week's issue, [please submit a
PR](https://github.com/cmr/this-week-in-rust/pulls).

# The big story

This week in Rust was crazy. Rust boldly moved to [1.0 alpha][alpha]
status, and the effort put in to make it happen in the last week was
staggering, involving a lot of massive patches, a lot of breakage, and
not a lot of sleep. Thanks to everybody for pitching in.

Rust 1.0 is going to arrive very quickly and from now to then the
focus is going to be on nestling it into a warm cradle of stability,
getting the Cargo ecosystem on the stability bandwagon and making sure
the stage is set for all those 1.0 newbies to have a swell ride.

It was discussed on [Hacker News][alpha-hn], [/r/rust][alpha-reddit],
[/r/programming][alpha-reddit2], [LWN][alpha-lwn] and surely other
corners of the Internet.

[alpha]: http://blog.rust-lang.org/2015/01/09/Rust-1.0-alpha.html
[alpha-hn]: https://news.ycombinator.com/item?id=8863451
[alpha-reddit]: https://www.reddit.com/r/rust/comments/2rvodx/announcing_rust_100_alpha/
[alpha-reddit2]: https://www.reddit.com/r/programming/comments/2rvoha/announcing_rust_100_alpha/
[alpha-lwn]: http://lwn.net/Articles/629025/

# What's cooking on master?

Around 171 pull requests were [merged in the last
week][1]. It was a long week.

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2015-01-05..2015-01-11

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* Boxed closures have finally been [removed][boxed]. The pull request
  includes a detailed description of the impact.
* Unused type parameters on `impl`s are [prohibited][unused]. This
  fixes some soundness holes and generally makes things more
  sensible. [RFC][unused-rfc].
* `int` and `uint` have been [renamed][isize] to `isize` and `usize`
  to emphasize that they are not the 'default' integers. The old names
  are temporarily behind the [`int_uint` feature gate][intgate] to
  provide a transitionary window. The `int` and `uint` [modules][imod]
  are now called `isize` and `usize`. [RFC][isize-rfc].
* There are [new restrictions][orph] in the orphan check for impls
  that ensure that if the implemented trait is not defined in the
  local crate that the Self type is constrained by local types. Fixes
  soundness holes revealed by multidispatch. The commit message
  includes more details.
* `c_str` and `c_vec` have been [redesigned][c_stuff]. There are no longer any
  scenerios where Rust frees strings allocated by C, and the APIs
  are more consistent with modern conventions. Details in the
  [RFC][c_stuff-rfc].
* The `trait Foo for Sized?` syntax has been [obsoleted][forsized]
  after a short deprecation period. `Self` is no longer implicitly
  `Sized`.
* Likewise, the `Sized? T` syntax for trait bounds has been
  [obsoleted][sizedbound] in favor of `T: ?Sized`.
* To futureproof for potential alternative designs, chain comparsion
  operators like `a == b == c` [must now be parenthesized][chain].
  [RFC][chain-rfc].
* The `FloatMath` trait has [merged into `Float`][float].
* `std::kinds` is [called `std::marker`][marker].
* The `std::thread` API has [changed again][thread]. `spawn` always
  creates a detached thread, and `scoped` creates one which must be
  joined.  The intention is that by requiring a completely different
  name to create an attached thread it will be harder to accidentally
  create a deadlock by misunderstanding the join semantics.
* The `target_word_size` compiler-defined cfg value has been [renamed
  to `target_pointer_width`][tpw].

[boxed]: https://github.com/rust-lang/rust/pull/20578
[c_stuff]: https://github.com/rust-lang/rust/pull/20507
[c_stuff-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0494-c_str-and-c_vec-stability.md
[forsized]: https://github.com/rust-lang/rust/pull/20556
[float]: https://github.com/rust-lang/rust/pull/20573
[unused]: https://github.com/rust-lang/rust/pull/20593
[unused-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0447-no-unused-impl-parameters.md
[orph]: ttps://github.com/rust-lang/rust/pull/20594
[sizedbound]: https://github.com/rust-lang/rust/pull/20602
[marker]: https://github.com/rust-lang/rust/pull/20607
[isize]: https://github.com/rust-lang/rust/pull/20609
[isize-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0544-rename-int-uint.md
[intgate]: https://github.com/rust-lang/rust/pull/20754
[imod]: https://github.com/rust-lang/rust/pull/20708
[thread]: https://github.com/rust-lang/rust/pull/20615
[tpw]: https://github.com/rust-lang/rust/pull/20680
[chain]: https://github.com/rust-lang/rust/pull/20726
[chain-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0558-require-parentheses-for-chained-comparisons.md

## Other Changes

* Feature staging has been partially implemented in a [series][fs1] of
  [patches][fs2].  Under the current behavior, stability attributes
  only mean anything when applied to the crates distributed with Rust
  (primarily std), and the 'experimental' stability level has merged
  into 'unstable'. Use of unstable APIs is now a warning on the
  nightly and beta (alpha) release channels, and a new lint,
  `unstable_features`, that checks for activation of feature gates, is
  set to warn in betas. [RFC][fs-rfc].
* The various guides have been [merged into a book][trpl] called 'The
  Rust Programming Language'.
* The `box` has been hidden behind the [`box_syntax` feature
  gate][box]. until it is more fully-baked. For the primary use of
  constructing boxes, just use `Box::new` for
  now. [/r/rust][box-r-rust].
* Syntax for [negative impls][neg] has been added been
  added behind the `optin_builtin_traits` feature gate. This will let
  `Sync` and `Send` be implemented completely in the library
  eventually. [RFC][neg-rfc].
* Florian Hahn landed a [series][lexer1] of [patches][lexer2] to
  [improve][lexer3] the model lexer.

[neg]: https://github.com/rust-lang/rust/pull/20285
[neg-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md#default-and-negative-impls
[lexer1]: https://github.com/rust-lang/rust/pull/20310
[lexer2]: https://github.com/rust-lang/rust/pull/20330
[lexer3]: https://github.com/rust-lang/rust/pull/20245
[fs1]: https://github.com/rust-lang/rust/pull/20663
[fs2]: https://github.com/rust-lang/rust/pull/20738
[fs-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md
[trpl]: https://github.com/rust-lang/rust/pull/19897
[box]: https://github.com/rust-lang/rust/pull/20723
[box-r-rust]: https://www.reddit.com/r/rust/comments/2rr990/box_expr_syntax_is_now_behind_a_feature_gate/

## New Contributors

* Cristian Kubis
* Dabo Ross
* Daniel Grunwald
* Dylan Ede
* FakeKane
* Guillaume Gomez
* Hyeon Kim
* Jakub Vrána
* John Ericson
* John Kåre Alsaker
* Kelvin Ly
* Kevin Rauwolf
* Laurence Tratt
* Mike English
* Nathan Stoddard
* Peter Schuller
* Raul Gutierrez S
* Sean T Allen
* Thiago Pontes
* Tim Dumol
* York Xiang
* 克雷

# Approved RFC's

* [447: Prohibit unused type parameters in impls][rfc-447]. Disallows
  unconstrained type parameters in trait impls to clean up
  semantics. [PR][rfc-447-pr].
* [494: Stabilize `c_str` and `c_vec`][rfc-494]. Overhauls these two
  interop modules for modern tastes. [PR][rfc-494-pr].
* [501: Consistent no_prelude attribute][rfc-501]. Renames
  `no_implicit_prelude` to `no_prelude` and makes it only apply to the
  current module. [PR][rfc-501-pr].
* [507: Release channels][rfc-507]. The post-1.0 release model and the
  'feature staging' process by which new features are
  intrtoduced. [PR][rfc-507-pr].
* [526: Statically enforce unicode in `std::fmt`][rfc-526]. By making
  `fmt` only deal in unicode some almost-always-redundant sanity
  checks can be eliminated. [PR][rfc-526-pr].
* [544: Rename `int`/`uint`][rfc-544]. Renames `int` to `isize` and
  `uint` to `usize`. [PR][rfc-544-pr].
* [558: Require parentheses for chained comparisons][rfc-558]. Minor
  futureproofing that leaves open the option of making `a == b == c`
  behave more like people expect. [PR][rfc-558-pr].

[rfc-447]: https://github.com/nikomatsakis/rfcs/blob/unused-impl-parameters/text/0000-no-unused-impl-parameters.md
[rfc-447-pr]: https://github.com/rust-lang/rfcs/pull/447
[rfc-494]: https://github.com/rust-lang/rfcs/blob/master/text/0494-c_str-and-c_vec-stability.md
[rfc-494-pr]: https://github.com/rust-lang/rfcs/pull/494
[rfc-501]: https://github.com/rust-lang/rfcs/blob/master/text/0501-consistent_no_prelude_attributes.md
[rfc-501-pr]: https://github.com/rust-lang/rfcs/pull/501
[rfc-507]: https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md
[rfc-507-pr]: https://github.com/rust-lang/rfcs/pull/507
[rfc-526]: https://github.com/rust-lang/rfcs/blob/master/text/0526-fmt-text-writer.md
[rfc-526-pr]: https://github.com/rust-lang/rfcs/pull/526
[rfc-544]: https://github.com/rust-lang/rfcs/blob/master/text/0544-rename-int-uint.md
[rfc-544-pr]: https://github.com/rust-lang/rfcs/pull/544
[rfc-558]: https://github.com/rust-lang/rfcs/blob/master/text/0558-require-parentheses-for-chained-comparisons.md
[rfc-558-pr]: https://github.com/rust-lang/rfcs/pull/558

# New RFC's

* [Macro future proofing][rfc-550-pr]. Places restrictions on some
  `macro_rules!` matchers to allow for Rust's grammer to evolve in the
  future (the implementation has already landed).
* [Unchecked downcast methods for `Any`][rfc-555-pr]. New unsafe
  methods are added to `Any` for unchecked downcasting, a small
  performance and code-bloat-avoidance optimisation in cases where the
  dynamic type is already known.
* [Lifetime parameters for unsafe pointer
  conversions][rfc-556-pr]. APIs for handling lifetimes correctly when
  dealing with raw pointers.
* [Integer overflow][rfc-560-pr]. Change the semantics of built-in
  integer types to be a program error on overflow. Implementations are
  not required to check for these errors except in debug mode. Adds
  wrapping int types to the library.
* [Remove official `ndebug` variable support][rfc-563-pr]. A more rusty
  solution to turning on debug assertions.
* [Relative paths by default][rfc-564-pr]. Make paths in `use`
  statements relative instead of absolute.
* [Guidelines for `fmt::Show` and `fmt::String`][rfc-565-pr]. When to
  use which of the two format types that convert to strings.
* [A byte string concatenation macro][rfc-566-pr]. Add a `bytes!`
  macro that produces static expressions of type `&'static [u8]` by
  concatenating the arguments.
* [Lose the tick][rfc-567-pr]. Remove the tick from some parts of the
  lifetime syntax.
* [Add methods to `String` and `Vec` which can remove multiple
  elements at once][rfc-570-pr].  Ads `remove_range` to `Vec` and
  `String` to remove multiple elements efficiently.
* [Change `foo::<T>(x)` to `foo@<T>(x)`][rfc-571-pr]. A different
  syntax for solving the ambiguity.
* [Reserve `#[rustc_*]` for future language
  features][rfc-572-pr]. Futureproofing around the lack of namespacing
  an attributes.

[rfc-550-pr]: https://github.com/rust-lang/rfcs/pull/550
[rfc-555-pr]: https://github.com/rust-lang/rfcs/pull/555
[rfc-556-pr]: https://github.com/rust-lang/rfcs/pull/556
[rfc-560-pr]: https://github.com/rust-lang/rfcs/pull/560
[rfc-563-pr]: https://github.com/rust-lang/rfcs/pull/563
[rfc-564-pr]: https://github.com/rust-lang/rfcs/pull/564
[rfc-565-pr]: https://github.com/rust-lang/rfcs/pull/565
[rfc-566-pr]: https://github.com/rust-lang/rfcs/pull/566
[rfc-567-pr]: https://github.com/rust-lang/rfcs/pull/567
[rfc-570-pr]: https://github.com/rust-lang/rfcs/pull/570
[rfc-571-pr]: https://github.com/rust-lang/rfcs/pull/571
[rfc-572-pr]: https://github.com/rust-lang/rfcs/pull/572

# Community

The #rust IRC channel now peaks at more than 900 users.

## From the Team

* [Announcing Rust 1.0
  Alpha][alpha]. [HN][alpha-hn]. [/r/rust][alpha-r-rust]. [/r/programming][alpha-r-programming].
* [weekly-meetings/2015-01-06][mtg]. fott; 1.0 alpha priorities; LLVM
  updates; the fate of `box`

[alpha]: http://blog.rust-lang.org/2015/01/09/Rust-1.0-alpha.html
[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-01-06.md
[alpha-hn]: https://news.ycombinator.com/item?id=8863451
[alpha-r-programming]: https://www.reddit.com/r/rust/comments/2rvodx/announcing_rust_100_alpha/
[alpha-r-rust]: https://www.reddit.com/r/programming/comments/2rvoha/announcing_rust_100_alpha/

## Blog Posts

* [Peeking inside Trait Objects][objs]. Huon explains the runtime
  representation of Rust's opaque types. [/r/rust][objs-r-rust].
* [The Sized trait][sized]. Then Huon explains
  DST. [/r/rust][objs-r-rust].
* [Object safety][safety]. Then Huon explains the conditions under
  which a trait may be cast to an object. [/r/rust][safety-r-rust].
* [Rust Patterns: Using traits for function
  overloading][func]. Jonathan Reem demonstrates how to overload
  functions. [/r/rust][func-r-rust].
* [My experience converting a project from Rust 0.12 to 1.0
  (alpha)][conv]. Looks painful...
* [Is now a good time to learn Rust?][now]. A: wait 6-12
  weeks. [/r/programming][now-reddit].
* [Iomrascálaí: A Great Way to Learn Rust or About AI][iom]. An AI for the
  game of Go.
* [Notch says something about Rust][notch]. Rust is official.

[objs]: http://huonw.github.io/blog/2015/01/peeking-inside-trait-objects/
[objs-r-rust]: https://www.reddit.com/r/rust/comments/2rutqb/peeking_inside_trait_objects/
[sized]: http://huonw.github.io/blog/2015/01/the-sized-trait/
[sized-r-rust]: https://www.reddit.com/r/rust/comments/2s2gee/the_sized_trait/
[safety]: http://huonw.github.io/blog/2015/01/object-safety/
[safety-r-rust]: https://www.reddit.com/r/rust/comments/2s2okp/object_safety/
[now]: https://www.codementor.io/learn-programming/now-good-time-learn-rust
[now-reddit]: https://www.reddit.com/r/programming/comments/2ruixg/is_now_a_good_time_to_learn_rust/
[iom]: http://bettong.net/2015/01/07/iomrascalai-a-great-way-to-learn-rust-or-about-ai/
[conv]: https://kentaromiura.wordpress.com/2015/01/10/my-experience-converting-a-project-from-rust-0-12-to-1-0/
[notch]: https://twitter.com/notch/status/554348548053929984
[func]: https://medium.com/@jreem/advanced-rust-using-traits-for-argument-overloading-c6a6c8ba2e17
[func-r-rust]: https://www.reddit.com/r/rust/comments/2s4tbx/advanced_rust_using_traits_for_function/

## Discussions

* [Rust book by Packt Publishing][packt]. There is opportunity to be
  involved in one of the first Rust books.
* [Final decision on builtin integer types. Again][int]. Aftermath.
* [Operating system development in Rust][os]. Well-commented hacker
  news discussion. Nothing new though.
* [151-byte static binary in Rust][bin-hn]. Keegan shows that Rust can
  be small. [/r/rust][bin-r-rust]. [/r/programming][bin-r-programming].
* ["hello world" contains Lovecraft quotes][love]. /me eyerolls
* [Google removes rust, Netflix, other GitHub repos after DMCA
  takedown][dmca-hn]. 'Cargo' looks suspiciously like
  pornography. [/r/rust][dmca-reddit].
* [How big a deal is Rust, Really][deal]? A: the biggest.
* [Pre-RFC: Linear type modifier][lin]. Requires certain types to be
  explicitly disposed.
* [Using Rust 1. for video game development?][vidja]. Some up to date
  info here.

[os]: https://news.ycombinator.com/item?id=8871357
[love]: http://news.ycombinator.com/item?id=8869572
[bin-hn]: http://news.ycombinator.com/item?id=8869167
[bin-r-rust]: https://www.reddit.com/r/rust/comments/2s0s9n/151byte_static_linux_binary_in_rust/
[bin-r-programming]: https://www.reddit.com/r/programming/comments/2s1sgg/151byte_static_linux_binary_in_rust/
[dmca-hn]: https://www.reddit.com/r/hackernews/comments/2rlaf5/tell_hn_google_removes_rust_netflix_other_github/
[dmca-reddit]: https://www.reddit.com/r/rust/comments/2rlaug/cargo_github_repo_link_has_been_dmcad_off_the/
[deal]: https://www.reddit.com/r/programming/comments/2rlef7/how_big_a_deal_is_rust_really/
[int]: https://www.reddit.com/r/rust/comments/2rg60o/final_decision_on_builtin_integer_types_again/
[packt]: https://www.reddit.com/r/rust/comments/2rnked/rust_book_by_packt_publishing/
[lin]: http://internals.rust-lang.org/t/pre-rfc-linear-type-modifier/1225
[vidja]: https://www.reddit.com/r/rust/comments/2s4kp9/using_rust_10_for_video_game_development/

## New Projects

* [multirust]. Manage multiple Rust toolchains.
* [Roogle]. Hoogle 4 Rust.
* [Rust for Clojurists][clj]. An introduction to
  Rust. [/r/programming][clj-r-rust].
* [Clippy for Rust][clippy]. A collection of
  lints. [/r/rust][clippy-r-rust].
* [rust-blas][blas]. Bindings to the immortal BLAS numerical library.
* [fallthrough]. A macro for fall-through match cases.
* [rust-vobject]. A vObject/iCalendar parser.
* [ProjectEuler]. A new attempt at the Project Euler problems.
* [forkallcc]. Continuations with fork(2)!
* [construct]. A macro for building arbitrary collections.
* [float_macros]. CTFE for some float functions.

[multirust]: https://github.com/brson/multirust
[Roogle]: https://github.com/ajtulloch/roogle
[clj]: https://gist.github.com/oakes/4af1023b6c5162c6f8f0
[clj-r-rust]: https://www.reddit.com/r/rust/comments/2rsl5s/rust_for_clojurists/
[clippy]: https://github.com/Manishearth/rust-clippy
[clippy-r-rust]: https://www.reddit.com/r/rust/comments/2rihkk/clippy_for_rust/
[blas]: https://github.com/mikkyang/rust-blas
[fallthrough]: https://github.com/pythonesque/fallthrough
[rust-vobject]: https://github.com/untitaker/rust-vobject
[ProjectEuler]: https://github.com/Ap0ph1s/ProjectEuler
[forkallcc]: https://github.com/kmcallister/forkallcc
[construct]: https://github.com/TyOverby/construct
[float_macros]: https://github.com/talevy/float_macros

## Project Updates

* [This Week in Servo 18][twis].
* [Grisu and rust-strconv][grisu]. Yurume talks about his
  implementation of the Grisu algorithm for converting floats to
  strings. This work is likely to make it into std someday.
* [Announcing support for 1.0 in Rust Explorer][expl]. Shows the
  assembly of Rust code.

[twis]: http://blog.servo.org/2015/01/06/twis-18/
[grisu]: https://lifthrasiir.github.io/rustlog/worklog-2015-01-10.html
[expl]: http://internals.rust-lang.org/t/announcing-support-for-rust-1-0-in-rust-explorer/1287

## Upcoming Events

[January 12 - Seattle Meetup][seattle].
[January 17 - Beijing Meetup][beijing]. Presantations about servo and
zmq.rs.
[January 17 - Getting starting contributing to Rust][sf]. A special SF
Bay area meetup.

[seattle]: https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg
[beijing]: https://www.eventbrite.com/e/rust-meet-up-in-beijing-tickets-14905925023
[sf]: http://www.meetup.com/Rust-Bay-Area/events/203782472/
