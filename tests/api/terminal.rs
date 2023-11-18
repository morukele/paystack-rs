//! Testing this route requires a physical or virtual terminal.
//! With this in mind, a vague test will be done to ensure that at the very least
//! only valid data is sent to the API and the API rejects invalid data as appropriate

use crate::helpers::get_paystack_client;
use paystack::{ActionType, EventData, EventType, SendEventBodyBuilder};

#[tokio::test]
async fn send_terminal_event_passes_with_correct_payload() {
    // Arrange
    let client = get_paystack_client();
    let body = SendEventBodyBuilder::default()
        .event_type(EventType::Invoice)
        .action(ActionType::Process)
        .data(EventData {
            id: "7895939".to_string(),
            reference: None,
        })
        .build()
        .unwrap();

    // Act
    let res = client.terminal.send_event("12345", body).await;

    // Assert
    // NOTE: the test will fail but it is because we do not have a terminal
    if let Err(err) = res {
        assert_eq!(err.to_string(), "Request failed - Status Code: 404 Not Found Body: {\"status\":false,\"message\":\"Device does not exist\"}")
    } else {
        panic!();
    };
}
