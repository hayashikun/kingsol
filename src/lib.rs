extern crate actix_web;

pub mod web_handler;
pub mod grpc_api;
pub mod redis;
pub mod kingsol_api;
mod entity;
mod use_case;
mod interactor;
mod repository;
mod redis_repository;
