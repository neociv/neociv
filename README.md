# ![NEOCIV](https://raw.githubusercontent.com/neociv/neociv/master/logo.svg)

## Examples

To run the examples you'll need [cargo-make](https://sagiegurari.github.io/cargo-make/) - this is because there is a submodule for the current [neociv-contrib](https://github.com/neociv/neociv-contrib) content that is used and pointed to by changing the `NEOCIV_CONTENT_ROOT` environment variable.

## Tests, Benches, and Coverage

`cargo test`

Will quickly run all tests

`cargo make bench`

Will run benchmarks

`cargo make test-coverage`

Will run tests with coverage, reports are in *target/tarpaulin*
