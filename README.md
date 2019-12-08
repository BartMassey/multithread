# Generate Debug Symbols For Massive Speedup

OK, that's obviously insane. Yet here we are. See this
[Reddit thread](https://www.reddit.com/r/learnrust/comments/e7dikc/weird_behavior_on_release_mode/)
for discussion.

The [original README](README-orig.md) describes the original
usage and some genesis of this code. I've changed it quite a
bit.

As before you need to start by generating the data to be
benchmarked over. Go to the `gen/` subdirectory and say

    cargo run --release ../acgt

This will take a while and generate 3GB of data.

Now go back to the top and say

    sh try.sh

This will build the two binary versions of the counter
program (counts instances of `G` and `C` in the datafile)
as `debug` and `ndebug`, and run each of them in sequential
mode with timing information.

From there, have fun with it. My friend and I did.
