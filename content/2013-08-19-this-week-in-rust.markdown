Title: This Week in Rust 11
Date: 2013-08-19 00:33
Category: This Week in Rust

Hello and welcome to the 11th edition of `This Week in Rust`! I'm starting uni
this week, so if you notice I'm not quite as omnipresent and omniprescient as
usual, that'd be why. **Please** [send me an email](mailto:corey@octayn.net)
if you would like your pull request, project, or blog post mentioned. I would
hate to overlook something cool or important.

<!-- more -->

The Mozilla Intern talks happened this past week or so. The ones I know about
relating to Rust are:

- [Default Methods in Rust
  (sully)](https://air.mozilla.org/intern-presentation-sullivan/)
- [Types of Types in Rust
  (bblum)](https://air.mozilla.org/ben-blum-from-the-research-team-presents-types-of-types-in-rust/)
- [A Work-stealing Runtime for Rust
  (toddaaro)](https://air.mozilla.org/2013-intern-todd/)
- [A Forest of Quadtrees: The Graphics of
  Servo](https://air.mozilla.org/eston-schweickart-from-the-research-team-presents-a-forest-of-quadtrees-the-graphics-of-servo/)
- [Layout in Servo: Parallel and Rustic Tree Traversals
  (eatkinson)](https://air.mozilla.org/2013-intern-presentations-august-13/)
- [Architecting Servo: Pipelines and Parallelism
  (tikue)](https://air.mozilla.org/2013-intern-kuehn/)

Congratulations to them all. The interns did a ton of great work over the
summer.

# What's cooking on master?

There were only 46 PRs merged this week. I don't quite know why that number is
so low this week. It certainly wasn't for lack of PRs: the queue has been
constantly backlogged. Issue churn was -26, yay!

## Breaking Changes

- [Some functions in Result and Either were replaced to work with external
  iterators](https://github.com/mozilla/rust/pull/8526)
- [The `priv` and `pub` visibility modifiers are now forbidden on contexts
  where they have no meaning](https://github.com/mozilla/rust/pull/8423). For
  example, marking a struct field `pub`, or a module `priv`. The compiler
  errors for this are quite informative, and the conversion is purely
  mechanical.
- [`to_c_str` now raises a condition if the string contains interior `NUL`s,
  as it is impossible to create a valid C string with interior
  `NUL`s](https://github.com/mozilla/rust/pull/8532).

## Library improvements, bugfixes, and cleanup

- [`ifmt!`, the new formatter, has been
  finished](https://github.com/mozilla/rust/pull/8446). Yay!
- [`extra::stats::write_boxplot` now works with negative or zero sample
  values](https://github.com/mozilla/rust/pull/8453).
- [Some missing pieces in libstd have been filled
  in](https://github.com/mozilla/rust/pull/8452).
- [A `sample` method has been added to `RngUtil`, for resevior
  sampling](https://github.com/mozilla/rust/pull/8491).

## Compiler improvements, bugfixes, and cleanup

- [A ton of work was done on a new
  visitor](https://github.com/mozilla/rust/pull/8527). This is the first of a
  series of five.
- [Vector repeat exprs (`[0, ..16]`) are now allowed in
  statics](https://github.com/mozilla/rust/pull/8483).
- [A hint has been added for incorrect use of static
  methods](https://github.com/mozilla/rust/pull/8477).
- [Trait object coercion to `&Trait` has been fixed to handle freezing and
  reborrowing more correctly](https://github.com/mozilla/rust/pull/8497).
- [Debuginfo of lexical scopes and variable shadowing has been massively
  improved](https://github.com/mozilla/rust/pull/8329).
- [A `--target-cpu` flag has been added to select the target CPU, rather than
  always using "generic"](https://github.com/mozilla/rust/pull/8410).
- [Support for owned and borrowed trait objects has been made better
  added](https://github.com/mozilla/rust/pull/8455).
- [An `address_insignificant` attribute has been
  added](https://github.com/mozilla/rust/pull/8421). LLVM will do merging of
  statics marked with that attribute.
- [Intrinsics for checked overflow on add, sub, and mul have been
  added](https://github.com/mozilla/rust/pull/8408).

## Tools, documentation, etc

- [The tutorial was translated into
  Japanese](https://github.com/mozilla/rust/pull/8469). I think this is the
  first translation of anything, so it's a pretty big milestone I think.

# Meeting

The [Tuesday
meeting](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-08-13)
discussed turning jemalloc back on, default arguments, and method invocation
ordering. It also discussed the new IO code and stage0 stdtest.

# Notable discourse

- [Phantom Types in
  Rust](http://bluishcoder.co.nz/2013/08/15/phantom_types_in_rust.html)
- [RFC: Runtimeless
  libstd](http://www.reddit.com/r/rust/comments/1k6hua/rustdev_rfc_runtimeless_libstd/)

# External projects

- [Bindings to elasticsearch](https://github.com/erickt/rust-elasticsearch)
- [zeromq bindings have been updated](https://github.com/erickt/rust-zmq)
- [A spellchecker for Rust code, written in Rust](https://github.com/huonw/spellck)
- [rust-encoding: character encoding support for
  Rust](http://www.reddit.com/r/rust/comments/1kd8ah/rustencoding_character_encoding_support_for_rust/)
- [A simple vocabulary
  trainer](http://www.reddit.com/r/rust/comments/1kctjn/my_first_rust_program_vocabulary_trainer/)
- [`rustdoc_ng`: 95%
  done](http://www.reddit.com/r/rust/comments/1k7mfn/rustdev_rustdoc_ng_95_done/)
- [Some pages as rendered by
  Servo](http://www.reddit.com/r/rust/comments/1k5kqx/some_pages_in_servo_as_of_20130810/)
- [d3cap: a libpcap-based network activity
  visualizer](https://github.com/jfager/d3cap)
- [postgres bindings](https://github.com/sfackler/rust-postgres)
- [RemoteJoy: a program for remotely viewing the screen of your PlayStation
  Portable](https://gist.github.com/luqmana/6264106).
  ([screnshot](http://i.imgur.com/9Kda25J.jpg))
- [The new OpenGL loader is working, pending the function pointer
  fix](https://github.com/bjz/gl-rs)
