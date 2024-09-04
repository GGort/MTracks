<script lang="ts">
    import {Input, Label, TabItem, Tabs} from "flowbite-svelte";
</script>

<Tabs>
    <TabItem open title="SongInfo" >
        <Label>
            Title:
            <Input placeholder="Naam"/>
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