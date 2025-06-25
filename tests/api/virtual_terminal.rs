use paystack::VirtualTerminalStatus;

use crate::helpers::get_paystack_client;

// TODO: to conduct the test, you need access to a paystack terminal which I do not have
#[tokio::test]
async fn can_list_virtual_terminals_in_integration() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client
        .virutal_terminal
        .list_virtual_terminals(VirtualTerminalStatus::Active, 10)
        .await
        .unwrap();

    // Assert
    assert!(res.status);
    assert!(res.message.contains("Virtual Terminals retrieved"))
}
