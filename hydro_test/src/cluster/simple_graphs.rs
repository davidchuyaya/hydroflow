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
        "map_h_map_h_map_h" => map_h_map_h_map_h,
        "map_h_map_h_map_l" => map_h_map_h_map_l,
        "map_h_map_l_map_h" => map_h_map_l_map_h,
        "map_l_map_h_map_h" => map_l_map_h_map_h,
        "map_h_map_l_map_l" => map_h_map_l_map_l,
        "map_l_map_h_map_l" => map_l_map_h_map_l,
        "map_l_map_l_map_h" => map_l_map_l_map_h,
        "map_l_map_l_map_l" => map_l_map_l_map_l,
        "map_l_first_map_l_second_union" => map_l_first_map_l_second_union,
        "map_l_first_map_h_second_union" => map_l_first_map_h_second_union,
        "map_h_first_map_l_second_union" => map_h_first_map_l_second_union,
        "map_h_first_map_h_second_union" => map_h_first_map_h_second_union,
        "map_l_map_l_first_payload_second_union" => map_l_map_l_first_payload_second_union,
        "map_l_map_h_first_payload_second_union" => map_l_map_h_first_payload_second_union,
        "map_h_map_l_first_payload_second_union" => map_h_map_l_first_payload_second_union,
        "map_h_map_h_first_payload_second_union" => map_h_map_h_first_payload_second_union,
        "map_l_first_payload_second_union_map_l" => map_l_first_payload_second_union_map_l,
        "map_l_first_payload_second_union_map_h" => map_l_first_payload_second_union_map_h,
        "map_h_first_payload_second_union_map_l" => map_h_first_payload_second_union_map_l,
        "map_h_first_payload_second_union_map_h" => map_h_first_payload_second_union_map_h,
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
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    let map_l1 = payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )));
    let map_l2 = payloads
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )));
    map_l1.interleave(map_l2)
}

pub fn map_l_first_map_h_second_union<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    let map_l1 = payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )));
    let map_h2 = payloads
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))));
    map_l1.interleave(map_h2)
}

pub fn map_h_first_map_l_second_union<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    let map_h1 = payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))));
    let map_l2 = payloads
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )));
    map_h1.interleave(map_l2)
}

pub fn map_h_first_map_h_second_union<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    let map_h1 = payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))));
    let map_h2 = payloads
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))));
    map_h1.interleave(map_h2)
}

pub fn map_l_map_l_first_payload_second_union<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .interleave(payloads)
}

pub fn map_l_map_h_first_payload_second_union<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .interleave(payloads)
}

pub fn map_h_map_l_first_payload_second_union<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .interleave(payloads)
}

pub fn map_h_map_h_first_payload_second_union<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .interleave(payloads)
}

pub fn map_l_first_payload_second_union_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .interleave(payloads)
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
}

pub fn map_l_first_payload_second_union_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
        .interleave(payloads)
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
}

pub fn map_h_first_payload_second_union_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .interleave(payloads)
        .map(q!(|(virt_client_id, n)| (
            virt_client_id,
            self::sha256(n % 2)
        )))
}

pub fn map_h_first_payload_second_union_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> KeyedStream<ClusterId<Client>, (u32, u32), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .clone()
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
        .interleave(payloads)
        .map(q!(|(virt_client_id, n)| (virt_client_id, self::sha256(n))))
}
