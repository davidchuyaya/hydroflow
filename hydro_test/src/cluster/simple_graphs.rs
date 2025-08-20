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
        "mapH_mapH_mapH_mapH" => mapH_mapH_mapH_mapH,
        "mapH_mapH_mapH_mapL" => mapH_mapH_mapH_mapL,
        "mapH_mapH_mapL_mapH" => mapH_mapH_mapL_mapH,
        "mapH_mapL_mapH_mapH" => mapH_mapL_mapH_mapH,
        "mapL_mapH_mapH_mapH" => mapL_mapH_mapH_mapH,
        "mapH_mapH_mapL_mapL" => mapH_mapH_mapL_mapL,
        "mapH_mapL_mapH_mapL" => mapH_mapL_mapH_mapL,
        "mapL_mapH_mapH_mapL" => mapL_mapH_mapH_mapL,
        "mapL_mapH_mapL_mapH" => mapL_mapH_mapL_mapH,
        "mapH_mapL_mapL_mapH" => mapH_mapL_mapL_mapH,
        "mapL_mapL_mapH_mapH" => mapL_mapL_mapH_mapH,
        "mapH_mapL_mapL_mapL" => mapH_mapL_mapL_mapL,
        "mapL_mapH_mapL_mapL" => mapL_mapH_mapL_mapL,
        "mapL_mapL_mapH_mapL" => mapL_mapL_mapH_mapL,
        "mapL_mapL_mapL_mapH" => mapL_mapL_mapL_mapH,
        _ => unimplemented!(),
    }
}

pub fn mapH_mapH_mapH_mapH<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

pub fn mapH_mapH_mapH_mapL<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapH_mapH_mapL_mapH<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapH_mapL_mapH_mapH<'a>(
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
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

pub fn mapL_mapH_mapH_mapH<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

pub fn mapH_mapH_mapL_mapL<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapH_mapL_mapH_mapL<'a>(
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
            (virt_client_id, self::sha256(n))
        )))
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapL_mapH_mapH_mapL<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapL_mapH_mapL_mapH<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

pub fn mapH_mapL_mapL_mapH<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

pub fn mapL_mapL_mapH_mapH<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}

pub fn mapH_mapL_mapL_mapL<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapL_mapH_mapL_mapL<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapL_mapL_mapH_mapL<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n % 2))
        )))
}

pub fn mapL_mapL_mapL_mapH<'a>(
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
        .map(q!(|(client_id, (virt_client_id, n))| (
            client_id,
            (virt_client_id, self::sha256(n))
        )))
}
