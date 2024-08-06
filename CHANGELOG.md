## 0.2.3 (15/07/2024)

- A complete rewrite of the paystack-rs crate. The rewrite is necessary because in the current state, the crate is not
extendable and will be difficult to add support for sync and async code. The rewrite is to achieve the following objectives:
  - Make the code less complex (relatively speaking)
  - Improve the maintainability of the crate
  - Support both async and sync code
  - Use high level data construct when creating request body
  - Have extensive Rust type for every possible response from the API
  - Use advanced rust type and how a better understand of the Rust programming language (personal reason)

- The following changes have been implemented
  - Change the project file layout to improve separation of concerns. The new layout includes the following
    - http (to handle all http related functionalities)
    - models (holds all request and response models from the API)
    - macros (utility macro to simplify code repetition)

## 0.2.2 (29/11/2023)

- Added support for create customer API route