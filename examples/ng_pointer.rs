use leptos::*;
use lerni::ng::*;

#[component]
pub fn Pointer(cx: Scope) -> impl IntoView {
    let (pointer, set_pointer) = create_signal(cx, true);
    let on_click = move |_| set_pointer.set(!pointer.get());

    view! { cx,
        <Slide pointer=pointer.into()>
            <Button text=|_| "Pointer ON/OFF" on_click=on_click/>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(Pointer);
}
