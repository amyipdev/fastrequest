<script lang="ts">
 import {Label, Input, Button} from "flowbite-svelte";
 import { loginType } from "../stores.ts";
 import * as Icon from "flowbite-svelte-icons";
 let showPwd: boolean = false;
</script>

<main>
    <form>
        <div class="mb-6">
            <Label for="username" class="block mb-2">Username</Label>
            <Input id="username" size="lg" placeholder="nicole47" required />
        </div>
        <div class="mb-6">
            <Label for="password" class="block mb-2">Password</Label>
            <Input id="password" type={showPwd ? "text" : "password"} placeholder="•••••••••" required>
                <button slot="right" on:click={() => {showPwd = !showPwd}} class="pointer-events-auto">
                    {#if showPwd}
                        <Icon.EyeOutline class="w-6 h-6" />
                    {:else}
                        <Icon.EyeSlashOutline class="w-6 h-6" />
                    {/if}
                </button>
            </Input>
        </div>
        <!-- TODO: remember me -->
        <Button type="submit">Login</Button>
        <Button color="alternative" on:click={() => loginType.set(1)}>Create account</Button>
    </form>
</main>

<style>
</style>
