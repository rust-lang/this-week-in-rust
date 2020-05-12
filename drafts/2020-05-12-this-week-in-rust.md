Title: This Week in Rust 338
Number: 338
Date: 2020-05-12
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

* [Announcing Rust 1.43.1](https://blog.rust-lang.org/2020/05/07/Rust.1.43.1.html)
* [Rust concurrency: the single-writer principle.](https://medium.com/@polyglot_factotum/rust-concurrency-the-single-writer-principle-applied-aada2cdc6fb0?source=friends_link&sk=cafc8dcf8babf4ec95b1b62ccde7e54b)
* [This Month in Rust OSDev (April 2020)](https://rust-osdev.com/this-month/2020-04/)
* [A no_std Rust binary](https://fasterthanli.me/blog/2020/a-no-std-rust-binary/)
* [Notes on io_uring](https://boats.gitlab.io/blog/post/io-uring/)
* [no_std async/await - soon on stable](https://ferrous-systems.com/blog/stable-async-on-embedded/)
* ["try fn" without special-casing Result ](https://dev.to/cad97/try-fn-without-special-casing-result-4m5b)
* [time_it: A case study in Rust macros](https://notes.iveselov.info/programming/time_it-a-case-study-in-rust-macros)
* [Dynamic stylesheets and Yew](https://conradludgate.com/posts/yew-css/)
* [A practical introduction to async programming in Rust](http://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en)
* [Rust Closures in FFI](http://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/)
* [Collecting many errors from an iterator of Results](https://tarquin-the-brave.github.io/blog/posts/collecting-all-the-errors/).
* [Auto-currying Rust Functions](https://peppe.rs/posts/auto-currying_rust_functions/)
* [Converting bits to integers in Rust using generics](https://dev.to/citizen_stig/converting-bits-to-integers-in-rust-using-generics-2nfg)
* [Magnifying Glasses for Rust Assembly](https://www.justanotherdot.com/posts/magnifying-glasses-for-rust-assembly.html)
* [More Rust and Load Balancer Adventures](https://medium.com/@bparli/more-rust-and-load-balancer-adventures-fad07f4fb095) 
* [Porting North Korean Dictionaries with Rust](https://digitalnk.com/blog/2020/05/08/porting-north-korean-dictionaries-with-rust/)
* [Rust verification tools](https://alastairreid.github.io/rust-verification-tools/)
* [Series Announcement - Zero to Production in Rust](https://www.lpalmieri.com/posts/2020-05-10-announcement-zero-to-production-in-rust/)
* [What I Learned Contributing to Rust-Analyzer](https://dev.to/bnjjj/what-i-learned-contributing-to-rust-analyzer-4c7e)
* [What’s the difference between a Rust char and a Go rune?](https://www.christianfscott.com/rust-chars-vs-go-runes/)
* [Writing A Wayland Compositor In Rust](https://wiki.alopex.li/WritingAWaylandCompositorInRust)
* [Yak shaving conditional compilation in Rust](https://bitshifter.github.io/2020/05/07/conditional-compilation-in-rust/)
* [Oxidize Global Workshop and CfP announcement](https://ferrous-systems.com/blog/oxidize-global-workshop-and-cfp-announcement/)
* [Collecting many errors from an iterator of Results](https://tarquin-the-brave.github.io/blog/posts/collecting-all-the-errors/).
* [Writing Python inside your Rust code - Part 3](https://blog.m-ou.se/writing-python-inside-rust-3/)
* [Rust FFI - Building an ASN1 codec](https://sjames.github.io/articles/2020-04-26-rust-ffi-asn1-codec/)
* [video] [Building a simple GraphQL API with Actix and Juniper](https://youtu.be/7v7ERnrC4fo)
* [video] [Named Field Init in C, C++20, Zig, Rust, & D](https://www.youtube.com/watch?v=c-NyXKbqmQc)

# Crate of the Week

This week's crate is [cargo-workspaces](https://github.com/pksunkara/cargo-workspaces), a cargo subcommand to manage your cargo workspace.

Thanks to [Pavan Kumar Sunkara](https://users.rust-lang.org/t/crate-of-the-week/2704/768) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [s3rename: Atomic renames and asynchronous destructors](https://github.com/jamesmcm/s3rename/issues/16)
* [sedregex: Add support for translation commands y/ and tr/](https://gitlab.com/mexus/sedregex/-/issues/4)
* [clap: Usage suggests help subcommand when using DisableHelpSubcommand](https://github.com/clap-rs/clap/issues/1463)
* [displaydoc](https://github.com/yaahc/displaydoc/issues/15)
* [GitUI is looking for contributors](https://github.com/extrawurst/gitui/issues)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

375 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-05-04..2020-05-11

* [Define UB in float-to-int casts to saturate](https://github.com/rust-lang/rust/pull/71269)
* [Improve bitcode generation for Apple platforms](https://github.com/rust-lang/rust/pull/71970)
* [Don't force rustc to do codegen for LTO builds](https://github.com/rust-lang/cargo/pull/8192)
* [Correctly handle UEFI targets as Windows-like when emitting sections for LLVM bitcode](https://github.com/rust-lang/rust/pull/71881)
* [Prevent compiler stack overflow for deeply recursive code](https://github.com/rust-lang/rust/pull/55617)
* [resolve: Relax fresh binding disambiguation slightly to fix regression](https://github.com/rust-lang/rust/pull/71846)
* [upgrade chalk and use chalk-solve/chalk-ir/chalk-rust-ir](https://github.com/rust-lang/rust/pull/69406)
* [Report cannot move errors in promoted MIR](https://github.com/rust-lang/rust/pull/71587)
* [Simplify the `tcx.alloc_map` API](https://github.com/rust-lang/rust/pull/71508)
* [Suggest removing semicolon in last expression only if it's type is known](https://github.com/rust-lang/rust/pull/71894)
* [Skip attempting to run `coerce_unsized` on an inference variable](https://github.com/rust-lang/rust/pull/69530)
* [Unify the undo log of all snapshot types](https://github.com/rust-lang/rust/pull/69464)
* [Reduce `TypedArena` creations in `check_match`](https://github.com/rust-lang/rust/pull/71975)
* [Shrink `LocalDecl`](https://github.com/rust-lang/rust/pull/71942)
* [Add `remove_current_as_list` to `LinkedList`'s `CursorMut`](https://github.com/rust-lang/rust/pull/71878)
* [Add `Arc::`{`incr`, `decr`}`_strong_count`](https://github.com/rust-lang/rust/pull/70733)
* [Add Option to Force Unwind Tables](https://github.com/rust-lang/rust/pull/69984)
* [Make `BTreeMap::new` and `BTreeSet::new` const](https://github.com/rust-lang/rust/pull/71839)
* [`Btreemap` iter intertwined](https://github.com/rust-lang/rust/pull/71510)
* [Add `core::future::`{`pending`, `ready`}](https://github.com/rust-lang/rust/pull/70834)
* [futures: Refactor to reduce the amount of unsafe and duplicated code](https://github.com/rust-lang/futures-rs/pull/2128)
* [cargo: Update assertions in LTO calculations](https://github.com/rust-lang/cargo/pull/8226)
* [cargo: Try to remove secrets from http.debug](https://github.com/rust-lang/cargo/pull/8222)
* [cargo features: allow activated_features_unverified to communicate not-present](https://github.com/rust-lang/cargo/pull/8194)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2585: FC for unsafe blocks in unsafe fn](https://github.com/rust-lang/rfcs/pull/2585)
* [RFC 2904: Major Change Proposal](https://github.com/rust-lang/rfcs/pull/2904)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Deduplicate Cargo workspace information](https://github.com/rust-lang/rfcs/pull/2906)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stablilize saturating_abs and saturating_neg](https://github.com/rust-lang/rust/pull/71886)
* [disposition: merge] [Tweak and stabilize AtomicN::fetch_update](https://github.com/rust-lang/rust/pull/71843)
* [disposition: merge] [impl From<Cow> for Box, Rc, and Arc](https://github.com/rust-lang/rust/pull/71447)
* [disposition: merge] [Stabilize fn-like proc macros in expression, pattern and statement positions](https://github.com/rust-lang/rust/pull/68717)
* [disposition: merge] [Tracking issue for Weak::into_raw/from_raw & similar](https://github.com/rust-lang/rust/issues/60728)
* [disposition: clone] [Tracking issue for non_static_type_id](https://github.com/rust-lang/rust/issues/41875)

## New RFCs
* [Do not warn about similar ASCII-only idents](https://github.com/rust-lang/rfcs/pull/2923)
* [RFC - cargo templates](https://github.com/rust-lang/rfcs/pull/2922)

# Upcoming Events

### Online
* [May 12. Seattle, WA, US - Seattle Rust Meetup](http://www.meetup.com/Seattle-Rust-Meetup/)
* [May 14. San Diego, CA, US - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/270394980/)
* [May 14, Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybchbsb/)
* [May 20. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybchbbc/).
* [May 21. Turin, IT - Rust Turin Meetup](https://community.mozilla.org/events/gruppo-di-studio-di-rust/)
* [May 26. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybchbjc/)


### North America
* [May 13. Vancouver, BC, CA - Vancouver Rust Meetup](https://www.meetup.com/Vancouver-Rust/events/)
* [May 14. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybchbsb/).
* [May 25. Durham, NC, US - Triangle Rustaceans - Project Night and Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybchbhc/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs
* [Backend Engineer, Data Processing - Kraken - Remote](https://jobs.lever.co/kraken/246f7fd2-000a-4f61-8f53-b1cc783d51cb)
* [Backend Engineer, Futures - Kraken - Remote](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Ownership is purely conceptual: it is not something you can see in a disassembler.

– [Jay Oster on rust-users](https://users.rust-lang.org/t/what-is-the-formal-definition-of-ownership/41984/7)

Thanks to [Daniel H-M](https://users.rust-lang.org/t/twir-quote-of-the-week/328/868) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [srikwit](https://github.com/srikwit), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]().</small>
