use anyhow::{Error, Result};
use rumqttc::{self, AsyncClient, Key, MqttOptions, QoS, Transport};

use std::time::Duration;
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
    requests(client, message, topic).await;
}

async fn requests<'a>(client: AsyncClient, message: Vec<u8>, topic: String) -> Result<(), Error> {
    client.subscribe(&topic, QoS::AtMostOnce).await.unwrap();

    task::spawn(async move {
        client
            .publish(topic, QoS::AtLeastOnce, false, message.as_slice())
            .await
            .unwrap();
        time::sleep(Duration::from_millis(100)).await;
    });
    time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
