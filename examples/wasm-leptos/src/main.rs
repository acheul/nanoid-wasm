use leptos::prelude::*;

pub fn main() {
  mount_to_body(|| {
    view! { <App /> }
  })
}

#[component]
pub fn App() -> impl IntoView {

  let (size, set_size) = signal(String::from("21"));
  let size_ = Memo::new(move |_| size.get().parse::<u32>().unwrap_or(21).min(100).max(2));

  let (ids, set_ids) = signal(Vec::<String>::new());

  // create nanoids
  let create_ids = move |_| {
    let ids = (0..10).map(|_| nanoid_wasm::nanoid!(size_.get())).collect::<Vec<_>>();
    set_ids.set(ids);
  };

  view! {
    <div>
      <div>
        <span>"size(2~100):  "</span>
        <input 
          type="text"
          style="width: 80px"
          bind:value=(size, set_size)
        />
        <br/>
        <button on:click=create_ids style="font-weight: bold;">"Create Random Ids with size " {move || size_.get()}</button>
      </div>
      <div>
        <For
          each=move || ids.get()
          key=|id| id.clone()
          children=move |id| {
            view! {
              <div>{id}</div>
            }
          }
        />
      </div>
    </div>
  }
}