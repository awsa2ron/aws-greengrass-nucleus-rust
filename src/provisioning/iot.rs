use aws_sdk_iot::{Client, Error, PKG_VERSION};
use std::fs;

// Get your IoT policy.
// snippet-start:[iot.rust.get-policy]
pub async fn get_policy(client: &Client, name: &str) -> Result<(), Error> {
    let resp = client.get_policy().policy_name(name).send().await?;

    println!(
        "  Name:  {}",
        resp.policy_name.as_deref().unwrap_or_default()
    );
    println!(
        "  ARN:   {}",
        resp.policy_arn.as_deref().unwrap_or_default()
    );
    println!(
        "  Document:   {}",
        resp.policy_document().as_deref().unwrap_or_default()
    );
    println!();

    Ok(())
}

// Create your IoT policy.
// snippet-start:[iot.rust.create-policy]
pub async fn create_policy(client: &Client, name: &str, doc: &str) -> Result<(), Error> {
    let resp = client
        .create_policy()
        .policy_name(name)
        .policy_document(doc)
        .send()
        .await?;

    println!(
        "  Name:  {}",
        resp.policy_name.as_deref().unwrap_or_default()
    );
    println!(
        "  ARN:   {}",
        resp.policy_arn.as_deref().unwrap_or_default()
    );
    println!(
        "  Document:   {}",
        resp.policy_document().as_deref().unwrap_or_default()
    );
    println!(
        "  Version Id:   {}",
        resp.policy_version_id().as_deref().unwrap_or_default()
    );
    println!();

    Ok(())
}

// Create your IoT keys and cert.
// snippet-start:[iot.rust.get-policy]
pub async fn create_keys_certificates(
    client: &Client,
    cert: &str,
    pub_key: &str,
    key: &str,
    active: bool,
) -> Result<(), Error> {
    let resp = client
        .create_keys_and_certificate()
        .set_as_active(active)
        .send()
        .await?;

    let cert_content = &resp.certificate_pem().unwrap_or_default();
    let keys_content = &resp.key_pair().unwrap();

    fs::write(cert, &cert_content).expect("Unable to write file");
    fs::write(pub_key, &keys_content.public_key().unwrap()).expect("Unable to write file");
    fs::write(key, &keys_content.private_key().unwrap()).expect("Unable to write file");

    println!("  certificate:  {}", &cert_content);
    println!(
        "  ARN:   {}",
        resp.certificate_arn().as_deref().unwrap_or_default()
    );
    println!("  key pair:   {:#?}", resp.key_pair().as_deref().unwrap());
    println!(
        "  Id:   {}",
        resp.certificate_id().as_deref().unwrap_or_default()
    );
    println!();

    Ok(())
}

// Create your IoT things.
// snippet-start:[iot.rust.create-things]
pub async fn create_thing(client: &Client, name: &str) -> Result<(), Error> {
    let thing = client.create_thing().thing_name(name).send().await?;

    println!("Things:");

    println!(
        "  Name:  {}",
        thing.thing_name.as_deref().unwrap_or_default()
    );
    println!("  Id:  {}", thing.thing_id.as_deref().unwrap_or_default());
    println!(
        "  ARN:   {}",
        thing.thing_arn.as_deref().unwrap_or_default()
    );
    println!();

    println!();

    Ok(())
}
