Title: This Week in Rust 56
Date: 2014-11-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

101 pull requests were [merged in the last week][1]. Woo!

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-03..2014-11-10

## Breaking Changes

* [Flexible target specification][flex] has finally landed. This makes
  it much easier to create custom toolchains for unsupported
  platforms. [RFC][flex-rfc].
* [Error interoperation][err] improves the ergonomics of error
  handling when multiple error types are involved. [RFC][err-rfc].
* The `rtio` abstraction layer that supported I/O on green threads
  [has been removed][rtio]. [RFC][rtio-rfc].
* There has been a minor breaking change to the [serialization of
  tuples][tup].
* Minor [changes to macro interpolation][mac] have resulted the
  removal the `$foo:matchers` type of `macro_rules!` argument.
* Socket construction is now more flexibly done through a
  [`ToSocketAddr`] type.
* Some changes have been made to the [`BytesContainer`], which is used
  to construct `Path`s, causing breakage is some cases.
* The comparision types have been [updated for DST][cmp], resulting in
  changes to how they are invoked for references to unsized types
  (i.e. `&str` and `&[T]`).
* As part of the recent [collections overhaul][coll-rfc], the prelude
  now contains a [`repeat`] function that returns an iterator that
  repeatedly yields the same value.
* Some changes to make [overloaded operators][ops] behave more
  consistently will cause some previous code to break.
* The collections crate has seen [major refactorings][coll1] and
  [updates][coll2] as part of the [collections
  overhaul][coll-rfc]. There was additional discussion about the
  impact on [reddit][coll-reddit].
* The json crate [works with string slices instead of strings][json],
  and now overloads the index operator.
* A number of prelude traits have been [renamed and consolidated][pre]
  as fallout from DST and to conform to new [conventions]. This should
  not break much code as these traits are rarely named explicitly.
* The rlibc crate, which provides a few libc functions expected to
  exist by LLVM's code generation, and is only useful for freestanding
  Rust projects, has been [moved out of the main rust
  distribution][rlibc], and now must be [installed via
  cargo][rlibc-cargo].

