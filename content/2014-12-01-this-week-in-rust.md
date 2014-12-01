Title: This Week in Rust 59
Date: 2014-12-01
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

71 pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-24..2014-12-01

## Breaking Changes

* [libgetopts has had][getopts] some public re-exports removed, Fail_ renamed to Fail,
and getopts::FailType has been removed.
* BinarySearchResult's variants [are no longer re-exported][bstenum], and must be accessed
through the enum itself.
* [libsync is dead][ripsync]. Long live `std::sync` and `std::comm`. All uses of libsync
should be able to cleanly migrate to these two modules.
* MaybeOwned[Vector] has been deprecated [in favour of Cow][cowabunga]. See the PR for
full details of all the consequences.
* Non-failing `unwrap` methods [have been renamed][into-winner] to `into_inner`, as
per [RFC 430][rfc430]
* You can [no longer invoke Dark Magicks][never4get] and match on an enum struct variant
as if it were a tuple. As a consequence, Rust is no longer a useful programming language.
* non-ASCII lifetime identifiers [have been feature-gated][whatishappening]. Rust is now
only of academic interest, and lacks any practical applications.

[getopts]: https://github.com/rust-lang/rust/pull/19365
[bstenum]: https://github.com/rust-lang/rust/pull/19287
[ripsync]: https://github.com/rust-lang/rust/pull/19255
[cowabunga]: https://github.com/rust-lang/rust/pull/19252
[into-winner]: https://github.com/rust-lang/rust/pull/19149
[rfc430]: https://github.com/rust-lang/rfcs/pull/430
[never4get]: https://github.com/rust-lang/rust/pull/19087
[whatishappening]: https://github.com/rust-lang/rust/pull/19073

## Other Changes

