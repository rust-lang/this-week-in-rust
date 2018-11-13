Title: This Week in Rust 26
Date: 2013-12-09 13:13
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*! It's a bit delayed
due to finals, but like the spinning of the Earth it goes on.

Shameless plug: [ask me to do
things](http://www.reddit.com/r/rust/comments/1sikak/ask_cmr_to_do_things/)
over my winter break!

<!-- more -->

# What's cooking on master?

52 PRs were merged this week.

## Breaking Changes

- The much-discussed [`Result` API
changes](https://github.com/mozilla/rust/pull/10364) have happened. The gist
of it is that `.ok()` and `.err()` return Options. The other changes didn't
land, I assume they will be coming later.
- The [JIT support has been
removed](https://github.com/mozilla/rust/pull/10758), due to lack of maintenance and upstream
changes (this came with the LLVM upgrade).
- Keywords (except self) are [no longer allowed as lifetime
parameters](https://github.com/mozilla/rust/pull/10675).
- `Path::init` has been [renamed](https://github.com/mozilla/rust/pull/10796)
back to `Path::new`.
- `std::str::from_utf8` [no longer
allocates](https://github.com/mozilla/rust/pull/10701).
- `std::util::ignore` [has been
renamed](https://github.com/mozilla/rust/pull/10701) to `std::prelude::drop`, to
better reflect what it does.
- Duplicate bindings are [no longer
allowed](https://github.com/mozilla/rust/pull/10776) in struct bindings. That
is, `let SomeStruct { foo, foo } = baz;` is no longer legal.
- Dynamic library propagation [is
reimplemented](https://github.com/mozilla/rust/pull/10777).
- `extra::c_vec` [has been
modernized](https://github.com/mozilla/rust/pull/10736).
- `MutableVector::mut_split` [has been
renamed](https://github.com/mozilla/rust/pull/10757) to `mut_split_at`.

## Other changes

- A part of the tutorial [has been
rewritten](https://github.com/mozilla/rust/pull/10690).
- Vim highlighting [has been
updated](https://github.com/mozilla/rust/pull/10793).
- `deriving` [has better error
messages](https://github.com/mozilla/rust/pull/10844).
- `StrSlice` has [grown some
documentation](https://github.com/mozilla/rust/pull/10824).
- Snapshots are now [statically
linked](https://github.com/mozilla/rust/pull/10809).
- From the "changes I don't really understand" department, [trait lifetime
parameters](https://github.com/mozilla/rust/pull/10506) are early bound in
associated functions.
- A race in the scheduler [has been
squelched](https://github.com/mozilla/rust/pull/10817).
- extra now has an [LRU cache](https://github.com/mozilla/rust/pull/10211).
- A `log_enabled!(level)` macro [has been
added](https://github.com/mozilla/rust/pull/10768), for checking if the
program is being run at a given log level.
- Static linking [now has docs](https://github.com/mozilla/rust/pull/10742)!
- Memory usage when compiling librustc has been killed by 130MB with some
[well-placed indirection](https://github.com/mozilla/rust/pull/10676).
- `MutableVector` has [another
iterator](https://github.com/mozilla/rust/pull/10739): `mut_chunks()`, for
iterating over mutable slices.
- The `reverse-complement` shootout benchmark [has been
rewritten](https://github.com/mozilla/rust/pull/10799).


## New contributors

- Alexandros Tasos
- Diego Ongaro
- Julia Evans
- osa1

# Meeting

The [weekly
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-12-03)
discussed using `box` for placement new, some rustpkg discussion, dropping the
dependency on a system C++ library, and the above `from_utf8` and `Result`
pull requests.

# This Week in Servo

Servo is a web browser engine written in Rust and is one of the primary test
cases for the Rust language.

We landed 19 PRs this week.

## Notable additions
- Simon Sapin added the CSS visibility property in
[#1331](https://github.com/mozilla/servo/pull/1331).
- Juneyoung Cho landed local bookmark navigation in
[#1262](https://github.com/mozilla/servo/pull/1262).
- ms2ger added reporting exceptions to JS when the DOM implementation fails in
[#1304](https://github.com/mozilla/servo/pull/1304).
- Keegan McAllister and Patrick Kim landed several changes to continue our
push to remove @-boxes to open up more of our data to safe parallel access
across task boundaries.

## New contributors
- Bruno de Oliveira Abinader
- Daniel Hedlund

## New intern
- Isabelle Carter will be joining us as part of the GNOME Outreach Program for
Women, [OPW]( https://wiki.gnome.org/OutreachProgramForWomen ), and will be
working on adding `position:fixed` support to Servo. The internship runs from
December 10 through March 10.

## Meetings
This week's
[meeting](https://github.com/mozilla/servo/wiki/Meeting-2013-12-02) was short
and mostly covered some build system changes that Jack is working on. In
short, we will use Rust's static linking to make it easier to distribute a
Servo binary and we are making a larger change to CMake in order to get both
more reliable parallel/incremental builds and get support for
cross-compilation.

# Announcements etc

- [Evict-BT](http://www.reddit.com/r/rust/comments/1s5laq/evictbt_an_issue_tracker_written_in_rust_that/)
 \- an issue tracker that integrates loosely with git.
- [Rust with
 Emscripten](http://www.reddit.com/r/rust/comments/1s8c0j/rust_with_emscripten/)
- [Day 36: On programming without
 malloc](http://www.reddit.com/r/rust/comments/1s3jgd/day_36_on_programming_without_malloc/)
- [Rust experience
 report](http://www.reddit.com/r/rust/comments/1s3osp/blast_from_the_past_pre01_raytracer_rustdev_rust/)
 \- a pre-0.1 raytracer and response to the language.
- [Rust frontend to
 GCC](http://www.reddit.com/r/rust/comments/1s0aj5/rust_frontend_to_gcc/)
- [Types in Rust, for
 Beginners](http://www.reddit.com/r/rust/comments/1ry4ym/types_in_rust_for_beginners/)
- [Thoughts on DST, part
 4](http://www.reddit.com/r/rust/comments/1rxj0x/thoughts_on_dst_part_4_including_a_recap_of_parts/)
- [Slides from pnkfelix's codemash
 presentation](http://pnkfelix.github.io/present-rust-codemesh2013/fklock-rust-codemesh2013.pdf).
- [An ML thread on redundant APIs involving
 `Option`](http://www.reddit.com/r/rust/comments/1seoe1/lets_avoid_having_both_foo_and_foo_opt/)
- [What do you want in a Rust Docker
 image?](http://www.reddit.com/r/rust/comments/1se6qa/rfc_what_do_you_want_in_a_rust_docker_image/)
- [A huge thread on lots of things
 Rust](http://www.reddit.com/r/rust/comments/1s9y7o/less_is_more_lambda_the_ultimate/)
- [Rust bindings to
 libsodium/NaCl](http://www.reddit.com/r/rust/comments/1s8opt/sodium_oxide_fast_cryptographic_library_for_rust/)
