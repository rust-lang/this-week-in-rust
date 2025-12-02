Title: This Week in Rust 4
Date: 2013-06-29 12:26
Category: This Week in Rust

Hello and welcome to the fourth issue of *This Week in Rust*, a weekly overview of Rust and its community.

`0.7` is being cut soon (today, I think). There are preliminary release notes [on
GitHub](https://github.com/mozilla/rust/blob/master/RELEASES.txt). The tree has been quite calm, with regards to
breakage. Cycle time is still high, but at least when things land they don't break master.

<!-- more -->

# What's cooking on master?

Issue churn this week was -1. Yay! Issue churn this month was -47. 61 people pushed 1,080 commits, changing 2,117 files
and adding a total of 53347 lines.  The top 10 committers were pcwalton, brson, dbaupp, strcat, bblum, nmatsakis,
acricto, Blei, me (cmr), and aatch.

Much of the work this week was cleanup or rebases of older PRs that just hadn't made it in yet.

## Notable additions, bugfixes, and cleanup

- Eridius has finished (I think!) the last bit of UNIX [terminal support](https://github.com/mozilla/rust/pull/7436),
  adding fallback and smarter detection. It should work in 8-color terminals now too.
- pcwalton [rewrote each_path](https://github.com/mozilla/rust/pull/7451), with the goal of future performance
  enhancements.
- tjc has done a [bunch](https://github.com/mozilla/rust/pull/7397) of
  [rustpkg](https://github.com/mozilla/rust/pull/7403) work.
- aatch did some [trans cleanup](https://github.com/mozilla/rust/pull/7272).
- gifnksm added [`max_by` and `min_by`](https://github.com/mozilla/rust/pull/7414) methods to `IteratorUtil` for getting
  the largest/smallest value in an iterator given a score function.
- brson got a fix that [releases large stacks](https://github.com/mozilla/rust/pull/7111) after they are used to lower
  memory usage.
- mw has a bunch of [debuginfo](https://github.com/mozilla/rust/pull/7432) work, as well as [docs and
  cleanup](https://github.com/mozilla/rust/pull/7255).
- Blei did an [intrinsic overhaul](https://github.com/mozilla/rust/pull/7254).
- acrichto implemented [`static mut`](https://github.com/mozilla/rust/pull/7291), for globals. Using them requires
  unsafe code.
- DaGenix [cleaned up and extended](https://github.com/mozilla/rust/pull/7207) the SHA code.
- acrichto renamed `.rc` files to `.rs`. `.rc` is deprecated and functionally equivalent to `.rs`, all new code should
  use it. He also [added](https://github.com/mozilla/rust/pull/7371) a `warnings` lint attribute for enabling/disabling
  warnings in bulk.
- indutny added [`mman` (including `mmap`)](https://github.com/mozilla/rust/pull/7257) FFI.
- dbaupp found a [curious performance win](https://github.com/mozilla/rust/pull/7297) by changing some ordering around,
  while also enabling conditionally defined macros and macro expansion to items with `#[cfg]` attributes.
- Luqman [fixed by-value self](https://github.com/mozilla/rust/pull/7410).
- acrichto expanded the `deriving(ToStr)` code to use `ToStr` on fields rather than using `fmt!("%?", x)`.
- Blei [fixed a lot of problems](https://github.com/mozilla/rust/pull/7214) with owned trait objects (`~Trait`).
- sully has [landed some default method fixes](https://github.com/mozilla/rust/pull/7471)

## Breaking Changes

- As usual, [a](https://github.com/mozilla/rust/pull/7274) [bunch](https://github.com/mozilla/rust/pull/7334)
  [of](https://github.com/mozilla/rust/pull/7373) [iterator](https://github.com/mozilla/rust/pull/7319) work.
- pcwalton renamed Owned to Send and Const to Freeze, better reflecting their actual semantics.
- He also disallowed `mut` from distributing over bindings. For example, the following code no longer works: `let mut
  (a, b) = (c, d)`.
- dbaupp [converted](https://github.com/mozilla/rust/pull/7430) many vector functions to methods. He has a [second
  part](https://github.com/mozilla/rust/pull/7487) in line.
- Luqman renamed the `finalize` method in the `Drop` trait to `drop`.

## Other changes

bblum did some trait/fn/closure bounds
[here](https://github.com/mozilla/rust/pull/7314) and
[here](https://github.com/mozilla/rust/pull/7354), but I don't really
understand what they do or their significance. He said:

> it changes the way traits/closures and captured data fulfill kind bounds, so
> you can restrict or loosen the requirements instead of having the fixed
> defaults of "can only capture Send things in ~fn/~Trait"
> 
> one example is that you can add the extra requirement of Freeze, so you can
> put existential data inside of ARCs
> 
> see https://github.com/mozilla/rust/blob/master/src/test/run-pass/trait-bounds-in-arc.rs
> 
> http://smallcultfollowing.com/babysteps/blog/2013/06/11/data-parallelism-in-rust/
> is another, more complicated but less contrived, example for how they would be
> useful

# Meetings

The [Tuesday meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-06-25)
mainly discussed `@mut` and iterators, as well as some minor 0.7 releng stuff.
I suggest reading the `@mut` discussion yourself, but essentially it revolves
around it not quite fitting into the language, and that it could be easily
punted to a library. The iterator discussion was not notable.

# Discussion + Blog posts

- [New container/iterator tutorial](http://static.rust-lang.org/doc/tutorial-container.html)
- ["Language support for external iterators"](http://thread.gmane.org/gmane.comp.lang.rust.devel/4528)
- ["Rust gets a lot of things right"](http://spin.atomicobject.com/2013/06/25/rust-language/)
- ["Planning a project in rust?"](http://www.reddit.com/r/rust/comments/1ha3yi/planning_a_project_in_rust/)
- ["Memory layout of types"](https://mail.mozilla.org/pipermail/rust-dev/2013-June/004572.html)
- [mw's second status report](http://michaelwoerister.github.io/2013/06/28/Status-Update-2.html)

# External projects

- QuickCheck for Rust.
	- [GitHub](https://github.com/blake2-ppc/qc.rs)
	- [Reddit discussion](http://www.reddit.com/r/rust/comments/1h0217/mockup_of_quickcheck/)
- RustGnuplot ([GitHub](https://github.com/SiegeLord/RustGnuplot))

