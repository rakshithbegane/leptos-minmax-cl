use leptos::*;
use leptos_minmax_cl::components::input::button::*;

#[component]
pub fn ButtonDemo(
    cx: Scope, 
) -> impl IntoView {
    let (is_loading, set_is_loading) = create_signal(cx, true);  
            

    view! { cx, 
        <div class="">
            <div class="mt-10" >
                <h2>"Button Colors"</h2>
                <div>
                    <Button text="Default Button" class="ml-5" />
                    <Button text="Primary Button" color=ButtonColor::Primary class="ml-5" />
                    <Button text="Secondary Button" color=ButtonColor::Secondary class="ml-5" />
                    <Button text="Accent Button" color=ButtonColor::Accent class="ml-5" />
                    <Button text="Info Button" color=ButtonColor::Info class="ml-5" />
                    <Button text="Success Button" color=ButtonColor::Success class="ml-5" />
                    <Button text="Warning Button" color=ButtonColor::Warning class="ml-5" />
                    <Button text="Error Button" color=ButtonColor::Error class="ml-5" />
                    <Button text="Ghost Button" color=ButtonColor::Ghost class="ml-5" />
                </div>
            </div>
            <div class="mt-10" >
                <h2>"Button Variants"</h2>
                <div>
                    <Button text="Default Button" class="ml-5" />
                    <Button text="Outline Button" variant=ButtonVariant::Outline class="ml-5" />
                    <Button text="Link Button" variant=ButtonVariant::Link class="ml-5" />
                    <Button text="X" variant=ButtonVariant::Square class="ml-5" />
                    <Button text="-" variant=ButtonVariant::Circle class="ml-5" /> 
                </div>
            </div>
            <div class="mt-10" >
                <h2>"Button Sizes"</h2>
                <div>
                    <Button text="Large Button" size=ButtonSize::Large class="ml-5" />
                    <Button text="Normal Button" class="ml-5" />
                    <Button text="Small Button" size=ButtonSize::Small class="ml-5" />
                    <Button text="Tiny Button" size=ButtonSize::Tiny class="ml-5" /> 
                </div>
            </div>
            <div class="mt-10" >
                <h2>"Button Width"</h2>
                <div>
                    <Button text="Default Width Button" width=ButtonWidth::Default color=ButtonColor::Primary class="ml-5" />
                    <Button text="Wide Button" width=ButtonWidth::Wide color=ButtonColor::Primary class="ml-5" />
                    <Button text="Block Button" width=ButtonWidth::Block color=ButtonColor::Primary class="ml-5 mt-2" />
                </div>
            </div>
            <div class="mt-10" >
                <h2>"Dynamic States"</h2>
                <div>
                    <Button text="Click me!" class=""  color=ButtonColor::Primary  is_loading=is_loading 
                        on:click=move |_| set_is_loading.update(|l| *l = !(*l))  
                    />
                    {is_loading}
                </div>
            </div>
        </div>
    }
}
