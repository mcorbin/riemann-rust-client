extern crate rustls;

use std::fs;
use std::io::{BufReader};
use std::sync::Arc;

fn load_certs_and_key(config: &mut rustls::ClientConfig, key_path: &str, cert_path: &str) {
    let cert_file = fs::File::open(cert_path).unwrap();
    let mut reader = BufReader::new(cert_file);
    let certs = rustls::internal::pemfile::certs(&mut reader).unwrap();

    let key_file = fs::File::open(key_path).unwrap();
    let mut reader = BufReader::new(key_file);
    let keys = rustls::internal::pemfile::rsa_private_keys(&mut reader).unwrap();
    config.set_single_client_cert(certs, keys[0].clone());
}

fn load_ca_cert_file(config: &mut rustls::ClientConfig, ca_cert_path: &str) {
    let ca_cert_file = fs::File::open(ca_cert_path).unwrap();
    let mut reader = BufReader::new(ca_cert_file);
    config.root_store.add_pem_file(&mut reader).unwrap();
}

pub fn get_config(key_path: &str, cert_path: &str, ca_cert_path: &str) -> Arc<rustls::ClientConfig> {
    let mut config = rustls::ClientConfig::new();
    load_certs_and_key(&mut config, key_path, cert_path);
    load_ca_cert_file(&mut config, ca_cert_path);
    Arc::new(config)
}
