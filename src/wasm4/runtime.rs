pub trait Runtime {
    /// Called at the start of the game, before the first update.
    fn start() -> Self;
    /// Called every frame, about 60 times per second.
    fn update(&mut self);
}

#[macro_export]
macro_rules! main {
    ($runtime:ty) => {
        static mut RUNTIME: core::mem::MaybeUninit<$runtime> = core::mem::MaybeUninit::uninit();

        #[no_mangle]
        unsafe extern "C" fn start() {
            // SAFETY: This call is described inside the doc comments for `Resources::new()`
            let rt = <$runtime as $crate::wasm4::runtime::Runtime>::start();
            // SAFETY: WASM-4 is single-threaded
            RUNTIME = core::mem::MaybeUninit::new(rt);
        }

        #[no_mangle]
        unsafe extern "C" fn update() {
            // SAFETY: WASM-4 is single-threaded. `update()` function is called after start by WASM-4 runtime
            let rt = RUNTIME.assume_init_mut();
            <$runtime as $crate::wasm4::runtime::Runtime>::update(rt);
        }
    };
}
