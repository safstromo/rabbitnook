use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Esp32Relay() -> impl IntoView {
    let create_bash = r#"
cargo generate esp-rs/esp-idf-template cargo
    "#;
    let run_bash = r#"
cargo run --release
    "#;
    let policy_json_all = r#"
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": "iot:*",
      "Resource": "*"
    }
  ]
}
"#;
    let policy_json = r#"
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": "iot:Connect",
      "Resource": "arn:aws:iot:your_endpoint:client/${iot:Connection.Thing.ThingName}"
    },
    {
      "Effect": "Allow",
      "Action": "iot:Publish",
      "Resource": "arn:aws:iot:your_endpoint:topic/esp32/pub"
    },
    {
      "Effect": "Allow",
      "Action": "iot:Subscribe",
      "Resource": "arn:aws:iot:your_endpoint:topicfilter/esp32/sub"
    },
    {
      "Effect": "Allow",
      "Action": "iot:Receive",
      "Resource": "arn:aws:iot:your_endpoint:topic/esp32/sub"
    }
  ]
}
    "#;

    let structs_rs = r#"
use std::{mem, slice};

use esp_idf_svc::tls::X509;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MqttMessage {
    pub message: String,
}

pub struct Config<'a> {
    pub ssid: String,
    pub password: String,
    pub client_id: String,
    pub server_cert: X509<'a>,
    pub client_cert: X509<'a>,
    pub private_key: X509<'a>,
    pub mqtts_url: String,
    pub sub_topic: String,
    pub pub_topic: String,
}

impl Config<'_> {
    pub fn new() -> Self {
        let server_cert_bytes: Vec<u8> = include_bytes!("../aws/AmazonRootCA1.pem").to_vec();
        let client_cert_bytes: Vec<u8> = include_bytes!("../aws/device.crt").to_vec();
        let private_key_bytes: Vec<u8> = include_bytes!("../aws/private.key").to_vec();

        let server_cert: X509 = convert_certificate(server_cert_bytes);
        let client_cert: X509 = convert_certificate(client_cert_bytes);
        let private_key: X509 = convert_certificate(private_key_bytes);

        Config {
            ssid: dotenv!("WIFI_SSID").into(),
            password: dotenv!("WIFI_PASSWORD").into(),
            client_id: dotenv!("CLIENT_ID").into(),
            server_cert,
            client_cert,
            private_key,
            mqtts_url: dotenv!("MQTTS_URL").into(),
            sub_topic: dotenv!("SUB_TOPIC").into(),
            pub_topic: dotenv!("PUB_TOPIC").into(),
        }
    }
}

fn convert_certificate(mut certificate_bytes: Vec<u8>) -> X509<'static> {
    // append NUL
    certificate_bytes.push(0);

    // convert the certificate
    let certificate_slice: &[u8] = unsafe {
        let ptr: *const u8 = certificate_bytes.as_ptr();
        let len: usize = certificate_bytes.len();
        mem::forget(certificate_bytes);

        slice::from_raw_parts(ptr, len)
    };
    // return the certificate file in the correct format
    X509::pem_until_nul(certificate_slice)
}
    "#;

    let env_file = r#"
WIFI_SSID=your_wifi_ssid
WIFI_PASSWORD=your_wifi_password
CLIENT_ID=your_client_id(esp32, the name of the thing in AWS IoT)
MQTTS_URL=your_mqtt_url("mqtts://your_endpoint.com")
SUB_TOPIC=esp32/sub
PUB_TOPIC=esp32/pub
    "#;

    let wifi_rs = r#"
use anyhow::{bail, Result};
use esp_idf_hal::{delay::FreeRtos, peripheral};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    mqtt::client::{EspMqttClient, QoS},
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};
use esp_idf_sys::EspError;
use log::info;

use crate::structs::Config;

