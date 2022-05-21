import type {KingsolAPIClient} from "./proto/Kingsol_apiServiceClientPb";
import {CreateRequest, Link, ListRequest} from "./proto/kingsol_api_pb";
import type grpcWeb from "grpc-web";

export default class API {
    client: KingsolAPIClient;
    metadata: grpcWeb.Metadata;

    constructor(client: KingsolAPIClient) {
        this.client = client;
        this.metadata = {};
    }

    setToken(token: string) {
        this.metadata["token"] = token;
    }

    async list() {
        const request = new ListRequest();
        let response = await this.client.list(request, this.metadata)
        return response.getLinksList()
    }

    async create(key: string, uri: string, overwrite: boolean = false) {
        const link = new Link().setKey(key).setUri(uri);
        const request = new CreateRequest().setLink(link).setOverwrite(overwrite);
        await this.client.create(request, this.metadata);
    }
}