[flex]: https://github.com/rust-lang/rust/pull/16156
[flex-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md
[err]: https://github.com/rust-lang/rust/pull/17753
[err-rfc]: https://github.com/rust-lang/rfcs/blob/master/active/0070-error-chaining.md
[tup]: https://github.com/rust-lang/rust/pull/17595
[mac]: https://github.com/rust-lang/rust/pull/17830
[`ToSocketAddr`]: https://github.com/rust-lang/rust/pull/18462
[`BytesContainer`]: https://github.com/rust-lang/rust/pull/18463
[cmp]: https://github.com/rust-lang/rust/pull/18467
[coll1]: https://github.com/rust-lang/rust/pull/18519
[coll2]: https://github.com/rust-lang/rust/pull/18605
[coll-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0235-collections-conventions.md
[coll-reddit]: https://www.reddit.com/r/rust/comments/2ljfnd/warning_some_collection_methods_have_had_their/
[`repeat`]: https://github.com/rust-lang/rust/pull/18468
[ops]: https://github.com/rust-lang/rust/pull/18486
[json]: https://github.com/rust-lang/rust/pull/18544
[pre]: https://github.com/rust-lang/rust/pull/18559
[rlibc]: https://github.com/rust-lang/rust/pull/18625
[rlibc-cargo]: https://github.com/rust-lang/rlibc
[rtio]: https://github.com/rust-lang/rust/pull/18557
[rtio-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0230-remove-runtime.md

## Other Changes

* New [blanket impls] of the unboxed closure types allow them to
  interoperate.  See
  [test](https://github.com/rust-lang/rust/blob/master/src/test/run-pass/unboxed-closures-fn-as-fnmut-and-fnonce.rs)
  [cases])https://github.com/rust-lang/rust/blob/master/src/test/run-pass/unboxed-closures-fnmut-as-fnonce.rs)
  for examples.
* impls can now be [defined on trait objects][impltrait].
* P1start has been [converting][help] compiler messages that provide
  suggestions from 'notes' to 'help' messages.
* The ['exceeding_bitshifts'][bitshift] lint catches overlong shifts
  (which are currently undefined behavior) of static size. Due to
  [bugs][bitshift-bugs] it is set to 'allow' be default.
* Ariel [removed][unsafe-rustc] a bunch of unsafe code from the
  compiler. Yay!
* A new `-l` [flag] to the compiler has been added to specify linkage
  to native libraries, primarily for use by cargo. In the same PR,
  `include!` was updated to expand its arguments, allowing cargo to do
  for more complex compile-time code generation. [RFC][flag-rfc].
* `#![cfg]` and `#[cfg_attr]` [can be applied to crates][cfg].
* On x86 Linux, random number generation now [prefers] the new
  [`getrandom`] syscall.

[blanket impls]: https://github.com/rust-lang/rust/pull/18388
[impltrait]: https://github.com/rust-lang/rust/pull/18447
[help]: https://github.com/rust-lang/rust/pull/18132
[bitshift]: https://github.com/rust-lang/rust/pull/18206
[bitshift-bugs]: https://github.com/rust-lang/rust/pull/18593
[unsafe-rustc]: https://github.com/rust-lang/rust/pull/18318
[flag]: https://github.com/rust-lang/rust/pull/18470
[flag-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0403-cargo-build-command.md
[cfg]: https://github.com/rust-lang/rust/pull/18634
[prefers]: https://github.com/rust-lang/rust/pull/18664
[getrandom]: http://lwn.net/Articles/606141/

# Approved RFC's
* [Num reform](https://github.com/rust-lang/rfcs/blob/master/text/0418-struct-variants.md): Strips down `std::num` to minimally support generic primitive numbers, without supporting a full mathematical hierarchy.

* [Higher-ranked trait bounds](https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md): Add the ability to have trait bounds that are polymorphic over lifetimes. Necessary for unboxed closures.

* [un-feature-gating struct variants](https://github.com/rust-lang/rfcs/blob/master/text/0418-struct-variants.md): Woo!

* [Multiple lifetime bounds](https://github.com/rust-lang/rfcs/blob/master/text/0192-bounds-on-object-and-generic-types.md): Removes special cases from the type system and makes more complex lifetime relationships be expressed that were previously only inferable.



# New RFC's
* [Macro reform](https://github.com/rust-lang/rfcs/pull/453): Prepares macros for 1.0 stabilization. Renames `macro_rules!` to `macro!`, and introduces more robust support for module importing and exporting.

* [Change integer fallback RFC to suggest `i32` instead of `int` as the fallback](https://github.com/rust-lang/rfcs/pull/452): Changes the fallback for performance and portability. 

* [Un-feature-gate if let and tuple indexing](https://github.com/rust-lang/rfcs/pull/450): The features are well-behaved and used by many projects; ship 'em!

* [Prohibit unused type parameters in impls](https://github.com/rust-lang/rfcs/pull/453): Require that every impl type parameter appears textually within the input type parameters of the trait reference or the impl self type.

* [ES6-style unicode string escaping](https://github.com/rust-lang/rfcs/pull/446): Remove `\u203D` and `\U0001F4A9` unicode string escapes, and add ECMAScript 6-style `\u{1F4A9}` escapes instead. Strong positive feedback, some concern with how it interacts with format strings.

* [extension trait conventions](https://github.com/rust-lang/rfcs/pull/445): Establishes a definition and naming convention for extension traits: traits which aren't intended for generic programing, but instead extending existing types. If extending a `Foo`, use `FooExt`. If Extending a `Foo` when it impls another trait like `Add`, use `FooAddExt`. 

* [cmp and ops reform](https://github.com/rust-lang/rfcs/pull/439): Refactors `Cmp` and the operator overloading traits. Generally positive feedback. Highlights include:
  * Make basic unary and binary operators work by value and use associated types.
  * Generalize comparison operators to work across different types; drop Equiv.
  * Refactor slice notation in favor of range notation so that special traits are no longer needed.
  * Add IndexSet to better support maps.
  * Clarify ownership semantics throughout.

* [Change precedence of `+` in type grammar](https://github.com/rust-lang/rfcs/pull/438): Update type grammar to make `+` have lower precedence, consistent with the expression grammar, resolving a grammatical ambiguity.

* [Relocate and improve c_str](https://github.com/rust-lang/rfcs/pull/435): 
  * Move the c_str module out of std to rid the latter of type dependencies on libc.
  * Split the current CString into a low-level type CStrBuf and a length-aware CString to make computation costs explicit.
  * Provide custom destructors and purpose-specific, mnemonically named constructors.
  * Add some methods and trait implementations to make the types more useful.
  * Remove the Clone implementation due to lack of purpose.
Lots of discussion of how to structure libc, not a lot of consensus.

* [rename `lifetime` to `scope`](https://github.com/rust-lang/rfcs/pull/431): Highly controversial. Some community members argue that this change in terminology has been much more effective when introducing the actual concepts to newbies. Others argue that scope is already a well established concept in programming languages.

* [Finalizing more naming conventions](https://github.com/rust-lang/rfcs/pull/430): finalizes a few long-running de facto conventions, including capitalization/underscores, and the role of the unwrap method. Generally positive feedback, some discussion of naming consts like enum variants.

* [Reserve macro identifiers](https://github.com/rust-lang/rfcs/pull/456): Preemptively reserve a class of $ identifiers for allowing backwards compatible improvements to the macro system.



# Community

## From the Team

* [Weekly-meetings/2014-11-04 (macros; multiple lifetime bounds; macro invocation syntax; higher-ranked trait bounds; pub trait methods; out-of-sync nightlies; struct variants; numerics)](https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-04.md)
  * [Discuss](https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-04.md)
  * [Reddit](http://www.reddit.com/r/rust/comments/2lrt7b/weeklymeetings20141104_macros_multiple_lifetime/)

* [Weekly-meetings/2014-10-30 (error conventions; cargo; namespaced enums; trait-based error handling; macro unification; coercions; dynamic linking, byte literals, failing dtors)](https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-10-30.md)
  * [Discuss](http://discuss.rust-lang.org/t/weekly-meetings-2014-10-30-error-conventions-cargo-namespaced-enums-trait-based-error-handling-macro-unification-coercions-dynamic-linking-byte-literals-failing-dtors/734)
  * [Reddit](http://www.reddit.com/r/rust/comments/2kuppu/weeklymeetings20141030_error_conventions_cargo/)

* [IRC notifications now going to #rust-bots](http://discuss.rust-lang.org/t/irc-notifications-now-going-to-rust-bots/735): If you have a bot you'd like to post here (which would be awesome!) please add a description and contact to [the wiki page](https://github.com/rust-lang/rust/wiki/IRC-notifications-channel).

## Videos

* [An introduction to Servo](https://air.mozilla.org/an-introduction-to-servo/): Lars Bergstrom from the Research team provides an overview of the Servo project, demonstrates its current status, and shows how to contribute to it.

* [November's Bay Area meetup](https://air.mozilla.org/bay-area-rust-meetup-november-2014/) happened
  on Thursday, featuring five presentations about Servo and browser architecture.

## Blog Posts

* [This Week In Servo (10)](http://blog.servo.org/2014/11/04/twis-10/)

* [Rewriting Rust Serialization, Part 2: Performance](http://erickt.github.io/blog/2014/11/03/performance/): A quick look at how Rust's JSON serialization performance compares to other languages and protocols.

* [Improved Error Handling in Rust](http://lucumr.pocoo.org/2014/11/6/error-handling-in-rust/): Some discussion of how Rust currently and theoretically handles erroes.

* [Don't Panic! The Hitchhiker's Guide to Unwinding](http://lucumr.pocoo.org/2014/10/30/dont-panic/): A nice discussion of the challenges of safe and ergonomic error handling, and how it relates to stack unwinding.

* [Let's build a browser engine! Part 7: Painting 101](http://limpet.net/mbrubeck/2014/11/05/toy-layout-engine-7-painting.html): Part of a longer series on writing a browser engine in Rust. *In this article, I will add very basic painting code.*

* [On pattern matching performance in Rust](http://www.cjqed.com/blog/rust-pattern-matching-performance/): A quick look at how the `match` statement can produce really efficient code.

* [Rust and Go](https://medium.com/@adamhjk/rust-and-go-e18d511fbd95): A quick look at Rust and Go from the perspective of a sysadmin used to high-level programming languages. 

* [Learning Rust](http://foon.uk/rust.html): *Inspired by Artyom's Learning Racket series, I've decided to log my efforts in learning Rust. I'm going to document my learning process as I go about trying to build a roguelike in Rust. I've downloaded the compiler, skimmed the getting started guide, and written “Hello World”. So let's get started!*




## Discuss

* [Pre-RFC: placement box with Placer trait](http://discuss.rust-lang.org/t/pre-rfc-placement-box-with-placer-trait/729/6): Add user-defined placement box expression (more succinctly, "a box
expression"), an operator analogous to "placement new" in C++. This
provides a way for a user to specify (1.) how the backing storage for
some datum should be allocated, (2.) that the allocation should be
ordered before the evaluation of the datum, and (3.) that the datum
should preferably be stored directly into the backing storage (rather
than temporary storage on the stack and then copying the datum from
the stack into the backing storage).

* [Forbid -(unsigned integer)](http://discuss.rust-lang.org/t/forbid-unsigned-integer/752): the eternal struggle continues. It's super handy when you want it, but also a common error to make.

* [Moving all built-in macros to plugins](http://discuss.rust-lang.org/t/moving-all-built-in-macros-to-plugins/737): Another proposal to handle some of the issues with macros for 1.0. May make it easier to bootstrap changes to the compiler. 

* [Lifetime Notation](http://discuss.rust-lang.org/t/lifetime-notation/751): `&'a` -> `a&`. Some discussion of tradeoffs and details.




## Reddit

* [Warning! Some collection methods have had their semantics changed transparently!](http://www.reddit.com/r/rust/comments/2ljfnd/warning_some_collection_methods_have_had_their/)

* [Error interoperation now available in the nightlies](https://www.reddit.com/r/rust/comments/2l98pn/error_interoperation_now_available_in_the/)

* [Trait-based Exception handling RFC postponed until after 1.0](http://www.reddit.com/r/rust/comments/2l8x2a/traitbased_exception_handling_rfc_postponed_till/)

* [Cargo now supports build-scripts!](http://www.reddit.com/r/rust/comments/2lgyne/cargo_now_support_build_scripts_xpost_rrust/)

* [What libraries would you like to see implemented in Rust?](https://www.reddit.com/r/rust/comments/2lmt99/what_libraries_would_you_like_to_see_implemented/)  

* [How good do you think the market for Rust developers will be 5 years from now, and in what area of programming?](http://www.reddit.com/r/rust/comments/2l3l07/rust_is_undoubtedly_one_of_the_upandcoming_big/)

* [I think Rust and I were made for each other](http://www.reddit.com/r/rust/comments/2ljrp2/i_think_rust_and_i_were_meant_for_each_other/)

* [How does the Rust community feel about FFI?](https://www.reddit.com/r/rust/comments/2lmkjw/how_does_the_rust_community_feel_about_ffi/)

* [How do you refactor Rust code?](https://www.reddit.com/r/rust/comments/2lo21r/how_do_you_refactor_rust_code/)

* [I may start contributing to Rust...can I get a few pointers?](http://www.reddit.com/r/rust/comments/2lduv6/i_may_start_contributing_to_rustcan_i_get_a_few/)




## New Projects

* [this-week-in-rust](https://github.com/cmr/this-week-in-rust): This Week in Rust's content is now publicly hosted in a Github repo! If you find any errors, just submit a PR to the relevant markdown file in `/content`! If you'd like to help out, please contact cmr, brson, or Gankro on Github/Reddit/IRC.

* [rustaceans.org](http://rustaceans.org/): *This website is for finding Rustaceans. Wondering who is behind that GitHub username or IRC nick? Here is where to find out.*

* [rust-modifier](https://github.com/reem/rust-modifier): *Convenient chaining APIs for free*

* [dockerfiles](https://github.com/schickling/dockerfiles): *Collection of lightweight and ready-to-use docker images* 

* [Window Tiling For The Win](https://github.com/Kintaro/wtftw): *A tiling window manager written in Rust*

* [cxx2rs](https://github.com/manuels/cxx2rs): *A rust-binding generator for C/C++ files*

* [sorting-rs](https://github.com/wackywendell/sorting-rs): *This is a set of sorting algorithms, written in Rust.*

* [rust-chatserver](https://github.com/BytePrelude/rust-chatserver): *A barebone command line TCP chatserver written in Rust*. [Looking for feedback](https://www.reddit.com/r/rust/comments/2lpsj8/tcp_chatserver_written_in_rust_looking_for/).

* [rust-irc](https://github.com/viperscape/rust-irc): *A simple example irc implementation*. [Looking for feedback](https://www.reddit.com/r/rust/comments/2lpw9k/rust_irc_example_looking_for_feedback/)

* [rusqlite](https://github.com/jgallagher/rusqlite): *Ergonomic, semi-safe bindings to SQLite for Rust*


## Upcoming Meetups

* [Rust Bay Area: Cryptography and Rust, December 18th](http://www.meetup.com/Rust-Bay-Area/events/210632582/)


# New Contributors
