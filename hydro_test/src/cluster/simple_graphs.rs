use hydro_lang::*;
use sha2::{Digest, Sha256};

pub struct Client {}
pub struct Server {}

pub trait GraphFunction<'a>:
    Fn(
    &Cluster<'a, Server>,
    KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>
{
}

impl<'a, F> GraphFunction<'a> for F where
    F: Fn(
        &Cluster<'a, Server>,
        KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
    )
        -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>
{
}

fn sha256(n: u32) -> u32 {
    let mut sha = Sha256::new();
    if n > 1 {
        // Recursively hash n times
        sha.update(sha256(n - 1).to_be_bytes());
    }
    sha.update(n.to_be_bytes());
    let out = sha.finalize();
    out[0].into() // Only take the 
}

// Note: H = high load, L = low load

pub fn get_graph_function<'a>(name: &str) -> impl GraphFunction<'a> {
    match name {
        "mapH_mapH_mapH" => map_h_map_h_map_h,
        "mapH_mapH_mapL" => map_h_map_h_map_l,
        "mapH_mapL_mapH" => map_h_map_l_map_h,
        "mapL_mapH_mapH" => map_l_map_h_map_h,
        "mapH_mapL_mapL" => map_h_map_l_map_l,
        "mapL_mapH_mapL" => map_l_map_h_map_l,
        "mapL_mapL_mapH" => map_l_map_l_map_h,
        "mapL_mapL_mapL" => map_l_map_l_map_l,
        _ => unimplemented!(),
    }
}

pub fn map_h_map_h_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
}

pub fn map_h_map_h_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
}

pub fn map_h_map_l_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
}

pub fn map_l_map_h_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
}

pub fn map_h_map_l_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
}

pub fn map_l_map_h_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
}

pub fn map_l_map_l_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
}

pub fn map_l_map_l_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
}

pub fn map_l_first_map_l_second_union<'a>(
    server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    let nondet = nondet!(/** test */);
    let tick = server.tick();
    let map_l1 = payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .entries()
        .batch(&tick, nondet);
    let map_l2 = payloads
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .entries()
        .batch(&tick, nondet);
    map_l1.chain(map_l2).all_ticks().into_keyed()
}

pub fn map_l_map_l_first_payload_second_union<'a>(
    server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    let nondet = nondet!(/** test */);
    let tick = server.tick();
    let map_l_map_l = payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .entries()
        .batch(&tick, nondet);
    let ticked_payloads = payloads.entries().batch(&tick, nondet);
    map_l_map_l.chain(ticked_payloads).all_ticks().into_keyed()
}

pub fn map_l_first_payload_second_union_map_l<'a>(
    server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    let nondet = nondet!(/** test */);
    let tick = server.tick();
    let map_l = payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .entries()
        .batch(&tick, nondet);
    let ticked_payloads = payloads.entries().batch(&tick, nondet);
    map_l
        .chain(ticked_payloads)
        .all_ticks()
        .into_keyed()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
}
