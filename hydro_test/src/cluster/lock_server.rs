use hydro_lang::*;

pub struct Server {}

/// Lock server implementation as described in https://dl.acm.org/doi/pdf/10.1145/3341301.3359651, with the difference being that each server can hold multiple locks.
/// Clients send (client_id, server_id, acquire) requesting a lock from the server.
///
/// If acquire = true, then:
/// - If the server currently holds the lock, it returns (client_id, server_id, true).
/// - Otherwise, it returns (client_id, server_id, false).
///
/// If acquire = false, then the client wants to release its lock. Return (client_id, server_id, true).
pub fn lock_server<'a>(
    server: &Process<'a, Server>,
    payloads: Stream<(u32, u32, bool), Process<'a, Server>, Unbounded, NoOrder>,
) -> Stream<(u32, u32, bool), Process<'a, Server>, Unbounded, NoOrder> {
    let server_tick = server.tick();
    let keyed_payloads = payloads.map(q!(|(client_id, server_id, acquire)| (
        server_id,
        (client_id, acquire)
    )));

    let batched_payloads = unsafe {
        // SAFETY: The order in which the server processes lock requests affects the lock state.
        keyed_payloads.tick_batch(&server_tick).assume_ordering()
    };
    let lock_state = batched_payloads.clone().persist().reduce_keyed(q!(
        |(curr_client_holder, is_held_by_client), (client_id, acquire)| {
            if acquire {
                // If the lock is currently held by the server, give the client the lock
                if !*is_held_by_client {
                    *curr_client_holder = client_id;
                    *is_held_by_client = true;
                }
            } else {
                // If the client is releasing the lock and it holds it, give the lock back to the server
                if *is_held_by_client && *curr_client_holder == client_id {
                    *is_held_by_client = false;
                }
            }
        }
    ));
    let results = batched_payloads.join(lock_state).all_ticks().map(q!(|(
        server_id,
        ((client_id, acquire), (curr_client_holder, is_held_by_client)),
    )| {
        if acquire {
            let acquired = is_held_by_client && curr_client_holder == client_id;
            (server_id, client_id, acquired)
        } else {
            (server_id, client_id, true)
        }
    }));
    results
}
