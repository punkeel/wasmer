;; This file was generated by https://github.com/wasmerio/wasi-tests

(wasi_test "poll_oneoff.wasm"
  (map_dirs "hamlet:test_fs/hamlet")
  (temp_dirs "temp")
  (assert_return (i64.const 101))
  (assert_stderr "thread 'main' panicked at 'Could not open file: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }', /Users/fs/Development/wasmer/tests/wasi-wast/wasi/tests/poll_oneoff.rs:155:14\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n")
)
