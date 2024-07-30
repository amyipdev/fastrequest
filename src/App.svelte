<script lang="ts">
 import LoginView from "./Login.svelte";
 import { fade } from "svelte/transition";
 import { SvelteComponent } from "svelte";
 const views: SvelteComponent[] = [null, LoginView];
 let viewportComponent: SvelteComponent | null = null;
 let currentView: number = 1;
 function toggleView(nv: number) {
     currentView = nv;
 }
 function updateViewportComponent() {
     viewportComponent = views[currentView];
 }
 updateViewportComponent();
</script>

<main class="bg-gray-900">
    {#if viewportComponent == views[currentView]}
        <div id="viewport" on:outroend={updateViewportComponent} transition:fade>
            <svelte:component this={viewportComponent} />
        </div>
    {/if}
</main>

<style>
</style>
