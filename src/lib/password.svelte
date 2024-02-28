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

    // temp joke function
    function joke() {
        const hex_colors = [
            "#FF0000",
            "#FFA500",
            "#FFFF00",
            "#008000",
            "#0000FF",
            "#4B0082",
            "#EE82EE",
        ];
        const random_color = hex_colors[Math.floor(Math.random() * 7)];
        const random_color2 = hex_colors[Math.floor(Math.random() * 7)];
        return `linear-gradient(to right, ${random_color}, ${random_color2})
        `;
    }
</script>

<div
    class="flex bg-slate-600/20 shadow-lg w-full flex-row text-gray-200 gap-2 rounded-lg font-mono lg:w-3/4 mx-auto items-center p-2"
>
    <h3
        class={`text-center bg-gradient-to-t h-full flex items-center rounded-l-md w-32 px-3 bg-blue-400/20`}
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
        class="rounded-md p-2 bg-slate-200/10 flex flex-row items-center w-1/3"
    >
        <p class="blur-md select-none hover:filter-none overflow-x-scroll">
         
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
