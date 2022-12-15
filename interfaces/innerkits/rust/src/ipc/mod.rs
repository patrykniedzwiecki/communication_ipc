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

pub mod remote_obj;
pub mod remote_stub;
pub mod macros;

use crate::{BorrowedMsgParcel, MsgParcel, Result, DeathRecipient};
use std::ops::{Deref};

// Export types of this module
pub use crate::RemoteObj;

/// Like C++ IRemoteObject class, define function for both proxy and stub object
pub trait IRemoteObj {
    // IPC request
    fn send_request(&self, code: u32, data: &MsgParcel, is_async: bool) -> Result<MsgParcel>;

    // Death Recipient
    fn add_death_recipient(&self, recipient: &mut DeathRecipient) -> bool;
    fn remove_death_recipient(&self, recipient: &mut DeathRecipient) -> bool;
}

// Like C++ IPCObjectStub class, define function for stub object only, like on_remote_request().
pub trait IRemoteStub: Send + Sync {
    fn get_descriptor() -> &'static str;

    fn on_remote_request(&self, code: u32, data: &BorrowedMsgParcel, reply: &mut BorrowedMsgParcel) -> i32;
}

// Like C++ IRemoteBroker class 
pub trait IRemoteBroker: Send + Sync {
    // Convert self to RemoteObject
    fn as_object(&self) -> Option<RemoteObj> {
        panic!("This is not a RemoteObject.")
    }
}

// Define function which how to convert a RemoteObj to RemoteObjRef, the later contains a
// dynamic trait object: IRemoteObject. For example, "dyn ITest" should implements this trait
pub trait FromRemoteObj: IRemoteBroker {
    fn from(object: RemoteObj) -> Result<RemoteObjRef<Self>>;
}

// Strong reference for "dyn IRemoteBroker" object, for example T is "dyn ITest"
pub struct RemoteObjRef<T: FromRemoteObj + ?Sized>(Box<T>);

impl<T: FromRemoteObj + ?Sized> RemoteObjRef<T> {
    pub fn new(object: Box<T>) -> Self {
        Self(object)
    }
}

impl<T: FromRemoteObj + ?Sized> Deref for RemoteObjRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: FromRemoteObj + ?Sized> Clone for RemoteObjRef<I> {
    fn clone(&self) -> Self {
        // non None
        FromRemoteObj::from(self.0.as_object().unwrap()).unwrap()
    }
}