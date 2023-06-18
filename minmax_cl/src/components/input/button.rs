use leptos::*;
use leptos_icons::*;
use crate::components::types::*;


#[derive(Debug, Clone)]
pub enum ButtonVariant {
    Default,
    Outline,
    Link,
    Square,
    Circle,
}

impl ButtonVariant {
    fn btn_class(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "",
            ButtonVariant::Outline => "btn-outline",
            ButtonVariant::Link => "btn-link",
            ButtonVariant::Square => "btn-square",
            ButtonVariant::Circle => "btn-circle",
        }
    }
}


#[derive(Debug, Clone)]
pub enum ButtonWidth {
    Default,
    Wide,
    Block,
}

impl ButtonWidth {
    fn btn_class(&self) -> &'static str {
        match self {
            ButtonWidth::Default => "",
            ButtonWidth::Wide => "btn-wide",
            ButtonWidth::Block => "btn-block",        
        }
    }
}


impl Color {
    fn btn_class(&self) -> String {
        match self {
            Color::Default => "".into(),
            x => format!("btn-{}", x.as_str()), 
        }
    }
}

impl Size {
    fn btn_class(&self) -> String {
        match self {
            Size::Md => "".into(),
            x => format!("btn-{}", x.as_str()),        
        }
    }
}

#[component]
pub fn Button( 
    cx: Scope,
 
    #[prop(into, default = "".into())]
    text: MaybeSignal<&'static str>,
    
    #[prop(into, default = false.into())]
    disabled: MaybeSignal<bool>,
    
    #[prop(into, default = false.into())]
    is_loading: MaybeSignal<bool>,

    #[prop(into, default = ButtonVariant::Default.into())]
    variant: MaybeSignal<ButtonVariant>,

    #[prop(into, default = Color::Default.into())]
    color: MaybeSignal<Color>,

    #[prop(into, default = Size::Md.into())]
    size: MaybeSignal<Size>,

    #[prop(into, default = ButtonWidth::Default.into())]
    width: MaybeSignal<ButtonWidth>,

    #[prop(into, default = "".into())]
    class: MaybeSignal<&'static str>,

    #[prop(into, optional, default = None.into())]
    href: MaybeSignal<Option<&'static str>>,

    #[prop(into, optional, default = None.into())]
    icon: MaybeSignal<Option<Icon>>,

    #[prop(into, optional, default = None.into())]
    start_icon: MaybeSignal<Option<Icon>>,

    #[prop(into, optional, default = None.into())]
    end_icon: MaybeSignal<Option<Icon>>,

) -> impl IntoView { 
    
    let disabled_cls = move || if disabled() { "btn-disabled" } else { "" } ; 
    let loading_cls = move || if is_loading() { "loading btn-disabled" } else { "" };
    let variant_cls = move || variant().btn_class();
    let color_cls = move || color().btn_class();
    let size_cls = move || size().btn_class();
    let width_cls = move || width().btn_class();

    let all_classes = move || format!("btn {} {} {} {} {} {} {}", variant_cls(), color_cls(), size_cls(), width_cls(), loading_cls(), disabled_cls(), class.get()) ; 


    let end_icon_view = move || {
        match end_icon() {
            Some(icon) => Some(Icon(cx, IconProps{ icon, height: None, width: None, class: None, style: Some(String::from("margin-left: 0.5rem;")) })),
            None => None,
        }
    };

    let start_icon_view = move || {
        match start_icon() {
            Some(icon) => Some(Icon(cx, IconProps{ icon, height: None, width: None, class: None, style: Some(String::from("margin-right: 0.5rem;")) })),
            None => None,
        }
    };

    let icon_view = move || {
        match icon() {
            Some(icon) => Some(Icon(cx, IconProps{ icon, height: None, width: None, class: None, style: None })),
            None => None,
        }
    };



    view! { cx,
        <>
            {match href() {
                Some(href) => {
                    view! { cx,
                        <a href=href class=all_classes>
                            {start_icon_view}
                            {icon_view}
                            {text}
                            {end_icon_view}
                        </a>
                    }
                        .into_any()
                }
                None => {
                    view! { cx,
                        <button class=all_classes>
                            {start_icon_view} 
                            {icon_view} 
                            {text} 
                            {end_icon_view}
                        </button>
                    }
                        .into_any()
                }
            }}
        </>
    }
}

 