use hydro_lang::*;
use sha2::{Digest, Sha256};

pub struct Client {}
pub struct Server {}

pub trait GraphFunction<'a>:
    Fn(
    &Cluster<'a, Server>,
    Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>
{
}

impl<'a, F> GraphFunction<'a> for F where
    F: Fn(
        &Cluster<'a, Server>,
        Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
    ) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>
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

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_h_map_h_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_h_map_h_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_h_map_l_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_l_map_h_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_h_map_l_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_l_map_h_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_l_map_l_map_h<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

#[expect(clippy::type_complexity, reason = "internal simple graphs code // TODO")]
pub fn map_l_map_l_map_l<'a>(
    _server: &Cluster<'a, Server>,
    payloads: Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(ClusterId<Client>, (u32, u32)), Cluster<'a, Server>, Unbounded, NoOrder> {
    payloads
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}
