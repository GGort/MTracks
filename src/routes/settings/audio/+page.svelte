<script lang="ts">
/*
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

    import {Tabs, TabItem, Button, Spinner} from "flowbite-svelte";
    import {invoke} from "@tauri-apps/api/core";

    let devices: String[] = [];
    let isLoading = false;

    async function get_audio_devices() {
        if (isLoading) return;
        isLoading = true;
        devices = [];
        devices = await invoke('get_audio_devices');
        isLoading = false;
    }
</script>

<Tabs>                          
    <TabItem open title="Audio Devices">
        <Button on:click={()=>get_audio_devices()}>
            Refresh Devices&nbsp;
            {#if (isLoading)}
                <Spinner class="me-3" size="4" color="white"/>
            {/if}
        </Button>

        {#each devices as device}
            <p>{device}</p>
        {/each}
    </TabItem>
    <TabItem title="Inputs">
        <p>Not implemented</p>
    </TabItem>
    <TabItem title="Outputs">

    </TabItem>
</Tabs>