/*
 * Copyright (C) 2022 Huawei Device Co., Ltd.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;
use crate::{ipc_binding, BorrowedMsgParcel, Result, AsRawPtr, result_status};
use std::convert::TryInto;

/// InterfaceToken packed a String type which transfered with C++ std::u16string.
pub struct InterfaceToken(String);

impl InterfaceToken {
    /// Create a InterfaceToken object by Rust String
    pub fn new(value: &str) -> Self {
        Self(String::from(value))
    }

    /// Get packed String of InterfaceToken
    pub fn get_token(&self) -> String {
        String::from(&self.0)
    }
}

impl Serialize for InterfaceToken {
    fn serialize(&self, parcel: &mut BorrowedMsgParcel<'_>) -> Result<()> {
        let token = &self.0;
        // SAFETY: `parcel` always contains a valid pointer to a  `CParcel`
        let ret = unsafe {
            ipc_binding::CParcelWriteInterfaceToken(
                parcel.as_mut_raw(), 
                token.as_ptr() as *const c_char,
                token.as_bytes().len().try_into().unwrap()  
            )};
        result_status::<()>(ret, ())
    }
}

impl Deserialize for InterfaceToken {
    fn deserialize(parcel: &BorrowedMsgParcel<'_>) -> Result<Self> {
        let mut vec: Option<Vec<u8>> = None;
        let ok_status = unsafe {
            // SAFETY: `parcel` always contains a valid pointer to a  `CParcel`
            ipc_binding::CParcelReadInterfaceToken(
                parcel.as_raw(), 
                &mut vec as *mut _ as *mut c_void,
                allocate_vec_with_buffer::<u8>
            )
        };

        if ok_status {
            let result = vec.map(|s| {
                match String::from_utf8(s) {
                    Ok(val) => val,
                    Err(_) => String::from("")
                }
            });
            if let Some(val) = result {
                Ok(Self(val))
            } else {
                println!("convert interface token to String fail");
                Err(-1)
            }
        }else{
            println!("read interface token from native fail");
            Err(-1)
        }
    }
}
