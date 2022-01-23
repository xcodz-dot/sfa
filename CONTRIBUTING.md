# Contributing to SFA

To contribute to SFA you have to ensure the following:

* no breaking changes until next major release.
  You can introduce new thing instead without
  breaking compatibility.

* Make sure that the following commands pass
  without any errors.
  
  * `codespell`
  * `cargo fmt`
  * `cargo test`
  * `cargo clippy`

* Your code should be documentated with the
  following sections at minimum

  * `General Description` (Starting of the function)
  * `# Arguments` - Name of arguments and what is their purpose.
  * `# Errors` - What conditions can possibly raise errors.
  * `# Example` - At least one working example.