<script lang="ts">
 import {Label, Input, Button} from "flowbite-svelte";
 import * as Icon from "flowbite-svelte-icons";
</script>

<main>
    <div class="h-screen flex items-center justify-center">
        <div>
            <span class="inline-flex items-center mb-12">
                <Icon.ArchiveArrowDownOutline class="w-14 h-14 me-6" />
                <h1 class="text-center text-5xl">FastRequest</h1>
            </span>
            <form>
                <div class="mb-6">
                    <Label for="username" class="block mb-2">Username</Label>
                    <Input id="username" size="lg" placeholder="nicole47" required />
                </div>
                <div class="mb-6">
                    <Label for="password" class="block mb-2">Password</Label>
                    <Input id="password" type="password" placeholder="•••••••••" required />
                </div>
                <Button type="submit">Login</Button>
                <Button color="alternative">Create account</Button>
            </form>
        </div>
    </div>
</main>

<style>
</style>
