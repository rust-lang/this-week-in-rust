Title: This Week in Rust 57
Date: 2014-11-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

# What's cooking on master?

55 pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-10..2014-11-17

## Breaking Changes

* Struct variants are [no longer feature-gated][structvars]. At the
  same time, struct variants no longer support visibility modifiers,
  for consistency with other variants. [RFC][structvars-rfc].
* The `time` crate, which is widely considered to be of poor quality,
  has been [moved out of the distribution][time], but can still be
  accessed via cargo.
* The new task pool that [reem announced on reddit][tp-reddit] earlier
  in the week was speedily [merged into the tree][tp], replacing the
  old `TaskPool`. It includes some breaking API changes.
* The compiler now treats `()` not as a distinct 'unit' type but as a
  [zero-length tuple][unit] (though the docs continue to allow that
  `()` may be referred to as 'unit'). This may cause breakage for
  macros that expect `()` to be a literal, whereas now it is an
  expression.
* `io::Buffer` has been [refactored to be object-safe][buffer], moving
  some methods into other traits.
* The `Extendable` trait for extending a collection via an `Iterator`
  has been [renamed][extend] to `Extend`, and can now be used with
  `EnumSet` and `LruCache`.
* The old 'once_fns' feature gate has been [removed][once] (everybody
  thought it had been removed long ago). This is unrelated to the
  modern `FnOnce` type.

