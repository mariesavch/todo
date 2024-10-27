#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use serde::{Deserialize, Serialize};

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

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
    let mut todos = use_synced_storage::<LocalStorage, Vec<Todo>>("todos".to_string(), || {
        vec![
            Todo {
                title: "task 1".to_string(),
                description: "task 1 desc".to_string(),
                completed: true,
                id: 1,
            },
            Todo {
                title: "task 2".to_string(),
                description: "task 2 desc".to_string(),
                completed: false,
                id: 2,
            },
            Todo {
                title: "task 3".to_string(),
                description: "task 3 desc".to_string(),
                completed: false,
                id: 3,
            },
            Todo {
                title: "task 4".to_string(),
                description: "task 4 desc".to_string(),
                completed: false,
                id: 4,
            },
            Todo {
                title: "task 5".to_string(),
                description: "task 5 desc".to_string(),
                completed: false,
                id: 5,
            },
            Todo {
                title: "task 6".to_string(),
                description: "task 6 desc".to_string(),
                completed: false,
                id: 6,
            },
            Todo {
                title: "task 7".to_string(),
                description: "task 7 desc".to_string(),
                completed: false,
                id: 7,
            },
            Todo {
                title: "task 8".to_string(),
                description: "task 8 desc".to_string(),
                completed: false,
                id: 8,
            },
        ]
    });

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
        main { class: "mx-auto max-w-3xl px-6 pb-20",
            div { class: "pt-16",
                form { class: "mb-8", onsubmit: on_submit,
                    input {
                        aria_label: "enter title",
                        placeholder: "enter title",
                        r#type: "text",
                        value: "{title}",
                        oninput: move |e| title.set(e.value().clone()),
                        class: "rounded-t-lg lg:rounded-none lg:rounded-l-lg w-full
                                lg:w-2/5 border border-surface0 bg-base py-2 px-4
                                outline-none transition-colors duration-300
                                placeholder:text-overlay0 hover:border-surface1
                                focus:text-text focus:border-surface2 mr-[-1] mb-[-1]"
                    }
                    input {
                        aria_label: "enter description",
                        placeholder: "enter description",
                        r#type: "text",
                        value: "{description}",
                        oninput: move |e| description.set(e.value().clone()),
                        class: "rounded-none border w-full lg:w-2/5
                                border-surface0 bg-base py-2 px-4
                                outline-none transition-colors duration-300
                                placeholder:text-overlay0 hover:border-surface1
                                focus:text-text focus:border-surface2 mr-[-1] mb-[-1]"
                    }
                    input {
                        r#type: "submit",
                        value: "add",
                        class: "rounded-b-lg lg:rounded-none lg:rounded-r-lg border w-full
                                lg:w-1/5 border-surface0 bg-base py-2 px-4
                                outline-none transition-colors duration-300
                                placeholder:text-overlay0 hover:border-surface1
                                focus:text-text focus:border-surface2"
                    }
                }
                ul { class: "animated-list",
                    {todos.iter().enumerate().map(|(i, todo)| rsx!(
                    li { class: "pb-5 flex",
                        button { class: "active:opacity-80", onclick: move |_| {
                                    let mut todolist = todos.cloned();
                                    let mut todo = todolist.remove(i);
                                    todo.completed = !todo.completed;
                                    todolist.insert(i, todo);
                                    todos.set(todolist);
                                },
                            span { class: [if todo.completed {"line-through"} else {""}, "font-bold mr-5"].join(" "), "{todo.title}"}
                            span { class: [if todo.completed {"line-through"} else {""}, "text-overlay0 italic"].join(" "), "{todo.description}"}
                        }
                        button { class: "ml-auto underlined",
                        onclick: move |_| {
                                    let mut todolist = todos.cloned();
                                    todolist.remove(i);
                                    todos.set(todolist);
                                },
                        "delete" }
                    }
                    ))}
                }
            }
        }
    }
}
