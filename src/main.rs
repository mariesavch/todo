#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_elements::canvas::width;
use dioxus_logger::tracing::field::display;
use dioxus_sdk::storage::*;
use serde::{Deserialize, Serialize};
use sir::{css, global_css, AppStyle};

fn main() {
    dioxus_sdk::storage::set_dir!();
    dioxus::launch(App);
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Todo {
    title: String,
    description: String,
    completed: bool,
    id: u64,
}

#[component]
fn App() -> Element {
    let mut todos = use_synced_storage::<LocalStorage, Vec<Todo>>("todos".to_string(), || vec![]);

    let mut title = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let on_submit = move |_e: FormEvent| {
        if title.to_string().is_empty() {
            return;
        }

        let new_todo = Todo {
            title: title.clone().to_string(),
            description: description.clone().to_string(),
            completed: false,
            id: todos.len() as u64 + 1,
        };

        let mut updated_todos = todos.cloned();
        updated_todos.push(new_todo);
        todos.set(updated_todos);

        title.set("".to_string());
        description.set("".to_string());
    };

    global_css!(
        "
       :root {
        --rosewater: #ff8389;
        --flamingo: #ff8389;
        --red: #ff8389;
        --maroon: #ff8389;
        --pink: #ff7eb6;
        --mauve: #be95ff;
        --peach: #d44a1c;
        --yellow: #ab8600;
        --green: #08bdba;
        --teal: #33b1ff;
        --sky: #33b1ff;
        --sapphire: #33b1ff;
        --blue: #78a9ff;
        --lavender: #78a9ff;
        --text: #ffffff;
        --subtext1: #f4f4f4;
        --subtext0: #e0e0e0;
        --overlay2: #adadad;
        --overlay1: #949494;
        --overlay0: #7a7a7a;
        --surface2: #4f4f4f;
        --surface1: #383838;
        --surface0: #2e2e2e;
        --base: #161616;
        --mantle: #0d0d0d;
        --crust: #000000;
    } 

    @media (prefers-color-scheme: light) {
        :root {
            --rosewater: #da1e28;
            --flamingo: #da1e28;
            --red: #da1e28;
            --maroon: #da1e28;
            --pink: #d02670;
            --mauve: #8a3ffc;
            --peach: #d44a1c;
            --yellow: #ab8600;
            --green: #007d79;
            --teal: #1192e8;
            --sky: #1192e8;
            --sapphire: #1192e8;
            --blue: #0f62fe;
            --lavender: #0f62fe;
            --text: #000000;
            --subtext1: #404040;
            --subtext0: #474747;
            --overlay2: #575757;
            --overlay1: #595959;
            --overlay0: #737373;
            --surface2: #8c8c8c;
            --surface1: #d1d1d1;
            --surface0: #e6e6e6;
            --base: #ffffff;
            --mantle: #f2f2f2;
            --crust: #ebebeb;
        }
    }

    :root {
        background-color: var(--base);
        color: var(--text);
        line-height: 1.6;
    }

    "
    );

    let underlined =css!("
    all: unset;
    cursor: pointer;
    text-decoration-line: underline; 
    text-decoration-thickness: 2px; 
    text-underline-offset: 4px; 
    transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; 
    transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); 
    transition-duration: 300ms;
    text-decoration-color: var(--surface2);

    &:hover {
        text-decoration-color: var(--overlay2);
    }

    &:active {
        text-decoration-color: var(--overlay1);
    }
    ");

    let animated_list = css!(
        "
    @media (hover: hover) and (pointer: fine) {
        li {
            all: unset;
            transition-property: all;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 300ms;
        }
        &:hover li {
            opacity: 0.5;
        }
        &:hover li:hover {
            opacity: 1;
        }
    }
    "
    );

    let input = css!(
        "
        all: unset;
        padding-top: 0.5rem;
        padding-bottom: 0.5rem; 
        padding-left: 1rem;
        padding-right: 1rem;
        border: 1px solid var(--surface0); 
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 300ms; 
        color: var(--text);
        width: 100%;

        &:hover {
            border-color: var(--surface1);
        }
        &:focus {
            border-color: var(--surface2);
        }
        &:placeholder {
            color: var(--overlay0);
        }
        "
    );

    let section = css!(
        "
    padding-top: 24px;
    @media(min-width: 950px) {
        padding-top: 64px
    }
    "
    );

    rsx! {
        AppStyle {}
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: section,
                form {
                    margin_bottom: "32px",
                    display: "flex",
                    onsubmit: on_submit,
                    input {
                        aria_label: "enter title",
                        placeholder: "enter title",
                        r#type: "text",
                        value: "{title}",
                        oninput: move |e| title.set(e.value().clone()),
                        class: input,
                    }
                    input {
                        aria_label: "enter description",
                        placeholder: "enter description",
                        r#type: "text",
                        value: "{description}",
                        oninput: move |e| description.set(e.value().clone()),
                        class: input,
                    }
                    input { r#type: "submit", value: "add", display: "none" }
                }
                ul { class: animated_list, padding: "unset",
                    {todos.iter().enumerate().map(|(i, todo)| rsx! {
                        li { padding_bottom: "20px", display: "flex",
                            button {
                                all: "unset",
                                cursor: "pointer",
                                onclick: move |_| {
                                    let mut todolist = todos.cloned();
                                    let mut todo = todolist.remove(i);
                                    todo.completed = !todo.completed;
                                    todolist.insert(i, todo);
                                    todos.set(todolist);
                                },
                                span {
                                    text_decoration_line: if todo.completed { "line-through" },
                                    font_style: "bold",
                                    margin_right: "20px",
                                    "{todo.title}"
                                }
                                span {
                                    text_decoration_line: if todo.completed { "line-through" },
                                    color: "var(--overlay0)",
                                    font_style: "italic",
                                    "{todo.description}"
                                }
                            }
                            button {
                                class: underlined,
                                margin_left: "auto",
                                onclick: move |_| {
                                    let mut todolist = todos.cloned();
                                    todolist.remove(i);
                                    todos.set(todolist);
                                },
                                "delete"
                            }
                        }
                    })}
                }
            }
        }
    }
}
