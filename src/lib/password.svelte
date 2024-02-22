<script lang="ts">
    export let password: { username: string; password: string };
</script>

<div
    class="flex bg-slate-600/10 shadow-lg w-full flex-col text-gray-200 p-4 gap-6"
>
    <p class="text-xl font-bold">
        Username:
        <span class="rounded-md p-2 bg-slate-200/10">
            {password.username}
        </span>
    </p>
    <p class="text-xl font-bold">
        Password:

        <span class="rounded-md p-2 bg-slate-200/10">
            {password.password}
        </span>
    </p>
</div>
