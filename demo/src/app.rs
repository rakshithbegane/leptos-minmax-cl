mod button_demo;

use leptos::*;
use leptos_meta::*;
use leptos_router::*; 
use leptos_minmax_cl::components::navigation::drawer::*;


use button_demo::*; 

#[component]
pub fn App(
    cx: Scope
) -> impl IntoView {
    provide_meta_context(cx); 

    let menu = move || {
        view! { cx,
            <ul class="menu bg-base-100 w-56" >
                // TODO: the sidebar width css is isn't working. May be tailwind css generation is required
                // <!-- Sidebar content here -->
                <li><a href="/" >"Home"</a></li>
                // <li><A href="/" >"Sidebar Item 1"</A></li>
                <li><a href="/input/button" >"Button"</a></li>
            </ul>
        }
    };

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Drawer id="mm-drawer-1" sidebar_menu=menu >
                <label for="mm-drawer-1" class="btn btn-primary drawer-button sm:hidden">"Open drawer"</label>
                <Routes>
                    <Route path="" view=  move |cx| view! { cx, <Home   /> }/>
                    <Route path="/input/button" view=  move |cx| view! { cx, <ButtonDemo  /> }/>
                </Routes>
            </Drawer>  
        </Router>
    }
}

#[component]
fn Home(
    cx: Scope, 
) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);  
            

    view! { cx, 
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | " 
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
                " | Some more text " 
            </button> 
        </div>
    }
}

