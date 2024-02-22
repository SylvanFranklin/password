<script lang="ts">
    export let password: { username: string; password: string };
</script>

<div class="flex flex-col w-1/2 p-4 bg-gray-200 rounded-lg shadow-lg my-4">
    <p class="text-xl font-bold">Username: {password.username}</p>
    <p class="text-xl font-bold">Password: {password.password}</p>
</div>
