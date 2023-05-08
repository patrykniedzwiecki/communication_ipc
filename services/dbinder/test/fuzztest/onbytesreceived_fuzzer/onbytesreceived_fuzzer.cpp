/*
 * Copyright (c) 2023 Huawei Device Co., Ltd.
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

#include "onbytesreceived_fuzzer.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <securec.h>
#include "dbinder_remote_listener.h"

namespace OHOS {

    bool DoSomethingInterestingWithMyAPI(const uint8_t* data, size_t size)
    {
        if (data == nullptr || size < sizeof(struct DHandleEntryTxRx)) {
            return false;
        }
        char tmp[DATA_SIZE_MAX] = {0};
        if (memcpy_s(tmp, sizeof(tmp) - 1, data, size) != EOK) {
            return false;
        }
        
        const char* testdata = tmp;
        std::shared_ptr<Session> session = nullptr;
        std::shared_ptr<DBinderRemoteListener> remoteListener = nullptr;
        ssize_t len = static_cast<ssize_t>(sizeof(struct DHandleEntryTxRx));
        remoteListener = std::make_shared<DBinderRemoteListener>(DBinderService::GetInstance());
        if (remoteListener == nullptr) {
            return false;
        }

        remoteListener->OnBytesReceived(session, testdata, len);

        return true;
    }
}

/* Fuzzer entry point */
extern "C" int LLVMFuzzerTestOneInput(const uint8_t* data, size_t size)
{
    /* Run your code on data */
    OHOS::DoSomethingInterestingWithMyAPI(data, size);
    return 0;
}
