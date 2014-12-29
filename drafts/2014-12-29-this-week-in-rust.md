Title: This Week in Rust XX
Date: YYYY-MM-DD
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

XXX pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-12-01..2014-12-08

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes



## Other Changes



## New Contributors



# Approved RFC's



# New RFC's



# Community

## From the Team

It's Christmas time and holidays, so no meetings at all this week.

- [A Tale of Two's Complement](http://discuss.rust-lang.org/t/a-tale-of-twos-complement/1062/1).
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
