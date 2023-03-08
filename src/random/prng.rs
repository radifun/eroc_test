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

pub type DefaultPrng = Xoshiro256ss;

// =================================================================================================
// xoshiro256** pseudorandom number generator
// =================================================================================================

pub struct Xoshiro256ss {
    state: [u64; 4],
}

// Constructors ------------------------------------------------------------------------------------

impl Xoshiro256ss {
    pub fn from_seed(seed: u64) -> Self {
        let mut gen = SplitMix64::from_seed(seed);
        let state = [gen.next(), gen.next(), gen.next(), gen.next()];

        return Self { state };
    }
}

// Generation --------------------------------------------------------------------------------------

impl Xoshiro256ss {
    pub fn next(&mut self) -> u64 {
        let result = Self::rotate(self.state[1] * 5, 7) * 9;

        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;
        self.state[3] = Self::rotate(self.state[3], 45);

        return result;
    }

    fn rotate(x: u64, k: u64) -> u64 {
        return (x << k) | (x >> (64 - k));
    }
}

// =================================================================================================
// SplitMix64 generator for xoshiro256** initialization
// =================================================================================================

struct SplitMix64 {
    state: u64,
}

// Constructors ------------------------------------------------------------------------------------

impl SplitMix64 {
    fn from_seed(seed: u64) -> Self {
        return Self { state: seed };
    }
}

// Generation --------------------------------------------------------------------------------------

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        let mut result = self.state;
        self.state += 0x9e3779b97f4a7c15;

        result = (result ^ (result >> 30)) * 0xbf58476d1ce4e5b9;
        result = (result ^ (result >> 27)) * 0x94d049bb133111eb;
        result = result ^ (result >> 31);

        return result;
    }
}
