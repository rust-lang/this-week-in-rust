Title: This Week in Rust 208
Number: 208
Date: 2017-11-14
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

# Crate of the Week

This week's crate is [failure](https://github.com/withoutboats/failure), a crate to deal with... you guessed it, failure. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help us benchmark incremental compilation](https://internals.rust-lang.org/t/help-us-benchmark-incremental-compilation/6153)!
* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [MusicBrainz crate is looking for maintainers](https://www.reddit.com/r/rust/comments/7a2nq4/looking_for_potential_maintainer_to_musicbrainz/).
* [docs] [rust-ffi-guide: CString.as_ptr() problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/50).
* [docs] [rust-ffi-guide: Enum from integer is UB problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/51).
* [docs] [rust-ffi-guide: More examples of UB and FFI footguns](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/52).
* [docs] [rust-ffi-guide: Beware of allocators mismatch](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/53).
* [docs] [rust-ffi-guide: Rust enums and tagged unions](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/54).
* [docs] [rust-ffi-guide: Unwinding problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/49).
* [docs] [rust-ffi-guide: Linker errors and name mangling problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/48).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

137 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-11-06..2017-11-13

* [add `Option::filter()`](https://github.com/rust-lang/rust/pull/45863) (RFC [#2124](https://github.com/LukasKalbertodt/rfcs/blob/8857fc3aa021058084e2a16af457e43249cc50ce/text/2124-option-filter.md)
* [refactor Option::filter method](https://github.com/rust-lang/rust/pull/45933)
* [MIR-borrowck: fix diagnostics for closures](https://github.com/rust-lang/rust/pull/45927)
* [compiletest: Fix a couple of test re-run issues](https://github.com/rust-lang/rust/pull/45917)
* [fix test case header parsing code in presence of multiple revisions](https://github.com/rust-lang/rust/pull/45914)
* [rustbuild: Disable ThinLTO for libtest](https://github.com/rust-lang/rust/pull/45908)
* [make saturating u128 -> f32 casts the default behavior](https://github.com/rust-lang/rust/pull/45900)
* [check::method - unify receivers before normalizing method signatures](https://github.com/rust-lang/rust/pull/45890)
* [fix core for targets with max-atomic-width = 0](https://github.com/rust-lang/rust/pull/45882)
* [restore move out dataflow, add report of move out errors](https://github.com/rust-lang/rust/pull/45877)
* [implement arbitrary_self_types](https://github.com/rust-lang/rust/pull/45870)
* [incr.comp.: Verify stability of incr. comp. hashes and clean up various other things](https://github.com/rust-lang/rust/pull/45867)
* [disable `mmap` in `libbacktrace` on Apple platforms](https://github.com/rust-lang/rust/pull/45866)
* [fix help for duplicated names: `extern crate (...) as (...)`](https://github.com/rust-lang/rust/pull/45856)
* [disable LLVM assertions on Nightly, enable them in "alt" builds](https://github.com/rust-lang/rust/pull/45810)
* [make positional argument error in format! clearer](https://github.com/rust-lang/rust/pull/45807)
* [deduplicate projection error (E0271) messages](https://github.com/rust-lang/rust/pull/45952)
* [add missing div and rem vector intrinsics](https://github.com/rust-lang/rust/pull/45804)
* [prefer libproc_macro APIs to libsyntax ones in the quasi-quoter](https://github.com/rust-lang/rust/pull/45791)
* [fixes to MIR effectck](https://github.com/rust-lang/rust/pull/45785)
* [display all emission types in error msg if user inputs invalid option](https://github.com/rust-lang/rust/pull/45782)
* [accept interpolated patterns in trait method parameters](https://github.com/rust-lang/rust/pull/45775)
* [add error for `...` in expressions](https://github.com/rust-lang/rust/pull/45773)
* [resolve: Use same rules for disambiguating fresh bindings in `match` and `let`](https://github.com/rust-lang/rust/pull/45050)
* [change MIR dump filenames from `rustc.nodeN...` to `rustc.<DefPath>`](https://github.com/rust-lang/rust/pull/45757)
* [fix MIR CopyPropagation errneously propagating assignments to function args](https://github.com/rust-lang/rust/pull/45753)
* [handle anon lifetime arg being returned with named lifetime return type](https://github.com/rust-lang/rust/pull/45751)
* [refactor internal suggestion API](https://github.com/rust-lang/rust/pull/45741)
* [extend NLL with preliminary support for free regions on functions](https://github.com/rust-lang/rust/pull/45668)
* [allow overriding the thread-local statics model](https://github.com/rust-lang/rust/pull/45666)
* [use a `Set<T>` instead of a `Map<T, bool>`](https://github.com/rust-lang/rust/pull/45736)
* [regenerate libcore/char_private.rs](https://github.com/rust-lang/rust/pull/45571)
* [detect `=` → `:` typo in let bindings](https://github.com/rust-lang/rust/pull/45452)
* [forbid casting to/from a pointer of unknown kind](https://github.com/rust-lang/rust/pull/45735)
* [working towards a libc-less (wasm32) libstd](https://github.com/rust-lang/rust/pull/45725)
* [rustc: add item name to deprecated lint warning](https://github.com/rust-lang/rust/pull/45707)
* [RwLock guards are Sync if T is](https://github.com/rust-lang/rust/pull/45682)
* [remove `T: Sized` on pointer `as_ref()` and `as_mut()`](https://github.com/rust-lang/rust/pull/44932)
* [impl FromIterator<()> for ()](https://github.com/rust-lang/rust/pull/45379)
* [improve SliceExt::binary_search performance](https://github.com/rust-lang/rust/pull/45333)
* [saturating casts between integers and floats](https://github.com/rust-lang/rust/pull/45205)
* [`OccupiedEntry::replace_entry`](https://github.com/rust-lang/rust/pull/45152)
* [cargo: List available binary names](https://github.com/rust-lang/cargo/pull/4673)
* [rustdoc: Fix duplicated impls with generics](https://github.com/rust-lang/rust/pull/45620)
* [rustdoc: Search over generic types in docs](https://github.com/rust-lang/rust/pull/45673)
* [rustdoc: add more elements in the sidebar](https://github.com/rust-lang/rust/pull/45766)
* [rustdoc: add `#[allow(unused)]` to every doctest](https://github.com/rust-lang/rust/pull/45764)

## New Contributors

* David Wood
* Jonathan Behrens
* Lance John
* laurent
* matt rice
* Rolf Karp

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

* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [Semantic build scripts for Cargo](https://github.com/rust-lang/rfcs/pull/2196).
* [Constants in array repeat expressions](https://github.com/rust-lang/rfcs/pull/2203).
* [The ConstDefault trait](https://github.com/rust-lang/rfcs/pull/2204).
* [`#[derive unfinished(..)]` and `#[unfinished]`](https://github.com/rust-lang/rfcs/pull/2205).

# Upcoming Events

* [Nov  9. Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/243672298/).
* [Nov  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/244164143/).
* [Nov  9. San Diego Rust November Meetup - Beginner's Training Session](https://www.meetup.com/San-Diego-Rust/events/244506375/).
* [Now 10. Rust Meetup Stuttgart (Germany)](https://blog.shackspace.de/?p=5723)
* [Nov 13. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/244037662/).
* [Nov 15. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/244340757/).
* [Nov 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov 16. Cambridge Rust Meetup #5](https://www.meetup.com/Cambridge-Rust-Meetup/events/244114730/).
* [Nov 16. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Nov 19. Rust India Community Monthly Call](https://reps.mozilla.org/e/rust-india-monthly-call/).
* [Nov 21. Beginning Rust and Rust Hack Night @ Valtech Stockholm Sweden](https://www.meetup.com/ruststhlm/events/244792464/).
* [Nov 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust jobs from "Who's Hiring" thread on Hacker News](https://www.reddit.com/r/rust/comments/7adboi/17_rustrelated_job_openings_in_novembers_whos/).
* [Rust + Machine Learning at Etsy](https://www.reddit.com/r/rust/comments/7aoiod/job_etsy_rust_machine_learning/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> durka42: IMO the name "dangling" is scary enough :)
> Havvy gives durka42 a ptr::dangling::<Candy>().
> durka42 declines to unwrap() it

— [durka42 and Havvy](https://botbot.me/mozilla/rust-internals/2017-11-02/?msg=93047552&page=2) discussing [PR #45527](https://github.com/rust-lang/rust/pull/45527).

Thanks to [Centril](https://users.rust-lang.org/t/twir-quote-of-the-week/328/464) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
