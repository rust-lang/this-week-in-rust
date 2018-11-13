Title: This Week in Rust 6
Date: 2013-07-13 16:07
Category: This Week in Rust

Hello and welcome to the sixth issue of *This Week in Rust*, a weekly overview
of Rust and its community.

<!-- more -->

# What's cooking on master?

Issue churn this week was -17! A total of 63 PRs were merged this week, twice
as many as last week. Not bad!

## Breaking changes

- The [task local storage (TLS)
  API](https://github.com/mozilla/rust/pull/7677) was cleaned up (there's
  still [one PR](https://github.com/mozilla/rust/pull/7751) in the queue that
  finishes it up).
- [DList was modernized](https://github.com/mozilla/rust/pull/7652).
- [`extra::json` now uses `Iterator<char>` rather than a
  `@Reader`](https://github.com/mozilla/rust/pull/7704)
- [Various free-standing functions in f32 etc were
  removed](https://github.com/mozilla/rust/pull/7117)
- [ref bindings in irrefutable patterns has been tightened
  up](https://github.com/mozilla/rust/pull/7262). This potentially fixes *and
  breaks* code. It's breaking because the compiler now rejects incorrect
  programs that it did not before.
- [str no longer encodes invalid
  utf-8](https://github.com/mozilla/rust/pull/7612).
- [`extra::rope` was removed](https://github.com/mozilla/rust/pull/7629)
- [`extra::net_ip` and so on were
  removed](https://github.com/mozilla/rust/pull/7594). They were redundant
  names for `extra::net::ip` and such
- [`Iterator::size_hint`'s lower bound is no longer an
  Option](https://github.com/mozilla/rust/pull/7570)

## Notable compiler additions, bugfixes, and cleanup

- [Unnecessary basic blocks were
  removed](https://github.com/mozilla/rust/pull/7763). This makes for much
  easier to read unoptimized IR.
- [Use of `*int`/`*uint` is now properly
  warned](https://github.com/mozilla/rust/pull/7734) in FFI functions.
- [More default method fixes](https://github.com/mozilla/rust/pull/7725).
- A [needless copy](https://github.com/mozilla/rust/pull/7717) was removed
  from immediate values (I'm pretty sure LLVM optimized it away when
  optimizations were on, not positive).
- A [lint for overqualified names](https://github.com/mozilla/rust/pull/7706)
  was added.
- [SIMD arithmetic](https://github.com/mozilla/rust/pull/7705) was
  implemented.
- A [graph abstraction and CFG](https://github.com/mozilla/rust/pull/7688) was
  introduced, to unify how the various pieces of the compiler use graphs.
- [The maximum lifetime of stack
  closures](https://github.com/mozilla/rust/pull/7455) is now constrained. Not
  quite sure what that means, but it fixes a segfault.
- [repr doesn't infinite loop](https://github.com/mozilla/rust/pull/7683) on
  zero-sized structs (ie, unit structs).
- [Type parameter pretty printing](https://github.com/mozilla/rust/pull/7698)
  was fixed, it now prints the type name rather than `'a` and `'b` and
  soforth.
- [`mut` in default method arguments is now
  allowed](https://github.com/mozilla/rust/pull/7631).
- [IR for calls with immediate return
  values](https://github.com/mozilla/rust/pull/7645) was improved.
- [Exchange allocation headers (ie, the headers on `~T`) were
  removed](https://github.com/mozilla/rust/pull/7605). This was a heroic
  effort by strcat and Luqman.
- [`-Z trans-stats` now reports perf-function
  statistics](https://github.com/mozilla/rust/pull/7456)
- [Scopes were decoupled from LLVM basic
  blocks](https://github.com/mozilla/rust/pull/7636), improving unoptimized
  builds, and allowing more things in optimized builds to be inlined.
- [An infinite loop when recursively including
  modules](https://github.com/mozilla/rust/pull/7585) was fixed.
- An [ICE involving struct-like enum
  variants](https://github.com/mozilla/rust/pull/7557) was fixed.
- The buildsystem [cleans up old
  libraries](https://github.com/mozilla/rust/pull/7637) when it needs to.
- [A bunch of managed boxes](https://github.com/mozilla/rust/pull/7615) were
  removed from the AST.

## Notable library additions, bugfixes, and cleanup

- [`print!` and `println!` macros](https://github.com/mozilla/rust/pull/7775)
  were added (though [#7779](https://github.com/mozilla/rust/issues/7779)
  renames them).
- [Ord now uses default methods](https://github.com/mozilla/rust/pull/7765),
  allowing you to get default implementations for everything but `lt`.
- [`extra::Bitv` now takes `&[bool]` rather than
  `~[uint]`](https://github.com/mozilla/rust/pull/7730).
- [x64 now uses large stacks (4 MiB) by
  default](https://github.com/mozilla/rust/pull/7728).
- [`is_utf8` is now 22% faster](https://github.com/mozilla/rust/pull/7696)
- [Metrics reporting and
  ratcheting](https://github.com/mozilla/rust/pull/7623) was added to the test
  harness.
- A [DoubleEndedIterator](https://github.com/mozilla/rust/pull/7707) was
  added.
- A [`mut_split` method was added](https://github.com/mozilla/rust/pull/7691)
  to partition a `&mut [T]` into two pieces.
- We [now have pointer arithmetic](https://github.com/mozilla/rust/pull/7631).
- A [month's work of runtime work](https://github.com/mozilla/rust/pull/7265)
  landed.
- [A safe, cross-platform `mmap`
  wrapper](https://github.com/mozilla/rust/pull/7528) was added.
- [SmallIntMap and SmallIntSet have external
  iterators](https://github.com/mozilla/rust/pull/7614).
- [JSON parsing got 93% faster](https://github.com/mozilla/rust/pull/7608)
- [Deque](https://github.com/mozilla/rust/pull/7562) got a good cleanup and
  speedup.
- [vec now implements `pop_opt` and `shift_opt`
  methods](https://github.com/mozilla/rust/pull/7602).
- A [`peek_` adaptor](https://github.com/mozilla/rust/pull/7604) was added,
  which calls a closure on ever item before returning it. Mostly useful for
  debugging your iterators.

## Documentation etc

- [vim](https://github.com/mozilla/rust/pull/7742)
  [improvements](https://github.com/mozilla/rust/pull/7665) landed.
- [`po4a` support for translation](https://github.com/mozilla/rust/pull/7641)
  was added.
- [`libc::c_void` is better
  documented](https://github.com/mozilla/rust/pull/7690).
- [Man pages](https://github.com/mozilla/rust/pull/7632) for all the tools are
  now included.
- The [iterator tutorial](https://github.com/mozilla/rust/pull/7736) was
  extended.

# Meetings

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-07-09) was
all about split stacks and FFI. It's complex and above my ability to
summarize, but from what I can tell no real consensus was made. But, it's the
best enumeration of all the various issues surrounding split stacks that I've
seen.

# Discussion + Blog posts

From now on I'm going to just link to the reddit thread if there is one, as it
often has additional comments or insights.

- [A simple, self-contained example of using a shared
  library](https://gist.github.com/jmptable/5980297)
- [Experimental Actor
  Library (reddit)](http://www.reddit.com/r/rust/comments/1i3c15/experimental_actor_library_in_rust/)
- [Herb Sutter describes Rust
  (reddit)](http://www.reddit.com/r/rust/comments/1i30sw/herb_sutter_describes_rust_c_questions_and/)
- [Philosophy and "for" loops
  (reddit)](http://www.reddit.com/r/rust/comments/1i2y9e/philosophy_and_for_loops_more_from_go_and_rust/)
- [Reddit thread about the weekly
  meeting](http://www.reddit.com/r/rust/comments/1hy6l9/meetingweekly20130709_split_stacks_ffi/)
- [BZIP2 bindings
  (reddit)](http://www.reddit.com/r/rust/comments/1hxp2s/little_bzip2_binding_library_as_well_as_a_bigger/)
- [Proposal for an additional use case of the "in" keyword besides for loops
  (reddit)](http://www.reddit.com/r/rust/comments/1hsqf5/proposal_for_an_additional_use_case_of_the_in/)
- [Technical Q&A on Servo
  (reddit)](http://www.reddit.com/r/rust/comments/1i6ykh/technical_qa_on_servo/)
