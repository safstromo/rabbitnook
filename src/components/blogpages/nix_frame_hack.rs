use leptos::prelude::*;

use crate::components::{blog_layout::BlogLayout, code_block::CodeBlock};

#[component]
pub fn NixFrameHack() -> impl IntoView {
    let disable_nix = r#"
adb -d shell pm disable-user --user 0 com.kitesystems.nix.prod
adb -d shell pm disable-user --user 0 com.kitesystems.nix.frame
    "#;
    let install_apk = r#"
adb -d install FILENAME_HERE.apk
    "#;

    let set_screensaver = r#"
adb -d shell settings put secure screensaver_activate_on_sleep 1
    "#;

    view! {
        <BlogLayout overflow_x=true>
            <h1 class="mt-8">NixPlay frame hacking</h1>
            <p class="my-2">April 26, 2025</p>
            <h2 class="underline my-6">Disclaimer</h2>
            <p>Do this at your own risk. This might brick your device.</p>
            <p>I fully own the hardware and this does not involve bypassing any DRM protection.</p>
            <h2 class="my-6">Introduction</h2>
            <p>
                So NixPlay decided to do the bait and switch and i didnt want my frame to just be another e-waste product in the landfill.
            </p>
            <p>
                "After searching around a bit i found this youtube video by "
                <a
                    class="hover:text-maroon underline"
                    href="https://youtu.be/TN5errM5UbA?si=ZpgWuQo7wNWGoUP8"
                    target="_blank"
                >
                    yo-less
                </a>"which i took inspiration from."
            </p>
            <p>
                The NixPlay frame is apparently an Android device, its running Android 7 to we can get some more use of it.
            </p>
            <p>This is a quick overview how i went about it.</p>
            <h2 class="my-6">Open the frame</h2>
            <p>"My Model:"</p>
            <div class="flex justify-center items-center w-full">
                <img
                    src="/blog/nix-frame/model.jpg"
                    alt="NixPlay Frame Model"
                    class="my-6 rounded-md max-h-72"
                />
            </div>
            <p>
                Its easy to open the frame. I used a opening tool from iFixit, went along the frame and pried it open. There is a plastic clip about every 1-2cm.
            </p>
            <p>
                Be careful when you get the top open because the screen is loose inside the frame.
            </p>
            <p>
                When you get it open you can carefully lift the screen and you will see the pcb under it.
            </p>
            <p>Here you will find a debug usb port to connect to.</p>
            <div class="flex justify-center items-center w-full">
                <img
                    src="/blog/nix-frame/nix_pcb.jpg"
                    alt="NixPlay Frame PCB"
                    class="my-6 rounded-md max-screen"
                />
            </div>
            <h2 class="my-6">"Installing software:"</h2>
            <p>You will need ADB(Android Debug Bridge) to connect to the device.</p>
            <ol class="flex flex-col w-full">
                <li class="mt-2">
                    <h4 class="text-lg my-6">1. Disable NixPlay applications.</h4>
                    <p>
                        These commands will disable NixPlay applications. I disabled the applications instead of uninstalling because that might break the device.
                    </p>
                    <CodeBlock class="bash text-white" code=disable_nix />
                </li>
                <li class="mt-2">
                    <h4 class="text-lg my-6">2. Add your own stuff</h4>
                    <p>
                        To install your own apps with adb you use this command make use the apps support Android 7:
                    </p>
                    <CodeBlock class="bash" code=install_apk />
                    <p class="mb-2">For me this is the applications i installed:</p>
                    <p class="underline">"Applauncher"</p>
                    <a
                        class="mb-4 hover:text-maroon"
                        href="https://novalauncher.com/"
                        target="_blank"
                    >
                        Nova Launcher
                    </a>
                    <p class="underline">"RDP software"</p>
                    <a class="mb-4 hover:text-maroon" href="https://rustdesk.com/" target="_blank">
                        RustDesk
                    </a>
                    <p class="underline">Files</p>
                    <a
                        class="mb-4 hover:text-maroon"
                        href="https://github.com/zhanghai/MaterialFiles"
                        target="_blank"
                    >
                        MaterialFiles
                    </a>
                    <p class="underline">Screensaver software for viewing images</p>
                    <a
                        class="mb-4 hover:text-maroon"
                        href="https://github.com/theothernt/AerialViews"
                        target="_blank"
                    >
                        AerialViews
                    </a>
                </li>
                <li class="mt-2">
                    <h4 class="text-lg my-6">
                        3. Activate screensaver when device goes to powersave
                    </h4>
                    <p>When the device goes to sleep we want it go into screensaver.</p>
                    <CodeBlock class="bash" code=set_screensaver />
                </li>
                <li class="mt-2">
                    <h4 class="text-lg my-6">4. Other settings</h4>
                    <p>
                        Now you can use
                        <a
                            class="underline hover:text-maroon"
                            href="https://github.com/Genymobile/scrcpy"
                        >
                            scrcpy
                        </a>to control your screen through adb.
                    </p>
                    <p>Edit the settings and make sure RustDesk works.</p>
                    <ul>
                        <li>Set sleep timer to something like 15-30 sec</li>
                        <li>Make sure Rustdesk and Nova launcher has full permisions.</li>
                        <li>Make sure Ruskdesk runs on boot.</li>
                        <li>Set Aerialviews as screensaver</li>
                    </ul>
                </li>
                <li class="mt-2">
                    <h4 class="text-lg my-6">5. Viewing images</h4>
                    <p>
                        For me, i created an SMB share on my TrueNas machine and added the images i wanted to view into that.
                    </p>
                    <p>I then added that SMB share to AerialViews.</p>
                    <p>You can add clock,date,etc. in AerialViews if you like.</p>
                    <p>
                        My frame had about 10gig of local storage, so you could add photos to the device instead.
                    </p>
                </li>
                <li class="mt-2">
                    <p>
                        Then your done! Make sure everything works, even after reboot before you close up the frame again.
                    </p>
                </li>
            </ol>
            <p>This is the result:</p>
            <div class="flex justify-center items-center w-full">
                <img
                    src="/blog/nix-frame/result.jpg"
                    alt="Result"
                    class="my-6 rounded-md max-h-screen"
                />
            </div>
            <p>
                I hope this helps someone out there that dont want to waste another perfectly fine device.
            </p>
            <p>
                "This is a quick and simple fix, for the future i might create my own screensaver application so i can customize it to my needs "
                "😊"
            </p>
            <p>Credits to yo-less for this, check out his video for more detailed information.</p>
        </BlogLayout>
    }
}
