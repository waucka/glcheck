# glchecklib

The only thing you really need to worry about in `glchecklib` is the `require_gl!` macro.
This macro check that a GL context with the specified version, profile, robustness, and extensions can be created.

For convenience (especially in `glcheckapp` and similar), `glchecklib` re-exports `GlProfile` and `Robustness` from `glutin`.

# glcheckapp

This is a simple app that uses `glchecklib` to check GL features.
The idea is that you modify this app to check for the features your
program needs.  You then compile it and distribute that compiled
binary so that people can use it to check if they can run your program
without needing to actually have your program.  After all, it's much
faster to download a small executable than a 100MB+ demo.

# Licensing

`glchecklib` is LGPLv3+.

`glcheckapp` is GPLv3+.  Why not LGPLv3+?  Because.
