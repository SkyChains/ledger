/*******************************************************************************
*   (c) 2022 Zondax AG
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
use super::prelude::*;

use constants::INS_GET_EXTENDED_PUBLIC_KEY as INS;

#[test]
fn extended_public_key() {
    let mut flags = 0u32;
    let mut tx = 0u32;
    let rx = 5;
    let mut buffer = [0u8; 260];

    buffer[..3].copy_from_slice(&[CLA, INS, 0]);
    prepare_buffer::<4>(&mut buffer, &[44, 9000, 0, 0], Some(&[]), Some(&[]));

    handle_apdu(&mut flags, &mut tx, rx, &mut buffer);

    assert_error_code!(tx, buffer, ApduError::Success);

    let pk_len = buffer[0] as usize;
    //secp256k1 pubkey and 32 bytes for chain code + 2 for response code
    assert_eq!(tx as usize, 1 + pk_len + 32 + 2);
}
