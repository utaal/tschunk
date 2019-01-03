#[macro_use] extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro;

#[proc_macro]
pub fn dothing(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::ItemStruct = syn::parse(input)
        .expect(concat!("Failed to parse input to `dothing! {",
                        stringify!(...),
                        "}`"));

    let ident = ast.ident;
    let fields = ast.fields.iter().map(|f| {
        let f_ident = &f.ident;
        match f.ty {
            syn::Type::Path(ref path) => eprintln!("{}", stringify!(#path)),
            _ => panic!("nope"),
        }
        let f_ty = &f.ty;
        quote!(#f_ident: #f_ty)
    });

    /*
     *
     */

    let o = proc_macro::TokenStream::from(quote! {
        #[repr(C)]
        struct #ident {
            #(#fields),*
        }

    });
    eprintln!("{}", o);

    o
}
