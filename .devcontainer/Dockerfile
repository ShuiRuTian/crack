# Use the [Choice] comment to indicate option arguments that should appear in VS Code UX. Use a comma separated list.
FROM rustlang/rust:nightly-bookworm
RUN export DEBIAN_FRONTEND=noninteractive \
  && rustup component add clippy rustfmt
# && apt-get update // not update, to make default system the same
