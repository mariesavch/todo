#![allow(non_snake_case)]

use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use serde::{Deserialize, Serialize};

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

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        ":root" {
            __rosewater: "#ff8389",
            __flamingo: "#ff8389",
            __red: "#ff8389",
            __maroon: "#ff8389",
            __pink: "#ff7eb6",
            __mauve: "#be95ff",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#08bdba",
            __teal: "#33b1ff",
            __sky: "#33b1ff",
            __sapphire: "#33b1ff",
            __blue: "#78a9ff",
            __lavender: "#78a9ff",
            __text: "#ffffff",
            __subtext1: "#f4f4f4",
            __subtext0: "#e0e0e0",
            __overlay2: "#adadad",
            __overlay1: "#949494",
            __overlay0: "#7a7a7a",
            __surface2: "#4f4f4f",
            __surface1: "#383838",
            __surface0: "#2e2e2e",
            __base: "#161616",
            __mantle: "#0d0d0d",
            __crust: "#000000",
        },
    "@media (prefers-color-scheme: light)" {
        ":root" {
            __rosewater: "#da1e28",
            __flamingo: "#da1e28",
            __red: "#da1e28",
            __maroon: "#da1e28",
            __pink: "#d02670",
            __mauve: "#8a3ffc",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#007d79",
            __teal: "#1192e8",
            __sky: "#1192e8",
            __sapphire: "#1192e8",
            __blue: "#0f62fe",
            __lavender: "#0f62fe",
            __text: "#000000",
            __subtext1: "#404040",
            __subtext0: "#474747",
            __overlay2: "#575757",
            __overlay1: "#595959",
            __overlay0: "#737373",
            __surface2: "#8c8c8c",
            __surface1: "#d1d1d1",
            __surface0: "#e6e6e6",
            __base: "#ffffff",
            __mantle: "#f2f2f2",
            __crust: "#ebebeb",
            }
        },
        ":root" {
            background_color: "var(--base)",
            color: "var(--text)",
            line_height: "1.6",
            font_family: "Cartograph CF",
        },
        "@media (hover: hover) and (pointer: fine)" {
            ".animated_list li" {
                all: "unset",
                transition_property: "all",
                transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
                transition_duration: "300ms",
            },
            ".animated_list:hover li" {
                opacity: "0.5",
            },
            ".animated_list:hover li:hover" {
                opacity: "1",
            }
        },
        ".input" {
            all: "unset",
            padding_top: "0.5rem",
            padding_bottom: "0.5rem",
            padding_left: "1rem",
            padding_right: "1rem",
            border: "1px solid var(--surface0)",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            color: "var(--text)",
            width: "100%",
        },
        ".input:hover" {
            border_color: "var(--surface1)",
        },
        ".input:focus" {
            border_color: "var(--surface2)",
        },
        ".input:placeholder" {
            color: "var(--overlay0)",
        },
        ".section" {
            padding_top: "24px",
        },
        "@media(min-width: 950px)" {
            ".section" {
                padding_top: "64px",
            }
        },
        ".underlined" {
            all: "unset",
            cursor: "pointer",
            text_decoration_line: "underline",
            text_decoration_thickness: "2px",
            text_underline_offset: "4px",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            text_decoration_color: "var(--surface2)",
        },
    }
}

#[component]
fn App() -> Element {
    use_style_provider_quickstart(|| EmptyTheme);
    let cls: &MyClasses = &MyClasses::use_style();

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

    rsx! {
        style {
            "
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Regular.woff2') format('woff2');
            }}
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-RegularItalic.woff2') format('woff2');
            font-style: italic;
            }}
            @font-face {{
            font-family: 'Cartograph CF';
            src: url('./assets/fonts/CartographCF-Bold.woff2') format('woff2');
            font-weight: bold;
            }}"
        }
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div { class: &cls.section as &str,
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
                        class: &cls.input as &str,
                    }
                    input {
                        aria_label: "enter description",
                        placeholder: "enter description",
                        r#type: "text",
                        value: "{description}",
                        oninput: move |e| description.set(e.value().clone()),
                        class: &cls.input as &str,
                    }
                    input { r#type: "submit", value: "add", display: "none" }
                }
                ul { class: &cls.animated_list as &str, padding: "unset",
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
                                class: &cls.underlined as &str,
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
