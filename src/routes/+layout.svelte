<script lang="ts">
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

<slot></slot>
</div>
