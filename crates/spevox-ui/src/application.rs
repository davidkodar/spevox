#[cxx::bridge(namespace = "spevox")]
mod ffi {
    unsafe extern "C++" {
        include!("spevox-ui/src/application.h");

        type SpevoxApplication;

        #[cxx_name = "newApplication"]
        fn new_application() -> UniquePtr<SpevoxApplication>;

        #[cxx_name = "execApplication"]
        fn exec_application(application: Pin<&mut SpevoxApplication>) -> i32;

        #[cxx_name = "isPrimaryInstance"]
        fn is_primary_instance(application: &SpevoxApplication) -> bool;

        #[cxx_name = "refreshApplicationIcon"]
        fn refresh_application_icon(application: Pin<&mut SpevoxApplication>);
    }
}

pub use ffi::{exec_application, is_primary_instance, new_application, refresh_application_icon};
