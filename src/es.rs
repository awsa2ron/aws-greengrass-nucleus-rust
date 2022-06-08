use anyhow::Result;
use std::fs;
use std::path::Path;
use tracing::{debug, event, info, span, Level};

const GG_TOKEN_EXCHANGE_ROLE_ACCESS_POLICY_SUFFIX: &str = "Access";
const GG_TOKEN_EXCHANGE_ROLE_ACCESS_POLICY_DOCUMENT: &str = r#"{
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Action": [
                    "logs:CreateLogGroup",
                    "logs:CreateLogStream",
                    "logs:PutLogEvents",
                    "logs:DescribeLogStreams",
                    "s3:GetBucketLocation"
                ],
                "Resource": "*"
            }
        ]
    }"#;
const ROOT_CA_URL: &str = "https://www.amazontrust.com/repository/AmazonRootCA1.pem";
const IOT_ROLE_POLICY_NAME_PREFIX: &str = "GreengrassTESCertificatePolicy";
const GREENGRASS_CLI_COMPONENT_NAME: &str = "aws.greengrass.Cli";
const INITIAL_DEPLOYMENT_NAME_FORMAT: &str = "Deployment for %s";
const IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::%s:policy/%s";
const MANAGED_IAM_POLICY_ARN_FORMAT: &str = "arn:%s:iam::aws:policy/%s";

const E2E_TESTS_POLICY_NAME_PREFIX: &str = "E2ETestsIotPolicy";
const E2E_TESTS_THING_NAME_PREFIX: &str = "E2ETestsIotThing";

// private final Map<EnvironmentStage, String> tesServiceEndpoints = ImmutableMap.of(
//         EnvironmentStage.PROD, "credentials.iot.amazonaws.com",
//         EnvironmentStage.GAMMA, "credentials.iot.test.amazonaws.com",
//         EnvironmentStage.BETA, "credentials.iot.test.amazonaws.com"
// );

/*
 * Download root CA to a local file.
 *
 * To support HTTPS proxies and other custom truststore configurations, append to the file if it exists.
 */
pub async fn downloadRootCAToFile(path: &Path) {
    if Path::new(path).exists() {
        info!("Root CA file found at . Contents will be preserved.%n");
    }
    info!("Downloading Root CA from {}", ROOT_CA_URL);

    // TODO: append

    let body = reqwest::get("https://www.amazontrust.com/repository/AmazonRootCA1.pem")
        .await
        .unwrap()
        .text()
        .await;

    debug!("body = {:?}", &body);
    fs::write(path, body.unwrap()).expect("Unable to write file");

    // downloadFileFromURL(ROOT_CA_URL, path);
    // removeDuplicateCertificates(f);
    // Do not block as the root CA file may have been manually provisioned
    info!("Failed to download Root CA - %s%n");
}

fn downloadFileFromURL(url: &str, path: &Path) {
    // let body = reqwest::get(url)
    // .await
    // .unwrap()
    // .text()
    // .await;

    // String certificates = new String(Files.readAllBytes(f.toPath()), StandardCharsets.UTF_8);
    // Set<String> uniqueCertificates =
    //         Arrays.stream(certificates.split(EncryptionUtils.CERTIFICATE_PEM_HEADER))
    //                 .map(s -> s.trim())
    //                 .collect(Collectors.toSet());

    // try (BufferedWriter bw = Files.newBufferedWriter(f.toPath(), StandardCharsets.UTF_8)) {
    //     for (String certificate : uniqueCertificates) {
    //         if (certificate.length() > 0) {
    //             bw.write(EncryptionUtils.CERTIFICATE_PEM_HEADER);
    //             bw.write("");
    //             bw.write(certificate);
    //             bw.write("");
    //         }
    //     }
    // }
    info!("Failed to remove duplicate certificates - %s%n");
}
// fn removeDuplicateCertificates() {
//     SdkHttpFullRequest request = SdkHttpFullRequest.builder()
//                 .uri(URI.create(url))
//                 .method(SdkHttpMethod.GET)
//                 .build();

//         HttpExecuteRequest executeRequest = HttpExecuteRequest.builder()
//                 .request(request)
//                 .build();

//         try (SdkHttpClient client = getSdkHttpClient()) {
//             HttpExecuteResponse executeResponse = client.prepareRequest(executeRequest).call();

//             int responseCode = executeResponse.httpResponse().statusCode();
//             if (responseCode != HttpURLConnection.HTTP_OK) {
//                 throw new IOException("Received invalid response code: " + responseCode);
//             }

//             try (InputStream inputStream = executeResponse.responseBody().get();
//                  OutputStream outputStream = Files.newOutputStream(f.toPath(), StandardOpenOption.CREATE,
//                          StandardOpenOption.APPEND)) {
//                 IoUtils.copy(inputStream, outputStream);
//             }
//         }}
