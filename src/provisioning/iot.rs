use aws_sdk_iot::{Client, Error, Region, PKG_VERSION};

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
