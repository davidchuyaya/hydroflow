use quote::{quote_spanned, ToTokens};
use syn::parse_quote;

use super::{
    DelayType, OpInstGenerics, OperatorCategory, OperatorConstraints, OperatorInstance,
    OperatorWriteOutput, Persistence, WriteContextArgs, RANGE_0, RANGE_1,
};
use crate::{diagnostic::{Diagnostic, Level}, graph::PortIndexValue};

/// TODO
pub const ZIP_WITH_FIRST: OperatorConstraints = OperatorConstraints {
    name: "zip_with_first",
    categories: &[OperatorCategory::Persistence],
    persistence_args: RANGE_0,
    type_args: RANGE_0,
    hard_range_inn: &(2..=2),
    soft_range_inn: &(2..=2),
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    num_args: 0,
    is_external_input: false,
    has_singleton_output: false,
    ports_inn: Some(|| super::PortListSpec::Fixed(parse_quote! { input, other })),
    ports_out: None,
    input_delaytype_fn: |idx| match idx {
        PortIndexValue::Path(path) if "other" == path.to_token_stream().to_string() => {
            Some(DelayType::Stratum)
        }
        _else => None,
    },
    flow_prop_fn: None,
    write_fn: |wc @ &WriteContextArgs {
                   context,
                   hydroflow,
                   ident,
                   op_span,
                   inputs,
                   is_pull,
                   op_name,
                   op_inst:
                       OperatorInstance {
                           generics:
                               OpInstGenerics {
                                   persistence_args, ..
                               },
                           ..
                       },
                   ..
               },
               diagnostics| {
        assert!(is_pull);

        let input = &inputs[0];
        let singleton = &inputs[1];
        let s_taken_ident = wc.make_ident("singleton_taken");
        let singleton_value_ident = wc.make_ident("singleton_value");

        let write_iterator = quote_spanned! {op_span=>
            let mut #s_taken_ident = #singleton;
            let #ident = #s_taken_ident.next().map(|#singleton_value_ident| {
                #input.map(move |x| (x, ::std::clone::Clone::clone(&#singleton_value_ident)))
            }).into_iter().flatten();
        };

        Ok(OperatorWriteOutput {
            write_prologue: Default::default(),
            write_iterator,
            ..Default::default()
        })
    },
};
