use leptos::*;
use leptos_icons::*;
use leptos_minmax_cl::components::input::range::*;
use leptos_minmax_cl::components::types::*;



#[component]
pub fn RangeDemo(
    cx: Scope, 
) -> impl IntoView {
    
    let (value, set_value) = create_signal(cx, 25);
    // let range_color = MaybeSignal::derive(cx, move || if is_loading() { Color::Default  } else { Color::Secondary });   
    
    
    view! { cx,
        <div class="">
            <div class="mt-10">
                <h2>"Range Colors"</h2>
                <div>
                    <Range value=10 />
                    <Range value=90 color=Color::Primary />
                    <Range value=20 color=Color::Secondary />
                    <Range value=80 color=Color::Accent />
                    <Range value=30 color=Color::Info />
                    <Range value=70 color=Color::Success />
                    <Range value=40 color=Color::Warning />
                    <Range value=60 color=Color::Error />
                    <Range value=50 color=Color::Ghost />
                </div>
            </div> 
            <div class="mt-10">
                <h2>"Range Sizes"</h2>
                <div>
                    <Range value=20 size=Size::Lg />
                    <Range value=30 size=Size::Md />
                    <Range value=40 size=Size::Sm />
                    <Range value=50 size=Size::Xs />
                </div>
            </div>  
            <div class="mt-10">
                <h2>"Dynamic States"</h2>
                <div>
                    <Range value={value.get() * 2} color=Color::Success on_change={Box::new(move |v| set_value.update(|x| *x = v / 2))} />
                    <Range value=value size=Size::Xs />
                    {value()}
                </div>
            </div>
        </div>
    }

}
