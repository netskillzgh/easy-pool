use proc_macro2::{Ident, Span};
use syn::{parse_macro_input, DeriveInput};

enum KindPool {
    Mutex,
    SegQueue,
    ArrayQueue,
}

#[proc_macro_derive(EasyPoolArayQueue)]
pub fn array_queue(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, vis, .. } = parse_macro_input!(input);
    build(ident, vis, KindPool::ArrayQueue)
}

#[proc_macro_derive(EasyPoolMutex)]
pub fn mutex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, vis, .. } = parse_macro_input!(input);
    build(ident, vis, KindPool::Mutex)
}

#[proc_macro_derive(EasyPoolSegQueue)]
pub fn seg_queue(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, vis, .. } = parse_macro_input!(input);
    build(ident, vis, KindPool::SegQueue)
}

fn build(ident: Ident, vis: syn::Visibility, kind_pool: KindPool) -> proc_macro::TokenStream {
    let name = format!("GLOBAL_{}", ident.to_string().to_uppercase());
    let name = Ident::new(&name, Span::call_site());

    let result = match kind_pool {
        KindPool::Mutex => quote::quote! {
            #vis static #name: easy_pool::Lazy<
            std::sync::Arc<easy_pool::PoolMutex<#ident>>>
         = easy_pool::Lazy::new(|| {
            let pool = std::sync::Arc::new(easy_pool::PoolMutex::with_config(1024, 1024));
            pool
          });
        },
        KindPool::SegQueue => quote::quote! {
            #vis static #name: easy_pool::Lazy<
            std::sync::Arc<easy_pool::PoolSegQueue<#ident>>>
         = easy_pool::Lazy::new(|| {
            let pool = std::sync::Arc::new(easy_pool::PoolSegQueue::new(1024));
            pool
          });
        },
        KindPool::ArrayQueue => quote::quote! {
            #vis static #name: easy_pool::Lazy<
            std::sync::Arc<easy_pool::PoolArrayQueue<#ident>>>
         = easy_pool::Lazy::new(|| {
            let pool = std::sync::Arc::new(easy_pool::PoolArrayQueue::new(1024));
            pool
          });
        },
    };

    let imp = quote::quote! {
        impl #ident {
            #vis fn create_with<F>(f: F) -> easy_pool::PoolObjectContainer<#ident>
           where
               F: FnOnce() -> #ident,
           {
               let pool = #name.create_with(|| f());

               pool
           }

           #vis fn create() -> easy_pool::PoolObjectContainer<#ident>
           {
               let pool = #name.create();

               pool
           }
       }
    };
    quote::quote! {
        #result

        #imp
    }
    .into()
}
