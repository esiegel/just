use super::*;

#[test]
fn allow_duplicate_recipes() {
  Test::new()
    .justfile(
      "
      b:
        echo foo
      b:
        echo bar

      set allow-duplicate-recipes
    ",
    )
    .stdout("bar\n")
    .stderr("echo bar\n")
    .run();
}

/*
Tests:
- Multi OS: Muliple Operating System, no generic
- Multi OS not top level chooses correctly
- Multi OS with generic: chooses specific OS (not sure how to have tests work for other os)
- Multi OS with generic: chooses generic if not running one of the OS (not sure how to test)
- Duplicate test: uses last and most specific recipe
 */

/*
Error Tests:
- invalid OS
 */

test! {
  name: unexpected_character,
  justfile: "!~",
  stderr: "
    error: Expected character `=`
      |
    1 | !~
      |  ^
  ",
  status: EXIT_FAILURE,
}

#[test]
fn argument_count_mismatch() {
  Test::new()
    .justfile("foo a b:")
    .args(&["foo"])
    .stderr(
      "
      error: Recipe `foo` got 0 arguments but takes 2
      usage:
          just foo a b
    ",
    )
    .status(EXIT_FAILURE)
    .run();
}
