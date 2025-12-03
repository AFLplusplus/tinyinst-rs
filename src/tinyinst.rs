use alloc::{ffi::CString, string::String, vec::Vec};
use core::{
    ffi::c_char,
    fmt::{self, Debug, Formatter},
};
#[cxx::bridge]
pub mod common {
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("common.h");
        #[must_use]
        fn GetCurTime() -> u64;
    }
}

#[allow(clippy::expl_impl_clone_on_copy)]
#[cxx::bridge]
pub mod litecov {
    #[derive(Debug, Copy, Clone)]
    #[repr(u32)]
    enum RunResult {
        OK,
        CRASH,
        HANG,
        OTHER_ERROR,
    }

    #[allow(missing_debug_implementations)]
    unsafe extern "C++" {
        // for constructors.
        include!("shim.h");
        include!("tinyinstinstrumentation.h");
        include!("aflcov.h");

        type ModuleCovData;
        pub fn ClearInstrumentationData(self: Pin<&mut ModuleCovData>);
        pub fn ClearCmpCoverageData(self: Pin<&mut ModuleCovData>);

        type Coverage;
        type ModuleCoverage;

        #[must_use]
        pub fn coverage_new() -> UniquePtr<Coverage>;

        pub unsafe fn get_coverage_map(
            bitmap: *mut u8,
            map_size: usize,
            coverage: Pin<&mut Coverage>,
        );

        // TinyinstInstrumentation
        type TinyInstInstrumentation;
        #[must_use]
        pub fn tinyinstinstrumentation_new() -> UniquePtr<TinyInstInstrumentation>;

        type RunResult;
        // type Coverage;
        #[allow(clippy::similar_names)]
        pub unsafe fn Init(
            self: Pin<&mut TinyInstInstrumentation>,
            argc: i32,
            argv: *mut *mut c_char,
        );
        #[allow(clippy::similar_names)]
        pub unsafe fn Run(
            self: Pin<&mut TinyInstInstrumentation>,
            argc: i32,
            argv: *mut *mut c_char,
            init_timeout: u32,
            timeout: u32,
        ) -> RunResult;

        #[allow(clippy::similar_names)]
        pub unsafe fn RunWithCrashAnalysis(
            self: Pin<&mut TinyInstInstrumentation>,
            argc: i32,
            argv: *mut *mut c_char,
            init_timeout: u32,
            timeout: u32,
        ) -> RunResult;

        pub fn CleanTarget(self: Pin<&mut TinyInstInstrumentation>);
        #[must_use]
        pub fn HasNewCoverage(self: Pin<&mut TinyInstInstrumentation>) -> bool;

        pub fn GetCoverage(
            self: Pin<&mut TinyInstInstrumentation>,
            coverage: Pin<&mut Coverage>,
            afl_coverage: &mut Vec<u64>,
            clear_coverage: bool,
        );
        pub fn ClearCoverage(self: Pin<&mut TinyInstInstrumentation>);
        pub fn IgnoreCoverage(
            self: Pin<&mut TinyInstInstrumentation>,
            coverage: Pin<&mut Coverage>,
        );

        // Testing AFLCOV
        // type AFLCov;
        // pub unsafe fn aflcov_new(coverage: *mut u8, capacity: usize) -> UniquePtr<AFLCov>;
        // pub fn add_coverage(self: Pin<&mut AFLCov>, addr: u8);
    }
}

use cxx::UniquePtr;
impl litecov::TinyInstInstrumentation {
    #[must_use]
    pub fn new() -> UniquePtr<litecov::TinyInstInstrumentation> {
        litecov::tinyinstinstrumentation_new()
    }
}

impl litecov::Coverage {
    #[must_use]
    pub fn new() -> UniquePtr<litecov::Coverage> {
        litecov::coverage_new()
    }
}

pub struct TinyInst {
    tinyinst_ptr: UniquePtr<litecov::TinyInstInstrumentation>,
    program_args_cstr: Vec<CString>,
    program_args_ptr: Vec<*mut c_char>,
    coverage_ptr: UniquePtr<litecov::Coverage>,
    timeout: u32,
}

impl Debug for TinyInst {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TinyInst")
            .field("program_args_cstr", &self.program_args_cstr)
            .field("timeout", &self.timeout)
            .finish_non_exhaustive()
    }
}

impl TinyInst {
    #[must_use]
    pub unsafe fn new(tinyinst_args: &[String], program_args: &[String], timeout: u32) -> TinyInst {
        // commented out by domenukk:
        // a) would require call to a libc, c++ or rust std fn
        // b) The program could actually be in the PATH, so, not accessible as file.
        /*
        if !Path::new(format!("{}", program_args[0]).as_str()).exists() {
            panic!("{} does not exist", program_args[0]);
        }*/
        let mut tinyinst_ptr = litecov::TinyInstInstrumentation::new();

        let tinyinst_args_cstr: Vec<CString> = tinyinst_args
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();

        let mut tinyinst_args_ptr: Vec<*mut c_char> = tinyinst_args_cstr
            .iter()
            .map(|arg| arg.as_ptr().cast_mut())
            .collect();
        tinyinst_args_ptr.push(core::ptr::null_mut());

        // Init TinyInst with Tinyinst arguments.
        tinyinst_ptr.pin_mut().Init(
            i32::try_from(tinyinst_args.len()).unwrap(),
            tinyinst_args_ptr.as_mut_ptr(),
        );

        let program_args_cstr: Vec<CString> = program_args
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap())
            .collect();

