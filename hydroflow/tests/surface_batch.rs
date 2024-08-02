use hydroflow::util::collect_ready;
use hydroflow::{assert_graphvis_snapshots, hydroflow_syntax};
use multiplatform_test::multiplatform_test;

#[multiplatform_test]
pub fn test_basic_2() {
    let (signal_tx, signal_rx) = hydroflow::util::unbounded_channel::<()>();
    let (egress_tx, mut egress_rx) = hydroflow::util::unbounded_channel();

    let mut df = hydroflow_syntax! {
        gate = defer_signal::<'static>();
        source_iter([1, 2, 3]) -> [input]gate;
        source_stream(signal_rx) -> [signal]gate;

        gate -> for_each(|x| egress_tx.send(x).unwrap());
    };
    assert_graphvis_snapshots!(df);

    df.run_available();
    let out: Vec<_> = collect_ready(&mut egress_rx);
    assert_eq!(out, [0; 0]);

    signal_tx.send(()).unwrap();
    df.run_available();

    let out: Vec<_> = collect_ready(&mut egress_rx);
    assert_eq!(out, vec![1, 2, 3]);
}

#[multiplatform_test(test, env_tracing)]
pub fn test_basic_2_tick() {
    let (signal_tx, signal_rx) = hydroflow::util::unbounded_channel::<()>();
    let (egress_tx, mut egress_rx) = hydroflow::util::unbounded_channel();

    let mut df = hydroflow_syntax! {
        gate = defer_signal::<'tick>();
        source_iter([1, 2, 3]) -> fold::<'tick>(|| 0, |acc, _v| *acc += 1) -> [input]gate;
        source_stream(signal_rx) -> [signal]gate;

        gate -> for_each(|x| egress_tx.send(x).unwrap());
    };
    assert_graphvis_snapshots!(df);

    df.run_available();
    let out: Vec<_> = collect_ready(&mut egress_rx);
    assert_eq!(out, [0; 0]);

    signal_tx.send(()).unwrap();
    println!("2nd run available");
    df.run_available();

    let out: Vec<_> = collect_ready(&mut egress_rx);
    assert_eq!(out, vec![0]);

    df.run_available();

    let out: Vec<_> = collect_ready(&mut egress_rx);
    assert_eq!(out, [0; 0]);
}
