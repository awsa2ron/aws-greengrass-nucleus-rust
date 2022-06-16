use anyhow::{Error, Result};
use rumqttc::{self, AsyncClient, Key, MqttOptions, QoS, Transport};
use std::time::Duration;
use tokio::{task, time};

/**
 * Publish the payload using MQTT.
 *
 * @param chunkablePayload  The common object payload included in all the messages
 * @param variablePayloads  The variable objects in the payload to chunk
 */
// pub async publish(Chunkable<T> chunkablePayload, List<T> variablePayloads) {
pub async fn publish(client: AsyncClient) -> Result<(), Error> {
    // client
    //     .publish("hello/world", QoS::ExactlyOnce, false, vec![1; 10])
    //     .await
    //     .unwrap();

    requests(client, "hello/world",).await?;
    // int start = 0;
    // int payloadVariableInformationSize = SERIALIZER.writeValueAsBytes(variablePayloads).length;
    // int payloadCommonInformationSize = SERIALIZER.writeValueAsBytes(chunkablePayload).length;

    // MqttChunkingInformation chunkingInformation =
    //         getChunkingInformation(payloadVariableInformationSize, variablePayloads.size(),
    //                 payloadCommonInformationSize);

    // for (int chunkId = 0; chunkId < chunkingInformation.getNumberOfChunks(); chunkId++,
    //         start += chunkingInformation.getNumberOfComponentsPerPublish()) {
    //     chunkablePayload.setVariablePayload(variablePayloads.subList(start,
    //             start + chunkingInformation.getNumberOfComponentsPerPublish()));
    //     this.mqttClient.publish(PublishRequest.builder()
    //             .qos(QualityOfService.AT_LEAST_ONCE)
    //             .topic(this.updateTopic)
    //             .payload(SERIALIZER.writeValueAsBytes(chunkablePayload)).build())
    //             .exceptionally((t) -> {
    //                 logger.atWarn().log("MQTT publish failed", t);
    //                 return 0;
    //             });

    //     logger.atInfo().kv("topic", updateTopic).log("{}", chunkablePayload);
    // }
    // logger.atError().cause(e).kv("topic", updateTopic).log("Unable to publish data via topic.");

    Ok(())
}

async fn requests(client: AsyncClient, topic:&'static str) -> Result<(), Error>{
    client
        .subscribe(topic, QoS::AtMostOnce)
        .await
        .unwrap();

    task::spawn(async move {
        client
            .publish(topic, QoS::AtLeastOnce, false, vec![1; 10 as usize])
            .await
            .unwrap();
        time::sleep(Duration::from_millis(100)).await;
    });
    time::sleep(Duration::from_secs(1)).await;
    Ok(())
}
