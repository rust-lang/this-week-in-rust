Title: This Week in Rust 404
Number: 404
Date: 2021-08-18
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [synth](https://github.com/getsynth/synth), a declarative test data generator written in Rust.

[llogiq](https://users.rust-lang.org/t/crate-of-the-week/2704/942) shamelessly used his recent self-suggestion for lack of another nomination.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Module of the Week

Continuing [Rust Module of the Week](https://motw.rs) this week is [std::fs Part 2: Dirs, Dirs, Dirs](https://motw.rs/blog/2021/08/08/stdfs-part-2-dirs-dirs-dirs/). Contribution and feedback welcome [here](https://github.com/slyons/rust-module-of-the-week).

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

197 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?p=20&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-08-09..2021-08-16&type=Issues

* [make deleted code in a suggestion clearer](https://github.com/rust-lang/rust/pull/86532)
* [point to where clause for GATs to add bound](https://github.com/rust-lang/rust/pull/87478)
* [use smaller spans when suggesting method call disambiguation](https://github.com/rust-lang/rust/pull/87889)
* [use a more accurate span on assoc types WF checks](https://github.com/rust-lang/rust/pull/87819)
* [constify implementations of (`Try`)`From` for int types](https://github.com/rust-lang/rust/pull/86840)
* [add automatic migration for `assert!(.., string)`](https://github.com/rust-lang/rust/pull/87982)
* [fix closure migration suggestion when the body is a macro](https://github.com/rust-lang/rust/pull/87956)
* [associated functions that contain `extern` indicator or have `#[rustc_std_internal_symbol]` are reachable](https://github.com/rust-lang/rust/pull/86492)
* [LLVM codegen: don't emit zero-sized padding for fields](https://github.com/rust-lang/rust/pull/87254)
* [uplift the invalid_atomic_ordering lint from clippy to rustc](https://github.com/rust-lang/rust/pull/84039)
* [run `RemoveZsts` pass at `mir-opt-level=1`](https://github.com/rust-lang/rust/pull/83417)
* [implement `black_box` using intrinsic](https://github.com/rust-lang/rust/pull/87916)
* [specialize `Vec::clone_from` for `Copy` types](https://github.com/rust-lang/rust/pull/87913)
* [add support for `clobber_abi` to `asm!`](https://github.com/rust-lang/rust/pull/87581)
* [deprecate `llvm_asm!`](https://github.com/rust-lang/rust/pull/87590)
* [add the Option::unzip() method](https://github.com/rust-lang/rust/pull/87636)
* [test and fix `size_hint` for slice’s (`r`)`split`* iterators](https://github.com/rust-lang/rust/pull/87974)
* [implement `Extend<(A, B)>` for `(Extend<A>, Extend<B>)`](https://github.com/rust-lang/rust/pull/85835)
* [cargo: teach cargo to failfast on recursive/corecursive aliases](https://github.com/rust-lang/cargo/pull/9791)
* [cargo: fix value-after-table error with profiles](https://github.com/rust-lang/cargo/pull/9789)
* [cargo: ability to specify the output name for a bin target different from the crate name](https://github.com/rust-lang/cargo/pull/9627)
* [clippy: `never_loop`: suggest using an if let instead of a for loop](https://github.com/rust-lang/rust-clippy/pull/7541)
* [clippy: properly handle `Self` type for `trivially_copy_pass_by_ref`](https://github.com/rust-lang/rust-clippy/pull/7535)
* [clippy: check expr usage for `manual_flatten`](https://github.com/rust-lang/rust-clippy/pull/7566)
* [clippy: fix `manual_map` non-compiling suggestions](https://github.com/rust-lang/rust-clippy/pull/7531)
* [clippy: fix false positive on `filter_next`](https://github.com/rust-lang/rust-clippy/pull/7562)
* [clippy: fix `nonstandard_macro_braces` false positive](https://github.com/rust-lang/rust-clippy/pull/7478)
* [clippy: use `avoid-breaking-exported-api` configuration in `types` module](https://github.com/rust-lang/rust-clippy/pull/7560)
* [clippy: add `unwrap_or_else_default` lint](https://github.com/rust-lang/rust-clippy/pull/7516)

### Rust Compiler Performance Triage

Quiet week for performance, with no large changes. Regressions are limited to just a few benchmarks.

Triage done by **@simulacrum**.
Revision range: [998cfe5..3354a44](https://perf.rust-lang.org/?start=998cfe5aad7c21eb19a4bca50f05a13354706970&end=3354a44d2fa8d5ba6b8d6b40d2596de2c8292ec1&absolute=false&stat=instructions%3Au)

2 Regressions, 0 Improvements, 0 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-08-03.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Overconstraining and omitting unsafe in impls of unsafe trait methods](https://github.com/rust-lang/rfcs/pull/2316)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for UnsafeCell::raw_get](https://github.com/rust-lang/rust/issues/66358)

### New RFCs

* [RFC: let-expression](https://github.com/rust-lang/rfcs/pull/3159)

## Upcoming Events

### Online

* [August 10, 2021, Dallas, TX, US - Second Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/vqtjcsycclbnb/)
* [August 18, 2021, Denver, CO, US - Level up our Rust skills by building an ECS by Brooks Patton - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/278909353/)
* [August 18, 2021, Vancouver, BC, CA - Solving LeetCode Problems with Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsycclbxb/)
* [August 19, 2021, Manchester, UK - Rust Manchester - Speeding Up the Snake: Extending Python with Rust](https://www.meetup.com/rust-manchester/events/279730616/)

### North America

* [August 11, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsycclbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> **Rust** : You can't move your object and try to keep it, too.  
> **Me** : Ok, I suppose I can clone it?  
> **Rust** : Then implement a clone method.  
> **Me** : Why am I getting a stack overflow?  
> **Rust** : It is never a good idea for a clone method to call itself.  
> **Me** : I just wanted to simplify the trivial cases.  
> **Rust** : It is still not a good idea for a clone method to call itself.  
> **Me** : I can't believe I have gotten myself into this.

– [Oliver Ruebenacker on rust-users](https://users.rust-lang.org/t/writing-my-first-multi-threaded-app-with-rust-be-like/63481)

Thanks to [MBartlett21](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1093) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
