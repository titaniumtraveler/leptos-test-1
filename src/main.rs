use leptos::*;

fn main() {
    let text = create_rw_signal("123".to_string());
    mount_to_body(move || {
        view! {
            <div inner_html = move || { text }/>
        }
    })
}
