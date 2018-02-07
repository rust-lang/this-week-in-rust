Title: This Week in Rust 220
Number: 220
Date: 2018-02-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [The 2018 Rust event lineup](https://blog.rust-lang.org/2018/01/31/The-2018-Rust-Event-Lineup.html).
* [A vision for portability in Rust](https://aturon.github.io/2018/02/06/portability-vision/).
* [Async/await II: Narrowing the scope of the problem](https://boats.gitlab.io/blog/post/2018-01-30-async-ii-narrowing-the-scope/).
* [Async/await III: Moving forward with something shippable](https://boats.gitlab.io/blog/post/2018-01-30-async-iii-moving-forward/).
* [In Rust, ordinary vectors are values](http://smallcultfollowing.com/babysteps/blog/2018/02/01/in-rust-ordinary-vectors-are-values/).
* [Make your own make (build system)](https://matklad.github.io/2018/01/03/make-your-own-make.html).
* [Introduction to procedural macros](https://tinkering.xyz/posts/introduction-to-proc-macros/).
* [Writing a command-line program in Rust](https://people.gnome.org/~federico/blog/writing-a-command-line-program-in-rust.html).
* [Writing eBPF tracing tools in Rust](https://jvns.ca/blog/2018/02/05/rust-bcc/).
* [Compiling to eBPF from Rust](https://unhandledexpression.com/2018/02/02/poc-compiling-to-ebpf-from-rust/).
* [Experimenting with the new I/O framework for embedded systems](http://pramode.in/2018/01/31/ti-launchpad-with-rust-new-io/).
* [These weeks in dev-tools 3](https://www.ncameron.org/blog/these-weeks-in-dev-tools-issue-3/).
* [This week in Rust docs 91](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-91).

# Crate of the Week

This week's crate is [rust-semverver](https://github.com/rust-lang-nursery/rust-semverver), an as-of-yet buggy, but already useful semantic versioning (semver) checking tool. Thanks to [Philipp Hansch](https://users.rust-lang.org/u/phansch) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [Help Rayon prepare for 1.0](https://users.rust-lang.org/t/rayon-1-0-on-feb-14/14950).
* [gutenberg: Make content::Section hold references](https://github.com/Keats/gutenberg/issues/205). Gutenberg is an opinionated static site generator with everything built-in.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

135 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-22..2018-01-29

* [stabilized `#[repr(align(x))]` attribute](https://github.com/rust-lang/rust/pull/47006) (RFC [#1358](https://rust-lang.github.io/rfcs/1358-repr-align))
* [implement intra-rustdoc links](https://github.com/rust-lang/rust/pull/47046) (RFC [#1946](https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links))
* [fix type inhabitedness check for arrays](https://github.com/rust-lang/rust/pull/47600)
* [fix ICE on const eval of union field](https://github.com/rust-lang/rust/pull/47794)
* [fix ICE when return type includes unconstrained anon region](https://github.com/rust-lang/rust/pull/47668)
* [fix ICE when use trees have multiple empty nested groups](https://github.com/rust-lang/rust/pull/47705)
* [fix ICE with `use self;`](https://github.com/rust-lang/rust/pull/47633)
* [fix never-type rvalue ICE](https://github.com/rust-lang/rust/pull/47746)
* [track recursion limit when expanding existential impl trait](https://github.com/rust-lang/rust/pull/47529)
* [rustc: SIMD types use pointers in Rust's ABI](https://github.com/rust-lang/rust/pull/47743)
* [add CGU size heuristic for partitioning](https://github.com/rust-lang/rust/pull/47415)
* [first round of LLVM 6.0.0 compatibility](https://github.com/rust-lang/rust/pull/47710)
* [let LLVM 5 add DW_OP_deref to indirect args itself](https://github.com/rust-lang/rust/pull/47688)
* [LLVM5: update DW_OP_plus to DW_OP_plus_uconst](https://github.com/rust-lang/rust/pull/47610)
* [rustc: load the `rustc_trans` crate at runtime](https://github.com/rust-lang/rust/pull/47671)
* [rustc: add `-C lto=`{`thin`, `fat`} option](https://github.com/rust-lang/rust/pull/47521)
* [properly pass down immutability info for thread-locals](https://github.com/rust-lang/rust/pull/47425)
* [simplify irrefutable slice patterns](https://github.com/rust-lang/rust/pull/47374)
* [make use of the implemented red/green algorithm for variance](https://github.com/rust-lang/rust/pull/47696)
* [do not capture stderr in the compiler. Instead just panic silently for fatal errors](https://github.com/rust-lang/rust/pull/47634)
* [fix spans in unused import lint for nested groups](https://github.com/rust-lang/rust/pull/47726)
* [add `-Z teach` flag to provide extended diagnostic help](https://github.com/rust-lang/rust/pull/47652)
* [on missing method do not suggest private traits](https://github.com/rust-lang/rust/pull/47534)
* [immovable generators](https://github.com/rust-lang/rust/pull/45337)
* [NLL test for mutating &mut references](https://github.com/rust-lang/rust/pull/47609)
* [make the constructors of Duration const fns](https://github.com/rust-lang/rust/pull/47300)
* [make core::ops::Place an unsafe trait](https://github.com/rust-lang/rust/pull/47299)
* [make UnsafeCell::into_inner safe](https://github.com/rust-lang/rust/pull/47204)
* [expose float `from_bits` and `to_bits` in libcore](https://github.com/rust-lang/rust/pull/46931)
* [use the slice length to hint the optimizer about iter.position result](https://github.com/rust-lang/rust/pull/47772)
* [add rustc-args option to test runner](https://github.com/rust-lang/rust/pull/47558)
* [rustdoc: show when traits are auto traits](https://github.com/rust-lang/rust/pull/47672)

## New Contributors

* Araam Borhanian
* dpc
* Jay Strict
* Jonathan Goodman
* Matthias Krüger
* oberien
* Onur Aslan
* penpalperson
* Per Lundberg

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2136: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [Allow trivial constraints to appear in where clauses](https://github.com/rust-lang/rfcs/pull/2056).
* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).
* [disposition: merge] [impl-only-use](https://github.com/rust-lang/rfcs/pull/2166). The `use …::{… as …}` syntax can now accept `_` as alias to a trait to only import the implementations of such a trait.
* [disposition: merge] [or-patterns in if / while let expressions](https://github.com/rust-lang/rfcs/pull/2175).
* [disposition: merge] [Formally define repr(u32, i8, etc...) and repr(C) on enums with payloads](https://github.com/rust-lang/rfcs/pull/2195).
* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [`?` repetition in macro rules](https://github.com/rust-lang/rfcs/pull/2298).
* [disposition: postpone] [Allow fields in traits that map to lvalues in impl'ing type](https://github.com/rust-lang/rfcs/pull/1546).
* [disposition: postpone] [Fix the handling of uninhabited types in pattern matching](https://github.com/rust-lang/rfcs/pull/1872).
* [disposition: postpone] [Unions 1.2](https://github.com/rust-lang/rfcs/pull/1897).
* [disposition: postpone] [Allow destructuring of structs that implement Drop](https://github.com/rust-lang/rfcs/pull/2061).
* [disposition: postpone] [Adding unsafe modules and unsafe blocks outside functions](https://github.com/rust-lang/rfcs/pull/2148).
* [disposition: close] [Guard Clause Flow Typing](https://github.com/rust-lang/rfcs/pull/2221).
* [disposition: close] [Legal double reference](https://github.com/rust-lang/rfcs/pull/2268).
* [disposition: close] [Add match/in statements](https://github.com/rust-lang/rfcs/pull/2144).

## New RFCs

* [Custom test frameworks](https://github.com/rust-lang/rfcs/pull/2318).
* [Default type parameter fallback revisited](https://github.com/rust-lang/rfcs/pull/2321).
* [Add macro expansion API to proc macros](https://github.com/rust-lang/rfcs/pull/2320).

# Upcoming Events

* [Feb  8. Helsinki - Finland Rust-lang Group](https://www.meetup.com/Finland-Rust-Meetup/events/246866694/).
* [Feb  8. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxdblb/).
* [Feb  8. San Diego Rust February Meetup](https://www.meetup.com/San-Diego-Rust/events/246906809/).
* [Feb  8. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb  9. Rust Table of Regulars Darmstadt / Germany](https://www.meetup.com/Rust-Rhein-Main/events/246744631)
* [Feb 10. Mangalore, India - RUSTCON2k18](https://www.rustcon2k18.in/).
* [Feb 11. Rust Dev in Mountain View - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxdbpb/).
* [Feb 12. Rust London User Group - LDN Talks: February 2018](https://www.meetup.com/Rust-London-User-Group/events/246860921/).
* [Feb 12. Rust Amsterdam - Perl FFI && Long-term reliability in Rust projects](https://www.meetup.com/Rust-Amsterdam/events/247120013/).
* [Feb 12. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxdbqb/).
* [Feb 13. Rust Roma - Rust learning and hacking evening #6](https://www.meetup.com/it-IT/Rust-Roma/events/247507331/).
* [Feb 13. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-content)
* [Feb 14. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 14. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 15. Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnyxdbtb/).
* [Feb 18. Rust Dev in Mountain View - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxdbxb/).
* [Feb 21. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 21. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 22. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb 22. Rust London User Group - LDN Talks: February 2018](https://www.meetup.com/Rust-London-User-Group/events/246860921/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Rust Engineer at Reddit](https://www.reddit.com/r/rust/comments/7utj4t/reddit_is_hiring_a_senior_rust_engineer/).
* [Rust Engineer at MaidSafe](https://maidsafe.net/careers.html#rust_engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> > Rust has a very high friction coefficient.
> We call it grip and it lets us drive fearlessly around hard corners very fast.

— [u/asmx85 on reddit](https://www.reddit.com/r/programming/comments/7ugm8e/c2_c_with_cleaner_syntax_a_module_system_no/dtkde2s/).

Thanks to [JustAPerson for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/488)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
