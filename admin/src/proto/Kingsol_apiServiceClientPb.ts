/**
 * @fileoverview gRPC-Web generated client stub for kingsol_api
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as kingsol_api_pb from './kingsol_api_pb';


export class KingsolAPIClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: any; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'binary';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname;
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodDescriptorGet = new grpcWeb.MethodDescriptor(
    '/kingsol_api.KingsolAPI/Get',
    grpcWeb.MethodType.UNARY,
    kingsol_api_pb.GetRequest,
    kingsol_api_pb.GetResponse,
    (request: kingsol_api_pb.GetRequest) => {
      return request.serializeBinary();
    },
    kingsol_api_pb.GetResponse.deserializeBinary
  );

  get(
    request: kingsol_api_pb.GetRequest,
    metadata: grpcWeb.Metadata | null): Promise<kingsol_api_pb.GetResponse>;

  get(
    request: kingsol_api_pb.GetRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: kingsol_api_pb.GetResponse) => void): grpcWeb.ClientReadableStream<kingsol_api_pb.GetResponse>;

  get(
    request: kingsol_api_pb.GetRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: kingsol_api_pb.GetResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/kingsol_api.KingsolAPI/Get',
        request,
        metadata || {},
        this.methodDescriptorGet,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/kingsol_api.KingsolAPI/Get',
    request,
    metadata || {},
    this.methodDescriptorGet);
  }

  methodDescriptorList = new grpcWeb.MethodDescriptor(
    '/kingsol_api.KingsolAPI/List',
    grpcWeb.MethodType.UNARY,
    kingsol_api_pb.ListRequest,
    kingsol_api_pb.ListResponse,
    (request: kingsol_api_pb.ListRequest) => {
      return request.serializeBinary();
    },
    kingsol_api_pb.ListResponse.deserializeBinary
  );

  list(
    request: kingsol_api_pb.ListRequest,
    metadata: grpcWeb.Metadata | null): Promise<kingsol_api_pb.ListResponse>;

  list(
    request: kingsol_api_pb.ListRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: kingsol_api_pb.ListResponse) => void): grpcWeb.ClientReadableStream<kingsol_api_pb.ListResponse>;

  list(
    request: kingsol_api_pb.ListRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: kingsol_api_pb.ListResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/kingsol_api.KingsolAPI/List',
        request,
        metadata || {},
        this.methodDescriptorList,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/kingsol_api.KingsolAPI/List',
    request,
    metadata || {},
    this.methodDescriptorList);
  }

  methodDescriptorCreate = new grpcWeb.MethodDescriptor(
    '/kingsol_api.KingsolAPI/Create',
    grpcWeb.MethodType.UNARY,
    kingsol_api_pb.CreateRequest,
    kingsol_api_pb.CreateResponse,
    (request: kingsol_api_pb.CreateRequest) => {
      return request.serializeBinary();
    },
    kingsol_api_pb.CreateResponse.deserializeBinary
  );

  create(
    request: kingsol_api_pb.CreateRequest,
    metadata: grpcWeb.Metadata | null): Promise<kingsol_api_pb.CreateResponse>;

  create(
    request: kingsol_api_pb.CreateRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: kingsol_api_pb.CreateResponse) => void): grpcWeb.ClientReadableStream<kingsol_api_pb.CreateResponse>;

  create(
    request: kingsol_api_pb.CreateRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: kingsol_api_pb.CreateResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/kingsol_api.KingsolAPI/Create',
        request,
        metadata || {},
        this.methodDescriptorCreate,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/kingsol_api.KingsolAPI/Create',
    request,
    metadata || {},
    this.methodDescriptorCreate);
  }

}

