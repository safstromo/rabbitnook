use leptos::*;

#[derive(Debug, Clone)]
pub struct Command {
    command: String,
    component: HtmlTag,
    value: String,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum HtmlTag {
    P,
    A,
}

#[component]
pub fn TerminalInput(
    set_command_history: WriteSignal<Vec<Command>>,
    command_history: ReadSignal<Vec<Command>>,
    input_element: NodeRef<html::Input>,
) -> impl IntoView {
    let (input, _set_input) = create_signal("".to_string());
    // let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        if command_history.get().len() > 12 {
            set_command_history.update(|commands| commands.clear());
        }

        match value.as_str() {
            "clear" => set_command_history.update(|commands| commands.clear()),
            "help" => {
                let help_command = Command {
                    command: value.clone(),
                    component: HtmlTag::P,
                    value: "Available commands: help, pwd, git, vim, email, sudo, linkedin, clear"
                        .to_string(),
                    name: "help".to_string(),
                };
                set_command_history.update(|commands| commands.push(help_command));
            }
            "sudo" => {
                open_link("https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string());
                let sudo_command = Command {
                    command: value.clone(),
                    component: HtmlTag::A,
                    value: "https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string(),
                    name: "sudo".to_string(),
                };
                set_command_history.update(|commands| commands.push(sudo_command));
            }
            "pwd" => {
                open_link("https://github.com/safstromo/rabbitnook".to_string());
                let pwd_command = Command {
                    command: value.clone(),
                    component: HtmlTag::A,
                    value: "https://github.com/safstromo/rabbitnook".to_string(),
                    name: "github.com/safstromo/rabbitnook".to_string(),
                };
                set_command_history.update(|commands| commands.push(pwd_command));
            }
            "git" => {
                open_link("https://github.com/safstromo".to_string());
                let git_command = Command {
                    command: value.clone(),
                    component: HtmlTag::A,
                    value: "https://github.com/safstromo".to_string(),
                    name: "github.com/safstromo".to_string(),
                };
                set_command_history.update(|commands| commands.push(git_command));
            }
            "email" => {
                open_link("mailto:safstrom.oliver@gmail.com".to_string());
                let email_command = Command {
                    command: value.clone(),
                    component: HtmlTag::A,
                    value: "mailto:safstrom.oliver@gmail.com".to_string(),
                    name: "safstrom.oliver@gmail.com".to_string(),
                };
                set_command_history.update(|commands| commands.push(email_command));
            }
            "linkedin" => {
                open_link("https://www.linkedin.com/in/safstromo/".to_string());
                let linkedin_command = Command {
                    command: value.clone(),
                    component: HtmlTag::A,
                    value: "https://www.linkedin.com/in/safstromo/".to_string(),
                    name: "linkedin.com/in/safstromo".to_string(),
                };
                set_command_history.update(|commands| commands.push(linkedin_command));
            }
            "vim" => {
                open_link(
                    "https://github.com/safstromo/.dotfiles/tree/main/nvim/.config/nvim"
                        .to_string(),
                );
                let vim_command = Command {
                    command: value.clone(),
                    component: HtmlTag::A,
                    value: "https://github.com/safstromo/.dotfiles/tree/main/nvim/.config/nvim"
                        .to_string(),
                    name: "nvim .dotfiles".to_string(),
                };
                set_command_history.update(|commands| commands.push(vim_command));
            }

            _ => {
                let invalid_command = Command {
                    command: value.clone(),
                    component: HtmlTag::P,
                    value: value.clone() + ": command not found",
                    name: "invalid".to_string(),
                };
                set_command_history.update(|commands| commands.push(invalid_command));
            }
        }
        let _ = input_element()
            .expect("input element should be mounted")
            .set_value("");
    };

    view! {
        <TerminalPwd/>
        <section class="flex flex-row w-full">
            <svg
                class="w-6 h-6 text-green"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                viewBox="0 0 24 24"
            >
                <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="m9 5 7 7-7 7"
                ></path>
            </svg>
            // TODO: Fix input
            <form class="w-full" on:submit=on_submit>
                <input
                    class="w-5/6 mx-2 bg-base border-none text-white focus:outline-none"
                    type="text"
                    value=input
                    node_ref=input_element
                    id="terminal-input"
                    autoFocus
                />
            // TODO: Add caret
            // <span class="caret"></span>
            </form>
        </section>
    }
}

fn open_link(url: String) {
    create_effect(move |_| {
        let window = web_sys::window().expect("window should be available");
        window.open_with_url_and_target(&url, "_blank").unwrap();
    });
}

#[component]
pub fn TerminalHistory(command_history: ReadSignal<Vec<Command>>) -> impl IntoView {
    view! {
        <ul class="overflow-hidden">
            <For each=command_history key=|command| command.command.clone() let:child>
                <TerminalCommand command=child/>
            </For>

        </ul>
    }
}

#[component]
fn TerminalCommand(command: Command) -> impl IntoView {
    if command.component == HtmlTag::A {
        return view! {
            <TerminalPwd/>
            <div class="flex flex-row mb-2">
                <svg
                    class="w-6 h-6 text-green"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="m9 5 7 7-7 7"
                    ></path>
                </svg>
                <li class="text-white">
                    <a class="text-blue mx-2" href=command.value.clone() target="_blank">
                        {command.name}
                    </a>
                </li>
            </div>
        };
    } else {
        return view! {
            <TerminalPwd/>
            <div class="flex flex-row mb-2">
                <svg
                    class="w-6 h-6 text-green"
                    aria-hidden="true"
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="m9 5 7 7-7 7"
                    ></path>
                </svg>
                <li class="text-white">
                    <p class="text-white mx-2">{command.value}</p>
                </li>
            </div>
        };
    }
}

