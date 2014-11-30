Title: This Week in Rust 59
Date: 2014-12-1
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

XXX pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-24..2014-12-1

## Breaking Changes



## Other Changes



## New Contributors



# Approved RFC's

* [Disallow type/lifetime parameter shadowing][rfc459]: Rarely what you want, confusing when it happens.
* [Add "function name macro"][rfc466]: This RFC proposes the addition of a function! macro that expands to the function it's used in. This will greatly help error reporting.
* [extension trait conventions][rfc445]: This is a conventions RFC establishing a definition and naming convention for extension traits: `FooExt`.

[rfc459]: https://github.com/rust-lang/rfcs/pull/459
[rfc466]: https://github.com/rust-lang/rfcs/pull/466
[rfc445]: https://github.com/rust-lang/rfcs/pull/445

# New RFC's

* [Error Reform and Failure Box][rfc492]: This RFC proposes changes to the `Error` trait and the introduction of a `Failure<T>` wrapper type for errors and modifications to the `Error` trait for better error interoperability and debugging support. [Reddit][red492]
* [Remove the `[]` notation for taking a whole slice][rfc491]: &* has emerged as less strange and more idomatic
* [Change DST syntax to `T: Sized?`][rfc490]: Change the syntax for dynamically sized type parameters from `Sized? T` to `T: Sized?`, and change the syntax for traits for dynamically sized types to `trait Foo: Sized?`. Extend this new syntax to work with `where` clauses.
* [rename `lifetime` to `lifespan`][rfc487]: 'nuff said
* [std::ascii reform][rfc486]: Move the `std::ascii::Ascii` type and related traits to a new Cargo package on crates.io, and instead expose its functionality for `u8`, `[u8]`, `char`, and `str` types. [Reddit][red487]
* [Make brackets optional for single attributes][rfc484]: This is an alternative to #483: instead of changing attributes to `@`, just make the brackets optional for `#`.
* [Change attribute syntax to @foo][rfc483]: Change attribute syntax from `#[foo]` and `#![foo]` to `@foo` and `@!foo`. [Reddit][red483]

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

* [Rust compiling rust: adventures with librustc][rcr]: A quick look at using librustc programmatically. [Reddit][rcr-reddit]
* [Interoperating Between Objective-C and Rust][objc]:
* [Purging Proc][proc]:
* [Installing Rust nightly builds into your home directory][home]:
* [Procedural Generation in Rust Part 1: Setting up your Environment][generation]:
* [Benchmarking is Confusing in Low Level Rust][confusing]:

[rcr]: http://jaredly.github.io/2014/11/22/rust-compiling-rust-adventures-with-librustc/
[rcr-reddit]: http://www.reddit.com/r/rust/comments/2noy0l/rust_compiling_rust_adventures_with_librustc/
[objc]: http://sasheldon.com/blog/2014/11/28/interoperating-between-objective-c-and-rust/
[proc]: http://smallcultfollowing.com/babysteps/blog/2014/11/26/purging-proc/
[home]: http://mpuppe.de/blog/2014/11/26/installing-rust-nightly-builds-into-your-home-directory/
[generation]: http://brandonmhaley.com/?p=8
[confusing]: http://erickt.github.io/blog/2014/11/22/benchmarking-is-confusing/

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

* [pitch_calc]: A library for musical pitch conversions! Provides functions and traits for converting between frequency, midi-step and letter-octave.
* [irc]: An IRC library for Rust.
* [SoundStream]: A simple-as-possible, fast audio I/O stream for Rust
* [capture.rs]: A macro for explicit capture clauses
* [morphism.rs]: A structure for suspended closure composition
* [collect-rs]: An experimental extension of the Rust standard library's libcollections
* [confsolve]: Command line tool for resolving Dropbox/Wuala conflicts
* [enum_ns]: A simple compiler plugin to enable the old enum namespacing behavior for select modules and crates
* [Multipart + Hyper][multipart]: An extension to Hyper that provides client- and server-side support for HTTP multipart/form-data requests

[pitch_calc]: https://github.com/RustAudio/pitch_calc
[irc]: https://github.com/aaronweiss74/irc
[SoundStream]: https://github.com/mitchmindtree/sound_stream
[capture.rs]: https://crates.io/crates/capture
[morphism.rs]: https://github.com/epsilonz/morphism.rs
[collect-rs]: https://github.com/Gankro/collect-rs
[confsolve]: https://github.com/dan-t/rust-confsolve
[enum_ns]: https://github.com/cybergeek94/enum_ns
[multipart]: https://github.com/cybergeek94/multipart

## Project Updates

* [This Week In Servo 13]

[twis]: http://blog.servo.org/2014/11/25/twis-13/

## Upcoming Meetups

* [Rust Berlin Hack and Learn][rust-berlin]: December 9th
* [Rust Amsterdam][rust-amsterdam]: Early 2015

[rust-berlin]: http://www.meetup.com/Rust-Berlin/events/218914766/
[rust-amsterdam]: http://www.meetup.com/Rust-Amsterdam
