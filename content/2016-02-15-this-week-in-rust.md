Title: This Week in Rust 118
Number: 118
Date: 2016-02-15
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [Andre](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [Binding threads and processes to CPUs in Rust](http://nitschinger.at/Binding-Threads-And-Processes-to-CPUs-in-Rust/).
* [The many kinds of code reuse in Rust](http://cglab.ca/~abeinges/blah/rust-reuse-and-recycle/).
* [Code of heat conductivity](https://llogiq.github.io/2016/02/10/code.html). Llogiq on Rust's Code of Conduct.
* [Why Rust's ownership/borrowing is hard](http://softwaremaniacs.org/blog/2016/02/12/ownership-borrowing-hard/en/).
* [video] [Rust: Unlocking Systems Programming by Aaron Turon](http://www.infoq.com/presentations/rust-thread-safety).
* [This week in Servo 50](http://blog.servo.org/2016/02/08/twis-50/).
* [This week in Amethyst 4](https://thisweekinamethyst.wordpress.com/2016/02/08/twia-4/). Amethyst is a data-oriented game engine written in Rust.

## Notable New Crates & Project Updates

* [A MOS6502 assembler implemented as a Rust macro](https://play.rust-lang.org/?gist=a18d697454f9261b28ff&version=stable).
* [Ketos](https://github.com/murarth/ketos). Lisp dialect scripting and extension language for Rust programs.
* [Parity](https://ethcore.io/parity.html). Next Generation Ethereum Client, written in Rust.
* [TensorFlow Rust](https://github.com/google/tensorflow-rust). Rust language bindings for TensorFlow from Google.
* [rpc-perf](https://github.com/twitter/rpc-perf). A tool for benchmarking RPC services from Twitter.
* [rust-lzma](https://github.com/fpgaminer/rust-lzma). A Rust crate that provides a simple interface for LZMA compression and decompression.

# Updates from Rust Core

125 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-02-08..2016-02-15

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-mon-feb-08-2016/3152
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2016-02-12/3175

## Notable changes

* [[breaking batch] don't glob export `ast::UnOp` variants](https://github.com/rust-lang/rust/pull/31487).
* [[breaking batch] Rename and refactor ast::Pat_ variants](https://github.com/rust-lang/rust/pull/31581).
* [[breaking batch] Remove some unnecessary indirection from AST structures](https://github.com/rust-lang/rust/pull/31583).
* [Allow registering MIR-passes through compiler plugins](https://github.com/rust-lang/rust/pull/31425).
* [Add a new i586 Linux target](https://github.com/rust-lang/rust/pull/31629).
* [std: Deprecate all `std::os::*::raw` types](https://github.com/rust-lang/rust/pull/31551).
* [Workaround LLVM optimizer bug by not marking &mut pointers as noalias](https://github.com/rust-lang/rust/pull/31545).
* [Don't let `remove_dir_all` recursively remove a symlink](https://github.com/rust-lang/rust/pull/31468).
* [Split dummy-idx node to fix expand_givens DFS](https://github.com/rust-lang/rust/pull/31442).
* [Do not expect blocks to have type str](https://github.com/rust-lang/rust/pull/31651).
* [Add _post methods for blocks and crates](https://github.com/rust-lang/rust/pull/31562).
* [fix: read_link cannot open some files reported as symbolic links on windows](https://github.com/rust-lang/rust/pull/31630).

## New Contributors

* Adam Perry
* Carlos E. Garcia
* Daniel Robertson
* Felix Gruber
* Johan Lorenzo
* Kenneth Koski
* Masood Malekghassemi
* Richard Bradfield
* Scott Whittaker
* Thomas Winwood

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1317: Rust Language Server (IDE support)](https://github.com/rust-lang/rfcs/pull/1317).
* [RFC 1415: Deprecate type aliases in `std::os::*::raw`](https://github.com/rust-lang/rfcs/pull/1415).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Stabilize volatile read and write](https://github.com/rust-lang/rfcs/pull/1467).
* [Move some net2 functionality into libstd](https://github.com/rust-lang/rfcs/pull/1461).
* [Extend atomic compare_and_swap](https://github.com/rust-lang/rfcs/pull/1443).
* [Safe `memcpy` from one slice to another of the same type and length](https://github.com/rust-lang/rfcs/pull/1419).
* [Add `maybe!` macro equivalent to `try!`](https://github.com/rust-lang/rfcs/pull/1394).
* [Add additional `try!(expr => return)` that will return without value](https://github.com/rust-lang/rfcs/pull/1393).
* [Add a `let...else` expression, similar to Swift's `guard let...else`](https://github.com/rust-lang/rfcs/pull/1303).
* [Add macros to get the values of configuration flags](https://github.com/rust-lang/rfcs/pull/1258).
* [Implement `Into`, `From` and new trait `IntegerCast` for primitive integer types](https://github.com/rust-lang/rfcs/pull/1218).
* [Add `retain_mut` to `Vec` and `VecDeque`](https://github.com/rust-lang/rfcs/pull/1353).
* [Propose a design for _specialization_, which permits multiple `impl` blocks to apply to the same type/trait](https://github.com/rust-lang/rfcs/pull/1210).

## New RFCs

* [Add octet-oriented interface to `std::net::Ipv6Addr`](https://github.com/rust-lang/rfcs/pull/1498).
* [Extend the pattern syntax for alternatives in `match` statement](https://github.com/rust-lang/rfcs/pull/1500).
* [Add `#[clear_on_drop]` and `#[clear_stack_on_return]` to securely clear sensitive data after use](https://github.com/rust-lang/rfcs/pull/1496).
* [Amend RFC 550 with misc. follow set corrections](https://github.com/rust-lang/rfcs/pull/1494).

# Upcoming Events

* [2/16. San Diego Rust: Eat– Drink– Rust! Downtown Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/228573576/).
*  2/17. Copenhagen Rust Group meetup.
* [2/17. Rust Los Angeles Monthly Meetup](http://www.meetup.com/Rust-Los-Angeles/events/228104697/).
* [2/17. Rust Berlin: Leaf and Collenchyma](http://www.meetup.com/Rust-Berlin/events/227321071/).
* [2/18. Rust Hack and Learn Hamburg @ 4=1](http://www.meetup.com/Rust-Meetup-Hamburg/events/228502426/?rv=ea1&_af=event&_af_eid=228502426&https=off).
* [2/24. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [2/25. Tokyo Rust Meetup #3](http://www.meetup.com/Tokyo-Rust-Meetup/events/228425744/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [rayon](https://crates.io/crates/rayon),
which gives us `par_iter()`/`par_iter_mut()` functions that use an internal thread pool to easily parallelize data-parallel operations.
There's also `rayon::join(|| .., || ..)` for Fork-Join-style tasks. Apart from the ease of use, it also performs very well, comparable to hand-optimized code.

Thanks to [LilianMoraru](https://users.rust-lang.org/users/LilianMoraru) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
