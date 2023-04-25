use leptos::*;


#[component]
pub fn Drawer<F, IV>( 
    cx: Scope,
    id: &'static str,

    //TODO: add props for mobile,
    //TODO: add open orientation 
    //TODO: add toggle sidebar vs navbar

    // sidebar 
    sidebar_menu: F,
    
    // rest of the page content
    children: Children,
) -> impl IntoView 
where 
    F: Fn() -> IV, 
    IV: IntoView, 
{  
    view! { cx,
        <div class="drawer drawer-mobile">
            <input id=id type="checkbox" class="drawer-toggle" />
            <div class="drawer-content flex flex-col">
                // <!-- Page content here -->
                {children(cx)}
            </div> 
            <div class="drawer-side">
                <label for=id class="drawer-overlay"></label> 
                <div class="menu p-4 w-80 bg-base-100 text-base-content" style="width: 80%" >
                    {sidebar_menu()}
                </div>
            </div>
        </div>
    }
}
