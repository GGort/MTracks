<script lang="ts">
    import {Input, Label, TabItem, Tabs} from "flowbite-svelte";
    import {page} from '$app/stores';

</script>

<Tabs>
    <TabItem open title="SongInfo">
        <Label>
            title:
            <Input placeholder="Name" value={$page.params.title}/>
        </Label>
        <Label>
            Artist:
            <Input placeholder="Artist"/>
        </Label>
        <Label>
            BPM:
            <Input placeholder="BPM"/>
        </Label>

    </TabItem>
</Tabs>