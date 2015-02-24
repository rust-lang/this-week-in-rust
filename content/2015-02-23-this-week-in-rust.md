Title: This Week in Rust 71
Date: 2015-02-23
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

# The big news

Rust [1.0.0-alpha.2] was released on Friday, but keep using
nightlies. Six more weeks until the beta, which should become
1.0. *Only* six more weeks.

[1.0.0-alpha.2]: http://blog.rust-lang.org/2015/02/20/Rust-1.0-alpha2.html

# What's cooking on master?

157 pull requests were [merged in the last week][merged], and 15 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-16..2015-02-23
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-16..2015-02-23

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* A [variety of tweaks][thread] have been made to `std::thread` before
  declaring it stable.
* The `vec!` macro accepts a new form, `vec![element; len]`, which
  [produces a vector containing a number of clones of the same
  element][elem].
* The `CString` type has seen a few modifications, as well as the
  [introduction of the `CStr` type][cstr].
* `IntoIterator` now has an [associated type for specifying the element
  type][into].
* `ExactSizeIterator` is [no longer implemented][exact] for 64-bit
  ranges.
* In target specs, `target-word-size` [is now called
  `target-pointer-width`][tpw], to match the recently changed
  `target_pointer_width` cfg attribute.
* [`fmt::Writer` was renamed to `fmt::Write`][write].
* A [number][audit1] of [modules][audit2] have been [audited][audit3]
  for incorrect use of `isize` and `usize`.
* Use of arbitrary attributes not defined by the language [is feature
  gated][attr].

[cstr]: https://github.com/rust-lang/rust/pull/22482
[elem]: https://github.com/rust-lang/rust/pull/22455
[thread]: https://github.com/rust-lang/rust/pull/22435
[tpw]: https://github.com/rust-lang/rust/pull/22191
[exact]: https://github.com/rust-lang/rust/pull/22299
[write]: https://github.com/rust-lang/rust/pull/22311
[into]: https://github.com/rust-lang/rust/pull/22313
[audit1]: https://github.com/rust-lang/rust/pull/22339
[audit2]: https://github.com/rust-lang/rust/pull/22401
[audit3]: https://github.com/rust-lang/rust/pull/22485
[attr]: https://github.com/rust-lang/rust/pull/22364

## Other Changes

* [`Send` no longer requires `'static`][send], which make it possible
  to express various data-parallel scenarious by sharing interior
  pointers. [RFC][send-rfc].
* The ['Macros'] chapter of TRPL has been overhauled and a new
  ['Advanced Macros'] chapter added.
* `rustc --version` now reports the [build date] in addition to the
  commit date, to make it more clear what nightly you are on.
* Florian Hahn [added a new category of 'parse-fail' tests][parse] to
  the test suite. This makes it easier to compare model parsers to the
  production parser.

