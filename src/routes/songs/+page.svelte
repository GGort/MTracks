<script lang="ts">
    import {Button, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell} from "flowbite-svelte";
    import {invoke} from "@tauri-apps/api/core";
    import type {Song} from "$lib/songs";
    import Icon from "@iconify/svelte";

    let songList: Song[] = [];

    async function updateSongs() {
        songList = await invoke('get_songs');
    }

    updateSongs();

</script>

<Button on:click={()=>updateSongs()}>Update</Button>

<Table>
    <TableHead>
        <TableHeadCell>title</TableHeadCell>
        <TableHeadCell>Artist</TableHeadCell>
        <TableHeadCell>BPM</TableHeadCell>
        <TableHeadCell>Key</TableHeadCell>
        <TableHeadCell>Meter</TableHeadCell>
        <TableHeadCell></TableHeadCell>
    </TableHead>        
    <TableBody>
        {#each songList as song}
            <TableBodyRow>
                <TableBodyCell>{song.title}</TableBodyCell>
                <TableBodyCell>{song.artist}</TableBodyCell>
                <TableBodyCell>{song.bpm}</TableBodyCell>
                <TableBodyCell class="capitalize    ">{song.key}</TableBodyCell>
                <TableBodyCell>{song.meter}</TableBodyCell>
                <TableBodyCell>
                    <Button href="/songs/{song.title}/edit"><Icon icon="mdi:pencil" /></Button>
                    <Button href="/songs/{song.title}"><Icon icon="mdi:open-in-new" /></Button>
                </TableBodyCell>
            </TableBodyRow>
        {/each}
    </TableBody>
</Table>
