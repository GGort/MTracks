<script>
    import {page} from '$app/stores';
    import {Sidebar, SidebarGroup, SidebarItem, SidebarWrapper} from 'flowbite-svelte';
    import Icon from '@iconify/svelte';

    $: activeUrl = $page.url.pathname;
</script>
<div class="flex flex-row gap-2 p-2">
    <Sidebar {activeUrl}>
        <SidebarWrapper>
            <SidebarGroup>
                <SidebarItem label="General" href="/settings">
                    <svelte:fragment slot="icon">
                        <Icon icon="mdi:gear"/>
                    </svelte:fragment>
                </SidebarItem>
                <SidebarItem label="Audio" href="/settings/audio">
                    <svelte:fragment slot="icon">
                        <Icon icon="mdi:audio-input-xlr"/>
                    </svelte:fragment>
                </SidebarItem>
            </SidebarGroup>
        </SidebarWrapper>
    </Sidebar>
    <div class="w-full">
        <slot></slot>
    </div>
</div>