pub fn wifi(
    ssid: &str,
    pass: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_svc::hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> Result<Box<EspWifi<'static>>> {
    let nvs = EspDefaultNvsPartition::take()?;

    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        bail!("Missing WiFi name")
    }
    if pass.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;

    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    info!("Starting wifi...");

    wifi.start()?;

    info!("Scanning...");

    let ap_infos = wifi.scan()?;

    let access_point = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(access_point) = access_point {
        info!(
            "Found configured access point with SSID:{} on channel {}",
            ssid, access_point.channel
        );
        Some(access_point.channel)
    } else {
        info!(
            "Configured access point with SSID:{} not found during scanning, will go with unknown channel",
            ssid
        );
        None
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().expect("Was not able to convert ssid"),
        password: pass.try_into().expect("Was not able to convert password"),
        channel,
        auth_method,
        ..Default::default()
    }))?;

    info!("Connecting wifi...");

    wifi.connect()?;

    info!("Waiting for DHCP lease...");

    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(Box::new(esp_wifi))
}

pub fn try_reconnect_wifi(
    wifi: &mut Box<EspWifi<'static>>,
    mqtt_client: &mut EspMqttClient<'static>,
    config: &Config,
) -> Result<(), EspError> {
    info!("Wifi disconnected");

    while !wifi.is_connected().unwrap() {
        info!("Reconnecting...");
        if wifi.as_mut().connect().is_err() {
            info!("No access point found, Sleeping for 10sec",);
            FreeRtos::delay_ms(10000);
        }
    }

    // Sleep to let mqtt client reconnect
    FreeRtos::delay_ms(10000);
    info!("Resubscribing to topic...");
    mqtt_client.subscribe(&config.sub_topic, QoS::AtLeastOnce)?;
    Ok(())
}
    "#;

    let main_rs = r#"
mod structs;
mod wifi;

use std::result::Result::Ok;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Result;

#[macro_use]
extern crate dotenv_codegen;
use embedded_svc::mqtt::client::QoS;
use esp_idf_hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_svc::mqtt::client::EventPayload;
use esp_idf_svc::mqtt::client::MqttClientConfiguration;
use log::error;
use log::info;
use rgb::RGB8;
use structs::Config;
use structs::MqttMessage;
use wifi::try_reconnect_wifi;
use wifi::wifi;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

