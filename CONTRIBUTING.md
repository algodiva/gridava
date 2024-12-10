# Contributing to GridAva

Thank you for considering contributing to GridAva! Contributions are what make open-source projects a thriving and collaborative environment, and we appreciate your interest in improving this project. Here’s a guide to help you get started.

---

## Code of Conduct

By participating in this project, you agree to uphold our [Code of Conduct](./CODE_OF_CONDUCT.md). Please ensure all interactions are respectful and constructive.

---

## Ways to Contribute

There are many ways you can contribute to GridAva:

1. **Reporting Issues**
   - Found a bug? Have a suggestion? Create an issue [here](https://github.com/algodiva/gridava/issues).
   - Ensure your report includes:
     - A clear and descriptive title.
     - Steps to reproduce (if reporting a bug).
     - Expected and actual behavior.
     - Any relevant logs or screenshots.

2. **Improving Documentation**
   - Enhancing the readability and completeness of the documentation is always welcome.
   - See an area for improvement? Submit a pull request!

3. **Proposing New Features**
   - If you have an idea for a feature, open a discussion or issue to gauge community interest.

4. **Contributing Code**
   - Fix bugs, add new features, or improve existing ones.
   - Follow the steps below to contribute code.

---

## How to Contribute

### Prerequisites
Ensure you have the following tools installedand read the contribution guide:
- [Rust](https://www.rust-lang.org/) (latest stable version recommended)
- [Git](https://git-scm.com/)
- [Contribution guide](https://docs.github.com/en/get-started/exploring-projects-on-github/contributing-to-a-project)

### Making Changes
- Follow the project’s coding style and conventions.
    - Use the provided [rustfmt.toml](.rustfmt.toml) file and `cargo fmt` command.
- Document any significant code changes or additions.
    - All code should have documentation such that `cargo rustc -- -D missing_docs` command passes.
- Write or update tests for your changes where applicable.
    - The project aims for maximum test coverage possible, so ensure that all code has unit tests and integration tests where possible. Ensure that adequate test cases are supplied as well.

---

## Pull Request Guidelines
- Ensure all tests pass before submitting your pull request.
- Write clear, concise commit messages.
- Link any related issues in the pull request description.
- Be prepared to make revisions based on reviewer feedback.

You can check our [CI/CD](.github/workflows/rust.yml) for what is checked during a pull request.
In summary the follow commands must pass:
- `cargo check`
- `cargo clippy -- -D warnings -D missing_docs`
- `cargo test --no-fail-test --all-features`

---

## Style Guide
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Use `cargo fmt` to format your code.
- Use `cargo clippy` to check for common mistakes and improve code quality.
---

## Need Help?
If you need help or have questions, feel free to:
- Open a discussion [here](https://github.com/algodiva/gridava/discussions).
- Reach out to the project maintainers.

We’re excited to see your contributions! 🚀
