# Gridava

![CI](https://github.com/algodiva/gridava/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/github/algodiva/gridava/branch/main/graph/badge.svg?token=6N3RBEJ7AX)](https://codecov.io/github/algodiva/gridava)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Gridava is a Rust library designed for creating and managing tiled coordinate systems. It provides robust support for various tile shapes, including hexagons, along with utilities for coordinate algorithms, transformations, and tile collections. Gridava is ideal for games, simulations, and applications requiring spatial logic.

## Features

- **Tile Coordinate Systems**: Seamless handling of hexagonal grids with efficient algorithms.
- **Tile Transformations**: Support for translations, rotations, and scaling operations.
- **Flexible Tile Collections**: Manage and manipulate groups of tiles with ease.
- **Modular and Extensible**: Build upon a strong foundation to suit your specific use cases.
- **High Performance**: Leverages Rust's speed and safety for demanding applications.

## Getting Started

### Installation
Add Gridava to your `Cargo.toml`:

```toml
[dependencies]
gridava = "0.2.0"
```

Then run:

```bash
cargo build
```

## Documentation

Comprehensive documentation is available [here](https://docs.rs/gridava). Explore the API, examples, and advanced topics to make the most of Gridava.

## Contributing

We welcome contributions! Here's how you can help:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/new-feature`).
3. Commit your changes (`git commit -m 'Add a new feature'`).
4. Push to the branch (`git push origin feature/new-feature`).
5. Open a pull request.

Please review our [contributing guidelines](CONTRIBUTING.md) for more details.

## Roadmap

- Integrate visual debugging and documentation tools.
- Square feature parity
- Triangle feature parity
- Documentation diagrams

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
