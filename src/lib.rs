extern crate actix_web;

mod entity;
pub mod interactor;
pub mod mysql_repository;
mod repository;
mod use_case;

pub mod grpc_api;
mod grpc_controller;
pub mod kingsol_api;

pub mod web_controller;
