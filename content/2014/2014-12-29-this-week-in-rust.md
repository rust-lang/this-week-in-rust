Title: This Week in Rust 63
Date: 2014-12-29
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on Github](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

96 pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-22..2014-12-29

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* `Send` and `Sync` are now [unsafe traits][oibit], partially
  implementing the [OIBIT RFC][oibit-rfc]. They are implemented by
  default for types that only contain `Send` and `Sync` types and can
  be opted into (unsafely) for other types, particularly those
  containing unsafe pointers.
* The way `fn` items are coerced has [changed in subtle ways][fn].
* There has been another [stabilization pass][str] over `std::str`
  which includes a number of minor breaking changes.
* The semantics of the `reserve` methods of `Bitv` and `BitSet` have
  [changed to match conventions][bitv] such that it reserves n
  *additional* units capacity instead of total. The free functions of
  `collections::bit` have been deprecated.
* `FPCategory`, for classifying floating point numbers, has been
  [renamed `FpCategory`][fp] to match conventions.
* `std::ascii` has [undergone some changes][ascii] with
  `to_ascii_lower` being renamed to `to_ascii_lowercase` and
  `to_ascii_upper` to `to_ascii_uppercase`.  The `Ascii` type has been
  removed in favor of the `AsciiExt` trait, implemented for `u8` and
  `char`. [RFC][ascii-rfc].
* `BinaryHeap::top` is [renamed to `peek`][peek].
* A number of iterator types [have been renamed][iter]. Only breaking
  if you name them somewhere.
* `include_bin!` [is now `include_bytes!`][bytes].

[fn]: https://github.com/rust-lang/rust/pull/19891
[str]: https://github.com/rust-lang/rust/pull/19741
[bitv]: https://github.com/rust-lang/rust/pull/19216
[fp]: https://github.com/rust-lang/rust/pull/19758
[ascii]: https://github.com/rust-lang/rust/pull/19916
[ascii-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0486-std-ascii-reform.md
[peek]: https://github.com/rust-lang/rust/pull/20053
[iter]: https://github.com/rust-lang/rust/pull/20056
[bytes]: https://github.com/rust-lang/rust/pull/20117
[oibit]: https://github.com/rust-lang/rust/pull/20119
[oibit-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md

## Other Changes

* A new [range syntax][range] allows for ranges to be specified with
  `m..n`, `..n`, `m..` syntaxes, which will soon let slicing be
  implemented as indexing over ranges. For now the `..n` notation is
  not implemented [because of an ambiguity][ambig] in the syntax that
  must be resolved first.  [RFC][range-rfc].
* The [new fixed length array syntax][array-rfc] that disambiguates
  the [new range syntax][range-rfc] has [been implemented][array]. The
  old syntax has not been removed yet.
* The new `{:?}` fmt specifier [has been implemented][fmt]. It
  corresponds to the `Show` trait and is intended to be implementable
  by all types, whereas the `String` format specifier (`{}`) is purely
  for types that can be losslessly converted to strings.
  [RFC][fmt-rfc].
* Return values have been optimized to [reduce stack usage
  dramatically][stack], and rustc now only allocates 8MB of stack
  instead of 32MB.

[range]: https://github.com/rust-lang/rust/pull/19858
[range-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0439-cmp-ops-reform.md
[ambig]: https://github.com/rust-lang/rfcs/blob/master/text/0520-new-array-repeat-syntax.md
[stack]: https://github.com/rust-lang/rust/pull/19898
[array]: https://github.com/rust-lang/rust/pull/20057
[array-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0520-new-array-repeat-syntax.md
[fmt]: https://github.com/rust-lang/rust/pull/20080
[fmt-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0504-show-stabilization.md

## New Contributors

* Brian J Brennan
* Florian Wilkens
* Johannes Hoff
* Maya Nitu
* Rolf Timmermans
* Simonas Kazlauskas

# Approved RFC's

None.

# New RFC's

* [Rename `int/uint` to `intx/uintx`][uint].

[uint]: https://github.com/rust-lang/rfcs/pull/544

# Community

## From the Team

It's Christmas time and holidays, so no meetings at all this week.

- [A Tale of Two's Complement](http://internals.rust-lang.org/t/a-tale-of-twos-complement/1062/1).
  [Reddit](http://www.reddit.com/r/rust/comments/2q40k2/a_tale_of_twos_complement/).


## Blog Posts

- [Using Rust from Perl and Julia][rust-perl-julia]. [Reddit][rust-perl-julia-reddit]
- [Rust, an anti-sloppy programming language][no-slop]. [Reddit][no-slop-reddit]
- [My thoughts on Rust in 2015][thoughts]. [Reddit][thoughts-reddit]
- [capnproto-rust's custom mutable reference types][and-mut]. [Reddit][and-mut-reddit]

[rust-perl-julia]: http://paul.woolcock.us/posts/rust-perl-julia-ffi.html
[rust-perl-julia-reddit]: http://www.reddit.com/r/rust/comments/2q76yn/using_rust_from_perl_and_julia/
[no-slop]: http://arthurtw.github.io/2014/12/21/rust-anti-sloppy-programming-language.html
[no-slop-reddit]: http://www.reddit.com/r/rust/comments/2q1e9f/rust_an_antisloppy_programming_language/
[thoughts]: http://featherweightmusings.blogspot.co.nz/2014/12/my-thoughts-on-rust-in-2015.html
[thoughts-reddit]: http://www.reddit.com/r/rust/comments/2qkgvu/my_thoughts_on_rust_in_2015/
[and-mut]: http://dwrensha.github.io/capnproto-rust/2014/12/27/custom-mutable-references.html
[and-mut-reddit]: http://www.reddit.com/r/rust/comments/2qibmh/capnprotorusts_custom_mutable_reference_types/

### The end of 24 Days of Rust

24 is too small!

- [Built with Rust][bwr]
- [Calling Rust from other languages][ffi]
- [The conclusion][the-end]

[bwr]: https://siciarz.net/24-days-of-rust-built-with-rust/
[ffi]: https://siciarz.net/24-days-of-rust-calling-rust-from-other-languages/
[the-end]: https://siciarz.net/24-days-of-rust-conclusion/

## Discussions

- [How's Rust for C100K?][c100k]
- [Thoughts on macros and syntax extensions][mac-syn-ext]
- [Dealing with `va_list` in FFI][va_list-ffi]
- [Why don't `if`/`else` expressions need to end with a `;` in Rust?][semicolon]


[c100k]: http://www.reddit.com/r/rust/comments/2q1xe4/hows_rust_for_c100k/
[mac-syn-ext]: http://www.reddit.com/r/rust/comments/2q83b9/thoughts_on_macros_and_syntax_extensions/
[va_list-ffi]: http://www.reddit.com/r/rust/comments/2qje69/ffi_dealing_with_va_list/
[semicolon]: http://www.reddit.com/r/rust/comments/2qjvzr/why_ifelse_expression_in_rust_doesnt_end_with_a/

## New Projects

- [Rust Conversion Reference][convert]. [Reddit][convert-reddit]
- [Rust-Net][rust-net]: a network stack in pure Rust. [Reddit][rust-net-reddit]
- [RACC][racc]: Rust Another Compiler-Compiler. [Reddit][racc-reddit]
- [nss-multipasswd][nss-multipasswd]: a glibc plugin for multiple passwd files
- [netaddr][netaddr]: Network addresses utilities for Rust
- [A data oriented Entity Component System in Rust][ecs]. [Reddit][ecs-reddit]

[convert]: http://carols10cents.github.io/rust-conversion-reference/
[convert-reddit]: http://www.reddit.com/r/rust/comments/2qfbog/merry_rustmas_a_rust_conversion_reference_for_you/
[rust-net]: https://github.com/Ericson2314/rust-net
[rust-net-reddit]: http://www.reddit.com/r/rust/comments/2qfuvz/a_network_stack_in_pure_rust/
[racc]: https://github.com/sivadeilra/racc
[racc-reddit]: http://www.reddit.com/r/rust/comments/2qewc0/racc_rust_another_compilercompiler/
[nss-multipasswd]: https://github.com/polachok/nss-multipasswd/
[netaddr]: https://crates.io/crates/netaddr
[ecs]: https://github.com/lholden/entity_system
[ecs-reddit]: http://www.reddit.com/r/rust/comments/2qh82p/a_data_oriented_entity_component_system_in_rust/

## Project Updates

* [This Week in Servo 16][twis]. [Reddit][twis-reddit]
* [A GUI was implemented for `img_dup`][img_dup-gui] using
  [Piston](http://www.piston.rs/) and
  [Conrod](https://github.com/PistonDevelopers/conrod). [Reddit][img_dup-gui-reddit]
* [Iron now supports unboxed closures][iron-without-boxes].
* [Superchan now has a simpler API with better documentation][superchan] (double bonus!)

[twis]: http://blog.servo.org/2014/12/23/twis-16/
[twis-reddit]: http://www.reddit.com/r/rust/comments/2qab98/this_week_in_servo_16/
[img_dup-gui]: https://github.com/cybergeek94/img_dup/blob/master/GUI.md
[img_dup-gui-reddit]: http://www.reddit.com/r/rust/comments/2qfozw/merry_belated_christmas_rustaceans_i_have_finally/
[iron-without-boxes]: http://www.reddit.com/r/rust/comments/2qhxyk/iron_now_supports_unboxed_closures/
[superchan]: http://www.reddit.com/r/rust/comments/2q2zzu/superchan_now_with_better_documentation_and_a/

## Upcoming Events

Nothing on [the calendar][calendar] until [the Seattle meetup][seattle] on 2015-01-12.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[seattle]: https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg
