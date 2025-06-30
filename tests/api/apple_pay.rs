use crate::helpers::get_paystack_client;

#[tokio::test]
async fn can_list_domains() {
    let client = get_paystack_client();

    let res = client
        .apple_pay
        .list_domains()
        .await
        .expect("unable to list domains");

    assert!(res.status);
    assert!(res
        .message
        .contains("Apple Pay registered domains retrieved"))
}

#[tokio::test]
async fn can_unregister_domain() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let domain_name = "example.com".to_string();
    let res = client
        .apple_pay
        .unregister_domain(domain_name)
        .await
        .expect("unable to unregister domain");

    // Assert
    assert!(res.status);
    assert!(res
        .message
        .contains("Domain successfully unregistered on Apple Pay"));
}

// TODO: need elevated permission for some tests on this route
#[tokio::test]
async fn can_register_domain() {}
