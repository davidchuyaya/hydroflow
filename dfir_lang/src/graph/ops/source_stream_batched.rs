use quote::quote_spanned;

use super::{
    FloType, OperatorCategory, OperatorConstraints, OperatorWriteOutput, WriteContextArgs, RANGE_0,
    RANGE_1,
};

/// > 0 input streams, 1 output stream
///
/// > Arguments: A stream and a batch limit (usize)
///
/// Like [`source_stream`](super::source_stream::SOURCE_STREAM) but caps the number of items
/// pulled per tick to the given `batch_limit`. When the limit is reached the waker is
/// triggered so remaining data is processed on the next tick.
pub const SOURCE_STREAM_BATCHED: OperatorConstraints = OperatorConstraints {
    name: "source_stream_batched",
    categories: &[OperatorCategory::Source],
    hard_range_inn: RANGE_0,
    soft_range_inn: RANGE_0,
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    num_args: 2,
    persistence_args: RANGE_0,
    type_args: RANGE_0,
    is_external_input: true,
    has_singleton_output: false,
    flo_type: Some(FloType::Source),
    ports_inn: None,
    ports_out: None,
    input_delaytype_fn: |_| None,
    write_fn: |wc @ &WriteContextArgs {
                   root,
                   context,
                   op_span,
                   ident,
                   arguments,
                   ..
               },
               _| {
        let receiver = &arguments[0];
        let batch_limit = &arguments[1];
        let stream_ident = wc.make_ident("stream");
        let write_prologue = quote_spanned! {op_span=>
            let mut #stream_ident = {
                #[inline(always)]
                fn check_stream<Stream: #root::futures::stream::Stream<Item = Item> + ::std::marker::Unpin, Item>(stream: Stream)
                    -> impl #root::futures::stream::Stream<Item = Item> + ::std::marker::Unpin
                {
                    stream
                }
                check_stream(#receiver)
            };
        };
        let write_iterator = quote_spanned! {op_span=>
            let #ident = #root::dfir_pipes::pull::stream_ready(
                &mut #stream_ident,
                #context.waker(),
                #batch_limit,
            );
        };
        Ok(OperatorWriteOutput {
            write_prologue,
            write_iterator,
            ..Default::default()
        })
    },
};
