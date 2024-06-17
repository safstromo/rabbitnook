use leptos::{component, view, IntoView};

#[component]
pub fn Esp32Relay() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen w-full bg-base items-start justify-center">
                <h1 class="my-6">Rust :heart: Esp32 remote relay using AWS IoT</h1>
                <h2>In space no one can hear you scream</h2>
                <p>
                    This is my first adventure into the world of embedded development.'🫠'
                    This project is an experiment of how to control a relay using an ESP32 board and AWS IoT. The relay is connected to the ESP32 board and can be controlled using the AWS IoT MQTT protocol.
                    Im writing this blog post because i had a hard time finding examples and guides on the subject and to share my experience and to help others save time and avoid some of the pitfalls I encountered.
                    This is the first time I am writing embedded code and I am still learning so if you see any mistakes or have any suggestions please let me know. :smile:
                </p>

                <h2>Hardware</h2>
                <a href="https://docs.espressif.com/projects/esp-idf/en/stable/esp32c3/hw-reference/esp32c3/user-guide-devkitm-1.html#esp32-c3-devkitm-1">
                    ESP32-C3-DevKitM-1 board
                </a>
                <a href="https://www.amazon.se/-/en/AZDelivery-KY-019-Module-compatible-Raspberry-including/dp/B07CNR7K9B?pd_rd_w=ZHQja&content-id=amzn1.sym.7aba3564-a536-4264-adad-b89dcc42bc21&pf_rd_p=7aba3564-a536-4264-adad-b89dcc42bc21&pf_rd_r=GXJRQ8D3SX1XZ0RSP4K4&pd_rd_wg=aqNnx&pd_rd_r=20de12af-0b3f-4ea3-a515-02d848173ab6&pd_rd_i=B07CNR7K9B&ref_=pd_bap_d_grid_rp_0_1_ec_pd_nav_hcs_rp_2_t&th=1">
                    1-Relay 5V KY-019-Module
                </a>
                <img src="" alt="Image of hardware"/>
                <h2>AWS IoT setup</h2>
                <p>First thing we need to do is creating a Policy and a Thing in AWS IoT.</p>

                <h3>Create a Policy</h3>
                <ol>
                    <li>1. Go to the AWS IoT console and click on "Security" in the left menu.</li>
                    <li>2. Click on "Policies" and then "Create a policy" .</li>
                    <li>3. Name your policy and go to JSON view.</li>
                </ol>

                <img src="" alt="Image of policy"/>
                <p>
                    Here you define what the thing is allowed to do.
                    This is where my first pitfall was.
                    I had not defined the correct permissions in the policy and the ESP32 could not connect to the AWS IoT endpoint. I had a hard time figuring out what was wrong and the log message from the ESP32 was not very helpful and the logs in AWS IoT did not give me any clues either.
                </p>

                <img src="" alt="Image of log"/>
                <p>
                    After some trial and error I finally got it to work by using the following policy.
                    This allows all actions on all resources. This is not recommended in a production environment but for now it will do. I recommend starting with this and make sure everything works before fine tuning the policy.
                </p>
                <code>
                    "" "
                    {
                    Version: 2012-10-17,
                    Statement: [
                    {
                    Effect: Allow,
                    Action: iot:*,
                    Resource: *
                    }
                    ]
                    }
                    " ""
                </code>
                <p>
                    When I got everything working I went back and fine tuned the policy to only allow the actions I needed. This was the final result:
                </p>
                <code>input policy code?</code>

                <p>
                    This will allow the ESP32 to connect, subscribe and receive messages on the topic
                    "esp32/sub" and publish messages on the topic "esp32/pub" .
                </p>
                <h3>Create a Thing</h3>
                <ol>
                    <li>
                        1. Go to the AWS IoT console and click on "All devices" and then "Things"
                        in the left menu.
                    </li>
                    <li>
                        2. Click on "Create things" in the right corner and then
                        "Create a single thing" .
                    </li>
                    <li>
                        <img src="" alt="Image of create1"/>
                    </li>
                    <li>3. Name your thing and click "Next" .</li>
                    <li>
                        <img src="" alt="Image of create2"/>
                    </li>
                    <li>4. Select "Auto-generate a new certificate" and click "Next" .</li>
                    <li>
                        <img src="" alt="Image of create3"/>
                    </li>
                    <li>
                        5. Next up we need to assign the policy we created earlier to the thing. Select the policy and click
                        "Create thing" .
                    </li>
                    <li>
                        <img src="" alt="Image of create4"/>
                    </li>
                    <li>
                        6. Now a popup will appear where we can download the certificates and keys. Download device certificate, private key and root CA 1 certificate. Dont forget to rename them so you know which is which.
                    </li>
                    <li>
                        <img src="" alt="Image of create5"/>
                    </li>
                </ol>
                <h2>Coding</h2>
                <p>
                    Now we have everything set up in AWS IoT and we can start with the fun part, coding the ESP32. I will write this in Rust because I love Rust so why not :shrug:
                </p>
                <h3>Project setup</h3>
                <p>
                    First thing we need to decide is if we will be using STD or no STD. I will be using std because it makes things easier and I am not too concerned about the size of the binary since the ESP32 has plenty of memory.
                </p>
                <p>
                    Im using the "esp-idf-template" as a base for this project. You can find it
                    <a href="https://github.com/esp-rs/esp-idf-template">here.</a>
                    Make sure you have installed all prerequisites for the template and embedded Rust development. Here is a great book to get you started with embedded Rust:
                    <a href="https://docs.esp-rs.org/std-training/">ESP STD Embedded Training</a>
                </p>
                <code>bash Create</code>
                <p>
                    This will create a new project with the name you specify. I will name mine
                    "esp32-aws-iot-relay"
                    . Follow the prompts and select the correct board and other settings.
                    Make sure the project builds and runs on the ESP32 before continuing.
                </p>
                <code>bash run code</code>

                <h3>Structs</h3>
                <p>
                    "structs.rs"
                    will contain the structs we need for the MQTT messages and the configuration.
                    Here is what i ended up with:
                </p>
                <code>all structs.rs</code>

                <p>
                    I am using the "dotenv" crate to load the configuration from a ".env"
                    file, witch i find very convenient. You can find the "env"
                    file in the root of the project.
                </p>
                <code>.env file</code>
                I created a Config struct that will hold the configuration for the project. The configuration is loaded from the
                ".env"
                file and the certificates are loaded from the
                "aws"
                folder in the project. The certificates are needed to connect to the AWS IoT endpoint.
                Converting the certificates to the correct format was someting i struggled with. I found a solution that works but I am not sure if it is the best way to do it. If you know a better way please let me know. Here is a link to the stackoverflow thread where I found the solution:
                <a href="https://stackoverflow.com/questions/75299434/rust-on-esp32-how-to-send-and-receive-data-using-the-mqtt-protocol-to-aws-io">
                    link
                </a>

                <h3>Wifi</h3>
                <p>"wifi.rs" will contain the wifi setup and the reconnect function.</p>
                <code>all wifi code</code>
                <p>
                    I choose the esp32 because it had great Rust support and libraries. I am using the
                    "esp-idf-sys" crate to interact with the ESP32 and the "esp-idf-svc"
                    crate for the wifi and mqtt setup. The "esp-idf-hal"
                    crate is used for interacting with the GPIO and other peripherals. These are great crates for someone new to embedded development like me since they abstract away a lot of the complexity of embedded development.
                </p>
                <h3>Main</h3>
                <p>"main.rs" will contain the main logic of the project.</p>
                <code>Main.rs code</code>
                <p>
                    First we set up the GPIO pins for the relay and the button(the two wires :smile:) . The relay is connected to GPIO10 and the button is connected to GPIO19. Im setting the button to pull up so it will be high when not pressed and low when pressed. High means its not pressed and low means its pressed. The relay is set to low so it is not activated when the ESP32 starts.
                </p>
                <p>
                    I had to use a Arc Mutex to be able to share the relay pin between the main loop and the mqtt callback. I then clone the arc to create a reference for the mqtt callback.
                </p>
                <p>
                    Next up im setting up the WS2812 LED. I decided to use the WS2812 LED to give some feedback on the wifi status of the ESP32. The LED will be green when everything is working and red when something is wrong. The LED is connected to GPIO8 acording to the ESP32-C3-DevKitM-1 board schematic. For this i found a crate called
                    "ws2812-esp32-rmt-driver" that makes it easy to control the WS2812 LED.
                </p>
                <p>

                    I then create a "Config" struct and load the configuration from the ".env"
                    file. The certificates are loaded from the "aws" folder in the project.
                </p>

                <p>Then i used the wifi setup function shown earlier to connect to the wifi.</p>
                <p>
                    Next up is the MQTT setup. I create a "MqttClientConfiguration" and a
                    "EspMqttClient"
                    instance.The MqttClient new function takes a callback that will be called when a message is received. In the callback i check if the message is the one i am looking for and then activate the relay if it is. I could not find any good guides and the doc.rs didnt have much info on how to use the
                    "esp-idf-svc"
                    crate so i looked at the examples on github and this is what i came up with after some trial and error.
                </p>
                <p>
                    I then subscribe to the topic and start the main loop. In the main loop i check if the wifi is connected and if not i try to reconnect. I then check if the button is pressed(the two cables are connected) and if it is i activate the relay and publish a message to the topic
                </p>
                <h3>Final project</h3>
                <p>Here is the final project:</p>
                <a href="gihub">esp32-aws-iot-relay</a>
                <p>
                    I hope this blog post was helpful and hope it will save someone some time and frustration.
                </p>
                <p>
                    If you have suggestions or improvements feel free to create a PR,open an issue or contact me :smile:
                </p>
                <p>Happy coding! :smile:</p>

                <h2>Useful links</h2>
                <ul>
                    <li>
                        <a href="https://docs.esp-rs.org/std-training/">Espressif STD-Traing</a>
                    </li>
                    <li>
                        <a href="https://docs.rust-embedded.org/book/">Embedded Rust Book</a>
                    </li>
                    <li>
                        <a href="https://blog.theembeddedrustacean.com/series/esp32c3-embedded-rust-hal">
                            The Embedded Rustacean blog
                        </a>
                    </li>
                </ul>
        </div>
    }
}
