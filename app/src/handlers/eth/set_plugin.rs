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
use crate::{constants::ApduError as Error, dispatcher::ApduHandler, sys, utils::ApduBufferRead};

pub struct SetPlugin;

// This instruction is sent in the process of providing
// more information regarding contract calls like erc721
// nft token information, we need to return ok for this
// in order the hw-app-eth package to continue with the
// provide_token_info/provide_erc20_info instructions
impl ApduHandler for SetPlugin {
    #[inline(never)]
    fn handle<'apdu>(_: &mut u32, tx: &mut u32, _: ApduBufferRead<'apdu>) -> Result<(), Error> {
        sys::zemu_log_stack("SetPlugin::handle\x00");

        *tx = 0;

        Ok(())
    }
}
