use leptos::*;
use leptos_meta::*;  


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

    #[prop(default = ButtonVariant::Default)]
    variant: ButtonVariant,

    #[prop(default = ButtonColor::Default)]
    color: ButtonColor,

    #[prop(default = ButtonSize::Normal)]
    size: ButtonSize,

    #[prop(default = ButtonWidth::Default)]
    width: ButtonWidth,

    #[prop(into, default = "".into())]
    class: MaybeSignal<&'static str>,
   
) -> impl IntoView { 
    let cls = format!("btn {} {} {} {} {}", variant.as_str(), color.as_str(), size.as_str(), width.as_str(), class.get());
    view! { cx,
        <button class = cls >
               {text}
        </button>
    }
}

 