[send]: https://github.com/rust-lang/rust/pull/22319
[send-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0458-send-improvements.md
[build date]: https://github.com/rust-lang/rust/pull/22201
[parse]: https://github.com/rust-lang/rust/pull/22118
['Macros']: http://doc.rust-lang.org/book/macros.html
['Advanced Macros']: http://doc.rust-lang.org/book/advanced-macros.html

## New Contributors

* Adam Jacob
* Alexander Bliskovsky
* Brian Brooks
* caipre
* Darrell Hamilton
* Dave Huseby
* Denis Defreyne
* Elantsev Serj
* Henrik Schopmans
* Ingo Blechschmidt
* Jormundir
* Lai Jiangshan
* posixphreak
* Ryan Riginding
* Wesley Wiser
* Will
* wonyong kim

## Approved RFCs

This covers two weeks since last week I wasn't able review RFCs in time.

* [RFC 458: Improve the Send trait][rfc-458].
* [RFC 505: API comment conventions][rfc-505].
* [RFC 528: Add a generic string pattern matching API][rfc-528].
* [RFC 563: Remove ndebug][rfc-563].
* [RFC 572: Feature gate unused attributes][rfc-572].
* [RFC 580: Rename some std colloctions][rfc-580].
* [RFC 592: CStr][rfc-592].
* [RFC 593: Forbid `Self` identifier][rfc-593].
* [RFC 599: Default object lifetime bounds][rfc-599].
* [RFC 601: Replace `be` with `become`][rfc-601].
* [RFC 735: Lift restrictions on defition site of inherent impls][rfc-735].
* [RFC 736: Privacy-respecting FRU][rfc-736].
* [RFC 738: Variance for type params][rfc-738].
* [RFC 769: Sound generic drop][rfc-769].
* [RFC 809: `box` and placement `in`][rfc-809].
* [RFC 823: Simplify `std::hash`][rfc-823].
* [RFC 832: Add back `Vec::from_elem`][rfc-832].
* [RFC 840: No panic in CString][rfc-840].

[rfc-458]: https://github.com/rust-lang/rfcs/blob/master/text/0458-send-improvements.md
[rfc-505]: https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md
[rfc-528]: https://github.com/rust-lang/rfcs/blob/master/text/0528-string-patterns.md
[rfc-563]: https://github.com/rust-lang/rfcs/blob/master/text/0563-remove-ndebug.md
[rfc-572]: https://github.com/rust-lang/rfcs/blob/master/text/0572-rustc-attribute.md
[rfc-580]: https://github.com/rust-lang/rfcs/blob/master/text/0580-rename-collections.md
[rfc-592]: https://github.com/rust-lang/rfcs/blob/master/text/0592-c-str-deref.md
[rfc-593]: https://github.com/rust-lang/rfcs/blob/master/text/0593-forbid-Self-definitions.md
[rfc-599]: https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md
[rfc-601]: https://github.com/rust-lang/rfcs/blob/master/text/0601-replace-be-with-become.md
[rfc-735]: https://github.com/rust-lang/rfcs/blob/master/text/0735-allow-inherent-impls-anywhere.md
[rfc-736]: https://github.com/rust-lang/rfcs/blob/master/text/0736-privacy-respecting-fru.md
[rfc-738]: https://github.com/rust-lang/rfcs/blob/master/text/0738-variance.md
[rfc-769]: https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md
[rfc-809]: https://github.com/rust-lang/rfcs/blob/master/text/0809-box-and-in-for-stdlib.md
[rfc-823]: https://github.com/rust-lang/rfcs/blob/master/text/0823-hash-simplification.md
[rfc-832]: https://github.com/rust-lang/rfcs/blob/master/text/0832-from-elem-with-love.md
[rfc-840]: https://github.com/rust-lang/rfcs/blob/master/text/0840-no-panic-in-c-string.md

## New RFCs

* [`rustdoc` or `cargo doc` should pass `--cfg doc`][doc].
* [Never allow reads from uninitialized memory in safe Rust][never].
* [Have collections impl Extend<&T> where T: Clone][extend].
* [Changing struct literals][struct].
* [Item grouping][group].
* [Put `div_rem` back in the standard library][rem].
* [New struct syntax][new].
* [Use globs as][as].
* [Remove lifetime elision in type parameter position][el].
* [Replace IteratorExt::zip with tuple iteration][tup].
* [Allow macros in types][mactype].
* [Lex binary and octal literals more eagerly][lex]
* [Improvements to range match patterns][range].
* [Make function pointer types look like borrowed pointer types for forwards compatability][fn].
* [Allow types to be parameterized by integer (and bool) constant values][bounds].
* [Allow `#[must_use]` on functions, rather than just types. Mark `Result::{ok,err}` `#[must_use]`][must].
* [Add single-threaded fences][fence].
* [Custom preludes][prelude].
* [Approx asserts][approx].

[doc]: https://github.com/rust-lang/rfcs/pull/834
[never]: https://github.com/rust-lang/rfcs/pull/837
[extend]: https://github.com/rust-lang/rfcs/pull/839
[struct]: https://github.com/rust-lang/rfcs/pull/841
[group]: https://github.com/rust-lang/rfcs/pull/849
[rem]: https://github.com/rust-lang/rfcs/pull/850
[new]: https://github.com/rust-lang/rfcs/pull/866
[as]: https://github.com/rust-lang/rfcs/pull/867
[el]: https://github.com/rust-lang/rfcs/pull/869
[tup]: https://github.com/rust-lang/rfcs/pull/870
[mactype]: https://github.com/rust-lang/rfcs/pull/873
[lex]: https://github.com/rust-lang/rfcs/pull/879
[range]: https://github.com/rust-lang/rfcs/pull/880
[fn]: https://github.com/rust-lang/rfcs/pull/883
[bounds]: https://github.com/rust-lang/rfcs/pull/884
[must]: https://github.com/rust-lang/rfcs/pull/886
[fence]: https://github.com/rust-lang/rfcs/pull/888
[prelude]: https://github.com/rust-lang/rfcs/pull/890
[approx]: https://github.com/rust-lang/rfcs/pull/897

# Friend of the Tree

The Rust Team likes to occassionally recognize people who have made
outstanding contributions to The Rust Project, its ecosystem, and its
community. These people are 'friends of the tree'.

This week's friend of the tree was ... Toby Scrace.

"Today I would like to nominate Toby Scrace as Friend of the
Tree. Toby emailed me over the weekend about a login vulnerability on
crates.io where you could log in to whomever the previously logged in
user was regardless of whether the GitHub authentication was
successful or not. I very much appreciate Toby emailing me privately
ahead of time, and I definitely feel that Toby has earned becoming
Friend of the Tree."

# Quote of the Week

```
<Manishearth> In other news, I have r+ on rust now :D
<Ms2ger> No good deed goes unpunished
```

[From #servo](http://logs.glob.uno/?c=mozilla%23servo&s=21+Feb+2015&e=21+Feb+2015&h=In+other+news#c175503). Thanks to SimonSapin for the tip.

# Notable Links

* [Weekly-meetings/2015-02-17][mtg]. fott; security bugs; code completion; open-ended proposals; struct syntax; alpha2; integer suffixes; overflow.
* [McPherrin succeeds at transcribing two hours of meetup talks, but
  at great cost][mc].
* [Radical statements about the mobile web][radical]. Servo is going to fix it al.
* [Embedded Rust Right Now!][now].
* [On Rust and Nim][nim]. [HN][nim-hn].
* [Rust Debuging in Emacs][emacs].
* [Thoughts of a Rustacean learning Go][go]. [/r/rust][go-r-rust].
* [Some notes on Send and Sync][sendand].
* [Turing tarpits in Rust's macro system][tarp].
* [Bay Area Rust Meetup: Blocking and Async I/O][air]. Video.
* [Memory management in Oxischeme][oxy].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-02-17.md
[mtg-reddit]:
[mc]: http://i.gyazo.com/bb91d17de95587ccbbf146dc5c638e85.png
[radical]: http://jlongster.com/Radical-Statements-about-the-Mobile-Web
[nim]: https://andreaferretti.github.io/on-rust-and-nim/
[nim-hn]: https://news.ycombinator.com/item?id=9088640
[now]: http://spin.atomicobject.com/2015/02/20/rust-language-c-embedded/
[emacs]: https://bjbell.wordpress.com/2015/02/17/emacs-rust-debugging/
[go]: http://inpursuitoflaziness.blogspot.in/2015/02/thoughts-of-rustacean-learning-go.html
[go-r-rust]: https://www.reddit.com/r/rust/comments/2wj6fh/thoughts_of_a_rustacean_learning_go/
[sendand]: http://huonw.github.io/blog/2015/02/some-notes-on-send-and-sync/
[tarp]: http://mainisusuallyafunction.blogspot.com/2015/02/turing-tarpits-in-rusts-macro-system.html
[air]: https://air.mozilla.org/bay-area-rust-meetup-february-2015/
[oxy]: http://fitzgeraldnick.com/weblog/60/

# Project Updates

* [Raft Update 2: Hacking the log][raft].
* [Rewriting Rust serialization part 2][ser].
* [rustdox.com]. Rust documentation hosting.
* [var]. A macro for declaring multiple mutable variables at once.
* [rexiv2]. A library for reading and writing EXIF data.
* There was a good HackerNews thread about [exa], the replacement for `ls` written in Rust. Also
  on [/r/rust][exa-r-rust].
* [rust-netbeans]. Rust plugin for the NetBeans IDE, with Cargo support.
* [This Week in Servo 24][twis].
* [rtracker]. A bittorrent tracker in Rust.
* [dns2]. A DNS library.
* [rust-media]. Portable media player framework.
* [ipc]. Clone of ipcalc.
* [multilist]. Safe intrusive doubly-linked lists.
* [rustle]. Install Cargo apps without installing Rust.
* [xsv]. BurntSushi's CSV toolkit.
* [suffix]. Suffix arrays for fast searching.
* [Rust Share]. Share to play.rust-lang.org from SublimeText.
* [chess]. Written in Rust!

[var]: http://users.rust-lang.org/t/ann-var-is-a-macro-for-declaring-multiple-mutable-variables-at-once/370
[rexiv2]: http://users.rust-lang.org/t/announcing-rexiv2-library-for-image-metadata-request-for-code-review/414
[exa]: https://news.ycombinator.com/item?id=9087108
[exa-r-rust]: https://www.reddit.com/r/rust/comments/2wp3pp/ive_added_loads_more_features_to_exa_my_ls/
[rust-netbeans]: https://github.com/drrb/rust-netbeans
[raft]: http://www.hoverbear.org/2015/02/18/raft-update-2/
[ser]: http://erickt.github.io/blog/2015/02/13/rewriting-rust-serialization-there-can-be-only-one-serde/
[twis]: http://blog.servo.org/2015/02/18/twis-24/
[rtracker]: https://github.com/brutal-chaos/rtracker
[dns2]: https://github.com/mahkoh/dns2
[rustdox.com]: https://www.reddit.com/r/rust/comments/2wku7e/introducing_rustdoxcom_alpha_an_easy_way_to_host/
[rust-media]: https://github.com/pcwalton/rust-media
[ipc]: https://github.com/mfs/ipc
[multilist]: https://github.com/pcwalton/multilist
[rustle]: https://github.com/brson/rustle
[xsv]: https://www.reddit.com/r/rust/comments/2wrtjn/a_fast_csv_toolkit_written_in_rust/
[suffix]: https://github.com/BurntSushi/suffix
[Rust Share]: https://github.com/GravityScore/Rust-Share
[chess]: https://github.com/Yayformee/chess

# Upcoming Events

* [Feb 26. Rust NY][ny].

[ny]: http://www.meetup.com/Rust-NYC/

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com
