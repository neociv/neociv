# ![NEOCIV](https://raw.githubusercontent.com/neociv/neociv/master/logo.svg)

## Prerequisites

All tasks are run via [cargo-make](https://sagiegurari.github.io/cargo-make/) as there are several custom steps that cannot be done using just cargo. This ensures that all tasks are performed in the correct order.

`cargo install --force cargo-make`

Before building it will be necessary to confirm you have all the required dependencies for your system to build [neociv-launcher](./packages/neociv-launcher).

## Building

Once all dependencies are satisfied you can run the following:

`cargo make`

This will install all cargo & npm dependencies, build each package, and run the tests.

## Examples

To run the examples you'll need [cargo-make](https://sagiegurari.github.io/cargo-make/) - this is because there is a submodule for the current [neociv-contrib](https://github.com/neociv/neociv-contrib) content that is used and pointed to by changing the `NEOCIV_CONTENT_ROOT` environment variable.

## Tests, Benches, and Coverage

`cargo make test`

Will quickly run all tests

`cargo make bench`

Will run benchmarks

`cargo make coverage`

Will run tests with coverage reporting. This uses [tarpaulin](https://github.com/xd009642/tarpaulin) by default to generate both json and html reports.
