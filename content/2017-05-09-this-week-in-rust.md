Title: This Week in Rust 181
Number: 181
Date: 2017-05-09
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

* [2017 State of Rust Survey](https://blog.rust-lang.org/2017/05/03/survey.html).
* [The Rust Libz Blitz](https://blog.rust-lang.org/2017/05/05/libz-blitz.html). Raising a solid core of the Rust crate ecosystem to a consistent level of completeness and quality as part of Rust language 2017 ergonomic improvements initiative.
* [David Tolnay (of Serde fame) joins the Rust libs team](https://internals.rust-lang.org/t/announcement-david-tolnay-joining-the-libs-team/5186).
* [How I broke Rust's package manager for all Windows users](http://sasheldon.com/blog/2017/05/07/how-i-broke-cargo-for-windows/).
* [The pain of real linear types in Rust](https://gankro.github.io/blah/linear-rust/).
* [The balance between cost, usability, and soundness in C bindings, and Rust-SDL2's release](https://cobrand.github.io/rust/sdl2/2017/05/07/the-balance-between-soundness-cost-useability.html).
* [gnome-class: Integrating Rust and the GNOME object system](http://smallcultfollowing.com/babysteps/blog/2017/05/02/gnome-class-integrating-rust-and-the-gnome-object-system/).
* [Fearless concurrency in your microcontroller](http://blog.japaric.io/fearless-concurrency/).
* [Rust on Teensy part 3: Improving safety](https://branan.github.io/teensy/2017/05/09/safety.html). PJRC Teensy is a USB-based microcontroller development system.
* [Ownership controls mutability](https://kasma1990.gitlab.io/2017/05/07/ownership-controls-mutability/).
* [Gtk-rs: New crates.io versions available and API improvements](http://gtk-rs.org/blog/2017/05/06/api-improvements.html).
* [Rust makes it into a AAA video game!... Sort of](https://twitter.com/alephtwo/status/860665238793986048).
* [This week in Rust docs 55](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-55).
* [This week in Servo 101](https://blog.servo.org/2017/05/08/twis-101/).

# Crate of the Week

This week's crate of the week is [remove_dir_all](https://crates.io/crates/remove_dir_all/), a safe, reliable implementation of `remove_dir_all` for Windows. Thanks to [brson](https://users.rust-lang.org/users/brson) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Crate evaluation for 2017-05-16: log](https://internals.rust-lang.org/t/crate-evaluation-for-2017-05-16-log/5185). People are needed to help fill out the log crate evaluation, write cookbook recipes for the log crate, and generally offer their opinions.
* [cross: OpenSSL missing for i686-musl](https://github.com/japaric/cross/issues/27). cross allows you to do "zero setup" cross compilation and "cross testing" of Rust crates.
* [cross: libmusl with static OpenSSL](https://github.com/japaric/cross/issues/21).
* [easy] [url: Implement Debug for many types](https://github.com/servo/rust-url/issues/305). Servo's `url` is a URL parser library for Rust.
* [url: Implement `Default` for `ParseOptions` and `ParseOptions::new`](https://github.com/servo/rust-url/issues/301).
* [easy] [url: Modify docs to put error conditions into `Errors` sections](https://github.com/servo/rust-url/issues/314).
* [url: Better documentation for quirks module](https://github.com/servo/rust-url/issues/311).
* [easy] [url: Improvements to `Origin` docs](https://github.com/servo/rust-url/issues/310).
* [easy] [url: Add examples to `Url` methods](https://github.com/servo/rust-url/issues/309).
* [easy] [url: Add examples to `ParseOptions`](https://github.com/servo/rust-url/issues/308).
* [url: Modify `define_encode_set` to support private definitions](https://github.com/servo/rust-url/issues/307).
* [url: Document the percent_encoding module](https://github.com/servo/rust-url/issues/298).
* [easy] [rust-cookbook: Switch error handling setup to quick_main! macro from error-chain](https://github.com/brson/rust-cookbook/issues/59). Rust Cookbook is a collection of simple examples that demonstrate good practices to accomplish common programming tasks.
* [memmap: Overhaul API](https://github.com/danburkert/memmap-rs/issues/33). memmap-rs is a Rust library for cross-platform memory-mapped file IO.
* [memmap: Document error conditions for fallible methods in a separate "Errors" section](https://github.com/danburkert/memmap-rs/issues/37).
* [memmap: Add examples to methods](https://github.com/danburkert/memmap-rs/issues/34).
* [memmap: Expand crate-level documentation](https://github.com/danburkert/memmap-rs/issues/32).
* [easy] [tokei: AutoHotKey support](https://github.com/Aaronepower/tokei/issues/106). Tokei is a program that displays statistics about your code.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

122 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-05-01..2017-05-08

* [`MutexGuard<T>` is now only `Sync` if `T` is](https://github.com/rust-lang/rust/pull/41624) (breaking change, fixes unsoundness)
* [far-ranging MIR pipeline changes](https://github.com/rust-lang/rust/pull/41625) (also adds "stealing" as a move violent form of borrowing)
* [fix arm-linux-androideabi](https://github.com/rust-lang/rust/pull/41656)
* [fix Windows' ULONG_PTR type on Windows 32 bit systems](https://github.com/rust-lang/rust/pull/41787)
* [Rust documenters can now write '`##`' to show `#` in rustdoc output](https://github.com/rust-lang/rust/pull/41785)
* [Windows doctests no longer deadlock](https://github.com/rust-lang/rust/pull/41769)
* [reworking `NonZero`, `Shared` and `Unique`](https://github.com/rust-lang/rust/pull/41064) (so they can be stabilized – fingers crossed)
* [`core::sync::atomic::hint_core_should_pause()` for optimized atomic spin loops](https://github.com/rust-lang/rust/pull/41207)
* [various LLVM backports](https://github.com/rust-lang/rust/pull/41739)
* [variance refactoring, no more `pub` map](https://github.com/rust-lang/rust/pull/41734)
* [on-demand-ify region mapping](https://github.com/rust-lang/rust/pull/41662)
* [remove ast-ty-to-ty cache](https://github.com/rust-lang/rust/pull/41733)
* [delete no longer used `syntax` features](https://github.com/rust-lang/rust/pull/41729)
* [remove unused `TyCtxt` fields](https://github.com/rust-lang/rust/pull/41754)
* [suggest `!` for bitwise negation when encountering `~` prefix](https://github.com/rust-lang/rust/pull/41722) (ease transition from C)
* [point to fields that make type recursive](https://github.com/rust-lang/rust/pull/40857)
* [lint against](https://github.com/rust-lang/rust/pull/41692) and [remove anonymous parameters](https://github.com/rust-lang/rust/pull/41693) (deprecated soon)
* [remove obsolete `--disable-elf-tls` switch](https://github.com/rust-lang/rust/pull/41687)
* [forbid `-Z` on stable and beta](https://github.com/rust-lang/rust/pull/41751)
* [the rust build process can now generate XZ-compressed tarballs](https://github.com/rust-lang/rust/pull/41600)
* [rust-installer will use available `xz` or `7z`](https://github.com/rust-lang/rust-installer/pull/60)
* [`cargo bench --all`](https://github.com/rust-lang/cargo/pull/3988)

## New Contributors

* acdenisSK
* Bobby Holley
* Charlie Sheridan
* Jing Zhao
* Joshua Sheard
* Masaki Hara
* Migi
* Raphaël Huchet
* Tommy Ip

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: close] [Allow any Displayable type for expect](https://github.com/rust-lang/rfcs/pull/1968).
* [disposition: merge] [Expand and stabilize `impl Trait`](https://github.com/rust-lang/rfcs/pull/1951).
* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).

## New RFCs

* [`Result::pass()`, turning `Result<T,E>` into `Result<U,F>`, if `From` is set up](https://github.com/rust-lang/rfcs/pull/1996).
* [Opaque Data structs for FFI](https://github.com/rust-lang/rfcs/pull/1993).
* [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [Match branch semicolon](https://github.com/rust-lang/rfcs/pull/1994).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

PRs:

* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/pull/81)

Issues in final comment period:

* [Ordering of types of groups within a module](https://github.com/rust-lang-nursery/fmt-rfcs/issues/71)
* [Struct and tuple literals](https://github.com/rust-lang-nursery/fmt-rfcs/issues/64)
* [Array literals](https://github.com/rust-lang-nursery/fmt-rfcs/issues/63)
* [Where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)
* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)

Other interesting issues:

* [Function calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/64) and [chains of calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/66)
* [Combining opening and closing delims](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)

# Upcoming Events

* [May 10. Rust Rome - Rust Meetup #2 - Intro + Rocket.rs](https://www.meetup.com/it-IT/Rust-Roma/events/239513275/).
* [May 10. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 10. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 11. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlywhbpb/).
* [May 11. Rust DC - Building high performance REST APIs with Rust and Rocket](https://www.meetup.com/RustDC/events/239115583/).
* [May 13. Mozilla Philippines - Introduction to Rust (Programming Language)](https://www.eventbrite.com/e/introduction-to-rust-programming-language-tickets-33749248912).
* [May 15. Rust Sydney Meetup - Happy Birthday Rust](https://www.meetup.com/Rust-Sydney/events/239659974/)!
* [May 16. Tokyo Rust Meetup #7 - Rust Birthday Party](https://www.meetup.com/Tokyo-Rust-Meetup/events/239301821/)!
* [May 16. Rust Paris Meetup #37](https://www.meetup.com/Rust-Paris/events/239723704/).
* [May 16. 1st Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/239688416/).
* [May 17. Rust LA May Meetup - Rust Birthday Party](https://www.meetup.com/Rust-Los-Angeles/events/239616841/)!
* [May 17. South Florida Rust - Rust Birthday Party](https://www.meetup.com/South-Florida-Rust-Meetup/events/239036595/)!
* [May 17. Rust Atlanta - Heterogeneous Collections in Rust at Tech Square Labs (Midtown)](https://www.meetup.com/Rust-ATL/events/239205124/).
* [May 17. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/239666001/).
* [May 17. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 17. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 18. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [May 20. Rust Bangalore community meetup](https://www.eventshigh.com/detail/Bangalore/9a49c6be73b6591e770d1cece7eec6fe-Rust-Bangalore-First-Meetup).
* [May 24. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 24. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The answer is obvious: it's the intersection of trust and frustration.

— [/u/kibwen answering - What does Rust mean?](https://www.reddit.com/r/rust/comments/69zca2/what_does_rust_mean/dhb20yb/).

Thanks to [Jean](https://github.com/cmr/this-week-in-rust/issues/438#issuecomment-300025367) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
