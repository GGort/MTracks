<script lang="ts">
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