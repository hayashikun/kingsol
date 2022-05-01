<script lang="ts">
    import {KingsolAPIClient} from "./proto/Kingsol_apiServiceClientPb";
    import {CreateRequest, Link, ListRequest} from "./proto/kingsol_api_pb";

    export let name: string;

    function add() {
        const client = new KingsolAPIClient("http://localhost:8081")
        const link = new Link().setKey("tw").setUri("https://twitter.com/home")
        const req = new CreateRequest().setLink(link)
        client.create(req, null, (err, res) => {
            console.log(err);
            console.log(res);
        })
    }

    function list() {
        const client = new KingsolAPIClient("http://localhost:8081")
        const req = new ListRequest()
        client.list(req, null, (err, res) => {
            console.log(err);
            console.log(res);
        })
    }
</script>

<main>
    <h1>Hello {name}!</h1>
    <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>

    <button on:click={add}>Add</button>
    <button on:click={list}>List</button>
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
</style>