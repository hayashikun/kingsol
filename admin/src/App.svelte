<script lang="ts">
    import {KingsolAPIClient} from "./proto/Kingsol_apiServiceClientPb";
    import API from "./api";
    import {Link} from "./proto/kingsol_api_pb";

    const client = new KingsolAPIClient("http://localhost:8081")
    const api = new API(client);

    let errorMessage: string | null = null;

    let key = "";
    let uri = "";
    let overwrite = false;
    let links: [Link] = [];
    let token = localStorage.getItem("token");

    function setToken() {
        localStorage.setItem("token", token);
        api.setToken(token);
    }

    setToken()

    async function add() {
        try {
            await api.create(key, uri, overwrite)
        } catch (e: Error) {
            errorMessage = e.toString();
        }
        await list()
    }

    async function list() {
        try {
            links = await api.list()
        } catch  (e: Error) {
            errorMessage = e.toString();
        }
    }

    list()

</script>

<main>
    <h1>Kingsol</h1>

    <div class="message-container" class:error-visible={errorMessage != null}>
        {errorMessage}
    </div>

    <div class="token-container">
        <label for="token-input">Token
            <input id="token-input" type="text" bind:value={token}>
        </label>

        <button on:click={setToken}>Set</button>
    </div>

    <div class="create-link-container">
        <label for="key-input">Key
            <input id="key-input" type="text" bind:value={key}>
        </label>

        <label for="uri-input">URI
            <input id="uri-input" type="text" bind:value={uri}>
        </label>

        <label for="overwrite-input">Overwrite
            <input id="overwrite-input" type="checkbox" bind:value={overwrite}>
        </label>

        <button on:click={add}>Add</button>
    </div>

    <ul class="link-ul">
        {#each links as link}
            <li>
                <div class="link-key">{link.getKey()}</div>
                <div class="link-uri">{link.getUri()}</div>
            </li>
        {/each}
    </ul>
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: none;
        margin: 0 auto;
    }

    h1 {
        color: orangered;
        font-size: 4em;
        font-weight: lighter;
    }

    .message-container {
        margin: 2em 4em;
        padding: 1em;
        display: none;
    }

    .error-visible {
        color: red;
        background: #ffe0e2;
        display: block;
    }

    .token-container {
        display: flex;
        justify-content: center;
        padding-bottom: 20px;
        margin-bottom: 20px;
    }

    #token-input {
        width: 200px;
    }

    .token-container label {
        margin: auto 0;
    }

    .token-container input {
        margin: 0 1.6em 0 0.4em;
    }

    .token-container button {
        margin: auto 0;
    }

    .create-link-container {
        display: flex;
        justify-content: center;
        padding-bottom: 40px;
        margin-bottom: 40px;
        border-bottom: solid 2px #AAAAAA;
    }

    #key-input {
        width: 100px;
    }

    #uri-input {
        width: 500px;
    }

    .create-link-container label {
        margin: auto 0;
    }

    .create-link-container input {
        margin: 0 1.6em 0 0.4em;
    }

    .create-link-container button {
        margin: auto 0;
    }

    .link-ul {
        font-size: 1.2em;
        list-style: none;
    }

    .link-ul li {
        display: flex;
        justify-content: center;
        margin-top: 4px;
    }

    .link-key {
        margin-right: 2em;
        width: 6em;
    }

    .link-uri {
        width: 20em;
        text-align: left;
    }
</style>