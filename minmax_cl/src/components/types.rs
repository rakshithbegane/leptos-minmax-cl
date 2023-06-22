
#[derive(Debug, Clone)]
pub enum Color {
    Default,
    Primary,
    Secondary,
    Accent,
    Ghost,
    Info,
    Success,
    Warning,
    Error,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
        match self {
            Color::Default => "default",
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Accent => "accent",
            Color::Info => "info",
            Color::Success => "success",
            Color::Warning => "warning",
            Color::Error => "error",
            Color::Ghost => "ghost",
        }
    }
}


#[derive(Debug, Clone)]
pub enum Size {
    Xs,
    Sm,
    Lg,
    Md,
}

impl Size {
    pub fn as_str(&self) -> &'static str {
        match self {
            Size::Md => "md",
            Size::Lg => "lg",
            Size::Sm => "sm",
            Size::Xs => "xs",        
        }
    }
}
    