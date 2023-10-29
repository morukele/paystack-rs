# Contributing to Paystack-rs

Welcome to Paystack-rs! We're thrilled that you're interested in contributing to our open-source crate. By participating, you can help us improve and grow the project together. Please take a moment to review this document to ensure a smooth and productive collaboration.

## Code of Conduct

Before you start contributing, please read and adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). We are committed to providing a safe and respectful environment for all contributors.

## Getting Started

1. **Fork** the repository to your GitHub account.
2. **Clone** your forked repository to your local machine:

   ```bash
   git clone https://github.com/your-username/paystack-rs.git
   ```

3. Create a new **branch** for your feature or bug fix:

    ```bash
    git checkout -b my-new-feature
    ```

4. Make your changes, test them, and ensure that your Rust code is well-documented.

5. **Test your code**: Before committing your changes, make sure to test your Rust code thoroughly. We provide a testing environment that requires a .env file for configuration. You can create this file based on the format found in the .env.example file. Make sure to provide any required environment variables.

   ```bash
   cp .env.example .env
   # Edit the .env file with your configuration
   ```

6. **Build and test the code**
   - Build the crate

   ```bash
      cargo build
   ```

   - Test the crate

   ```bash
      cargo test
   ```

7. **Commit** your changes with clear and concise messages

   ```bash
   git commit -m "Add feature: your feature name"
   ```

8. **Push** your changes to your fork

   ```bash
   git push origin my-new-feature
   ```

9. Create a **Pull Request (PR)** from your fork to the main project's repository. Make sure to describe your changes and reference any related issues.

10. After creating the PR, the maintainers will review your changes, provide feedback, and eventually merge the PR.

## Help and Support

If you have any questions or need assistance, feel free to reach out to the maintainers or the community on our Discussion page.

We look forward to your contributions and thank you for your interest in Paystack-rs!
