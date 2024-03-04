<script lang="ts">
    import { writeText } from "@tauri-apps/api/clipboard";
    export let query: string;
    export let password: {
        appname: string;
        username: string;
        password: string;
    };
    // thought that index, might be useful in numbering ??
    // export let index: number;
    const copy = async (text: string) => {
        await writeText(text);
    };

    function highlightMatch(query: string, name: string) {
        const regex = new RegExp(query, "gi"); // Create a case-insensitive regex
        const highlightedName = name.replace(
            regex,
            (match) => `<mark>${match}</mark>`,
        );
        return highlightedName;
    }
</script>

<div
    class="flex bg-slate-600/20 shadow-lg w-full flex-row text-gray-200 gap-2 rounded-lg font-mono lg:w-3/4 mx-auto items-center p-2 overflow-clip"
>
    <button class="text-white scale-75">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="28"
            height="32"
            viewBox="0 0 448 512"
            ><path
                fill="currentColor"
                d="m170.5 51.6l-19 28.4h145l-19-28.4c-1.5-2.2-4-3.6-6.7-3.6h-93.7c-2.7 0-5.2 1.3-6.7 3.6zm147-26.6l36.7 55H424c13.3 0 24 10.7 24 24s-10.7 24-24 24h-8v304c0 44.2-35.8 80-80 80H112c-44.2 0-80-35.8-80-80V128h-8c-13.3 0-24-10.7-24-24s10.7-24 24-24h69.8l36.7-55.1C140.9 9.4 158.4 0 177.1 0h93.7c18.7 0 36.2 9.4 46.6 24.9zM80 128v304c0 17.7 14.3 32 32 32h224c17.7 0 32-14.3 32-32V128zm80 64v208c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16m80 0v208c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16m80 0v208c0 8.8-7.2 16-16 16s-16-7.2-16-16V192c0-8.8 7.2-16 16-16s16 7.2 16 16"
            /></svg
        >
    </button>
    <h3
        class={`text-center bg-gradient-to-t h-full flex items-center rounded-md w-1/4 px-3 bg-blue-400/20 overflow-clip whitespace-nowrap`}
    >
        {@html highlightMatch(query, password.appname)}
    </h3>
    <span
        class="rounded-md p-2 bg-slate-200/10 flex flex-row items-center w-1/2 bg-blue overflow-clip"
    >
        {@html highlightMatch(query, password.username)}
        <button
            class="ml-auto hover:scale-90 duration-75"
            on:click={() => copy(password.appname)}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="32"
                height="32"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M12 12q-1.65 0-2.825-1.175T8 8q0-1.65 1.175-2.825T12 4q1.65 0 2.825 1.175T16 8q0 1.65-1.175 2.825T12 12m-8 6v-.8q0-.85.438-1.562T5.6 14.55q1.55-.775 3.15-1.162T12 13q1.65 0 3.25.388t3.15 1.162q.725.375 1.163 1.088T20 17.2v.8q0 .825-.587 1.413T18 20H6q-.825 0-1.412-.587T4 18"
                /></svg
            >
        </button>
    </span>

    <span
        class="rounded-md p-2 bg-slate-200/10 flex-row flex items-center w-1/3 text-nowrap h-14 overflow-clip"
    >
        <p
            class="blur-md select-none hover:filter-none overflow-hidden w-full py-2 text-nowrap whitespace-nowrap"
        >
            {password.password}
        </p>
        <button
            class="ml-auto hover:scale-90 duration-75"
            on:click={() => copy(password.password)}
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="32"
                height="32"
                viewBox="0 0 24 24"
                ><path
                    fill="currentColor"
                    d="M12 10q1.25 0 2.125-.875T15 7q0-1.25-.875-2.125T12 4q-1.25 0-2.125.875T9 7q0 1.25.875 2.125T12 10m.025 12.625q-.2 0-.375-.062t-.325-.188l-2.575-2.25q-.15-.125-.238-.288t-.112-.362q-.025-.2.038-.387t.187-.338L10 17l-1.3-1.3q-.15-.15-.213-.325T8.426 15q0-.2.062-.375T8.7 14.3L10 13v-.35q-1.8-.625-2.9-2.175T6 7q0-2.5 1.75-4.25T12 1q2.5 0 4.25 1.75T18 7q0 2.025-1.15 3.538T14 12.65v7.95q0 .2-.075.388t-.225.337l-1.025 1.025q-.125.125-.288.2t-.362.075"
                /></svg
            >
        </button>
    </span>
</div>
