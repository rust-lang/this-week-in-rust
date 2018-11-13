Title: This Week in Rust 13
Date: 2013-08-31 19:10
Category: This Week in Rust

Hello and welcome to the lucky 13th issue of *This Week in Rust*. Graydon has
stepped down as project lead. It seems to be a bit ambiguous whether he'll
still be working on the project. Brian (brson) will be taking over as lead.

<!-- more -->

# What's cooking in master?

## Breaking Changes

- [Some functions have been removed from
  `std::str`](https://github.com/mozilla/rust/pull/8857).
- [The unit test framework now uses RUST_TEST_TASKS instead of
  RUST_THREADS](https://github.com/mozilla/rust/pull/8823).
- [`typeof` is now a reserved
  keyword](https://github.com/mozilla/rust/pull/8718).
- [`offset_inbounds` has been
  removed](https://github.com/mozilla/rust/pull/8807).
- [Some edge cases with writing to a borrowed `&mut` have been
  closed](https://github.com/mozilla/rust/pull/8797), rejecting more incorrect
  programs.
- [Option no longer implements
  Add](https://github.com/mozilla/rust/pull/8772).
- [Some pass handling stuff has
  changed](https://github.com/mozilla/rust/pull/8700).
- [Enum descriminants are now always
  u64](https://github.com/mozilla/rust/pull/8744).

## Other Changes

- [String byte conversion functions which return an Option have been
  added](https://github.com/mozilla/rust/pull/8750).
- [The main tutorial links to the condition and error handling
  tutorials](https://github.com/mozilla/rust/pull/8764).
- [A trait for default initialization has been
  added](https://github.com/mozilla/rust/pull/8438).
- [debuginfo tests have been enabled on Windows, and debuginfo generation
  works on Windows!](https://github.com/mozilla/rust/pull/8757).
- [A better, more complete module tutorial has been
  written](https://github.com/mozilla/rust/pull/8777).
- [Some fixes to repr (the code that powers `%?` in fmt) have been
  made](https://github.com/mozilla/rust/pull/8771).
- [`rustpkg build` with no arguments now behaves a bit
  differently](https://github.com/mozilla/rust/pull/8697).
- [`std::run` has been reimplemented on top of
  libuv](https://github.com/mozilla/rust/pull/8645).
- [Frame pointer elimination has been
  re-disabled](https://github.com/mozilla/rust/pull/8838).
- [LLVM has been updated, giving us mingw-w64
  support](https://github.com/mozilla/rust/pull/8840).
- [Unit tests have been enabled on
  Windows](https://github.com/mozilla/rust/pull/8819).
- [Some compile speedups landed](https://github.com/mozilla/rust/pull/8802).

# Meeting

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-08-27)
discussed rustpkg's timeline, extern fns, and LLVM asserts.

# Projects, discussion, and announcements

- Jeaye says that anyone having problems with the q3 repo should re-clone. If
  that doesn't fix it, pop into IRC: #q3 on irc.freenode.net.
- ["Making rustpkg work"](http://tim.dreamwidth.org/1820526.html)
- ["nphysics: a 2d and 3d rigid body physics engine for
  Rust"](http://www.reddit.com/r/rust/comments/1lai9u/nphysics_a_2d_and_3d_rigid_body_physics_engine/)
- ["First university computer science class taught in
  Rust"](http://www.reddit.com/r/rust/comments/1l8hd4/first_university_computer_science_class_taught/)
