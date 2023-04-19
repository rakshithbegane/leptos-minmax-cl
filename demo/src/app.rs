
use leptos::*;
use leptos_meta::*;
use leptos_router::*; 
use leptos_minmax_cl::components::input::button::*;
use leptos_minmax_cl::components::navigation::drawer::*;

#[component]
pub fn App(
    cx: Scope
) -> impl IntoView {
    provide_meta_context(cx);

    let (p, setp) = create_signal(cx, 10);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! { cx, <Home progress=p  /> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(
    cx: Scope,
    #[prop(into)]
    // `Signal<T>` is a wrapper for several reactive types.
    // It can be helpful in component APIs like this, where we
    // might want to take any kind of reactive value
    /// How much progress should be displayed.
    progress: Signal<i32>,
) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0); 

    let menu = move || {
        view! { cx,
            <ul >
                // TODO: the sidebar width css is isn't working. May be tailwind css generation is required
                // <!-- Sidebar content here -->
                <li><a>"Sidebar Item 1"</a></li>
                <li><a>"Sidebar Item 2"</a></li>
            </ul>
        }
    };

            

    view! { cx, 
        <Drawer id="mm-drawer-1" sidebar_menu=menu >
            <label for="mm-drawer-1" class="btn btn-primary drawer-button lg:hidden">"Open drawer"</label>
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
                {progress}
                </button>
                <Button text="Test Text of Button" variant=ButtonVariant::Default color=ButtonColor::Error class="ml-20" />
            </div>
        </Drawer> 
    }
}
