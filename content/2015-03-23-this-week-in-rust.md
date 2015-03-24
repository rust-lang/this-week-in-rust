Title: This Week in Rust 75
Date: 2015-03-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

79 pull requests were [merged in the last week][merged], and 9 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-16..2015-03-23
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-16..2015-03-23

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://rawgit.com/mrmonday/bitrust/gh-pages/index.html

# Breaking Changes

* [Fixed-size byte string literals][byte]. [RFC][byte-rfc].
* [allow inherent implementations on primitives, remove some extension traits][impl].
* [Stabilize `std::net`][net].
* [Deprecate range, range_step, count, distributions][range].
* [Stabilize `io::ErrorKind`][err].
* [Remove old_io and old_path from the prelude][old].
* [Require braces when a closure has an explicit return type][close].
* [`RUST_TEST_TASKS` -> `RUST_TEST_THREADS`][test].

[byte]: https://github.com/rust-lang/rust/pull/22838
[byte-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0339-statically-sized-literals.md
[impl]: https://github.com/rust-lang/rust/pull/23104
[net]: https://github.com/rust-lang/rust/pull/23352
[range]: https://github.com/rust-lang/rust/pull/23347
[err]: https://github.com/rust-lang/rust/pull/23430
[old]: https://github.com/rust-lang/rust/pull/23470
[close]: https://github.com/rust-lang/rust/pull/23475
[test]: https://github.com/rust-lang/rust/pull/23525

# Other Changes

* [Implement `thread::sleep`][sleep].

[sleep]: https://github.com/rust-lang/rust/pull/23330

# New Contributors

* Johannes Oertel
* kjpgit
* Nicholas
* Paul ADENOT
* Sae-bom Kim
* Tero Hänninen

# Approved RFCs

* [RFC 529: Generic conversion traits][conv].
* [RFC 803: Type ascription][asc].
* [RFC 909: Move `std::thread_local` to `std::thread::local`][local].
* [RFC 921: Entry API v3][entry].
* [RFC 940: Disallow hyphens in crate names][hype].
* [RFC 968: Tweak closure return type syntax][close].

[conv]: https://github.com/rust-lang/rfcs/blob/master/text/0529-conversion-traits.md
[asc]: https://github.com/rust-lang/rfcs/blob/master/text/0803-type-ascription.md
[local]: https://github.com/rust-lang/rfcs/blob/master/text/0909-move-thread-local-to-std-thread.md
[entry]: https://github.com/rust-lang/rfcs/blob/master/text/0921-entry_v3.md
[hype]: https://github.com/rust-lang/rfcs/blob/master/text/0940-hyphens-considered-harmful.md
[close]: https://github.com/rust-lang/rfcs/blob/master/text/0968-closure-return-type-syntax.md

# New RFCs

* [Add std::env::concurrency_hint][conc].
* [Deprecate `CharExt::to_{upper,lower}case`][low].
* [Make type ascription expressions lvalues][asc].
* [Function pointers reform][fn].
* [Add `AtomicI32` and `AtomicU32`][atom].
* [Allow unstable features in 1.0][unst].

[unst]: https://github.com/rust-lang/rfcs/pull/1007
[atom]: https://github.com/rust-lang/rfcs/pull/1000
[fn]: https://github.com/rust-lang/rfcs/pull/996
[asc]: https://github.com/rust-lang/rfcs/pull/987
[low]: https://github.com/rust-lang/rfcs/pull/986
[conc]: https://github.com/rust-lang/rfcs/pull/985

# Notable Links

* [Would Rust have prevented Heartbleed? Another look][heart]. Bascule
  comes to Rust's defense! [/r/rust][heart-r]. [HN][heart-hn].
* [Rust infrastructure can be your infrastructure][inf].
* [Weekly-meetings/2015-03-17][mtg]. Checked overflow and casts; hyphens in crate names.
* [Rust programming language presentation at Sungkyunkwan University, South Korea][sk].
* [What is Rust bad at?][bad]
* Roguelike game architecture in Rust, parts [1][r1], [2][r2], [3][r3].
* [Using Rust from the Unreal engine][unreal].

[unreal]: https://www.reddit.com/r/rust_gamedev/comments/2zw3e1/using_rust_from_the_unreal_engine/
[r3]: http://rsaarelm.github.io/Roguelike-architecture-in-Rust-3/
[r2]: http://rsaarelm.github.io/Roguelike-architecture-in-Rust-2/
[r1]: http://rsaarelm.github.io/Roguelike-architecture-in-Rust-1/
[bad]: https://www.reddit.com/r/rust/comments/2zu3eo/what_is_rust_bad_at/
[sk]: http://www.slideshare.net/jaejukim9/rust-programming-language
[inf]: http://huonw.github.io/blog/2015/03/rust-infrastructure-can-be-your-infrastructure/
[heart]: http://tonyarcieri.com/would-rust-have-prevented-heartbleed-another-look
[heart-r]: https://www.reddit.com/r/rust/comments/2zd797/would_rust_have_prevented_heartbleed_another_look/
[heart-hn]: https://news.ycombinator.com/item?id=9219432
[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-03-17.md

# Project Updates

* [gfx_scene]. High-level rendering for [gfx-rs].
* [glium project update, march edition][glium].
* [html5ever project update: one year!][html5ever].
* [capnproto-rust: error handling revisited][cap]. David Renshaw has
  recently modernized Cap'n'Proto's error handling.
* [google-apis-rs] has been relased with bindings for many Google APIs.
* [rust-backtrace]. Klutzy is implementing backtracing in Rust.
* [Playform gets voxel terrain][vox] and [even better voxel terrain][vox2].

[glium]: https://www.reddit.com/r/rust_gamedev/comments/304268/glium_project_update_march_edition/
[gfx-rs]: https://github.com/gfx-rs/gfx-rs
[gfx_scene]: https://www.reddit.com/r/rust_gamedev/comments/2zzxhb/show_rgd_gfx_scene_high_level_rendering_for_gfxrs/
[vox]: https://www.reddit.com/r/rust_gamedev/comments/2zzf8v/wip_playform_gets_voxel_terrain/
[vox2]: https://www.reddit.com/r/rust_gamedev/comments/304s11/improved_soontobe_destructible_voxel_terrain_in/
[google-apis-rs]: https://www.reddit.com/r/rust/comments/300c49/google_apis_for_rust_v010_released_on_cratesio/
[html5ever]: http://mainisusuallyafunction.blogspot.com/2015/03/html5ever-project-update-one-year.html
[cap]: http://dwrensha.github.io/capnproto-rust/2015/03/21/error-handling-revisited.html
[rust-backtrace]: https://github.com/klutzy/rust-backtrace/tree/tmp

# Upcoming Events

* [4/1 Amsterdam Hack Night][am].

[am]: http://www.meetup.com/Rust-Amsterdam/events/220668018/

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

```
<mbrubeck> the 5 stages of loss and rust
<mbrubeck> 1. type check. 2. borrow check. 3. anger. 4. acceptance. 5. rust upgrade
```

Thanks to jdm for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