[extend]: https://github.com/rust-lang/rust/pull/18475
[once]: https://github.com/rust-lang/rust/pull/18743
[time]: https://github.com/rust-lang/rust/pull/18858
[unit]: https://github.com/rust-lang/rust/pull/18752
[buffer]: https://github.com/rust-lang/rust/pull/18788
[tp]: https://github.com/rust-lang/rust/pull/18941
[tp-reddit]: https://www.reddit.com/r/rust/comments/2ltjwm/a_loadbalancing_taskpool_resistant_to_child_panics/
[structvars]: https://github.com/rust-lang/rust/pull/18994
[structvars-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0418-struct-variants.md

## Other Changes

* BTree [implements][btree] the [collection views][view-rfc] API.
* The `#[stable]` attribute is [no longer inherited][stable] by child
  AST elements. This is intended to reduce the risk of accidentally
  marking things stable.
* `AsRefReader` and `AsRefWriter` have been [renamed][asref] to
  `ByRefReader` and `ByRefWriter` for consistency with their method
  names. The original types remain and are deprecated.
* Performance of `RingBuf` has [improved][ringbuf]. Some subsequent
  [reddit discussion][ringbuf-reddit] lamented the introduction of
  unsafe code.
* On Windows, rustc [once again prefers the bundled MinGW linker][win]
  over any system-installed MinGW linker in an attempt to make Rust on
  Windows cause the fewest surprises.

[btree]: https://github.com/rust-lang/rust/pull/18287
[view-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0216-collection-views.md
[stable]: https://github.com/rust-lang/rust/pull/18887
[asref]: https://github.com/rust-lang/rust/pull/18891
[ringbuf]: https://github.com/rust-lang/rust/pull/18747
[ringbuf-reddit]: https://www.reddit.com/r/rust/comments/2mfcuk/ringbuf_remove_optiont/
[win]: https://github.com/rust-lang/rust/pull/18797

## New Contributors

* Adam Szkoda
* Artem
* Barosl Lee
* Ian Connolly
* Jeff Parsons
* Josh Haberman
* Josh Stone
* Murarth
* Ricky Taylor

# Approved RFC's

* [RFC 369: Numerics reform][num]. Conservatively dismantle the
  numeric trait hierarchy for later reconsideration. [PR][num-pr].
* [RFC 380: Stabilize std::fmt][fmt]. Primarily stabilizes the user-facing
  formatting syntax, while leaving the library APIs
  unstable. [PR][fmt-pr].

[num]: https://github.com/rust-lang/rfcs/blob/master/text/0369-num-reform.md
[num-pr]: https://github.com/rust-lang/rfcs/pull/369
[fmt]: https://github.com/rust-lang/rfcs/blob/master/text/0380-stabilize-std-fmt.md
[fmt-pr]: https://github.com/rust-lang/rfcs/pull/380

# New RFC's

* [RFC 457: Version attribute][457]. Provides a mechanism for
  identifying which version of the language a given source corresponds
  to.
* [RFC 458: Improve the `Send` trait][458]. This RFC proposes
  extending the Send trait in some relatively small but backwards
  incompatible ways, to allow more safe behaviors to be exposed in
  Rust's type system. In particular, this involves removing the
  'static bound from Send in a way that preserves thread safety.
* [RFC 459: Disallow type/lifetime parameter shadowing][459]. Source
  of confusing bugs.
* [RFC 461: Add a thread-local storage module][461]. Another TLS
  design.
* [RFC 462: Future-proof `box` and `&` patterns][462]. Futureproof box
  patterns by renaming them to deref.  In an effort to consolidate box
  and & patterns, change the latter to use the deref syntax as well,
  in recognition of them being semantically equivalent to box
  patterns.  Make the newly introduced deref keyword a non-strict
  keyword.
* [RFC 463: Restrict identifiers after literals][463]. Futureproofing.
* [RFC 464: Rename uint/int][464]. Latest in a series of RFC's to
  discourage use of pointer-sized integers as the 'default' type.

[457]: https://github.com/rust-lang/rfcs/pull/457
[458]: https://github.com/rust-lang/rfcs/pull/458
[459]: https://github.com/rust-lang/rfcs/pull/459
[461]: https://github.com/rust-lang/rfcs/pull/461
[462]: https://github.com/rust-lang/rfcs/pull/462
[463]: https://github.com/rust-lang/rfcs/pull/463
[464]: https://github.com/rust-lang/rfcs/pull/464

# Community

## From the Team

* [Weekly-meetings/2014-11-11][mtg]: fott; std::fmt; default typarams; rfc authors; 'coerce' vs. 'view', etc.; precent of + in type grammar; jemalloc. [Reddit][mtg-reddit].
* [Brian Koropoff (unwound) is a friend of the tree!][fott]
* [Allocators in Rust][alloc]: Niko attempts to lay out the tradeoffs
  involved in integrating jemalloc with
  Rust. [Reddit][alloc-reddit]. [HN][alloc-hn].


[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-11.md
[mtg-reddit]: https://www.reddit.com/r/rust/comments/2me6r7/rustupps1_a_rustupsh_equivalent_for_windows/
[alloc]: http://smallcultfollowing.com/babysteps/blog/2014/11/14/allocators-in-rust/
[alloc-reddit]: https://www.reddit.com/r/rust/comments/2mcew2/allocators_in_rust_from_nmatsakiss_blog/
[alloc-hn]: https://news.ycombinator.com/item?id=8612430
[fott]: https://github.com/rust-lang/rust/wiki/Doc-friends-of-the-tree#2014-11-11-brian-koropoff-unwound

## Blog Posts

* [Two hours after Rust][two]. Reports from a new Rust user. [Reddit][two-reddit].
* [Rust Serialization part 2.1: now with more benchmarks: Cap'n Proto, MessagePack, and Protobuf][bench]. [Reddit][bench-reddit].
* [Serialization part 2.2: Everything's faster (especially capn proto)][bench2]. [Reddit][bench2-reddit].
* [Go vs. Rust: Productivity vs. Performance][govr]. [Reddit][govr-reddit].

[two]: http://jbowles.github.io/lambda-bowles/programs/two-hours-after-rust/
[two-reddit]: https://www.reddit.com/r/rust/comments/2ly7q8/two_hours_after_rust/
[bench]: http://erickt.github.io/blog/2014/11/11/benchmarks/
[bench-reddit]: https://www.reddit.com/r/rust/comments/2lzc9n/rust_serialization_part_21_now_with_more/
[bench2]: http://erickt.github.io/blog/2014/11/13/benchmarks-2/
[bench2-reddit]: https://www.reddit.com/r/rust/comments/2m72br/serialization_part_22_everythings_faster/
[govr]: http://joshitech.blogspot.com/2014/11/go-vs-rust-productivity-vs-performance.html
[govr-reddit]: https://www.reddit.com/r/rust/comments/2maqi7/go_vs_rust_productivity_vs_performance/

## Discussions

* [Closures vs. Unboxed Closures][cl]. Useful explanation for the uninitiated.
* [Where is artithmetic (signed) right-shift][sh]. (A: `>>` is either arithmetic or logical based on type).
* `Foo::new()` vs. `Foo()`, both on [discuss][ctor-discuss] and [Reddit][ctor-reddit].
* ["Hello, world" on a PSP via Rust!][psp]. Another supreme hack from Luqman. Nice use of [target specs].
* [Does Rust have anything like C# async await][await]. Take 20.
* [`std::sync::Future` is almost useless for async processing][future]. (`Future` is ancient and unloved).
* [Single-source GPU support][gpu]. An inquiry about the prospects of compiling Rust to GPUs.
* [Experienced users: how easy is Rust's memory management system to use?][mm]. Mostly gushing about how awsome Rust is (seconded!).
* [Pre RFC: Remove `FromError` trait, add `From` trait][from]

[cl]: https://www.reddit.com/r/rust/comments/2lo6yt/closures_vs_unboxed_closures/
[sh]: https://www.reddit.com/r/rust/comments/2lp3il/where_is_arithmetic_signed_rightshift/
[ctor-discuss]: http://internals.rust-lang.org/t/poll-foo-new-vs-foo-as-the-default-constructor/758
[ctor-reddit]: https://www.reddit.com/r/rust/comments/2lvrf5/foonew_vs_foo_as_the_default_constructor/
[psp]: https://www.reddit.com/r/rust/comments/2m10id/hello_world_on_a_psp_via_rust/
[target specs]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md
[await]: https://www.reddit.com/r/rust/comments/2m5rin/does_rust_have_something_like_c_async_wait/
[future]: https://www.reddit.com/r/rust/comments/2m64o5/stdsyncfuture_is_almost_useless_for_async/
[gpu]: http://internals.rust-lang.org/t/single-source-gpu-support/898
[mm]: https://www.reddit.com/r/rust/comments/2m9qw9/experienced_users_how_easy_is_rusts_memory/
[from]: http://internals.rust-lang.org/t/pre-rfc-remove-fromerror-trait-add-from-trait/783

## New Projects

* [rust-id3 and rust-metaflac][id3]. Reading and writing audio file metadata.
* [rust-resistant-taskpool][taskpool]. A load-balancing task pool.
* [Rust-Welder]. Experiments with error interop. [Second discussion][Rust-Welder2].
* [json_macros]. Create JSON via Rust syntax.
* [rust-bitfield]. A macro to generate bitfields.
* [rust-smtp]. SMTP client.
* [yaglw]. Yet another high-level OpenGL wrapper.
* [rustup.ps1]. A rustup.sh equivalent for Windows, installs Cargo
  alongside Rust, which the Rust installer currently fails to do.

[id3]: https://www.reddit.com/r/rust/comments/2lsfrd/rustid3_and_rustmetaflac_libraries_to_read_and/
[taskpool]: https://www.reddit.com/r/rust/comments/2ltjwm/a_loadbalancing_taskpool_resistant_to_child_panics/
[Rust-Welder]: https://www.reddit.com/r/rust/comments/2lwciy/feedback_and_discussion_wanted_rustwelder_a_crate/
[Rust-Welder2]: https://www.reddit.com/r/rust/comments/2m84s1/updated_rustwelder_errorresult_handling/
[json_macros]: https://www.reddit.com/r/rust/comments/2m3bjj/json_macros_construct_json_objects_in_rust_from/
[rust-bitfield]: https://www.reddit.com/r/rust/comments/2m82o9/a_procedural_macro_to_generate_bitfieldlike_stuct/
[rust-smtp]: https://www.reddit.com/r/rust/comments/2m8nla/rust_smtp_client_looking_for_feedback/
[yaglw]: https://www.reddit.com/r/rust_gamedev/comments/2m7l9a/yaglw_yet_another_opengl_wrapper/
[rustup.ps1]: https://www.reddit.com/r/rust/comments/2me6r7/rustupps1_a_rustupsh_equivalent_for_windows/

## Project Updates

* [This Week in Servo 11][twis].
* Piston has [seen some refactoring][pist] to how it handles event loops and windows.

[twis]: http://blog.servo.org/2014/11/11/twis-11/
[pist]: https://www.reddit.com/r/rust_gamedev/comments/2m7k6v/piston_update_glutin_loop/

## Upcoming Meetups

* [Rust Paris, Nov 17](http://www.meetup.com/Rust-Paris/events/185461312/)
* [Rust Bay Area: Cryptography and Rust, December 18th](http://www.meetup.com/Rust-Bay-Area/events/210632582/)
