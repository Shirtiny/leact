use dioxus::prelude::Event;

pub fn prevent<T>(e: Event<T>) {
    e.prevent_default();
    e.stop_propagation();
}
