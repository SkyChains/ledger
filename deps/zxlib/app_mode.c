/*******************************************************************************
*   (c) 2020 Zondax GmbH
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

#include "app_mode.h"

typedef struct {
    uint8_t expert;
} app_mode_persistent_t;

typedef struct {
    uint8_t secret;
} app_mode_temporary_t;

app_mode_temporary_t app_mode_temporary;

#if defined(TARGET_NANOS) || defined(TARGET_NANOX) || defined(TARGET_NANOS2)
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
app_mode_persistent_t NV_CONST N_appmode_impl __attribute__ ((aligned(64)));
#define N_appmode (*(NV_VOLATILE app_mode_persistent_t *)PIC(&N_appmode_impl))

void app_mode_reset(){
    app_mode_temporary.secret = 0;
}

bool app_mode_expert() {
    return N_appmode.expert;
}

void app_mode_set_expert(uint8_t val) {
    app_mode_persistent_t mode;
    mode.expert = val;
    MEMCPY_NV( (void*) PIC(&N_appmode_impl), (void*) &mode, sizeof(app_mode_persistent_t));
}

#else
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

app_mode_persistent_t app_mode;

void app_mode_reset() {
    app_mode.expert = 0;
    app_mode_temporary.secret = 0;
}

bool app_mode_expert() {
    return app_mode.expert;
}

void app_mode_set_expert(uint8_t val) {
    app_mode.expert = val;
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

#endif

bool app_mode_secret() {
    return app_mode_temporary.secret;
}

void app_mode_set_secret(uint8_t val) {
    app_mode_temporary.secret = val;
}
