struct TestFixture {}

impl TestFixture {
    pub fn set_up(test_name: String) {
        std::env::set_current_dir(format!("./tests/{}/", test_name)).expect("Test does not exist");

        std::fs::create_dir_all("./build").unwrap();
        std::env::set_current_dir("./build").unwrap();
    }

    pub fn prepare_cmake() -> bool {
        let cmake = std::process::Command::new("cmake")
            .arg("..")
            .arg("-DCMAKE_TOOLCHAIN_FILE=../../avr.cmake")
            .output()
            .unwrap();

        println!("stdout: {}", String::from_utf8(cmake.stdout).unwrap());
        println!("stderr: {}", String::from_utf8(cmake.stderr).unwrap());
        cmake.status.success()
    }

    pub fn call_make() -> bool {
        let make = std::process::Command::new("make").output().unwrap();

        println!("stdout: {}", String::from_utf8(make.stdout).unwrap());
        println!("stderr: {}", String::from_utf8(make.stderr).unwrap());
        make.status.success()
    }

    pub fn run_tests() -> bool {
        let test = std::process::Command::new("make")
            .arg("test")
            .output()
            .unwrap();
        println!("stdout: {}", String::from_utf8(test.stdout).unwrap());
        println!("stderr: {}", String::from_utf8(test.stderr).unwrap());

        test.status.success()
    }
}

#[test]
fn test_nop_in_while() {
    TestFixture::set_up("nop_in_while".to_owned());

    assert!(TestFixture::prepare_cmake());
    assert!(TestFixture::call_make());
    assert!(TestFixture::run_tests());
}
