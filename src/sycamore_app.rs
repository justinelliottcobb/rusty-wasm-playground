#[cfg(feature = "sycamore")]
use sycamore::prelude::*;
#[cfg(feature = "sycamore")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "sycamore")]
// Example 1: Reactive counter with two-way binding
#[component]
fn Counter<G: Html>(cx: Scope) -> View<G> {
    let count = create_signal(cx, 0);
    
    view! { cx,
        div(class="sycamore-example") {
            h3 { "Reactive Counter" }
            p { "Count: " (count.get()) }
            button(on:click=move |_| count.set(*count.get() + 1)) { "+" }
            button(on:click=move |_| count.set(*count.get() - 1)) { "-" }
            button(on:click=move |_| count.set(0)) { "Reset" }
        }
    }
}

#[cfg(feature = "sycamore")]
// Example 2: Todo list with reactive state
#[derive(Clone, Debug, PartialEq)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[cfg(feature = "sycamore")]
#[component]
fn TodoList<G: Html>(cx: Scope) -> View<G> {
    let todos = create_signal(cx, vec![
        Todo { id: 1, text: "Learn Sycamore".to_string(), completed: false },
        Todo { id: 2, text: "Build reactive apps".to_string(), completed: false },
    ]);
    
    let input_text = create_signal(cx, String::new());
    let next_id = create_signal(cx, 3);
    
    let add_todo = move |_| {
        let text = input_text.get().to_string();
        if !text.is_empty() {
            let mut current_todos = (*todos.get()).clone();
            current_todos.push(Todo {
                id: *next_id.get(),
                text,
                completed: false,
            });
            todos.set(current_todos);
            next_id.set(*next_id.get() + 1);
            input_text.set(String::new());
        }
    };
    
    let toggle_todo = move |id: usize| {
        let mut current_todos = (*todos.get()).clone();
        if let Some(todo) = current_todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
        todos.set(current_todos);
    };
    
    let remove_todo = move |id: usize| {
        let current_todos = (*todos.get()).clone();
        let filtered: Vec<Todo> = current_todos.into_iter()
            .filter(|t| t.id != id)
            .collect();
        todos.set(filtered);
    };
    
    view! { cx,
        div(class="sycamore-example") {
            h3 { "Todo List (Reactive)" }
            
            div(class="todo-input") {
                input(
                    type="text",
                    placeholder="Enter a new todo",
                    bind:value=input_text
                )
                button(on:click=add_todo) { "Add Todo" }
            }
            
            ul(class="todo-list") {
                Keyed(
                    iterable=todos,
                    view=move |cx, todo| {
                        let todo_id = todo.id;
                        let todo_completed = todo.completed;
                        
                        view! { cx,
                            li(class=if todo_completed { "completed" } else { "" }) {
                                input(
                                    type="checkbox",
                                    checked=todo_completed,
                                    on:change=move |_| toggle_todo(todo_id)
                                )
                                span(
                                    style=if todo_completed { 
                                        "text-decoration: line-through; opacity: 0.6;" 
                                    } else { "" }
                                ) { (todo.text.clone()) }
                                button(
                                    on:click=move |_| remove_todo(todo_id),
                                    style="margin-left: 10px;"
                                ) { "Remove" }
                            }
                        }
                    },
                    key=|todo| todo.id,
                )
            }
            
            p { "Total todos: " (todos.get().len()) }
            p { 
                "Completed: " 
                (todos.get().iter().filter(|t| t.completed).count())
                " / "
                (todos.get().len())
            }
        }
    }
}

#[cfg(feature = "sycamore")]
// Example 3: Computed values and conditional rendering
#[component]
fn ReactiveForm<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let age = create_signal(cx, String::new());
    let show_greeting = create_signal(cx, false);
    
    // Computed value
    let is_valid = create_memo(cx, || {
        !name.get().is_empty() && age.get().parse::<u32>().is_ok()
    });
    
    let greeting = create_memo(cx, || {
        if *is_valid.get() {
            format!("Hello, {}! You are {} years old.", name.get(), age.get())
        } else {
            String::new()
        }
    });
    
    view! { cx,
        div(class="sycamore-example") {
            h3 { "Reactive Form with Computed Values" }
            
            div {
                input(
                    type="text",
                    placeholder="Enter your name",
                    bind:value=name
                )
            }
            
            div {
                input(
                    type="number",
                    placeholder="Enter your age",
                    bind:value=age
                )
            }
            
            button(
                disabled=!*is_valid.get(),
                on:click=move |_| show_greeting.set(!*show_greeting.get())
            ) { 
                (if *show_greeting.get() { "Hide" } else { "Show" })
                " Greeting" 
            }
            
            (if *show_greeting.get() && *is_valid.get() {
                view! { cx,
                    div(class="greeting") {
                        p { (greeting.get()) }
                    }
                }
            } else {
                view! { cx, }
            })
            
            p(style="color: #888; font-size: 0.9em;") {
                (if *is_valid.get() {
                    "✓ Form is valid"
                } else if name.get().is_empty() {
                    "Please enter your name"
                } else if age.get().is_empty() {
                    "Please enter your age"
                } else {
                    "Please enter a valid age"
                })
            }
        }
    }
}

#[cfg(feature = "sycamore")]
// Main app component that combines all examples
#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        div(class="sycamore-container") {
            h1 { "🦀 Sycamore Examples" }
            p { "Reactive UI components with fine-grained reactivity" }
            
            Counter {}
            TodoList {}
            ReactiveForm {}
        }
    }
}

#[cfg(feature = "sycamore")]
// Entry point for Sycamore app
#[wasm_bindgen]
pub fn run_sycamore_app() {
    console_error_panic_hook::set_once();
    
    sycamore::render(|cx| {
        view! { cx,
            App {}
        }
    });
}