const GREEN: RGB8 = rgb::RGB8::new(0, 128, 0);
const RED: RGB8 = rgb::RGB8::new(128, 0, 0);

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. 
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;

    //Config IO
    let mut button = PinDriver::input(peripherals.pins.gpio19)?;
    button.set_pull(esp_idf_hal::gpio::Pull::Up)?;

    // Mutex to be able to share pointers
    let relay = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio10)?));
    relay
        .lock()
        .expect("Unable to lock pin mutex")
        .set_level(esp_idf_hal::gpio::Level::Low)?;
    let led_pin = peripherals.pins.gpio8;

    // Clone to create a reference for mqtt
    let relay_clone = Arc::clone(&relay);

    let channel = peripherals.rmt.channel0;
    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin)?;

    let pixels_red = std::iter::repeat(RED).take(25);
    ws2812.write_nocopy(pixels_red)?;

    let config = Config::new();

    let mut wifi = wifi(&config.ssid, &config.password, peripherals.modem, sysloop)?;

    //MQTT
    // Set up handle for MQTT Config
    let mqtt_config = MqttClientConfiguration {
        client_id: Some(&config.client_id),
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        server_certificate: Some(config.server_cert),
        client_certificate: Some(config.client_cert),
        private_key: Some(config.private_key),
        ..Default::default()
    };

    // Create Client Instance and Define Behaviour on Event
    info!("Creating mqtt client");
    let mut client =
        EspMqttClient::new_cb(&config.mqtts_url, &mqtt_config, move |message_event| {
            match message_event.payload() {
                EventPayload::Connected(_) => info!("Connected"),
                EventPayload::Subscribed(id) => info!("Subscribed to id: {}", id),
                EventPayload::Received { data, .. } => {
                    if !data.is_empty() {
                        let mqtt_message: Result<MqttMessage, serde_json::Error> =
                            serde_json::from_slice(data);

                        match mqtt_message {
                            Ok(message) => {
                                info!("Recieved {:?}", message);

                                if message.message == "Hello from AWS IoT console" {
                                    info!("Activating relay from MQTT message");
                                    let mut relay =
                                        relay_clone.lock().expect("Unable to lock relay mutex");
                                    relay.set_high().expect("Unable to set relay to high");
                                    FreeRtos::delay_ms(5000);
                                    relay.set_low().expect("Unable to set relay to low");
                                }
                            }
                            Err(err) => error!(
                                "Could not parse message: {:?}. Err: {}",
                                std::str::from_utf8(data).unwrap(),
                                err
                            ),
                        }
                    }
                }
                _ => info!("{:?}", message_event.payload()),
            };
        })?;

    // Subscribe to MQTT Topic
    info!("Subscribing to topic");
    client.subscribe(&config.sub_topic, QoS::AtLeastOnce)?;

    info!("Starting main loop");

    let activated_message = MqttMessage {
        message: "Relay activated".into(),
    };

    let activated_json = serde_json::to_string(&activated_message)?;

    loop {
        // we are using thread::sleep here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(10);

        let pixel_color = std::iter::repeat(GREEN).take(25);

        if !wifi.is_connected()? {
            let pixel_color = std::iter::repeat(RED).take(25);
            ws2812.write_nocopy(pixel_color)?;

            try_reconnect_wifi(&mut wifi, &mut client, &config)?;
        }

        ws2812.write_nocopy(pixel_color)?;

        if button.is_low() {
            info!("Button pressed, activating relay");
            let mut relay = relay.lock().expect("Unable to lock relay mutex");
            relay.set_high()?;
            FreeRtos::delay_ms(5000);
            relay.set_low()?;
            client.publish(
                &config.pub_topic,
                QoS::AtLeastOnce,
                false,
                activated_json.as_bytes(),
            )?;
        }
    }
}
    "#;

    view! {
        <div class="flex flex-col min-h-screen w-full max-w-5xl bg-base items-start justify-center">
            <h1 class="mt-8">Rust "❤️" Esp32 remote relay using AWS IoT</h1>
            <p class="my-2">June 20, 2024</p>
            <h2 class="my-6">In space no one can hear you scream</h2>
            <p>This is my first adventure into the world of embedded development. "🫠"</p>
            <p>
                This project is an experiment of how to control a relay using an ESP32 board and AWS IoT. The relay is connected to the ESP32 board and can be controlled using the AWS IoT with MQTT protocol.
            </p>
            <p>
                Im writing this blog post because i had a hard time finding examples and guides on the subject and to share my experience and to help others save time and avoid some of the pitfalls I encountered.
            </p>
            <p>
                Since is the first time I am writing embedded code and I am still learning, if you see any mistakes or have any suggestions please let me know.
                "😊"
            </p>

            <h2 class="my-6">Hardware</h2>
            <a
                class="hover:text-maroon underline"
                target="_blank"
                href="https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html#esp32-c3-devkitm-1"
            >
                ESP32-C3-DevKitM-1 board
            </a>
            <a
                class="hover:text-maroon underline"
                target="_blank"
                href="https://www.amazon.se/-/en/AZDelivery-KY-019-Module-compatible-Raspberry-including/dp/B07CNR7K9B?pd_rd_w=ZHQja&content-id=amzn1.sym.7aba3564-a536-4264-adad-b89dcc42bc21&pf_rd_p=7aba3564-a536-4264-adad-b89dcc42bc21&pf_rd_r=GXJRQ8D3SX1XZ0RSP4K4&pd_rd_wg=aqNnx&pd_rd_r=20de12af-0b3f-4ea3-a515-02d848173ab6&pd_rd_i=B07CNR7K9B&ref_=pd_bap_d_grid_rp_0_1_ec_pd_nav_hcs_rp_2_t&th=1"
            >
                1-Relay 5V KY-019-Module
            </a>
            <a href="/blog/esp32-relay/hardware.jpg" target="_blank">
                <img
                    class="my-6 rounded-md"
                    src="/blog/esp32-relay/hardware.jpg"
                    alt="Image of hardware"
                />
            </a>
            <h2 class="my-6">AWS IoT setup</h2>
            <p>First thing we need to do is creating a Policy and create a Thing in AWS IoT.</p>

            <h3 class="my-6">Create a Policy</h3>
            <ol>
                <li>"1. Go to the AWS IoT console and click on `Security` in the left menu."</li>
                <li>"2. Click on `Policies` and then `Create a policy`" .</li>
                <li>3. Name your policy and go to JSON view.</li>
            </ol>

            <a href="/blog/esp32-relay/policy.png" target="_blank">
                <img
                    class="my-6 rounded-md"
                    src="/blog/esp32-relay/policy.png"
                    alt="Image of policy"
                />
            </a>
            <p>
                Here you define what the thing is allowed to do.
                This is where my first pitfall was.
                I had not defined the correct permissions in the policy and the ESP32 could not connect to the AWS IoT endpoint. I had a hard time figuring out what was wrong and the log message from the ESP32 was not very helpful and the logs in AWS IoT did not give me any clues either.
            </p>
            <p>This is the log from the ESP32:</p>
            <a href="/blog/esp32-relay/relay-log.png" target="_blank">
                <img
                    class="my-6 rounded-md"
                    src="/blog/esp32-relay/relay-log.png"
                    alt="Image of log"
                />
            </a>
            <p>
                After some trial and error I finally got it to work by using the following policy.
                This allows all actions on all resources. This is not recommended in a production environment but for now it will do. I recommend starting with this and make sure everything works before fine tuning the policy.
            </p>

            <div class="code-block">
                <pre>
                    <code class="json">{policy_json_all}</code>
                </pre>
            </div>
            <p>
                When I got everything working I went back and fine tuned the policy to only allow the actions I needed. This was the final result:
            </p>
            <div class="code-block">
                <pre>
                    <code class="json">{policy_json}</code>
                </pre>
            </div>

            <a href="/blog/esp32-relay/policy-final.png" target="_blank">
                <img
                    class="my-6 rounded-md"
                    src="/blog/esp32-relay/policy-final.png"
                    alt="Image of final policy"
                />
            </a>
            <p>
                "This will allow the ESP32 to connect, subscribe and receive messages on the topic
                `esp32/sub` and publish messages on the topic `esp32/pub`."
            </p>
            <h3 class="my-6">Create a Thing</h3>
            <ol>
                <li>
                    "1. Go to the AWS IoT console and click on `All devices` and then `Things`
                    in the left menu."
                </li>
                <li>
                    "2. Click on `Create things` in the right corner and then `Create a single thing`."
                </li>
                <li>
                    <a href="/blog/esp32-relay/create-thing.png" target="_blank">
                        <img
                            class="my-6 rounded-md"
                            src="/blog/esp32-relay/create-thing.png"
                            alt="Image of create1"
                        />
                    </a>
                </li>
                <li>
                    <a href="/blog/esp32-relay/create-thing2.png" target="_blank">
                        <img
                            class="my-6 rounded-md"
                            src="/blog/esp32-relay/create-thing2.png"
                            alt="Image of create2"
                        />
                    </a>
                </li>

                <li>"3. Name your thing and click `Next`."</li>
                <li>
                    <a href="/blog/esp32-relay/create-thing3.png" target="_blank">
                        <img
                            class="my-6 rounded-md"
                            src="/blog/esp32-relay/create-thing3.png"
                            alt="Image of create3"
                        />
                    </a>
                </li>
                <li>"4. Select `Auto-generate a new certificate` and click `Next`."</li>
                <li>
                    <a href="/blog/esp32-relay/create-thing4.png" target="_blank">
                        <img
                            class="my-6 rounded-md"
                            src="/blog/esp32-relay/create-thing4.png"
                            alt="Image of create4"
                        />
                    </a>
                </li>
                <li>
                    "5. Next up we need to assign the policy we created earlier to the thing. Select the policy and click
                    `Create thing`."
                </li>
                <li>
                    <a href="/blog/esp32-relay/create-thing5.png" target="_blank">
                        <img
                            class="my-6 rounded-md"
                            src="/blog/esp32-relay/create-thing5.png"
                            alt="Image of create5"
                        />
                    </a>
                </li>
                <li>
                    6. Now a popup will appear where we can download the certificates and keys. Download device certificate, private key and root CA 1 certificate. Dont forget to rename them so you know which is which.
                </li>
                <li>
                    <a href="/blog/esp32-relay/create-thing6.png" target="_blank">
                        <img
                            class="my-6 rounded-md"
                            src="/blog/esp32-relay/create-thing6.png"
                            alt="Image of create6"
                        />
                    </a>
                </li>
            </ol>
            <h2 class="my-6">Coding</h2>
            <p>
                Now we have everything set up in AWS IoT and we can start with the fun part, coding the ESP32. I will write this in Rust because I love Rust so why not
                "🤷"
            </p>
            <h3 class="my-6">Project setup</h3>
            <p>
                First thing we need to decide is if we will be using STD or no STD. I will be using std because it makes things easier and I am not too concerned about the size of the binary since the ESP32 has plenty of memory.
            </p>
            <p>
                "Im using the `esp-idf-template` as a base for this project. You can find it "
                <a
                    class="underline hover:text-maroon"
                    href="https://github.com/esp-rs/esp-idf-template"
                >
                    here.
                </a>
                Make sure you have installed all prerequisites for the template and embedded Rust development. Here is a great book to get you started with embedded Rust:
                <a class="underline hover:text-maroon" href="https://docs.esp-rs.org/std-training/">
                    ESP STD Embedded Training
                </a>
            </p>

            <div class="code-block">
                <code class="bash text-white">{create_bash}</code>
            </div>
            <p>
                This will create a new project with the name you specify. I will name mine
                "esp32-aws-iot-relay"
                . Follow the prompts and select the correct board and other settings.
                Make sure the project builds and runs on the ESP32 before continuing.
            </p>

            <div class="code-block">
                <code class="bash text-white">{run_bash}</code>
            </div>

            <h3 class="my-6">Structs</h3>
            <p>
                "structs.rs"
                will contain the structs we need for the MQTT messages and the configuration.
                Here is what i ended up with:
            </p>
            <div class="code-block">
                <pre>
                    <code class="rust">{structs_rs}</code>
                </pre>
            </div>
            <p>
                "I am using the `dotenv` crate to load the configuration from a `.env`
                file, witch i find very convenient during development. You can add the `.env`
                file in the root of the project."
            </p>

            <div class="code-block">
                <pre>
                    <code class="docker">{env_file}</code>
                </pre>
            </div>

            <p>
                "I created a Config struct that will hold the configuration for the project. The configuration is loaded from the
                `.env` file and the certificates are loaded from the `aws`
                folder in the project. The certificates are needed to establish an encrypted connection to the AWS IoT endpoint.
                Converting the certificates to the correct format was someting i struggled with. I found a solution that works but I am not sure if it is the best way to do it. If you know a better way please let me know. Here is a link to the stackoverflow thread where I found the solution: "
                <a
                    class="underline hover:text-maroon"
                    href="https://stackoverflow.com/questions/75299434/rust-on-esp32-how-to-send-and-receive-data-using-the-mqtt-protocol-to-aws-io"
                >
                    link
                </a>
            </p>

            <h3 class="my-6">Wifi</h3>
            <p>"wifi.rs" will contain the wifi setup and the reconnect function.</p>

            <div class="code-block">
                <pre>
                    <code class="rust">{wifi_rs}</code>
                </pre>
            </div>
            <p>
                "I choose the esp32 because it had great Rust support and libraries. I am using the
                `esp-idf-sys` crate to interact with the ESP32 and the `esp-idf-svc`
                crate for the wifi and mqtt setup. The `esp-idf-hal`
                crate is used for interacting with the GPIO and other peripherals. These are great crates for someone new to embedded development like me since they abstract away a lot of the complexity of embedded development."
            </p>
            <h3 class="my-6">Main</h3>
            <p>"main.rs" will contain the main logic of the project.</p>

            <div class="code-block">
                <pre>
                    <code class="rust">{main_rs}</code>
                </pre>
            </div>
            <p>
                "First, we set up the GPIO pins for the relay and the button (the two green wires). The relay is connected to GPIO 10, and the button is connected to GPIO 19. The button is configured with a pull-up resistor, so it will read high when not pressed and low when pressed. `High` means it is not pressed, and `low` means it is pressed. The relay is set to low to ensure it is not activated when the ESP32 starts."
            </p>
            <p>
                I had to use a Arc Mutex to be able to share the relay pin between the main loop and the mqtt callback. I then clone the arc to create a reference for the mqtt callback.
            </p>
            <p class="mb-4">
                "Next up im setting up the WS2812 LED. I decided to use the WS2812 LED to give some feedback on the wifi status of the ESP32. The LED will be green when everything is working and red when something is wrong. The LED is connected to GPIO 8 acording to the ESP32-C3-DevKitM-1 board schematic. For this i found a crate called
                `ws2812-esp32-rmt-driver` that makes it easy to control the WS2812 LED."
            </p>
            <p>
                "I then create a `Config` struct and load the configuration from the `.env`
                file. The certificates are loaded from the `aws` folder in the project."
                Then i used the wifi setup function shown earlier to connect to the wifi.
            </p>
            <p class="mb-4">
                "Next up is the MQTT setup. I create a `MqttClientConfiguration` and a
                `EspMqttClient` instance. The MqttClient::new function takes a callback that will be called when a message is received. In the callback i check if the message is the one i am looking for and then activate the relay if it is. I could not find any good guides and the doc.rs didnt have much info on how to use the `esp-idf-svc` crate so i looked at the examples on github and this is what i came up with after some trial and error."
            </p>
            <p>
                I then subscribe to the topic and start the main loop. In the main loop I check if the wifi is connected and if not i try to reconnect. I then check if the button is pressed(the two cables are connected) and if it is i activate the relay and publish a message to the topic.
            </p>
            <p>Now we can test using AWS MQTT test client.</p>

            <a href="/blog/esp32-relay/mqtt-test.png" target="_blank">
                <img
                    class="my-6 rounded-md"
                    src="/blog/esp32-relay/mqtt-test.png"
                    alt="Image of mqtt-test"
                />
            </a>
            <h3 class="my-6">Final project</h3>
            <p>Here is the final project:</p>
            <a
                class="hover:text-maroon underline mb-4"
                href="https://github.com/safstromo/esp32-aws-iot-relay"
            >
                esp32-aws-iot-relay
            </a>
            <p class="underline">Future improvments will be:</p>
            <ul class="mb-6">
                <li>Remote logging/saving log</li>
                <li>Setting the config remote, maybe bluetooth?</li>
                <li>OTA updates</li>
                <li>Adding a screen to show status</li>
                <li>Test</li>

            </ul>
            <p>
                I hope this blog post was helpful and it will save someone a bit of time and frustration.
            </p>
            <p class="mb-6">
                If you have suggestions or improvements feel free to create a PR, open an issue or contact me.
                "😉"
            </p>
            <p>Happy coding! "😊"</p>

            <h2 class="my-6">Useful links</h2>
            <ul>
                <li>
                    <a
                        class="hover:text-maroon underline"
                        href="https://docs.esp-rs.org/std-training/"
                    >
                        Espressif STD-Traing
                    </a>
                </li>
                <li>
                    <a
                        class="hover:text-maroon underline"
                        href="https://docs.rust-embedded.org/book/"
                    >
                        Embedded Rust Book
                    </a>
                </li>
                <li>
                    <a
                        class="hover:text-maroon underline"
                        href="https://blog.theembeddedrustacean.com/series/esp32c3-embedded-rust-hal"
                    >
                        The Embedded Rustacean blog
                    </a>
                </li>
            </ul>
        </div>
    }
}
