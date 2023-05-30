use leptos::*;

#[derive(Debug, Clone)]
pub enum ButtonColor {
    Default,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
    Ghost,
}

impl ButtonColor {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonColor::Default => "",
            ButtonColor::Primary => "btn-primary",
            ButtonColor::Secondary => "btn-secondary",
            ButtonColor::Accent => "btn-accent",
            ButtonColor::Info => "btn-info",
            ButtonColor::Success => "btn-success",
            ButtonColor::Warning => "btn-warning",
            ButtonColor::Error => "btn-error",
            ButtonColor::Ghost => "btn-ghost",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ButtonVariant {
    Default,
    Outline,
    Link,
    Square,
    Circle,
}

impl ButtonVariant {
    fn as_str(&self) -> &'static str {
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
pub enum ButtonSize {
    Normal,
    Large,
    Small,
    Tiny,
}

impl ButtonSize {
    fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Normal => "",
            ButtonSize::Large => "btn-lg",
            ButtonSize::Small => "btn-sm",
            ButtonSize::Tiny => "btn-xs",        
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
    fn as_str(&self) -> &'static str {
        match self {
            ButtonWidth::Default => "",
            ButtonWidth::Wide => "btn-wide",
            ButtonWidth::Block => "btn-block",        
        }
    }
}


#[component]
pub fn Button( 
    cx: Scope,
 
    #[prop(into)]
    text: MaybeSignal<&'static str>,
    
    #[prop(into, default = false.into())]
    disabled: MaybeSignal<bool>,
    
    #[prop(into, default = false.into())]
    is_loading: MaybeSignal<bool>,

    #[prop(into, default = ButtonVariant::Default.into())]
    variant: MaybeSignal<ButtonVariant>,

    #[prop(into, default = ButtonColor::Default.into())]
    color: MaybeSignal<ButtonColor>,

    #[prop(into, default = ButtonSize::Normal.into())]
    size: MaybeSignal<ButtonSize>,

    #[prop(into, default = ButtonWidth::Default.into())]
    width: MaybeSignal<ButtonWidth>,

    #[prop(into, default = "".into())]
    class: MaybeSignal<&'static str>,
   
) -> impl IntoView { 
    
    let disabled_cls = move || if disabled() { " btn-disabled" } else { "" } ; 
    let loading_cls = move || if is_loading() { "loading  btn-disabled" } else { "" };
    let variant_cls = move || variant().as_str();
    let color_cls = move || color().as_str();
    let size_cls = move || size().as_str();
    let width_cls = move || width().as_str();
    let cls = move || format!("btn {} {} {} {} {} {} {}", variant_cls(), color_cls(), size_cls(), width_cls(), loading_cls(), disabled_cls(), class.get()) ; 
    
    view! { cx, <button class=cls>{text}</button> }
}

 