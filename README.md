# ms_teams_webhook

This is a Rust crate that can be used for sending messages to Microsoft Teams via webhooks.

## Installation

To use this crate, add it to your Cargo.toml file as a dependency. Then, add the appropriate extern crate declaration to the top of your Rust source file.
Usage

To send a simple message to a Microsoft Teams channel via a webhook, use the send_message function. To send an adaptive card to a Microsoft Teams channel via a webhook, use the send_adaptive_card function.

Note that both of these functions return a Result object, which you should handle appropriately in your code. If the request to the webhook succeeds, the result will be Ok, and if it fails, the result will be an Err containing an error object.
License

This crate is licensed under the MIT License. See the LICENSE file for more details.
