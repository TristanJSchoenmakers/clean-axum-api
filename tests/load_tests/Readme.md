# Loadtests

This directory contains loadtests for the api.
Loadtests are done with the HTTP load testing tool [Drill](https://github.com/fcsonline/drill).
This tool is one of the tools installed with `cargo xtask init`

To run a benchmark run the following command:

``` bash
drill -b {{loadtest_name}}.yml --help
```