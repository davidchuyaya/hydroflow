use hydroflow_plus::futures::stream::Stream;
use hydroflow_plus::tokio::sync::mpsc::UnboundedSender;
use hydroflow_plus::tokio_stream::wrappers::UnboundedReceiverStream;
use hydroflow_plus::*;
use stageleft::{q, Quoted, RuntimeData};

#[stageleft::entry(UnboundedReceiverStream<u32>)]
pub fn teed_join<'a, S: Stream<Item = u32> + Unpin + 'a>(
    flow: FlowBuilder<'a, MultiGraph>,
    input_stream: RuntimeData<S>,
    output: RuntimeData<&'a UnboundedSender<u32>>,
    send_twice: bool,
    subgraph_id: RuntimeData<usize>,
) -> impl Quoted<'a, Hydroflow<'a>> {
    let node_zero = flow.process(());
    let node_one = flow.process(());

    let source = flow.source_stream(&node_zero, input_stream);
    let map1 = source.clone().map(q!(|v| (v + 1, ())));
    let map2 = source.map(q!(|v| (v - 1, ())));

    let joined = map1.join(map2).map(q!(|t| t.0));

    joined.clone().for_each(q!(|v| {
        output.send(v).unwrap();
    }));

    if send_twice {
        joined.for_each(q!(|v| {
            output.send(v).unwrap();
        }));
    }

    let source_node_id_1 = flow.source_iter(&node_one, q!(0..5));
    source_node_id_1.for_each(q!(|v| {
        output.send(v).unwrap();
    }));

    flow.with_default_optimize()
        .compile()
        .with_dynamic_id(subgraph_id)
}

#[stageleft::runtime]
#[cfg(test)]
mod tests {
    use hydroflow_plus::assert_graphvis_snapshots;
    use hydroflow_plus::util::collect_ready;

    #[test]
    fn test_teed_join() {
        let (in_send, input) = hydroflow_plus::util::unbounded_channel();
        let (out, mut out_recv) = hydroflow_plus::util::unbounded_channel();

        let mut joined = super::teed_join!(input, &out, false, 0);
        assert_graphvis_snapshots!(joined);

        in_send.send(1).unwrap();
        in_send.send(2).unwrap();
        in_send.send(3).unwrap();
        in_send.send(4).unwrap();

        joined.run_tick();

        assert_eq!(&*collect_ready::<Vec<_>, _>(&mut out_recv), &[2, 3]);
    }

    #[test]
    fn test_teed_join_twice() {
        let (in_send, input) = hydroflow_plus::util::unbounded_channel();
        let (out, mut out_recv) = hydroflow_plus::util::unbounded_channel();

        let mut joined = super::teed_join!(input, &out, true, 0);
        assert_graphvis_snapshots!(joined);

        in_send.send(1).unwrap();
        in_send.send(2).unwrap();
        in_send.send(3).unwrap();
        in_send.send(4).unwrap();

        joined.run_tick();

        assert_eq!(&*collect_ready::<Vec<_>, _>(&mut out_recv), &[2, 2, 3, 3]);
    }

    #[test]
    fn test_teed_join_multi_node() {
        let (_, input) = hydroflow_plus::util::unbounded_channel();
        let (out, mut out_recv) = hydroflow_plus::util::unbounded_channel();

        let mut joined = super::teed_join!(input, &out, true, 1);
        assert_graphvis_snapshots!(joined);

        joined.run_tick();

        assert_eq!(
            &*collect_ready::<Vec<_>, _>(&mut out_recv),
            &[0, 1, 2, 3, 4]
        );
    }
}
