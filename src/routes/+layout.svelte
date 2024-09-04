<script lang="ts">
    import {Navbar, NavBrand, NavUl, NavLi, NavHamburger, DarkMode} from "flowbite-svelte";

    import '../app.css';


</script>

<Navbar>
    <NavBrand>MTracks</NavBrand>
    <div class="flex flex-row">
        <NavUl>
            <NavLi>Home</NavLi>
        </NavUl>
        <NavHamburger/>
        <DarkMode/>

    </div>
</Navbar>

<slot></slot>