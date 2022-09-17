use crate::config;
use anyhow::{Error, Ok, Result};
use rumqttc::{self, AsyncClient, EventLoop, Key, MqttOptions, QoS, Transport};

use std::{fs, path::Path, time::Duration};
use tokio::{task, time};

pub struct PublishRequest {
    topic: String,
    qos: QoS,
    /**
     * Retain the message in the cloud MQTT broker (only last message with retain is actually kept).
     * Subscribers will immediately receive the last retained message when they first subscribe.
     */
    retain: bool,
    // byte[] payload;

    // @Builder
    // protected PublishRequest(String topic, QualityOfService qos, boolean retain, byte[] payload) {
    //     // Intern the string to deduplicate topic strings in memory
    //     this.topic = topic.intern();
    //     if (qos == null) {
    //         qos = QualityOfService.AT_LEAST_ONCE;
    //     }
    //     this.qos = qos;
    //     this.retain = retain;
    //     this.payload = payload;
    // }
}

/**
 * Publish to a MQTT topic.
 *
 * @param request publish request
 */
pub async fn publish<'a>(
    client: AsyncClient,
    message: Vec<u8>,
    topic: String,
    qos: QoS,
    retain: bool,
) {
    // return connect().thenCompose((b) -> {
    //     // Take the tokens from the limiters' token buckets.
    //     // This is guaranteed to not block because we've already slept the required time
    //     // in the spooler thread before calling this method.
    //     transactionLimiter.acquire();
    //     bandwidthLimiter.acquire(message.getPayload().length);
    //     synchronized (this) {
    //         throwIfNoConnection();
    //         logger.atTrace().kv(TOPIC_KEY, message.getTopic()).kv(QOS_KEY, qos.name()).kv("retain", retain)
    //                 .log("Publishing message");
    //         return connection.publish(message, qos, retain);
    //     }
    // });
    // requests(client, message, topic).await;
}

pub fn init(name: &str) -> Result<(AsyncClient, EventLoop), Error> {
    let endpoint = config::Config::global()
        .services
        .kernel
        .configuration
        .iot_data_endpoint
        .as_str();
    // info!("Endpoint: {}", endpoint);

    let root_dir = Path::new(".");
    let ca_file_path = root_dir.join("rootCA.pem");
    let priv_key_file_path = root_dir.join("privKey.key");
    let cert_file_path = root_dir.join("thingCert.crt");
    // info!("{:?}", endpoint);

    let mut mqtt_options = MqttOptions::new(name, endpoint, 8883);
    mqtt_options
        .set_keep_alive(Duration::from_secs(30))
        .set_transport(Transport::tls(
            fs::read(ca_file_path)?,
            Some((
                fs::read(cert_file_path)?,
                Key::RSA(fs::read(priv_key_file_path)?),
            )),
            None,
        ));
    Ok(AsyncClient::new(mqtt_options, 10))
}
