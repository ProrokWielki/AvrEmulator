struct TestFixture {}

impl TestFixture {
    pub fn set_up(test_name: String) {
        std::env::set_current_dir(std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))).unwrap();
        std::env::set_current_dir(format!("./tests/{}/", test_name)).expect("Test does not exist");

        std::fs::create_dir_all("./build").unwrap();
        std::env::set_current_dir("./build").unwrap();
    }

    pub fn prepare_cmake() -> bool {
        let cmake = std::process::Command::new("cmake")
            .arg("..")
            .arg("-DCMAKE_TOOLCHAIN_FILE=../../avr.cmake")
            .status()
            .unwrap();

        cmake.success()
    }

    pub fn call_make() -> bool {
        let make = std::process::Command::new("make").status().unwrap();

        make.success()
    }

    pub fn run_tests() -> bool {
        let test = std::process::Command::new("make")
            .arg("test")
            .status()
            .unwrap();

        test.success()
    }
}

#[test]
#[serial_test::serial]
fn test_nop_in_while() {
    TestFixture::set_up("nop_in_while".to_owned());

    assert!(TestFixture::prepare_cmake());
    assert!(TestFixture::call_make());
    assert!(TestFixture::run_tests());
}

#[test]
#[serial_test::serial]
fn test_local_variables() {
    TestFixture::set_up("local_variables".to_owned());

    assert!(TestFixture::prepare_cmake());
    assert!(TestFixture::call_make());
    assert!(TestFixture::run_tests());
}

#[test]
#[serial_test::serial]
fn test_if_statements() {
    TestFixture::set_up("if_statements".to_owned());

    assert!(TestFixture::prepare_cmake());
    assert!(TestFixture::call_make());
    assert!(TestFixture::run_tests());
}
