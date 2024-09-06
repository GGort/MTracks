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

    import {Navbar, NavBrand, NavUl, NavLi, NavHamburger,Dropdown, DropdownItem, DarkMode} from "flowbite-svelte";
    import Icon from '@iconify/svelte';

    import '../app.css';
    import { page } from '$app/stores';
    $: activeUrl = $page.url.pathname;
    console.log(activeUrl);

</script>
<div class="min-h-screen">

<Navbar>
    <NavBrand>MTracks</NavBrand>
    <div class="flex flex-row">
        <NavUl  {activeUrl}>
            <NavLi href="/" >Home</NavLi>
            <NavLi href="/songs">Song</NavLi>
            <NavLi href="/setLists">Set lists</NavLi>
            <NavLi class="cursor-pointer inline">settings<Icon class="inline"   inline icon="mdi:chevron-down" /></NavLi>
            <Dropdown class="w-44 z-20">
                <DropdownItem href="/settings">General</DropdownItem>
                <DropdownItem href="/settings/audio">audio</DropdownItem>
            </Dropdown>
        </NavUl>
        <NavHamburger/>
        <DarkMode/>

    </div>
</Navbar>

<slot/>
</div>