#[component]
fn TerminalPwd() -> impl IntoView {
    view! {
        <section class="flex flex-row mx-2 items-start gap-1">
            <p class="text-teal text-lg font-semibold">rabbitnook</p>
            <p class="text-white text-lg font-semibold">on</p>
            <svg
                class="w-6 h-6 text-pink"
                aria-hidden="true"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="currentColor"
                viewBox="0 0 24 24"
            >
                <path d="M8 3a3 3 0 0 0-1 5.83v6.34a3.001 3.001 0 1 0 2 0V15a2 2 0 0 1 2-2h1a5.002 5.002 0 0 0 4.927-4.146A3.001 3.001 0 0 0 16 3a3 3 0 0 0-1.105 5.79A3.001 3.001 0 0 1 12 11h-1c-.729 0-1.412.195-2 .535V8.83A3.001 3.001 0 0 0 8 3Z"></path>
            </svg>
            <p class="text-pink text-lg font-semibold">main</p>

            <svg
                class="w-6 h-6 text-peach mx-1"
                fill="currentColor"
                width="1200pt"
                height="1200pt"
                version="1.1"
                viewBox="0 0 1200 1200"
                xmlns="http://www.w3.org/2000/svg"
            >
                <g>
                    <path d="m1200 392.95c0-0.80469-0.10938-1.5977-0.16797-2.3867-0.046875-0.74219-0.046875-1.5117-0.15625-2.2578-0.10938-0.75781-0.28906-1.5-0.44531-2.2422-0.15625-0.78125-0.26562-1.5586-0.46875-2.3281-0.19141-0.69531-0.46875-1.3672-0.70703-2.0508-0.25391-0.76953-0.48047-1.5703-0.78125-2.3281-0.28906-0.68359-0.67188-1.3203-0.99609-2.0039-0.33594-0.70703-0.64844-1.4414-1.043-2.125-0.39453-0.68359-0.86328-1.3086-1.3086-1.957-0.40625-0.61328-0.77734-1.2734-1.2461-1.875-0.48047-0.63672-1.0547-1.2227-1.5859-1.8359-0.49219-0.53906-0.92578-1.1289-1.4531-1.6562-0.55078-0.53906-1.1758-1.0312-1.7539-1.5469-0.58984-0.50391-1.1289-1.043-1.7539-1.5234-1.2344-0.96094-2.543-1.8242-3.9102-2.6289l-565.73-326.61c-11.027-6.3711-24.625-6.3477-35.641 0.046875l-561.84 326.51s-0.011718 0-0.011718 0.011718c0 0-0.011719 0-0.011719 0.011719l-0.10938 0.058594c-0.60156 0.35938-1.1406 0.78125-1.7266 1.1641-0.70703 0.48047-1.4531 0.91016-2.125 1.4414-0.63672 0.48047-1.1992 1.0312-1.7891 1.5586-0.5625 0.50391-1.1641 0.97266-1.7031 1.5-0.57422 0.58984-1.0781 1.2344-1.6211 1.8477-0.46875 0.53906-0.97266 1.0664-1.4141 1.6211-0.50391 0.66016-0.92578 1.3672-1.3789 2.0625-0.38281 0.58984-0.81641 1.1523-1.1758 1.7656-0.39453 0.69531-0.70703 1.4297-1.0664 2.1602-0.32812 0.64844-0.69922 1.2852-0.97656 1.9688-0.29688 0.71875-0.51563 1.4883-0.76563 2.2305-0.25391 0.70703-0.52734 1.3906-0.73047 2.125-0.20312 0.76953-0.32422 1.5703-0.48047 2.3633-0.14453 0.73047-0.33594 1.4531-0.44531 2.1953-0.10937 0.79297-0.10937 1.6094-0.16797 2.4102-0.046875 0.74219-0.15625 1.4648-0.15625 2.2188l-1.1523 414.03c-0.035156 12.73 6.7422 24.516 17.773 30.875l565.74 326.63c5.4961 3.1914 11.641 4.7656 17.762 4.7656 1.5469 0 3.0859-0.097656 4.6211-0.28906 0.27734-0.046875 0.55078-0.13281 0.82812-0.17969 1.2461-0.19141 2.4844-0.40625 3.7188-0.74219 0.27734-0.058594 0.55078-0.20312 0.82812-0.26562 1.2109-0.35938 2.4102-0.74219 3.6016-1.2227 0.33594-0.15625 0.66016-0.33594 0.99609-0.49219 1.0547-0.48047 2.125-0.96094 3.1562-1.5469 0.023437-0.023437 0.046874-0.035156 0.070312-0.046875 0.023438-0.023438 0.046875-0.023438 0.058594-0.035156l562-326.61c10.906-6.3359 17.652-18 17.688-30.637l1.1641-413.99v-0.070313-0.085937zm-601.23-285.48 212.96 122.95-492.72 284.47-211.36-122.03zm-32.859 964.67-494.75-285.65 0.94922-332.04 494.73 285.64zm36.492-393.64-212.29-122.55 492.73-284.47 210.67 121.63zm525.38 108.01-490.77 285.21 0.9375-331.64 490.78-285.2z"></path>
                    <path d="m525.8 1001-212.44-122.65 0.39844-142.58 212.45 122.65z"></path>
                </g>
            </svg>
            <p class="text-peach text-lg font-semibold">v1.0.0</p>
        </section>
    }
}
