What it is
==========

A hack to use the build script to simulate attribute macros where global state is required.

With normal proc macros, you can't really rely on them being expanded in order or even being expanded at all (if incremental compilation is enabled). So it doesn't cover the use case of collecting items that are somehow "registered" (think of a custom test harness, or a web framework with routing functions, etc).

This hack provides a function to call in the build script, which uses `syn` to parse your entire crate and find the registered functions. It then generates a function that calls all those registered functions, as an example of what you could do. A tiny proc macro crate cooperates to swallow the attributes themselves and import the generated code.

This code is bad and you shouldn't use it!

