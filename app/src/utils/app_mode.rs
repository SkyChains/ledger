/*******************************************************************************
*   (c) 2021 Zondax GmbH
*
*  Licensed under the Apache License, Version 2.0 (the "License");
*  you may not use this file except in compliance with the License.
*  You may obtain a copy of the License at
*
*      http://www.apache.org/licenses/LICENSE-2.0
*
*  Unless required by applicable law or agreed to in writing, software
*  distributed under the License is distributed on an "AS IS" BASIS,
*  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*  See the License for the specific language governing permissions and
*  limitations under the License.
********************************************************************************/
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(any(unix,windows)))] {
        extern "C" {
            pub fn app_mode_expert() -> bool;
        }
    } else {
    }
}

pub fn is_app_mode_expert() -> bool {
    cfg_if! {
        if #[cfg(not(any(unix,windows)))] {
            unsafe { app_mode_expert() }
        } else {
           true
        }
    }
}
