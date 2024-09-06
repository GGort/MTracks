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

    import {Tabs, TabItem, Button, Spinner, Accordion, AccordionItem} from "flowbite-svelte";
    import {Label, Select, MultiSelect} from 'flowbite-svelte';
    import {invoke} from "@tauri-apps/api/core";

    import {type SelectOptionType} from 'flowbite-svelte';

    let audioHost: SelectOptionType<String>[] = [];
    let devices: SelectOptionType<string>[] = [];
    let isLoading_devices = false;

    let SelectedHost: string; // TODO: get current/default host from store
    let SelectedDevices: string[] = [];

    async function get_audio_host() {
        let options = await invoke('get_audio_hosts', {host: SelectedHost}).catch((e) => {
            console.error(e);
            return [];
        }) as string[];
        audioHost = options.map(host => {
            return {value: host, name: host}
        });
    }

    async function get_audio_devices() {
        if (isLoading_devices) return;
        isLoading_devices = true;
        let options = await invoke('get_audio_devices', {host: SelectedHost}).catch((e) => {
            console.error(e);
            return [];
        }) as string[];
        devices = options.map(device => {
            return {value: device, name: device}
        });
        isLoading_devices = false;
    }

    get_audio_host();

</script>

<Tabs>
    <TabItem open title="Audio Devices">
        <Label>
            Audio Engine:
            <Select items={audioHost} bind:value={SelectedHost} ></Select>
        </Label>
        <br>
        <Button on:click={()=>get_audio_devices()}>
            Refresh Devices&nbsp;
            {#if (isLoading_devices)}
                <Spinner class="me-3" size="4" color="white"/>
            {/if}
        </Button>
        <br>
        <br>
        <Label>
            Audio Devices:
            <MultiSelect items={devices} bind:value={SelectedDevices} />
        </Label>

    </TabItem>
    <TabItem title="Mapping">
        <Accordion>
            {#each SelectedDevices as device}
                <AccordionItem>
                    <span slot="header">{device}</span>
                    <p>Not Implemented</p>
                </AccordionItem>
            {/each}
        </Accordion>
    </TabItem>
</Tabs>