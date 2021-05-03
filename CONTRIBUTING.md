# Contributing to Basiliq

Basiliq welcomes contribution from everyone in the form of suggestions, bug reports, pull requests, and feedback.
This document gives some guidance if you are thinking of helping us.

## Submitting bug reports and feature requests

Basiliq development is spread across multiple repositories.

The main repository (basiliqio/basiliq) is a safe choice for opening any issues related to Basiliq.

When reporting a bug or asking for help, please include enough details so that
the people helping you can reproduce the behavior you are seeing. For some tips
on how to approach this, read about how to produce a [Minimal, Complete, and
Verifiable example].

[Minimal, Complete, and Verifiable example]: https://stackoverflow.com/help/mcve

When making a feature request, please make it clear what problem you intend to
solve with the feature, any ideas for how Basiliq could support solving that
problem, any possible alternatives, and any disadvantages.

## Running the test suite

We encourage you to check that the test suite passes locally before submitting a pull request with your changes.
If anything does not pass, typically it will be easier to iterate and fix it locally than waiting for the CI servers to run tests for you.

Beware that some repository requires you to have a local,
test database running and the environment variable `DATABASE_URL` for the tests to work properly.

## Conduct

See the [Code of Conduct](https://github.com/basiliqio/basiliq/blob/main/CODE_OF_CONDUCT.md) document at the root of the repository.
