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
  provide a transitionary window. [RFC][isize-rfc].
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
* The `FloatMath` trait has [merged into `Float`][float].
* `std::kinds` is [called `std::marker`][marker].
* The `std::thread` API has [changed again][thread]. `spawn` always
  creates a detached thread, and `scoped` creates one which must be
  joined.  The intention is that by requiring a completely different
  name to create an attached thread it will be harder to accidentally
  create a deadlock by misunderstanding the join semantics.

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
[thread]: https://github.com/rust-lang/rust/pull/20615

## Other Changes

* Feature staging has been partially implemented in a [series][fs1] of
  [patches][fs2].  Under the current behavior, stability attributes
  only mean anything when applied to the crates distributed with Rust
  (primarily std), and the 'experimental' stability level has merged
  into 'unstable'. Use of unstable APIs is now a warning on the
  nightly and beta (alpha) release channels, and a new lint,
  `unstable_features`, that checks for activation of feature gates, is
  set to warn in betas. [RFC][fs-rfc].
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

## New Contributors



# Approved RFC's



# New RFC's



# Community

## From the Team



## Blog Posts



## Discussions



## New Projects



## Project Updates



## Upcoming Events


