<script lang="ts">
 import {Label, Input, Button, Popover, Toast} from "flowbite-svelte";
 import { loginType } from "../stores.ts";

 let bindPwd = "";
 let bindFirstName = "";
 let bindLastName = "";
 let bindUsername = "";
 let passwordStrength: string = "weak";
 $: barColor = passwordStrength == "weak" ? "bg-red-400" : (passwordStrength == "okay" ? "bg-orange-400" : "bg-green-400");
 let hasEight: boolean = false;
 let hasTwelve: boolean = false;
 let hasUpper: boolean = false;
 let hasLower: boolean = false;
 let hasNumber: boolean = false;
 let showPwd: boolean = false;
 let hasNameComponent: boolean = false;
 import * as Icon from "flowbite-svelte-icons";

 function updatePasswordStrength() {
     hasEight = bindPwd.length >= 8;
     hasTwelve = bindPwd.length >= 12;
     hasUpper = /\p{Lu}/u.test(bindPwd);
     hasLower = /\p{Ll}/u.test(bindPwd);
     hasNumber = /\d/.test(bindPwd);
     let tmp = bindPwd.toLowerCase();
     hasNameComponent = false;
     if (bindFirstName != "") {
         hasNameComponent |= tmp.includes(bindFirstName.toLowerCase());
     }
     if (bindLastName != "") {
         hasNameComponent |= tmp.includes(bindLastName.toLowerCase());
     }
     if (bindUsername != "") {
         hasNameComponent |= tmp.includes(bindUsername.toLowerCase());
     }
     if (hasEight && hasUpper && hasLower && hasNumber && !hasNameComponent) {
         passwordStrength = "okay";
         if (hasTwelve) {
             passwordStrength = "strong";
         }
     } else {
         passwordStrength = "weak";
     }
 }

 let toastbin = [];

 function submitter() {
     toastbin.push(new Toast({
         target: document.body,
         props: {
             dismissable: false,
         }
     }));
 }
</script>

<main>
    <form on:submit|preventDefault={submitter}>
        <div class="grid gap-5 mb-6 md:grid-cols-2">
            <div class="mb-6">
                <Label for="firstname" class="block mb-2">First name</Label>
                <Input type="text" id="firstname" size="lg" placeholder="Sky" bind:value="{bindFirstName}" on:input={updatePasswordStrength} required />
            </div>
            <div class="mb-6">
                <Label for="lastname" class="block mb-2">Last name</Label>
                <Input type="text" id="lastname" size="lg" placeholder="Garcia" bind:value="{bindLastName}" on:input={updatePasswordStrength} required />
            </div>
            <!-- pop up notification: "Username already taken!" -->
            <div class="mb-6">
                <Label for="username" class="block mb-2">Username</Label>
                <Input type="text" id="username" size="lg" placeholder="nicole47" bind:value="{bindUsername}" on:input={updatePasswordStrength} required />
            </div>
            <div class="mb-6">
                <Label for="Email" class="block mb-2">Email</Label>
                <Input type="email" id="Email" size="lg" placeholder="sky.garcia@gmail.com" required />
            </div>
        </div>
        <div class="mb-6">
            <Label for="organization" class="block mb-2">Organization</Label>
            <Input type="text" id="organization" size="lg" placeholder="None" required />
        </div>
        <!-- Pop up something while typing in about password requirements; custom pre-submit validation alongside server validation -->
        <div class="mb-6">
            <Label for="password" class="block mb-2">Password</Label>
            <Input id="password" type={showPwd ? "text" : "password"} bind:value="{bindPwd}" on:input={updatePasswordStrength} placeholder="•••••••••" required>
                <button type="button" slot="right" on:click={() => {showPwd = !showPwd}} class="pointer-events-auto">
                    {#if showPwd}
                        <Icon.EyeOutline class="w-6 h-6" />
                    {:else}
                        <Icon.EyeSlashOutline class="w-6 h-6" />
                    {/if}
                </button>
            </Input>
        </div>
        <Popover class="text-sm" triggered-by="#password" placement="bottom">
            <h3 class="font-semibold mb-1">Your password is {passwordStrength}</h3>
            <div class="grid grid-cols-3 gap-2 my-2">
                <!-- TODO: dynamic colors -->
                <div class="h-1 {barColor}" />
                <div class="h-1 {passwordStrength == "okay" || passwordStrength == "strong" ? barColor : "bg-gray-600"}" />
                <div class="h-1 {passwordStrength == "strong" ? barColor : "bg-gray-600"}" />
            </div>
            <p>Your password should have:</p>
            <ul>
                <!-- TODO: make these their own component -->
                <li class="flex items-center mb-1">
                    {#if hasUpper && hasLower}
                        <Icon.CheckOutline class="me-2 w-4 h-4 text-green-500" />
                    {:else}
                        <Icon.CloseOutline class="me-2 w-4 h-4 text-gray-400" />
                    {/if}
                    An uppercase and lowercase letter
                </li>
                <li class="flex items-center mb-1">
                    {#if hasNumber}
                        <Icon.CheckOutline class="me-2 w-4 h-4 text-green-500" />
                    {:else}
                        <Icon.CloseOutline class="me-2 w-4 h-4 text-gray-400" />
                    {/if}
                    At least one number
                </li>
                <li class="flex items-center mb-1">
                    {#if hasEight}
                        <Icon.CheckOutline class="me-2 w-4 h-4 text-green-500" />
                    {:else}
                        <Icon.CloseOutline class="me-2 w-4 h-4 text-gray-400" />
                    {/if}
                    At least eight characters, ideally 12 or more
                </li>
                <li class="flex items-center mb-1">
                    {#if !hasNameComponent}
                        <Icon.CheckOutline class="me-2 w-4 h-4 text-green-500" />
                    {:else}
                        <Icon.CloseOutline class="me-2 w-4 h-4 text-gray-400" />
                    {/if}
                    No part of the name or username in the password
                </li>
            </ul>
        </Popover>
        <Button type="submit">Create account</Button>
        <Button color="alternative" on:click={() => loginType.set(0)}>Login</Button>
</main>

<style>
</style>
