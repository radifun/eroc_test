// =================================================================================================
// Copyright (c) 2023 Viet-Hoa Do <doviethoa@doviethoa.com>
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// =================================================================================================

use std::collections::hash_map;
use std::hash::Hasher;
use std::mem::MaybeUninit;
use std::sync::{Arc, Once, RwLock};

use crate::random::DefaultPrng;

pub type TestCaseId = u64;

// =================================================================================================
// Test framework
// =================================================================================================

pub struct TestFramework;

// Global test framework ---------------------------------------------------------------------------

impl TestFramework {
    pub fn get() -> Arc<RwLock<Self>> {
        static mut INSTANCE: MaybeUninit<Arc<RwLock<TestFramework>>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            let framework = TestFramework;
            unsafe {
                INSTANCE.write(Arc::new(RwLock::new(framework)));
            }
        });

        return unsafe { INSTANCE.assume_init_ref().clone() };
    }
}

// Register test case ------------------------------------------------------------------------------

impl TestFramework {
    pub fn register_test_case(&mut self, name: &'static str) -> TestCaseId {
        let mut hasher = hash_map::DefaultHasher::new();
        hasher.write_usize(name.len());

        hasher.write(name.as_bytes());
        let test_case_id = hasher.finish();

        return test_case_id;
    }
}

// Randomization -----------------------------------------------------------------------------------

impl TestFramework {
    pub fn seed(&self) -> u64 {
        return 0;
    }
}

// =================================================================================================
// Test case
// =================================================================================================

pub struct TestCase {
    name: &'static str,
    test_seed_gen: DefaultPrng,
}

// Constructors ------------------------------------------------------------------------------------

impl TestCase {
    pub fn new(name: &'static str) -> Self {
        let mut hasher = hash_map::DefaultHasher::new();
        hasher.write_usize(name.len());
        hasher.write(name.as_bytes());
        let unique_hash = hasher.finish();

        let shared_framework = TestFramework::get();
        let framework = shared_framework.read().unwrap();
        let seed = framework.seed() ^ unique_hash;

        let test_seed_gen = DefaultPrng::from_seed(seed);

        return Self { name, test_seed_gen };
    }
}

// Properties --------------------------------------------------------------------------------------

impl TestCase {
    pub fn name(&self) -> &'static str {
        return self.name;
    }
}

// Test case iteration -----------------------------------------------------------------------------

impl TestCase {
    pub fn create_test(&mut self) -> Test {
        let name = self.name;
        let seed = self.test_seed_gen.next();

        return Test { name, seed };
    }
}

// =================================================================================================
// Test
// =================================================================================================

pub struct Test {
    name: &'static str,
    seed: u64,
}

// Properties --------------------------------------------------------------------------------------

impl Test {
    pub fn name(&self) -> &'static str {
        return self.name;
    }

    pub fn seed(&self) -> u64 {
        return self.seed;
    }
}

// Unit tests --------------------------------------------------------------------------------------

#[cfg(test)]
use crate as eroc_test;

#[eroc_test_macro::test_case]
fn test_sanity(test: &mut Test) {
    let mut rnd = DefaultPrng::from_seed(test.seed());

    let a = rnd.next();

    println!("a = {}", a);
    assert!(a % 100 < 99);
}
