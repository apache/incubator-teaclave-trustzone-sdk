extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// # Examples
/// ``` no_run
/// #[ta_crate]
/// fn ta_crate() -> Result<()> { }
/// ```
#[proc_macro_attribute]
pub fn ta_create(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;

    quote!(
        #[no_mangle]
        pub extern "C" fn TA_CreateEntryPoint() -> optee_utee_sys::TEE_Result {
            match #ident() {
                Ok(_) => optee_utee_sys::TEE_SUCCESS,
                Err(e) => e.raw_code()
            }
        }

        #f
    )
    .into()
}

/// # Examples
/// ``` no_run
/// #[ta_destroy]
/// fn ta_destroy() { }
/// ```
#[proc_macro_attribute]
pub fn ta_destroy(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;

    quote!(
        #[no_mangle]
        pub extern "C" fn TA_DestroyEntryPoint() {
            #ident();
        }

        #f
    )
    .into()
}

/// # Examples
/// ``` no_run
/// #[ta_open_session]
/// fn open_session(params: &mut Parameters, sess_ctx: *mut *mut T) -> Result<()> { }
///
/// #[ta_open_session]
/// fn open_session(params: &mut Parameters) -> Result<()> { }
/// ```
#[proc_macro_attribute]
pub fn ta_open_session(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;

    match f.decl.inputs.len() {
        1 => quote!(
            #[no_mangle]
            pub extern "C" fn TA_OpenSessionEntryPoint(
                param_types: libc::uint32_t,
                params: &mut [optee_utee_sys::TEE_Param; 4],
                sess_ctx: *mut *mut libc::c_void,
            ) -> optee_utee_sys::TEE_Result {
                let mut parameters = Parameters::new(params, param_types);
                match #ident(&mut parameters) {
                    Ok(_) => optee_utee_sys::TEE_SUCCESS,
                    Err(e) => e.raw_code()
                }
            }

            #f
        )
        .into(),
        2 => {
            let input_types: Vec<_> = f
                .decl
                .inputs
                .iter()
                .map(|arg| match arg {
                    &syn::FnArg::Captured(ref val) => &val.ty,
                    _ => unreachable!(),
                })
                .collect();
            let t = input_types.last().unwrap();

            quote!(
                #[no_mangle]
                pub extern "C" fn TA_OpenSessionEntryPoint(
                    param_types: libc::uint32_t,
                    params: &mut [optee_utee_sys::TEE_Param; 4],
                    sess_ctx: *mut *mut libc::c_void,
                ) -> optee_utee_sys::TEE_Result {
                    let mut parameters = Parameters::new(params, param_types);
                    match #ident(&mut parameters, sess_ctx as #t) {
                        Ok(_) => optee_utee_sys::TEE_SUCCESS,
                        Err(e) => e.raw_code()
                    }
                }

                #f
            )
            .into()
        }
        _ => unreachable!(),
    }
}

/// # Examples
/// ``` no_run
/// #[ta_close_session]
/// fn close_session(sess_ctx: &mut T) { }
///
/// #[ta_close_session]
/// fn close_session() { }
/// ```
#[proc_macro_attribute]
pub fn ta_close_session(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;
    match f.decl.inputs.len() {
        0 => quote!(
            #[no_mangle]
            pub extern "C" fn TA_CloseSessionEntryPoint(sess_ctx: *mut libc::c_void) {
                #ident();
            }

            #f
        )
        .into(),
        1 => {
            let input_types: Vec<_> = f
                .decl
                .inputs
                .iter()
                .map(|arg| match arg {
                    &syn::FnArg::Captured(ref val) => &val.ty,
                    _ => unreachable!(),
                })
                .collect();
            let t = match input_types.first().unwrap() {
                &syn::Type::Reference(ref r) => &r.elem,
                _ => unreachable!(),
            };

            quote!(
                #[no_mangle]
                pub extern "C" fn TA_CloseSessionEntryPoint(sess_ctx: *mut libc::c_void) {
                    if sess_ctx.is_null() {
                        panic!("sess_ctx is null");
                    }
                    let mut b = unsafe {Box::from_raw(sess_ctx as *mut #t)};
                    #ident(&mut b);
                    drop(b);
                }

                #f
            )
            .into()
        }
        _ => unreachable!(),
    }
}

/// # Examples
/// ``` no_run
/// #[ta_invoke_command]
/// fn invoke_command(sess_ctx: &mut T, cmd_id: u32, params: &mut Parameters) -> Result<()> { }
///
/// #[ta_invoke_command]
/// fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> { }
/// ```
#[proc_macro_attribute]
pub fn ta_invoke_command(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;
    match f.decl.inputs.len() {
        2 => quote!(
            #[no_mangle]
            pub extern "C" fn TA_InvokeCommandEntryPoint(
                sess_ctx: *mut libc::c_void,
                cmd_id: u32,
                param_types: libc::uint32_t,
                params: &mut [optee_utee_sys::TEE_Param; 4],
            ) -> optee_utee_sys::TEE_Result {
                let mut parameters = Parameters::new(params, param_types);
                match #ident(cmd_id, &mut parameters) {
                    Ok(_) => {
                        optee_utee_sys::TEE_SUCCESS
                    },
                    Err(e) => e.raw_code()
                }
            }

            #f
        )
        .into(),
        3 => {
            let input_types: Vec<_> = f
                .decl
                .inputs
                .iter()
                .map(|arg| match arg {
                    &syn::FnArg::Captured(ref val) => &val.ty,
                    _ => unreachable!(),
                })
                .collect();
            let t = match input_types.first().unwrap() {
                &syn::Type::Reference(ref r) => &r.elem,
                _ => unreachable!(),
            };

            quote!(
                #[no_mangle]
                pub extern "C" fn TA_InvokeCommandEntryPoint(
                    sess_ctx: *mut libc::c_void,
                    cmd_id: u32,
                    param_types: libc::uint32_t,
                    params: &mut [optee_utee_sys::TEE_Param; 4],
                ) -> optee_utee_sys::TEE_Result {
                    if sess_ctx.is_null() {
                        return optee_utee_sys::TEE_ERROR_SECURITY;
                    }
                    let mut parameters = Parameters::new(params, param_types);
                    let mut b = unsafe {Box::from_raw(sess_ctx as *mut #t)};
                    match #ident(&mut b, cmd_id, &mut parameters) {
                        Ok(_) => {
                            std::mem::forget(b);
                            optee_utee_sys::TEE_SUCCESS
                        },
                        Err(e) => e.raw_code()
                    }
                }

                #f
            )
            .into()
        }
        _ => unreachable!(),
    }
}
