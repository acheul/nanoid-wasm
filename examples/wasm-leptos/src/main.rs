use leptos::prelude::*;

pub fn main() {
  mount_to_body(|| {
    view! { <App /> }
  })
}

#[component]
pub fn App() -> impl IntoView {

  let (size, set_size) = signal(String::from("21"));
  let size_ = Memo::new(move |_| {
    if let Ok(size) = size.get().parse::<u32>() {
      size
    } else {
      21
    }
  });

  let (ids, set_ids) = signal(Vec::<String>::new());

  let create_ids = move |_| {
    let ids = (0..10).map(|_| nanoid_wasm::nanoid(size_.get())).collect::<Vec<_>>();
    set_ids.set(ids);
  };

  view! {
    <div>
      <span>"size: "</span>
      <input 
        type="text"
        bind:value=(size, set_size)
      />
      <br/>
      <button on:click=create_ids>"Create some Ids with size " {move || size_.get()}</button>
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