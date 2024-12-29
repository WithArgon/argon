use leptos::prelude::*;

#[component]
pub fn not_found() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"Page not found."</h1>
            <h5>Blame this guy:</h5>
            <img src="https://cataas.com/cat" width="auto" height="auto" alt="cat" />
        </div>
    }
}
