<script lang="ts">
 import {Footer, FooterLinkGroup, FooterLink} from "flowbite-svelte";
 import { fade } from "svelte/transition";
 import { SvelteComponent } from "svelte";
 import { loginType } from "./stores.ts";
 import LoginForm from "./subs/LoginForm.svelte";
 import CreateAccountForm from "./subs/CreateAccountForm.svelte";
 import * as Icon from "flowbite-svelte-icons";

 let cv = 1;
 const vs: SvelteComponent[] = [LoginForm, CreateAccountForm];
 let vc: SvelteComponent | null = null;
 function toggleView(nv: number) {
     cv = nv;
 }
 function uvc() {
     vc = vs[cv];
 }
 loginType.subscribe((v) => toggleView(v));
 uvc();
</script>

<main>
    <div class="h-screen flex items-center justify-center">
        <div>
            <div class="flex w-full justify-center">
                <span class="inline-flex items-center mb-12">
                    <Icon.ArchiveArrowDownOutline class="w-14 h-14 me-6" />
                    <h1 class="text-center text-5xl">FastRequest</h1>
                </span>
            </div>
            {#if vc == vs[cv]}
                <div on:outroend={uvc} transition:fade>
                    <svelte:component this={vc} />
                </div>
            {/if}
        </div>
    </div>
    <Footer class="absolute bg-inherit w-full z-20 bottom-0 start-0" footerType="sitemap">
        <FooterLinkGroup ulClass="flex bg-inherit mb-3 flex-wrap items-center justify-center text-sm text-gray-400">
            <FooterLink href="https://github.com/amyipdev/fastrequest">GitHub</FooterLink>
            <FooterLink href="https://oic.amyip.net">OIC</FooterLink>
            <!-- TODO: link to a promotional site for FastRequest -->
            <FooterLink href="#" disabled>About</FooterLink>
        </FooterLinkGroup>
    </Footer>
</main>

<style>
</style>
