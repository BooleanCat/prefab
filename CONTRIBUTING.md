# Running tests

Tests are split into unit tests and integration tests. In order to run all
tests you can use:

```bash
make test-deps  # one time, to install lua dependencies
make test
```

Unit tests are written in rust and can be invoked with:

```bash
make test-unit
```

Unit tests may take longer to run the first time whilst Cargo pulls the rust
dependencies.

Integration tests are written in lua and will require lua to be installed in
order to run them. They can be run with:

```bash
make test-deps  # one time, to install lua dependencies
make test-integration
```
