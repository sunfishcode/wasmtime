use crate::wasi::{clocks::monotonic_clock, io::poll};

wit_bindgen::generate!({
    path: "../../wasi/wit",
    world: "wasmtime:wasi/command-extended",
});

fn main() {
    // Sleep ten milliseconds.  Note that we call the relevant host functions directly rather than go through
    // libstd, since we want to ensure we're calling `monotonic_clock::subscribe` with an `absolute` parameter of
    // `true`, which libstd won't necessarily do (but which e.g. CPython _will_ do).
    eprintln!("calling subscribe");
    let p = monotonic_clock::subscribe(monotonic_clock::now() + 10_000_000, true);
    dbg!(&p as *const _);
    let list = &[&p];
    dbg!(list.as_ptr());
    eprintln!("calling poll");
    poll_list(list);
    eprintln!("done");
}

/// FIXME: This is a copy of the `poll_list` bindings generated with a
/// wit-bindgen with this fix:
/// <https://github.com/bytecodealliance/wit-bindgen/pull/670>
///
/// One that PR is in a published release, delete this code and use the
/// bindings.
///
/// Poll for completion on a set of pollables.
///
/// This function takes a list of pollables, which identify I/O sources of
/// interest, and waits until one or more of the events is ready for I/O.
///
/// The result `list<u32>` contains one or more indices of handles in the
/// argument list that is ready for I/O.
///
/// If the list contains more elements than can be indexed with a `u32`
/// value, this function traps.
///
/// A timeout can be implemented by adding a pollable from the
/// wasi-clocks API to the list.
///
/// This function does not return a `result`; polling in itself does not
/// do any I/O so it doesn't fail. If any of the I/O sources identified by
/// the pollables has an error, it is indicated by marking the source as
/// being reaedy for I/O.
pub fn poll_list(in_: &[&poll::Pollable]) -> wit_bindgen::rt::vec::Vec<u32> {
    #[allow(unused_imports)]
    use wit_bindgen::rt::{alloc, string::String, vec::Vec};
    unsafe {
        #[repr(align(4))]
        struct RetArea([u8; 8]);
        let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
        let vec0 = in_;
        let len0 = vec0.len() as i32;
        let layout0 = alloc::Layout::from_size_align_unchecked(vec0.len() * 4, 4);
        let result0 = if layout0.size() != 0 {
            let ptr = alloc::alloc(layout0);
            if ptr.is_null() {
                alloc::handle_alloc_error(layout0);
            }
            ptr
        } else {
            ::core::ptr::null_mut()
        };
        for (i, e) in vec0.into_iter().enumerate() {
            let base = result0 as i32 + (i as i32) * 4;
            {
                *((base + 0) as *mut i32) = (e).handle() as i32;
            }
        }
        let ptr1 = ret_area.as_mut_ptr() as i32;
        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "wasi:io/poll")]
        extern "C" {
            #[link_name = "poll-list"]
            fn wit_import(_: i32, _: i32, _: i32);
        }

        #[cfg(not(target_arch = "wasm32"))]
        fn wit_import(_: i32, _: i32, _: i32) {
            unreachable!()
        }
        wit_import(result0 as i32, len0, ptr1);
        let l2 = *((ptr1 + 0) as *const i32);
        let l3 = *((ptr1 + 4) as *const i32);
        let len4 = l3 as usize;
        if layout0.size() != 0 {
            alloc::dealloc(result0, layout0);
        }
        Vec::from_raw_parts(l2 as *mut _, len4, len4)
    }
}