* Unboxed closure captures are [now avaiable in debuginfo][debuginfo].
* tomjakubowski has taught Rustdoc about [several][tomja1] of [rust's][tomja2] new
[features][tomja3].
* Some missing collection iterators [have][iter1] [been][iter2] [added][iter3].
* The inner contents of Buffered io types [are now accessible mutably][buffers].
* Tests [now add less useless whitespace][notabs] to your terminal's output.
* Statically allocated TLS keys are now [explicitly leaked][leaky].
* Fields of consts are [now transitevly interpreted as const][constmemaybe],
allowing e.g. `[T, ..MY_TUPLE.0]`.
* `::::` [no longer appears][nonono] in module paths in debug logs.
* AtomicOption now [correctly requires `Send`][atomic] for memory safety.
* japaric has DST-ified more of the standard libs
* The iterator module has been [partially stabalized][iterstab].
* Platform-specific io modules `std::or::unix` and `std::os::windows`
[have been added][i-oh-my] for working with lower-level `io` details
like file descriptors, SOCKETS, HANDLES, etc.
* Slice iterators can now [be converted to slices][sliceit] via `as_slice`.
* Rng [now supports][random-floats] `next_f64` and `next_f32`.

[debuginfo]: https://github.com/rust-lang/rust/pull/19363
[tomja1]: https://github.com/rust-lang/rust/pull/19349
[tomja2]: https://github.com/rust-lang/rust/pull/19272
[tomja3]: https://github.com/rust-lang/rust/pull/19174
[iter1]: https://github.com/rust-lang/rust/pull/19330
[iter2]: https://github.com/rust-lang/rust/pull/19296
[iter3]: https://github.com/rust-lang/rust/pull/19231
[buffers]: https://github.com/rust-lang/rust/pull/19328
[notabs]: https://github.com/rust-lang/rust/pull/19299
[leaky]: https://github.com/rust-lang/rust/pull/19285
[constmemaybe]: https://github.com/rust-lang/rust/pull/19266
[nonono]: https://github.com/rust-lang/rust/pull/19262
[atomic]: https://github.com/rust-lang/rust/pull/19250
[dst1]: https://github.com/rust-lang/rust/pull/19248
[iterstab]: https://github.com/rust-lang/rust/pull/19176
[i-oh-my]: https://github.com/rust-lang/rust/pull/19169
[sliceit]: https://github.com/rust-lang/rust/pull/18966
[random-floats]: https://github.com/rust-lang/rust/pull/18534

## New Contributors

* Ben S
* olivren
* Pascal Hertleif
* Roy Crihfield
* Ulysse Carion

# Approved RFC's

* [Disallow type/lifetime parameter shadowing][rfc459]: Rarely what you want, confusing when it happens.
* [Add "function name macro"][rfc466]: This RFC proposes the addition of a function! macro that expands to the function it's used in. This will greatly help error reporting.
* [extension trait conventions][rfc445]: This is a conventions RFC establishing a definition and naming convention for extension traits: `FooExt`.

[rfc459]: https://github.com/rust-lang/rfcs/pull/459
[rfc466]: https://github.com/rust-lang/rfcs/pull/466
[rfc445]: https://github.com/rust-lang/rfcs/pull/445

# New RFC's

* [Stabilize `std::{c_str, c_vec}`][rfc494]: Stabilize the std::{c_str, c_vec} modules by re-working their interfaces and refocusing each primitive for one particular task.
* [Error Reform and Failure Box][rfc492]: This RFC proposes changes to the `Error` trait and the introduction of a `Failure<T>` wrapper type for errors and modifications to the `Error` trait for better error interoperability and debugging support. [Reddit][red492]
* [Remove the `[]` notation for taking a whole slice][rfc491]: &* has emerged as less strange and more idomatic
* [Change DST syntax to `T: Sized?`][rfc490]: Change the syntax for dynamically sized type parameters from `Sized? T` to `T: Sized?`, and change the syntax for traits for dynamically sized types to `trait Foo: Sized?`. Extend this new syntax to work with `where` clauses.
* [rename `lifetime` to `lifespan`][rfc487]: 'nuff said
* [std::ascii reform][rfc486]: Move the `std::ascii::Ascii` type and related traits to a new Cargo package on crates.io, and instead expose its functionality for `u8`, `[u8]`, `char`, and `str` types. [Reddit][red487]
* [Make brackets optional for single attributes][rfc484]: This is an alternative to #483: instead of changing attributes to `@`, just make the brackets optional for `#`.
* [Change attribute syntax to @foo][rfc483]: Change attribute syntax from `#[foo]` and `#![foo]` to `@foo` and `@!foo`. [Reddit][red483]

[rfc494]: https://github.com/rust-lang/rfcs/pull/494
[rfc492]: https://github.com/rust-lang/rfcs/pull/492
[rfc491]: https://github.com/rust-lang/rfcs/pull/491
[rfc490]: https://github.com/rust-lang/rfcs/pull/490
[rfc487]: https://github.com/rust-lang/rfcs/pull/487
[rfc486]: https://github.com/rust-lang/rfcs/pull/486
[rfc484]: https://github.com/rust-lang/rfcs/pull/484
[rfc483]: https://github.com/rust-lang/rfcs/pull/483

[red492]: http://www.reddit.com/r/rust/comments/2nsufx/rfc_error_reform_and_failure_box/
[red487]: http://www.reddit.com/r/rust/comments/2nlar3/rfc_rename_lifetime_to_lifespan/
[red483]: http://www.reddit.com/r/rust/comments/2nfxtf/rfc_change_attribute_syntax_to_foo/

# Community

## From the Team

* [Weekly-meetings/2014-25-12][mtg]: extension trait conventions; shadowed lifetimes; problems with unused type params; es6-style string escaping; int fallback redux redux [Reddit][mtg-reddit].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-25.md
[mtg-reddit]: http://www.reddit.com/r/rust/comments/2nhmgj/weekly_meeting_20141125_extension_trait/



## Blog Posts

* [Rust borrow and lifetimes][rs-borrows]: Another survey of how borrows and ownership works
in Rust. [Reddit][rs-borrows-red]
* [Rust, Generics, and Collections][gen-col]: A look at some of the space of generic
programming in Rust, and how it interacts with collections.
* [Rust compiling rust: adventures with librustc][rcr]: A quick look at using librustc
programmatically. [Reddit][rcr-reddit]
* [Interoperating Between Objective-C and Rust][objc]: How to write FFI code for Objective-C
[Reddit][objc-red]
* [Purging Proc][proc]: Niko explains the present and future of unboxed closures.
[Reddit][proc-red]
* [Installing Rust nightly builds into your home directory][home]: A simple guide for setting up
 Rust in your terminal on a Mac. [Reddit][home-red]
* [Procedural Generation in Rust Part 1: Setting up your Environment][generation]: Setting up Rust
in Windows. (this week's author of TWiR can testify that this is sometimes harrowing)
[Reddit][generation-red]
* [Benchmarking is Confusing in Low Level Rust][confusing]: A dive into trying to re-implement
Reader and Writer performantly, and determining if you've actually done that.
[Reddit][confusing-red]

[rcr]: http://jaredly.github.io/2014/11/22/rust-compiling-rust-adventures-with-librustc/
[rcr-reddit]: http://www.reddit.com/r/rust/comments/2noy0l/rust_compiling_rust_adventures_with_librustc/
[objc]: http://sasheldon.com/blog/2014/11/28/interoperating-between-objective-c-and-rust/
[objc-red]: http://www.reddit.com/r/rust/comments/2nno39/interoperating_between_objectivec_and_rust/
[proc]: http://smallcultfollowing.com/babysteps/blog/2014/11/26/purging-proc/
[proc-red]: http://www.reddit.com/r/rust/comments/2nipwp/purging_proc/
[home]: http://mpuppe.de/blog/2014/11/26/installing-rust-nightly-builds-into-your-home-directory/
[home-red]: http://www.reddit.com/r/rust/comments/2niux7/installing_rust_nightly_builds_into_your_home/
[generation]: http://brandonmhaley.com/?p=8
[generation-red]: http://www.reddit.com/r/rust/comments/2ncc4x/procedural_generation_in_rust_part_1_setting_up/
[confusing]: http://erickt.github.io/blog/2014/11/22/benchmarking-is-confusing/
[confusing-red]: http://www.reddit.com/r/rust/comments/2n8l35/benchmarking_is_confusing_in_low_level_rust/
[gen-col]: http://cglab.ca/~abeinges/blah/rust-generics-and-collections/
[gen-col-red]: http://www.reddit.com/r/rust/comments/2nwuct/rust_generics_and_collections/
[rs-borrows]: http://arthurtw.github.io/2014/11/30/rust-borrow-lifetimes.html
[rs-borrows-red]: http://www.reddit.com/r/rust/comments/2nuybm/rust_borrow_and_lifetimes/

## Discussions

* [which classes of errors remain with rust?][error-classes]
* [The Sad State of Zero-on-Drop][sad-state] [Reddit][sad-state-reddit]
* [Are there any Rust books in the pipeline?][rust-books]
* [Crates.io Package Groups Proposal][crate-names]
* [Are there plans for a Rust runtime?][plans]

[error-classes]: http://www.reddit.com/r/rust/comments/2nlis8/which_classes_of_errors_remain_with_rust/
[sad-state]: http://discuss.rust-lang.org/t/the-sad-state-of-zero-on-drop/944
[sad-state-reddit]: http://www.reddit.com/r/rust/comments/2nef5d/the_sad_state_of_zeroondrop/
[rust-books]: http://www.reddit.com/r/rust/comments/2ne8jz/are_there_any_rust_books_in_the_pipeline/
[crate-names]: https://github.com/rust-lang/cargo/issues/975
[plans]: http://www.reddit.com/r/rust/comments/2n9pkl/are_there_plans_for_a_rust_runtime/

## New Projects

* [pitch_calc][pitch_calc]: A library for musical pitch conversions! Provides functions and traits for converting between frequency, midi-step and letter-octave.
* [irc][irc]: An IRC library for Rust.
* [SoundStream][SoundStream]: A simple-as-possible, fast audio I/O stream for Rust
* [capture.rs][capture.rs]: A macro for explicit capture clauses
* [morphism.rs][morphism.rs]: A structure for suspended closure composition
* [collect-rs][collect-rs]: An experimental extension of the Rust standard library's libcollections
* [confsolve][confsolve]: Command line tool for resolving Dropbox/Wuala conflicts
* [enum_ns][enum_ns]: A simple compiler plugin to enable the old enum namespacing behavior for select modules and crates
* [Multipart + Hyper][multipart]: An extension to Hyper that provides client- and server-side support for HTTP multipart/form-data requests
* [sersve][sersve]: A directory server in Rust with Iron

[pitch_calc]: https://github.com/RustAudio/pitch_calc
[irc]: https://github.com/aaronweiss74/irc
[SoundStream]: https://github.com/mitchmindtree/sound_stream
[capture.rs]: https://crates.io/crates/capture
[morphism.rs]: https://github.com/epsilonz/morphism.rs
[collect-rs]: https://github.com/Gankro/collect-rs
[confsolve]: https://github.com/dan-t/rust-confsolve
[enum_ns]: https://github.com/cybergeek94/enum_ns
[multipart]: https://github.com/cybergeek94/multipart
[sersve]: http://till.hoeppner.ws/2014/11/30/Introducing-sersve-a-directory-server-in-Rust-with-Iron/

## Project Updates

* [This Week In Servo 13]

[twis]: http://blog.servo.org/2014/11/25/twis-13/

## Upcoming Meetups

* [Rust Berlin Hack and Learn][rust-berlin]: December 9th
* [Rust Amsterdam][rust-amsterdam]: Early 2015

[rust-berlin]: http://www.meetup.com/Rust-Berlin/events/218914766/
[rust-amsterdam]: http://www.meetup.com/Rust-Amsterdam
