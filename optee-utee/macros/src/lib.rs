extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
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
/// #[ta_destory]
/// fn ta_destory() { }
/// ```
#[proc_macro_attribute]
pub fn ta_destory(_args: TokenStream, input: TokenStream) -> TokenStream {
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
/// fn open_session(_params: &mut Parameters, _sess_ctx: *mut *mut libc::c_void) -> Result<()> { }
/// ```
#[proc_macro_attribute]
pub fn ta_open_session(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;

    quote!(
        #[no_mangle]
        pub extern "C" fn TA_OpenSessionEntryPoint(
            param_types: libc::uint32_t,
            params: &mut [optee_utee_sys::TEE_Param; 4],
            sess_ctx: *mut *mut libc::c_void,
        ) -> optee_utee_sys::TEE_Result {
            let mut parameters = Parameters::new(params, param_types);
            match #ident(&mut parameters, sess_ctx) {
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
/// #[ta_close_session]
/// fn close_session(_sess_ctx: *mut *mut libc::c_void) { }
/// ```
#[proc_macro_attribute]
pub fn ta_close_session(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;

    quote!(
        #[no_mangle]
        pub extern "C" fn TA_CloseSessionEntryPoint(sess_ctx: *mut *mut libc::c_void) {
            #ident(sess_ctx)
        }

        #f
    )
    .into()
}

/// # Examples
/// ``` no_run
/// #[ta_invoke_command]
/// fn invoke_command(_sess_ctx: *mut libc::c_void, cmd_id: u32, params: &mut Parameters) -> Result<()> { }
/// ```
#[proc_macro_attribute]
pub fn ta_invoke_command(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as syn::ItemFn);
    let ident = &f.ident;

    quote!(
        #[no_mangle]
        pub extern "C" fn TA_InvokeCommandEntryPoint(
            sess_ctx: *mut libc::c_void,
            cmd_id: u32,
            param_types: libc::uint32_t,
            params: &mut [optee_utee_sys::TEE_Param; 4],
        ) -> optee_utee_sys::TEE_Result {
            let mut parameters = Parameters::new(params, param_types);
            match #ident(sess_ctx, cmd_id, &mut parameters) {
                Ok(_) => optee_utee_sys::TEE_SUCCESS,
                Err(e) => e.raw_code()
            }
        }

        #f
    )
    .into()
}
