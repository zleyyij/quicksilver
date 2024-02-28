<script lang="ts">
    import '../app.css';
    import './Dropdown.svelte';
    import Dropdown from './Dropdown.svelte';

    enum PasteMode {
        TEXT,
        FILE_UPLOAD,
        URL
    }
    /** A list of possible formats for the paste to be stored as*/
    const formatOptions = [
        {id: 1, text: "markdown"},
        {id: 2, text: "plaintext"},
        {id: 3, text: "html"},
        {id: 4, text: "file"},
        {id: 5, text: "url"},
    ];
    let pasteMode: PasteMode = PasteMode.TEXT;
    function handleFormatSelection(message: CustomEvent) {
        switch (message.detail.id) {
            case 4:
                pasteMode = PasteMode.FILE_UPLOAD;
            break;
            case 5:
                pasteMode = PasteMode.URL
            break;
            default:
                pasteMode = PasteMode.TEXT;
        }
    }


    const expiryOptions = [
        {id: 1, text: "24h"},
        {id: 2, text: "1h"},
        {id: 3, text: "1w"}
    ];
</script>

<style>
    textarea {
        background-color: var(--secondary-bg-color);
        color: var(--primary-text-color);
        font-size: 20px;
        /* by default, the user can change the size of the input box */
        resize: none;
        width: 100%;
    }

    button {
        background-color: transparent;
        /* background-color: var(--primary-bg-color); */
        color: var(--secondary-text-color);
        font-size: 20px;
        /* margin-top: 2%; */
        border-radius: 2px;
        border-style: none;
    }

    svg {
        /* https://stackoverflow.com/questions/22252472/how-can-i-change-the-color-of-an-svg-element */
        color: var(--secondary-text-color);
        margin-right: -2%;
        margin-top: .1%;
    }
    .paste-button {
        text-align: center;
        display: flex;
        flex: 1;
        margin-left: 0.5%;
        margin-top: 1%;
    }

    .wrapper {
        padding: 2%;
        display: flex;
        flex: 1;
        min-width: 0;
    }
    
    .options {
        padding-top: 1%;
        padding-left: 1%;
        display: flex;
    }
</style>
<div class="options">
    <Dropdown options={formatOptions} on:message={handleFormatSelection}/>
    <Dropdown options={expiryOptions}/>
    <div class="paste-button">
        <button>
            <svg xmlns="http://www.w3.org/2000/svg" height="26" viewBox="0 -960 960 960" width="26"><path fill="currentColor" stroke="currentColor" d="M382-320 155-547l57-57 170 170 366-366 57 57-423 423ZM200-160v-80h560v80H200Z"/></svg>
        </button>
    </div>
</div>
{#if pasteMode === PasteMode.TEXT}
    <div class="wrapper">
        <textarea id="pasteArea" cols="1" rows="20" spellcheck="true"></textarea>
    </div>
{/if}
{#if pasteMode === PasteMode.FILE_UPLOAD}
    <!-- TODO -->
    <p>TODO</p>
{/if}
{#if pasteMode === PasteMode.URL}
    <!-- TODO  -->
    <p>TODO</p>
{/if}