        let mut program_args_ptr: Vec<*mut c_char> = program_args_cstr
            .iter()
            .map(|arg| arg.as_ptr().cast_mut())
            .collect();
        program_args_ptr.push(core::ptr::null_mut());

        TinyInst {
            tinyinst_ptr,
            program_args_cstr,
            program_args_ptr,
            timeout,
            coverage_ptr: litecov::Coverage::new(),
        }
    }

    pub unsafe fn run(&mut self) -> litecov::RunResult {
        self.tinyinst_ptr.pin_mut().Run(
            i32::try_from(self.program_args_cstr.len()).unwrap(),
            self.program_args_ptr.as_mut_ptr(),
            self.timeout,
            self.timeout,
        )
    }

    // pub unsafe fn bitmap_coverage(
    //     &mut self,
    //     bitmap: *mut u8,
    //     map_size: usize,
    //     clear_coverage: bool,
    // ) {
    //     self.tinyinst_ptr
    //         .pin_mut()
    //         .GetCoverage(self.coverage_ptr.pin_mut(), clear_coverage);
    //     litecov::get_coverage_map(bitmap, map_size, self.coverage_ptr.pin_mut());
    // }

    pub fn vec_coverage(&mut self, afl_coverage: &mut Vec<u64>, clear_coverage: bool) {
        // Clear coverage if there was previous coverage
        afl_coverage.clear();
        self.tinyinst_ptr.pin_mut().GetCoverage(
            self.coverage_ptr.pin_mut(),
            afl_coverage,
            clear_coverage,
        );
        // This will mark coverage we have seen as already seen coverage and won't report it again.
        self.ignore_coverage();
    }
    fn ignore_coverage(&mut self) {
        self.tinyinst_ptr
            .pin_mut()
            .IgnoreCoverage(self.coverage_ptr.pin_mut());
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use std::{
        fs::File,
        io::{Seek, Write},
        path::Path,
        string::ToString,
    };

    #[cfg(target_os = "windows")]
    const TEST_FILENAME: &str = "test.exe";
    #[cfg(any(target_vendor = "apple", target_os = "linux"))]
    const TEST_FILENAME: &str = "test";

    #[cfg(any(target_vendor = "apple", target_os = "linux"))]
    const TEST_PATH: &str = "test/build/";
    #[cfg(target_os = "windows")]
    const TEST_PATH: &str = "test/build/Debug/";

    #[test]
    fn tinyinst_ok() {
        let tinyinst_args = vec!["-instrument_module".to_string(), TEST_FILENAME.to_string()];
        // Create file to test.
        let mut file = File::create("./test/test_file.txt").unwrap();
        file.write_all(b"test1").unwrap();

        let program_args = vec![
            Path::new(TEST_PATH)
                .join(TEST_FILENAME)
                .display()
                .to_string(),
            "./test/test_file.txt".to_string(),
        ];
        let mut coverage = Vec::new();

        unsafe {
            let mut tinyinst = super::TinyInst::new(&tinyinst_args, &program_args, 5000);

            // First test case
            let result = tinyinst.run();
            tinyinst.vec_coverage(&mut coverage, true);
            assert_eq!(result, super::litecov::RunResult::OK);
            assert!(coverage.len() <= 1412);

            // Second test case for b
            _ = file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file.write_all(b"b").unwrap();
            let result = tinyinst.run();
            tinyinst.vec_coverage(&mut coverage, true);
            assert_eq!(result, super::litecov::RunResult::OK);

            // Second test case for ba
            _ = file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file.write_all(b"ba").unwrap();
            let result = tinyinst.run();
            tinyinst.vec_coverage(&mut coverage, true);
            assert_eq!(result, super::litecov::RunResult::OK);
        }
    }
    #[test]
    fn tinyinst_crash() {
        use alloc::{string::ToString, vec::Vec};

        let tinyinst_args = vec!["-instrument_module".to_string(), TEST_FILENAME.to_string()];

        let program_args = vec![
            Path::new(TEST_PATH)
                .join(TEST_FILENAME)
                .display()
                .to_string(),
            "./test/crash_input.txt".to_string(),
        ];
        let mut coverage = Vec::new();

        unsafe {
            let mut tinyinst = super::TinyInst::new(&tinyinst_args, &program_args, 5000);
            let result = tinyinst.run();
            tinyinst.vec_coverage(&mut coverage, true);
            assert_eq!(result, super::litecov::RunResult::CRASH);
        }
    }
}
