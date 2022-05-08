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
use core::{mem::MaybeUninit, ptr::addr_of_mut};
use nom::{
    bytes::complete::take,
    number::complete::{be_u32, be_u64},
    sequence::tuple,
};
use zemu_sys::ViewError;

use crate::handlers::{handle_ui_message, parser_common::ParserError};

use crate::parser::{Address, DisplayableItem, ADDRESS_LEN};

type Fields<'b> = Result<(&'b [u8], (u64, u32, &'b [[u8; ADDRESS_LEN]])), nom::Err<ParserError>>;

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct SECPMintOutput<'b> {
    // groups locktime(u64), threshold(u32)
    pub locktime: u64,
    pub threshold: u32,
    // list of addresses allowed to use this output
    pub addresses: &'b [[u8; ADDRESS_LEN]],
}

impl<'b> SECPMintOutput<'b> {
    pub const TYPE_ID: u32 = 0x00000006;

    fn fields_from_bytes(input: &'b [u8]) -> Fields {
        let (rem, (locktime, threshold, addr_len)) = tuple((be_u64, be_u32, be_u32))(input)?;

        let (rem, addresses) = take(addr_len as usize * ADDRESS_LEN)(rem)?;

        let addresses =
            bytemuck::try_cast_slice(addresses).map_err(|_| ParserError::InvalidAddressLength)?;

        if (threshold as usize > addresses.len()) || (addresses.is_empty() && threshold != 0) {
            return Err(ParserError::InvalidThreshold.into());
        }

        Ok((rem, (locktime, threshold, addresses)))
    }

    #[inline(never)]
    pub fn from_bytes(input: &'b [u8]) -> Result<(&'b [u8], Self), nom::Err<ParserError>> {
        crate::sys::zemu_log_stack("SECPMintOutput::from_bytes\x00");
        let (rem, (locktime, threshold, addresses)) = Self::fields_from_bytes(input)?;

        Ok((
            rem,
            Self {
                locktime,
                threshold,
                addresses,
            },
        ))
    }

    #[inline(never)]
    pub fn from_bytes_into(
        input: &'b [u8],
        out: &mut MaybeUninit<Self>,
    ) -> Result<&'b [u8], nom::Err<ParserError>> {
        crate::sys::zemu_log_stack("SECPMintOutput::from_bytes_into\x00");

        let (rem, (locktime, threshold, addresses)) = Self::fields_from_bytes(input)?;
        let out = out.as_mut_ptr();

        //good ptr and no uninit reads
        unsafe {
            addr_of_mut!((*out).locktime).write(locktime);
            addr_of_mut!((*out).threshold).write(threshold);
            addr_of_mut!((*out).addresses).write(addresses);
        }

        Ok(rem)
    }
}

impl<'a> DisplayableItem for SECPMintOutput<'a> {
    fn num_items(&self) -> usize {
        // output-type, threshold and addresses
        let items = 1 + 1 + self.addresses.len();
        // do not show locktime if it is 0
        items + (self.locktime > 0) as usize
    }

    #[inline(never)]
    fn render_item(
        &self,
        item_n: u8,
        title: &mut [u8],
        message: &mut [u8],
        page: u8,
    ) -> Result<u8, ViewError> {
        use bolos::{pic_str, PIC};
        use lexical_core::{write as itoa, Number};

        let mut buffer = [0; usize::FORMATTED_SIZE];
        let addr_item_n = self.num_items() - self.addresses.len();

        match item_n as usize {
            0 => {
                let title_content = pic_str!(b"Output");
                title[..title_content.len()].copy_from_slice(title_content);

                handle_ui_message(pic_str!(b"SECPMint"), message, page)
            }

            1 if self.locktime > 0 => {
                let title_content = pic_str!(b"Locktime");
                title[..title_content.len()].copy_from_slice(title_content);
                itoa(self.locktime, &mut buffer);

                handle_ui_message(&buffer, message, page)
            }

            x @ 1.. if (x == 1 && self.locktime == 0) || (x == 2 && self.locktime > 0) => {
                let title_content = pic_str!(b"Threshold");
                title[..title_content.len()].copy_from_slice(title_content);

                itoa(self.threshold, &mut buffer);

                handle_ui_message(&buffer, message, page)
            }

            x @ 2.. if x >= addr_item_n => {
                let idx = x - addr_item_n;
                if let Some(data) = self.addresses.get(idx as usize) {
                    let addr = Address::from_bytes(data.as_ref())
                        .map_err(|_| ViewError::Unknown)?
                        .1;
                    addr.render_item(0, title, message, page)
                } else {
                    Err(ViewError::NoData)
                }
            }
            _ => Err(ViewError::NoData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_secp256k1_mint_output() {
        let raw_output = [
            0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 107, 106, 1, 167, 20, 122,
            95, 155, 189, 52, 132, 21, 94, 230, 26, 133, 92, 231, 53, 186, 0, 0, 0, 0, 0, 0, 0, 0,
            127, 201, 61, 133, 198, 214, 44, 91, 42, 192, 181, 25, 200, 112, 16, 234, 82, 148, 1,
            45, 30, 64, 112, 48, 214, 172, 208, 2, 28, 172, 16, 213, 0, 0, 0, 1, 71, 17, 128, 245,
            190, 100, 113, 53, 172, 8, 240, 180, 27, 164, 33, 138, 21, 117, 13, 78, 36, 121, 31,
            186, 118, 70, 237, 151, 61, 204, 110, 123, 0, 0, 0, 0, 61, 155, 218, 192, 237, 29, 118,
            19, 48, 207, 104, 14, 253, 235, 26, 66, 21, 158, 179, 135, 214, 210, 149, 12, 150, 247,
            210, 143, 97, 187, 226, 170, 0, 0, 0, 5, 0, 0, 0, 0, 5, 230, 158, 192, 0, 0, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 1, 69, 136, 235, 111, 206, 248, 241, 99, 249,
            22, 126, 93, 117, 195, 56, 35, 129, 23, 81, 11, 56, 150, 186, 96, 172, 81, 75, 154,
            159, 54, 203, 31, 16, 82, 38, 127, 166, 131, 153, 81, 171, 12, 160, 85, 169, 248, 58,
            101, 211, 76, 120, 5, 137, 18, 213, 222, 36, 191, 169, 28, 203, 145, 255, 8, 0,
        ];

        let output = SECPMintOutput::from_bytes(&raw_output[4..]).unwrap().1;
        assert_eq!(output.locktime, 0);
        assert_eq!(output.threshold, 0);
        assert_eq!(output.addresses.len(), 1);
    }
}
