use leptos::prelude::*;
use leptos::web_sys::{HtmlInputElement, HtmlTextAreaElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::js_sys;

#[component]
pub fn editor() -> impl IntoView {
    let input_ref = NodeRef::new();

    view! {
        <div class="argon">
            <textarea
                class="editor"
                spellcheck="false"
                on:input=move |event| {
                    let textarea = event
                        .target()
                        .unwrap()
                        .unchecked_into::<HtmlTextAreaElement>();

                    let value = textarea.value();
                    let selection_start = textarea.selection_start().unwrap_or(Some(0)).unwrap();
                    if selection_start > 0 {
                        let last_typed_char = value.chars().nth(selection_start as usize - 1);
                        let mut new_value = value.clone();
                        let mut new_cursor_pos = selection_start;

                        if let Some(char) = last_typed_char {
                            match char {
                                '{' => {
                                    new_value = format!(
                                        "{}{{\n\n}}{}",
                                        &value[..(selection_start as usize)],
                                        &value[selection_start as usize..]
                                    );
                                    new_cursor_pos = selection_start + 2;
                                }
                                '(' => {
                                    new_value = format!(
                                        "{}(){}",
                                        &value[..(selection_start as usize)],
                                        &value[selection_start as usize..]
                                    );
                                    new_cursor_pos = selection_start + 1;
                                }
                                '[' => {
                                    new_value = format!(
                                        "{}[]{}",
                                        &value[..(selection_start as usize)],
                                        &value[selection_start as usize..]
                                    );
                                    new_cursor_pos = selection_start + 1;
                                }
                                '"' => {
                                    new_value = format!(
                                        "{}\"\"{}",
                                        &value[..(selection_start as usize)],
                                        &value[selection_start as usize..]
                                    );
                                    new_cursor_pos = selection_start + 1;
                                }
                                '\'' => {
                                    new_value = format!(
                                        "{}''{}",
                                        &value[..(selection_start as usize)],
                                        &value[selection_start as usize..]
                                    );
                                    new_cursor_pos = selection_start + 1;
                                }
                                _ => {}
                            }

                            textarea.set_value(&new_value);
                            textarea
                                .set_selection_range(new_cursor_pos, new_cursor_pos)
                                .unwrap();
                        }
                    }
                }
                on:keydown=move |event| {
                    let key_event = event.unchecked_into::<web_sys::KeyboardEvent>();

                    if key_event.ctrl_key() && key_event.key() == "s" {
                        let textarea = key_event
                            .target()
                            .unwrap()
                            .unchecked_into::<HtmlTextAreaElement>();
                        let contents = textarea.value();

                        let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::of1(&contents.into())).unwrap();
                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                        let download_link = web_sys::window()
                            .unwrap()
                            .document()
                            .unwrap()
                            .create_element("a")
                            .unwrap()
                            .unchecked_into::<web_sys::HtmlAnchorElement>();
                        download_link.set_href(&url);
                        download_link.set_download("document.txt");
                        download_link.click();

                        key_event.prevent_default();
                    }

                    if key_event.ctrl_key() && key_event.key() == "o" {
                        let input: HtmlInputElement = input_ref.get().unwrap();
                        input.click();
                        key_event.prevent_default();
                    }

                    if key_event.key() == "Tab" {
                        let textarea = key_event
                            .target()
                            .unwrap()
                            .unchecked_into::<HtmlTextAreaElement>();
                        let value = textarea.value();
                        let selection_start = textarea.selection_start().unwrap_or(Some(0)).unwrap();
                        let selection_end = textarea.selection_end().unwrap_or(Some(0)).unwrap();
                        let new_value = format!(
                            "{}    {}",
                            &value[..selection_start as usize],
                            &value[selection_end as usize..]
                        );

                        textarea.set_value(&new_value);
                        let new_cursor_pos = selection_start + 4;
                        textarea
                            .set_selection_range(new_cursor_pos, new_cursor_pos)
                            .unwrap();

                        key_event.prevent_default();
                    }

                    if key_event.key() == "Enter" {
                        let textarea = key_event
                            .target()
                            .unwrap()
                            .unchecked_into::<HtmlTextAreaElement>();
                        let value = textarea.value();
                        let selection_start = textarea.selection_start().unwrap_or(Some(0)).unwrap();
                        let before_cursor = &value[..selection_start as usize];
                        let after_cursor = &value[selection_start as usize..];
                        let current_line = before_cursor
                            .rsplit_once('\n')
                            .map_or(before_cursor, |(_, line)| line);
                        let current_indent = current_line
                            .chars()
                            .take_while(|c| c.is_whitespace())
                            .collect::<String>();
                        let mut new_indent = current_indent.clone();

                        if current_line.trim_end().ends_with('{') {
                            new_indent.push_str("    ");
                        }

                        let indented_newline = format!("\n{}", new_indent);
                        let new_value = format!(
                            "{}{}{}",
                            before_cursor, indented_newline, after_cursor
                        );

                        textarea.set_value(&new_value);
                        let new_cursor_pos = (selection_start as usize + indented_newline.len()) as u32;
                        textarea.set_selection_range(new_cursor_pos, new_cursor_pos).unwrap();

                        key_event.prevent_default();
                    }
                }
            ></textarea>
            <input
                type="file"
                style="display: none;"
                node_ref=input_ref
                on:change=move |event| {
                    let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if let Some(file_list) = input.files() {
                        if file_list.length() > 0 {
                            let file = file_list.item(0).unwrap();
                            let reader = web_sys::FileReader::new().unwrap();

                            let onload = Closure::wrap(Box::new(move |event: web_sys::ProgressEvent| {
                                let target = event.target().unwrap();
                                let reader = target.unchecked_into::<web_sys::FileReader>();

                                if let Some(text) = reader.result().unwrap().as_string() {
                                    let textarea = web_sys::window()
                                        .unwrap()
                                        .document()
                                        .unwrap()
                                        .query_selector(".editor")
                                        .unwrap()
                                        .unwrap()
                                        .unchecked_into::<HtmlTextAreaElement>();
                                    textarea.set_value(&text);
                                }
                            }) as Box<dyn FnMut(_)>);

                            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                            reader.read_as_text(&file).unwrap();
                            onload.forget();
                        }
                    }
                }
            />
        </div>
    }
}
