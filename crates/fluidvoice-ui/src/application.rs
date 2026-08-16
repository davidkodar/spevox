#[cxx::bridge(namespace = "fluidvoice")]
mod ffi {
    unsafe extern "C++" {
        include!("fluidvoice-ui/src/application.h");

        type FluidVoiceApplication;

        #[cxx_name = "newApplication"]
        fn new_application() -> UniquePtr<FluidVoiceApplication>;

        #[cxx_name = "execApplication"]
        fn exec_application(application: Pin<&mut FluidVoiceApplication>) -> i32;

        #[cxx_name = "isPrimaryInstance"]
        fn is_primary_instance(application: &FluidVoiceApplication) -> bool;
    }
}

pub use ffi::{exec_application, is_primary_instance, new_application};
