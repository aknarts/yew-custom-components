# yew-custom-components

`yew-custom-components` is a library of reusable components for the Yew framework, designed to simplify the creation of web applications in Rust. This library provides a variety of components that can be easily integrated into your Yew projects to enhance functionality and user experience.

## TABLE OF CONTENTS

-   [Features](#features)
-   [Installation](#installation)
    -   [Setting up a Yew Project](#setting-up-a-yew-project)
-   [Usage Examples](#usage-examples)
    -   [Table Component Example](#table-component-example)
-   [Contributing](#contributing)
-   [License](#license)

## FEATURES

-   **Dynamic Data Display**: Easily display of tabular data with the `Table` component.
-   **Pagination Support**: Manage large datasets efficiently with built-in pagination.
-   **Easy Integration**: Designed to be easily integrated into any Yew project with minimal setup.

## INSTALLATION

### Setting up a Yew Project

If you are new to Yew, you can set up a new Yew project by following the official guide: https://crates.io/crates/yew.

### Integrating Yew Custom Components

To use `yew-custom-components` in your project, add the following to your `Cargo.toml`:

```toml
    [dependencies]
    yew  = "0.18"
    yew-custom-components = { version = "0.2.2", features = ["table", "tabs", "pagination"] }
    serde = "1.0.203"
    serde-value = "0.7.0"
    serde_json = "1.0.117"
```

## USAGE EXAMPLE

For comprehensive, easy-to-follow usage examples, checkout: https://github.com/iniadeniyi/yew-custom-components/tree/main/examples/overview

## CONTRIBUTING

Contributions are welcome! To get started, please follow these steps:

    1. Fork the repository.
    2. Clone your fork and set up the development environment.
    3. Create a new branch for your feature or bug fix.
    4. Commit your changes and push the branch to your fork.
    5. Open a pull request with a description of your changes.
    6. Please open an issue or submit a pull request with any improvements or bug fixes.

## LICENSE

This project is licensed under the MIT License.
