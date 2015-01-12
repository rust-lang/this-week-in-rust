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
and [/r/programming][alpha-reddit] and surely other corners of the
Internet.

[alpha]: http://blog.rust-lang.org/2015/01/09/Rust-1.0-alpha.html
[alpha-hn]: https://news.ycombinator.com/item?id=8863451
[alpha-reddit]: https://www.reddit.com/r/rust/comments/2rvodx/announcing_rust_100_alpha/
[alpha-reddit2]: https://www.reddit.com/r/programming/comments/2rvoha/announcing_rust_100_alpha/

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
* `c_str` and `c_vec` have been [redesigned]. There are no longer any
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
[unusef]: https://github.com/rust-lang/rust/pull/20593
[unused-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0447-no-unused-impl-parameters.md
[orph]: ttps://github.com/rust-lang/rust/pull/20594
[sizedbound]: https://github.com/rust-lang/rust/pull/20602
[marker]: https://github.com/rust-lang/rust/pull/20607
[isize]: https://github.com/rust-lang/rust/pull/20609
[isize-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0544-rename-int-uint.md
[intgate]: https://github.com/rust-lang/rust/pull/20754
[imode]: https://github.com/rust-lang/rust/pull/20708
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
* Syntax for [negative impls][neg] has been added been
  added behind the `optin_builtin_traits` feature gate. This will let
  `Sync` and `Send` be implemented completely in the library
  eventually. [RFC][neg-rfc].
* Florian Hahn landed a [series][lexer1] of [patches][lexer2] to
  [improved][lexer3] the model lexer.

[neg]: https://github.com/rust-lang/rust/pull/20285
[neg-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md#default-and-negative-impls
[lexer1]: https://github.com/rust-lang/rust/pull/20310
[lexer2]: https://github.com/rust-lang/rust/pull/20330
[lexer3]: https://github.com/rust-lang/rust/pull/20245
[fs1]: https://github.com/rust-lang/rust/pull/20663
[fs2]: https://github.com/rust-lang/rust/pull/20738
[fs-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0507-release-channels.md
[trpl]: https://github.com/rust-lang/rust/pull/19897

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
* [Remove official `ndebug` variable support][rfc-563]. A more rusty
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

## From the Team



## Blog Posts



## Discussions



## New Projects



## Project Updates



## Upcoming Events


