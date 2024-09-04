<script lang="ts">
    import {Button, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell} from "flowbite-svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {Song} from "../../lib/songs";

    let songlist: Song[] = [];

    async function updateSongs() {
        songlist = await invoke('get_songs');
    }

    updateSongs();

</script>

<Button on:click={()=>updateSongs()}>Update</Button>

<Table>
    <TableHead>
        <TableHeadCell>title</TableHeadCell>
        <TableHeadCell>artist</TableHeadCell>
        <TableHeadCell>BPM</TableHeadCell>
    </TableHead>
    <TableBody>
        {#each songlist as song}
            <TableBodyRow>
                <TableBodyCell>{song.title}</TableBodyCell>
                <TableBodyCell>artist</TableBodyCell>
                <TableBodyCell>BPM</TableBodyCell>
            </TableBodyRow>
        {/each}
    </TableBody>
</Table>
