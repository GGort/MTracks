<script lang="ts">
    import {Button} from "flowbite-svelte";
    import {page} from '$app/stores';

</script>

<Button href="/songs/{$page.params.title}/edit">Edit</Button>