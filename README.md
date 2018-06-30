# Getting started

**Requirements**:

- **[pipenv](https://pypi.org/project/pipenv/)** installed
- [rust and cargo installation](https://www.rust-lang.org/en-US/install.html)

**Running**:

> `make` is used for convinience, but you can see in the Makefile the straight forward commands available there.

Instructions for Python 2:

- `pipenv install --python 2 -r requirements.txt` creates the environment with the requirements installed
- `pipenv shell` to activate the environment
- `make compile` to build the **awesome** module and copy it to the root folder
- `make test` to see the `sum_as_string` function defined in Rust take action

Instructions for Python 3:
- `pipenv install --python 3 -r requirements.txt` creates the environment with the requirements installed
- `pipenv shell` to activate the environment
- Update the `[dependencies.cpython]` section in [awesomelib/Cargo.toml](awesomelib/Cargo.toml)
- `make compile` to build the **awesome** module and copy it to the root folder
- `make test` to see the `sum_as_string` Rust function working


_Example inspired by [rochacbruno/rust-python-example](https://github.com/rochacbruno/rust-python-example) and [dgrunwald/rust-cpython](https://github.com/dgrunwald/rust-cpython)._