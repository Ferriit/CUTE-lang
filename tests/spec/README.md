# Specification fixtures

The `.cute` files under `valid/` should be accepted, and the files under
`invalid/` should be rejected, according to `spec/grammar.md`. Each invalid
file isolates one documented error where practical.

These are source fixtures only; the current executable does not yet include a
CUTE parser or a test harness to run them automatically.
