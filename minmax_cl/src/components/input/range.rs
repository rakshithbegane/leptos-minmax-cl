use leptos::{*, html::AnyElement};
use leptos_meta::*; 
use crate::components::types::*;

impl Color {
    fn range_class(&self) -> String {
        match self {
            Color::Default => "".into(),
            x => format!("range-{}", x.as_str()), 
        }
    }
}

impl Size {
    fn range_class(&self) -> String {
        match self {
            Size::Md => "".into(),
            x => format!("range-{}", x.as_str()),        
        }
    }
}

// Default implementation for `on_change` prop
fn default_on_change(_: usize) {
    // Do nothing by default
}

#[component]
pub fn Range( 
    cx: Scope,
 
    #[prop(into, default = 0.into())]
    min: MaybeSignal<usize>,

    #[prop(into, default = 100.into())]
    max: MaybeSignal<usize>,

    #[prop(into, default = 100.into())]
    value: MaybeSignal<usize>,    

    #[prop(optional, default = None)]
    step: Option<usize>,   
    
    #[prop(into, default = Color::Default.into())]
    color: MaybeSignal<Color>,

    #[prop(into, default = Size::Md.into())]
    size: MaybeSignal<Size>,

    #[prop(into, default = false.into())]
    disabled: MaybeSignal<bool>,

    #[prop(into, default = "".into())]
    class: MaybeSignal<&'static str>,

    #[prop(default = Box::new(default_on_change))]
    on_change: Box<dyn Fn(usize)>,

) -> impl IntoView { 
    
    let disabled_cls = move || if disabled() { "disabled" } else { "" } ;  
    let color_cls = move || color().range_class();
    let size_cls = move || size().range_class();
    
    let all_classes = move || format!("range {} {} {} {}", color_cls(), size_cls(), disabled_cls(), class.get()) ; 

    let step_view: HtmlElement<AnyElement> = match step {
        Some(s) => { 
            let totalBars = (max()/s) + 1;
            let line_span = (0..totalBars)
                .map(|_| view! { cx, <span>"|"</span> } )
                .collect::<Vec<_>>();
            view! { cx, 
                    <div class="w-full flex justify-between text-xs px-2">
                        {line_span}
                    </div> 
                }.into_any()
        },
        None => {
            view! { cx, <span></span> }.into_any()
        },
    };

    view! { cx,
        <>
            <input
                type="range"
                class=all_classes
                min=min
                max=max
                value=value
                step=step
                on:change=move |e| {
                    let value: String = event_target_value(&e);
                    let parsed_value = value.parse::<usize>();
                    let converted_value = match parsed_value {
                        Ok(v) => v,
                        Err(_) => 0,
                    };
                    on_change(converted_value);
                }
            />
            {step_view}
        </>
    }
}