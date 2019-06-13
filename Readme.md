# rust_win32_sample

An exercise in directly exercising `winapi-rs` for fun and profit.

## Why

* "0" dependencies (well, minus `winapi-rs` of course)
* I'll want to do interop with existing C++ rendering pipelines someday for middleware
* `bgfx-rs` shows that even just maintaining C++ builds is as painful as always, why not try a pure 100% Rust stack?

## Takeaways

1)  Using `winapi-rs`'s COM interop is nicer than expected (mostly thanks to powerful macro codegen).
2)  Dealing with `winapi-rs` 's explosion of modules is an exercise in annoyance.  Fortunately this is something of a
    one-time cost, since I can simply collapse them back into a single module: [src\win32.rs](src/win32.rs)
3)  RLS intellisense doesn't work on COM interfaces.  Probably because of all the macros.  Boo!  I've been keeping a
    copy of `winapi-rs` open in another VS Code instance and `Ctrl+Shift+F`ing through the source instead.
4)  Default rust panic/assert behavior is lame, doesn't even breakpoint on the correct source line.
    Especially annoying for multithreaded work (`bgfx-rs`), where panics noop until rethrown from `thread.join()`.
    Easy to write your own `expect!(...)` macros, although trailing `.unwrap!()` might not be possible.
5)  `winapi-rs` COM interfaces practice interior mutability (all methods are implemented vs `&self`).
6)  Hygenic macros are hygenic.  Specifically, a macro can apparently return `[a, b, c]` (single AST node?) but not
    `a, b, c`.  Error messages for early versions of `input_layout!` were quite opaque before I realized this.
7)  Lack of `StaticMutex` or `lazy_static!` in stable stdlib gets annoying fast for shared internal resources like win32
    `WNDCLASS` registration, but can be worked around with manual `AtomicUsize` spinlocks and the like.
8)  Directly using `winapi-rs` is mostly an exercise in wrapping everything in `unsafe { ... }`.
