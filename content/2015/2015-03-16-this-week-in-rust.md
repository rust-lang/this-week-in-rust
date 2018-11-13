Title: This Week in Rust 74
Date: 2015-03-16
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

99 pull requests were [merged in the last week][merged], and 3 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-09..2015-03-16
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-09..2015-03-16

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://rawgit.com/mrmonday/bitrust/gh-pages/index.html

# Breaking Changes

* [Rename `should_fail` to `should_panic`][fail].
* [Remove `?Sized` bounds from many I/O functions][sized].
* [Stabilize portions of `std::ios::$platform`][os].
* [Stabilize `std::io`][io].
* [Stabilize `std::path`][path].

[sized]: https://github.com/rust-lang/rust/pull/23316
[fail]: https://github.com/rust-lang/rust/pull/21824
[os]: https://github.com/rust-lang/rust/pull/23353
[io]: https://github.com/rust-lang/rust/pull/23292
[path]: https://github.com/rust-lang/rust/pull/23229

# Other Changes

* [Improvements to `Debug` formatting][fmt]. [RFC][fmt-rfc].

[fmt]: https://github.com/rust-lang/rust/pull/23162
[fmt-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0640-debug-improvements.md

# New Contributors

* John Zhang
* Ricardo Martins
* Trent Nadeau

# Approved RFCs

* [RFC 803: Type ascription][asc].

[asc]: https://github.com/rust-lang/rfcs/blob/master/text/0803-type-ascription.md

# New RFCs

* [DST custom coercions][dst].
* [read_all].
* [Align the count parameter of splitn with other languages][splitn].
* [Add unsized return values][unsized].
* [Generalize over mut and non-mut items][gen].
* [Make std::io iterators more convenient][iter].
* [Add material to stdio handling][stdio].
* [Add back bufferless read_to_string/end methods][buf].
* [Change Seek to be less enum-y][seek].
* [Make rustc and cargo produce optimized binaries by default][opt].
* [Add &own T][own].

[dst]: https://github.com/rust-lang/rfcs/pull/982
[read_all]: https://github.com/rust-lang/rfcs/pull/980
[splitn]: https://github.com/rust-lang/rfcs/pull/979
[unsized]: https://github.com/rust-lang/rfcs/pull/977
[gen]: https://github.com/rust-lang/rfcs/pull/976
[iter]: https://github.com/rust-lang/rfcs/pull/974
[stdio]: https://github.com/rust-lang/rfcs/pull/973
[buf]: https://github.com/rust-lang/rfcs/pull/970
[seek]: https://github.com/rust-lang/rfcs/pull/969
[opt]: https://github.com/rust-lang/rfcs/pull/967
[own]: https://github.com/rust-lang/rfcs/pull/965

# Notable Links

* [A Swift Guide to Rust][swift]. Good intro for systems programmers.
* [Weekly-meetings/2015-03-10][mtg].
* [Learning Cap'n'Proto RPC][cap].
* [Tutorial: how to collect test coverages for Rust
  project][cov]. lifthrasiir has gotten kcov working with Rust.

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-03-10.md
[cap]: http://www.hoverbear.org/2015/03/09/learning-capn-proto-rpc/
[swift]: http://faq.sealedabstract.com/rust/
[cov]: http://users.rust-lang.org/t/tutorial-how-to-collect-test-coverages-for-rust-project/650

# Project Updates

* [Phage]. A 7-day roguelike with pretty graphics.
* [This Week in Servo 27][twis].
* [urlp]. A simple CLI for parsing URLs.
* [solicit]. An HTTP/2 library.
* [google-apis-rs]. Bindings to many Google APIs.
* [handmade_hero_nostd]. An implementation of [Handmade Hero](https://handmadehero.org/).
* [crc-rs]. CRC32 and CRC64.
* [titanium]. A 64-bit kernel for ARM.

[Phage]: https://www.reddit.com/r/rust_gamedev/comments/2z01t0/phage_a_completed_7day_roguelike_in_rust/
[twis]: http://blog.servo.org/2016/03/11/twis-27/
[urlp]: https://github.com/clayallsopp/urlp
[solicit]: https://www.reddit.com/r/rust/comments/2ytj97/an_http2_library_in_rust/
[google-apis-rs]: https://www.reddit.com/r/rust/comments/2yxjbp/rfc_google_rust_client_apis/
[handmade_hero_nostd]: https://www.reddit.com/r/rust/comments/2z3i26/my_personal_implementation_of_handmade_hero_in/
[crc-rs]: https://www.reddit.com/r/rust/comments/2z6m9c/rust_implementation_of_crc32_64/
[titanium]: https://www.reddit.com/r/rust/comments/2z71vz/a_far_from_complete_kernel_in_rust_armv8_aarch64/

# Upcoming Events

* [3/16 Rust Paris][paris].
* [3/18 Copenhagen Tech Polyglot Meetup][cope].
* [4/1 Amsterdam Hack Night][am].

[paris]: http://www.meetup.com/Rust-Paris
[cope]: http://www.meetup.com/Copenhagen-Tech-Polyglots/events/220800093/
[am]: http://www.meetup.com/Rust-Amsterdam/events/220668018/

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

```text
< reem> I'm quite interested in discovering this HTTP/2 library, but I can't
        bring myself to read four paragraphs of small caps
```

In reference to last week's [celebration] of Terry Pratchett on /r/rust.

Thanks to bstrie for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
[celebration]: https://www.reddit.com/r/rust/comments/2yuumb/ok_what_happened_to_the_font_on_this_sub/