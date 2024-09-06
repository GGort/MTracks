<script>
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