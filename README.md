# glcheck
Simple tool for checking if a machine supports the GL features you want

The idea is that you compile this and give it to anybody who wants to see if they can run your program.
If glcheck exits with code 0, their OpenGL setup is fine.  If not, then it's not.

Just change the call to `require_gl!` in `main()` to specify the version, core/compat, robustness, and extensions you want.
