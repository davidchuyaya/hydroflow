use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::rc::Rc;
use std::fmt;

use hydroflow::tokio::signal;
use hydroflow_lang::graph::FlatGraphBuilder;
use hydroflow_lang::parse::Pipeline;
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse_quote;

#[derive(Clone)]
pub struct DebugExpr(pub Box<syn::Expr>);

impl From<syn::Expr> for DebugExpr {
    fn from(expr: syn::Expr) -> DebugExpr {
        DebugExpr(Box::new(expr))
    }
}

impl Deref for DebugExpr {
    type Target = syn::Expr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for DebugExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl std::fmt::Debug for DebugExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_token_stream())
    }
}

#[derive(Clone)]
pub struct DebugPipelineFn(pub Rc<dyn Fn() -> Pipeline + 'static>);

impl std::fmt::Debug for DebugPipelineFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function>")
    }
}

/// A source in a Hydroflow+ graph, where data enters the graph.
#[derive(Clone, Debug)]
pub enum HfPlusSource {
    Stream(DebugExpr),
    Iter(DebugExpr),
    Interval(DebugExpr),
    Spin(),
}

// a enum class that represent all types of node in the graph. for create inverted graph purpose.
#[derive(Clone, Debug)]
pub enum HfPlusGraphNode {
    HfPlusLeaf(HfPlusLeaf),
    HfPlusNode {
        node: HfPlusNode,
        next: Vec<Rc<RefCell<HfPlusGraphNode>>>,
    },
    HfPlusSource {
        node: HfPlusNode,
        next: Vec<Rc<RefCell<HfPlusGraphNode>>>,
    },
}

/// An leaf in a Hydroflow+ graph, which is an pipeline that doesn't emit
/// any downstream values. Traversals over the dataflow graph and
/// generating Hydroflow IR start from leaves.
#[derive(Clone, Debug)]
pub enum HfPlusLeaf {
    ForEach {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },
    DestSink {
        sink: DebugExpr,
        input: Box<HfPlusNode>,
    },
    CycleSink {
        ident: syn::Ident,
        location_id: usize,
        input: Box<HfPlusNode>,
    },
}

impl HfPlusLeaf {
    
    // create a inverted graph, with each node a HfPlusGraphNode, return the SourceNode
    pub fn create_inverted_graph(
        self,
        seen_tees: &mut SeenTees,
    ) -> Vec<Rc<RefCell<HfPlusGraphNode>>> {
        match self {
            HfPlusLeaf::ForEach { f, input } => {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusLeaf(
                    HfPlusLeaf::ForEach {
                        f,
                        input: input.clone(),
                    },
                )));
                input.create_inverted_graph(vec![cur], seen_tees)
            },
            HfPlusLeaf::DestSink { sink, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusLeaf(HfPlusLeaf::DestSink {
                sink,
                input: input.clone(),
            })));
                input.create_inverted_graph( vec![cur], seen_tees)
            },
            HfPlusLeaf::CycleSink {
                ident,
                location_id,
                input,
            } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusLeaf( HfPlusLeaf::CycleSink {
                ident,
                location_id,
                input: input.clone(),
            })));
                input.create_inverted_graph( vec![cur], seen_tees)
            },
        }
    }

    pub fn transform_children(
        self,
        mut transform: impl FnMut(HfPlusNode, &mut SeenTees) -> HfPlusNode,
        seen_tees: &mut SeenTees,
    ) -> HfPlusLeaf {
        match self {
            HfPlusLeaf::ForEach { f, input } => HfPlusLeaf::ForEach {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusLeaf::DestSink { sink, input } => HfPlusLeaf::DestSink {
                sink,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusLeaf::CycleSink {
                ident,
                location_id,
                input,
            } => HfPlusLeaf::CycleSink {
                ident,
                location_id,
                input: Box::new(transform(*input, seen_tees)),
            },
        }
    }

    pub fn emit(
        self,
        graph_builders: &mut BTreeMap<usize, FlatGraphBuilder>,
        built_tees: &mut HashMap<*const RefCell<HfPlusNode>, (syn::Ident, usize)>,
        next_stmt_id: &mut usize,
    ) {
        match self {
            HfPlusLeaf::ForEach { f, input } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                graph_builders
                    .entry(input_location_id)
                    .or_default()
                    .add_statement(parse_quote! {
                        #input_ident -> for_each(#f);
                    });
            }

            HfPlusLeaf::DestSink { sink, input } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                graph_builders
                    .entry(input_location_id)
                    .or_default()
                    .add_statement(parse_quote! {
                        #input_ident -> dest_sink(#sink);
                    });
            }

            HfPlusLeaf::CycleSink {
                ident,
                location_id,
                input,
            } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                assert_eq!(
                    input_location_id, location_id,
                    "cycle_sink location mismatch"
                );

                graph_builders
                    .entry(location_id)
                    .or_default()
                    .add_statement(parse_quote! {
                        #ident = #input_ident;
                    });
            }
        }
    }
}

/// An intermediate node in a Hydroflow+ graph, which consumes data
/// from upstream nodes and emits data to downstream nodes.
#[derive(Clone, Debug)]
pub enum HfPlusNode {
    Placeholder,

    Source {
        source: HfPlusSource,
        location_id: usize,
    },

    CycleSource {
        ident: syn::Ident,
        location_id: usize,
    },

    Tee {
        inner: Rc<RefCell<HfPlusNode>>,
    },

    Persist(Box<HfPlusNode>),
    Delta(Box<HfPlusNode>),

    Union(Box<HfPlusNode>, Box<HfPlusNode>),
    CrossProduct(Box<HfPlusNode>, Box<HfPlusNode>),
    ZipWithSingleton(Box<HfPlusNode>, Box<HfPlusNode>),
    Join(Box<HfPlusNode>, Box<HfPlusNode>),
    Difference(Box<HfPlusNode>, Box<HfPlusNode>),
    AntiJoin(Box<HfPlusNode>, Box<HfPlusNode>),

    Map {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },
    FlatMap {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },
    Filter {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },
    FilterMap {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },

    DeferTick(Box<HfPlusNode>),
    GateSignal(Box<HfPlusNode>, Box<HfPlusNode>),
    Enumerate(Box<HfPlusNode>),
    Inspect {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },

    Unique(Box<HfPlusNode>),

    Sort(Box<HfPlusNode>),
    Fold {
        init: DebugExpr,
        acc: DebugExpr,
        input: Box<HfPlusNode>,
    },
    FoldKeyed {
        init: DebugExpr,
        acc: DebugExpr,
        input: Box<HfPlusNode>,
    },

    Reduce {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },
    ReduceKeyed {
        f: DebugExpr,
        input: Box<HfPlusNode>,
    },

    Network {
        to_location: usize,
        serialize_pipeline: Option<Pipeline>,
        sink_expr: DebugExpr,
        source_expr: DebugExpr,
        deserialize_pipeline: Option<Pipeline>,
        input: Box<HfPlusNode>,
    },
}

pub type SeenTees = HashMap<*const RefCell<HfPlusNode>, Rc<RefCell<HfPlusNode>>>;

impl HfPlusNode {

    // return a Vec here?
    pub fn create_inverted_graph(
        self,
        prev: Vec<Rc<RefCell<HfPlusGraphNode>>>,
        seen_tees: &mut SeenTees,
    ) -> Vec<Rc<RefCell<HfPlusGraphNode>>> {
        match self {
            // if it is source, return it self.
            HfPlusNode::Placeholder => 
            {
                vec![Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {node: HfPlusNode::Placeholder, next: prev}))]
            }

            HfPlusNode::Source {
                source,
                location_id,
            } => vec![Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                node: HfPlusNode::Source {
                source,
                location_id,
            },
                next: prev,
            }))],

            HfPlusNode::CycleSource { ident, location_id } => vec![Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                node: HfPlusNode::CycleSource { ident, location_id },
                next: prev,
            }))],

            HfPlusNode::Tee { inner } => {
                if let Some(already_seen) =
                    seen_tees.get(&(inner.as_ref() as *const RefCell<HfPlusNode>))
                {
                    vec![Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                        node: HfPlusNode::Tee {
                            inner: already_seen.clone(),
                        },
                        next: prev,
                    }))]
                } else {
                    let transformed_cell = Rc::new(RefCell::new(HfPlusNode::Placeholder));
                    seen_tees.insert(
                        inner.as_ref() as *const RefCell<HfPlusNode>,
                        transformed_cell.clone(),
                    );
                    let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                        node: HfPlusNode::Tee { inner: inner.clone() },
                        next: prev,
                    }));

                    // Recursively create the inverted graph for the inner node
                    let mut result = inner.borrow().clone().create_inverted_graph(vec![cur.clone()], seen_tees);
                    // Replace the placeholder with the actual processed node
                     
                    result

                }
            },

            HfPlusNode::Persist(inner) => {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Persist(inner.clone()),
                    next: prev,
                }));
                inner.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Delta(inner) => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Delta(inner.clone()),
                    next: prev,
                }));
                inner.create_inverted_graph(vec![cur], seen_tees)
            },
            
            HfPlusNode::Union(left, right) => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Union(
                        left.clone(),
                        right.clone(),
                    ),
                    next: prev
                }));
                let mut source_left = left.create_inverted_graph(vec![cur.clone()], seen_tees);
                source_left.extend(right.create_inverted_graph(vec![cur], seen_tees));
                source_left
            },

            HfPlusNode::CrossProduct(left, right) =>
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                    node: HfPlusNode::CrossProduct(left.clone(), right.clone()),
                    next: prev
                }));
                let mut source_left = left.create_inverted_graph(vec![cur.clone()], seen_tees);
                source_left.extend(right.create_inverted_graph(vec![cur], seen_tees));
                source_left
            },

            HfPlusNode::ZipWithSingleton(left, right) =>
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                    node: HfPlusNode::ZipWithSingleton(left.clone(), right.clone()),
                    next: prev
                }));
                let mut source_left = left.create_inverted_graph(vec![cur.clone()], seen_tees);
                source_left.extend(right.create_inverted_graph(vec![cur], seen_tees));
                source_left
            },

            HfPlusNode::Join(left, right) => {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                    node: HfPlusNode::Join(left.clone(), right.clone()),
                    next: prev
                }));
                let mut source_left = left.create_inverted_graph(vec![cur.clone()], seen_tees);
                source_left.extend(right.create_inverted_graph(vec![cur], seen_tees));
                source_left
            },

            HfPlusNode::Difference(left, right) => {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                    node: HfPlusNode::Difference(left.clone(), right.clone()),
                    next: prev
                }));
                let mut source_left = left.create_inverted_graph(vec![cur.clone()], seen_tees);
                source_left.extend(right.create_inverted_graph(vec![cur], seen_tees));
                source_left
            },

            HfPlusNode::AntiJoin(left, right) => {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                    node: HfPlusNode::AntiJoin(left.clone(), right.clone()),
                    next: prev
                }));
                let mut source_left = left.create_inverted_graph(vec![cur.clone()], seen_tees);
                source_left.extend(right.create_inverted_graph(vec![cur], seen_tees));
                source_left
            },

            HfPlusNode::Map { f, input } =>             
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Map{f, input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },


            HfPlusNode::FlatMap { f, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::FlatMap{f, input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Filter { f, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Filter{f, input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::FilterMap { f, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::FilterMap{f, input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Sort(input) => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Sort(input.clone()),
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::DeferTick(input) => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::DeferTick(input.clone()),
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::GateSignal(input, signal) => {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {
                    node: HfPlusNode::GateSignal(input.clone(), signal.clone()),
                    next: prev
                }));
                let mut source_input = input.create_inverted_graph(vec![cur.clone()], seen_tees);
                source_input.extend(signal.create_inverted_graph(vec![cur], seen_tees));
                source_input
            },

            HfPlusNode::Enumerate(input) => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Enumerate(input.clone()),
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Inspect { f, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Inspect{f, input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Unique(input) => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Unique(input.clone()),
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Fold { init, acc, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Fold{init, acc, input:input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::FoldKeyed { init, acc, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::FoldKeyed{init, acc, input:input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Reduce { f, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Reduce{f, input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::ReduceKeyed { f, input } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::ReduceKeyed{f, input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            HfPlusNode::Network {
                to_location,
                serialize_pipeline,
                sink_expr,
                source_expr,
                deserialize_pipeline,
                input,
            } => 
            {
                let cur = Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode{
                    node: HfPlusNode::Network{to_location,
                        serialize_pipeline,
                        sink_expr,
                        source_expr,
                        deserialize_pipeline, 
                        input: input.clone()},
                    next: prev,
                }));
                input.create_inverted_graph(vec![cur], seen_tees)
            },

            // o => vec![Rc::new(RefCell::new(HfPlusGraphNode::HfPlusNode {node: o, next: prev}))]

        }
    }


    pub fn transform_children(
        self,
        mut transform: impl FnMut(HfPlusNode, &mut SeenTees) -> HfPlusNode,
        seen_tees: &mut SeenTees,
    ) -> HfPlusNode {
        match self {
            HfPlusNode::Placeholder => HfPlusNode::Placeholder,

            HfPlusNode::Source {
                source,
                location_id,
            } => HfPlusNode::Source {
                source,
                location_id,
            },

            HfPlusNode::CycleSource { ident, location_id } => {
                HfPlusNode::CycleSource { ident, location_id }
            }

            HfPlusNode::Tee { inner } => {
                if let Some(transformed) =
                    seen_tees.get(&(inner.as_ref() as *const RefCell<HfPlusNode>))
                {
                    HfPlusNode::Tee {
                        inner: transformed.clone(),
                    }
                } else {
                    let transformed_cell = Rc::new(RefCell::new(HfPlusNode::Placeholder));
                    seen_tees.insert(
                        inner.as_ref() as *const RefCell<HfPlusNode>,
                        transformed_cell.clone(),
                    );
                    let orig = inner.borrow().clone();
                    *transformed_cell.borrow_mut() = transform(orig, seen_tees);
                    HfPlusNode::Tee {
                        inner: transformed_cell,
                    }
                }
            }

            HfPlusNode::Persist(inner) => {
                HfPlusNode::Persist(Box::new(transform(*inner, seen_tees)))
            }
            HfPlusNode::Delta(inner) => HfPlusNode::Delta(Box::new(transform(*inner, seen_tees))),

            HfPlusNode::Union(left, right) => HfPlusNode::Union(
                Box::new(transform(*left, seen_tees)),
                Box::new(transform(*right, seen_tees)),
            ),
            HfPlusNode::CrossProduct(left, right) => HfPlusNode::CrossProduct(
                Box::new(transform(*left, seen_tees)),
                Box::new(transform(*right, seen_tees)),
            ),
            HfPlusNode::ZipWithSingleton(left, right) => HfPlusNode::ZipWithSingleton(
                Box::new(transform(*left, seen_tees)),
                Box::new(transform(*right, seen_tees)),
            ),
            HfPlusNode::Join(left, right) => HfPlusNode::Join(
                Box::new(transform(*left, seen_tees)),
                Box::new(transform(*right, seen_tees)),
            ),
            HfPlusNode::Difference(left, right) => HfPlusNode::Difference(
                Box::new(transform(*left, seen_tees)),
                Box::new(transform(*right, seen_tees)),
            ),
            HfPlusNode::AntiJoin(left, right) => HfPlusNode::AntiJoin(
                Box::new(transform(*left, seen_tees)),
                Box::new(transform(*right, seen_tees)),
            ),

            HfPlusNode::Map { f, input } => HfPlusNode::Map {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusNode::FlatMap { f, input } => HfPlusNode::FlatMap {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusNode::Filter { f, input } => HfPlusNode::Filter {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusNode::FilterMap { f, input } => HfPlusNode::FilterMap {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusNode::Sort(input) => HfPlusNode::Sort(Box::new(transform(*input, seen_tees))),
            HfPlusNode::DeferTick(input) => {
                HfPlusNode::DeferTick(Box::new(transform(*input, seen_tees)))
            }
            HfPlusNode::GateSignal(input, signal) => HfPlusNode::GateSignal(
                Box::new(transform(*input, seen_tees)),
                Box::new(transform(*signal, seen_tees)),
            ),
            HfPlusNode::Enumerate(input) => {
                HfPlusNode::Enumerate(Box::new(transform(*input, seen_tees)))
            }
            HfPlusNode::Inspect { f, input } => HfPlusNode::Inspect {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },

            HfPlusNode::Unique(input) => HfPlusNode::Unique(Box::new(transform(*input, seen_tees))),

            HfPlusNode::Fold { init, acc, input } => HfPlusNode::Fold {
                init,
                acc,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusNode::FoldKeyed { init, acc, input } => HfPlusNode::FoldKeyed {
                init,
                acc,
                input: Box::new(transform(*input, seen_tees)),
            },

            HfPlusNode::Reduce { f, input } => HfPlusNode::Reduce {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },
            HfPlusNode::ReduceKeyed { f, input } => HfPlusNode::ReduceKeyed {
                f,
                input: Box::new(transform(*input, seen_tees)),
            },

            HfPlusNode::Network {
                to_location,
                serialize_pipeline,
                sink_expr,
                source_expr,
                deserialize_pipeline,
                input,
            } => HfPlusNode::Network {
                to_location,
                serialize_pipeline,
                sink_expr,
                source_expr,
                deserialize_pipeline,
                input: Box::new(transform(*input, seen_tees)),
            },
        }
    }

    pub fn emit(
        self,
        graph_builders: &mut BTreeMap<usize, FlatGraphBuilder>,
        built_tees: &mut HashMap<*const RefCell<HfPlusNode>, (syn::Ident, usize)>,
        next_stmt_id: &mut usize,
    ) -> (syn::Ident, usize) {
        match self {
            HfPlusNode::Placeholder => {
                panic!()
            }

            HfPlusNode::Persist(inner) => {
                let (inner_ident, location) = inner.emit(graph_builders, built_tees, next_stmt_id);

                let persist_id = *next_stmt_id;
                *next_stmt_id += 1;

                let persist_ident =
                    syn::Ident::new(&format!("stream_{}", persist_id), Span::call_site());

                let builder = graph_builders.entry(location).or_default();
                builder.add_statement(parse_quote! {
                    #persist_ident = #inner_ident -> persist::<'static>();
                });

                (persist_ident, location)
            }

            HfPlusNode::Delta(inner) => {
                let (inner_ident, location) = inner.emit(graph_builders, built_tees, next_stmt_id);

                let delta_id = *next_stmt_id;
                *next_stmt_id += 1;

                let delta_ident =
                    syn::Ident::new(&format!("stream_{}", delta_id), Span::call_site());

                let builder = graph_builders.entry(location).or_default();
                builder.add_statement(parse_quote! {
                    #delta_ident = #inner_ident -> multiset_delta();
                });

                (delta_ident, location)
            }

            HfPlusNode::Source {
                source,
                location_id,
            } => {
                let source_id = *next_stmt_id;
                *next_stmt_id += 1;

                let source_ident =
                    syn::Ident::new(&format!("stream_{}", source_id), Span::call_site());

                let source_stmt = match source {
                    HfPlusSource::Stream(expr) => {
                        parse_quote! {
                            #source_ident = source_stream(#expr);
                        }
                    }

                    HfPlusSource::Iter(expr) => {
                        parse_quote! {
                            #source_ident = source_iter(#expr);
                        }
                    }

                    HfPlusSource::Interval(expr) => {
                        parse_quote! {
                            #source_ident = source_interval(#expr);
                        }
                    }

                    HfPlusSource::Spin() => {
                        parse_quote! {
                            #source_ident = spin();
                        }
                    }
                };

                graph_builders
                    .entry(location_id)
                    .or_default()
                    .add_statement(source_stmt);

                (source_ident, location_id)
            }

            HfPlusNode::CycleSource { ident, location_id } => (ident.clone(), location_id),

            HfPlusNode::Tee { inner } => {
                if let Some(ret) = built_tees.get(&(inner.as_ref() as *const RefCell<HfPlusNode>)) {
                    ret.clone()
                } else {
                    let (inner_ident, inner_location_id) = inner
                        .replace(HfPlusNode::Placeholder)
                        .emit(graph_builders, built_tees, next_stmt_id);

                    let tee_id = *next_stmt_id;
                    *next_stmt_id += 1;

                    let tee_ident =
                        syn::Ident::new(&format!("stream_{}", tee_id), Span::call_site());

                    let builder = graph_builders.entry(inner_location_id).or_default();
                    builder.add_statement(parse_quote! {
                        #tee_ident = #inner_ident -> tee();
                    });

                    built_tees.insert(
                        inner.as_ref() as *const RefCell<HfPlusNode>,
                        (tee_ident.clone(), inner_location_id),
                    );

                    (tee_ident, inner_location_id)
                }
            }

            HfPlusNode::Union(left, right) => {
                let (left_ident, left_location_id) =
                    left.emit(graph_builders, built_tees, next_stmt_id);
                let (right_ident, right_location_id) =
                    right.emit(graph_builders, built_tees, next_stmt_id);

                assert_eq!(
                    left_location_id, right_location_id,
                    "union inputs must be in the same location"
                );

                let union_id = *next_stmt_id;
                *next_stmt_id += 1;

                let union_ident =
                    syn::Ident::new(&format!("stream_{}", union_id), Span::call_site());

                let builder = graph_builders.entry(left_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #union_ident = union();
                });

                builder.add_statement(parse_quote! {
                    #left_ident -> [0]#union_ident;
                });

                builder.add_statement(parse_quote! {
                    #right_ident -> [1]#union_ident;
                });

                (union_ident, left_location_id)
            }

            HfPlusNode::ZipWithSingleton(left, right) => {
                let (left_ident, left_location_id) =
                    left.emit(graph_builders, built_tees, next_stmt_id);
                let (right_ident, right_location_id) =
                    right.emit(graph_builders, built_tees, next_stmt_id);

                assert_eq!(
                    left_location_id, right_location_id,
                    "zip_with_singleton inputs must be in the same location"
                );

                let union_id = *next_stmt_id;
                *next_stmt_id += 1;

                let union_ident =
                    syn::Ident::new(&format!("stream_{}", union_id), Span::call_site());

                let builder = graph_builders.entry(left_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #union_ident = zip_with_first();
                });

                builder.add_statement(parse_quote! {
                    #left_ident -> [input]#union_ident;
                });

                builder.add_statement(parse_quote! {
                    #right_ident -> [other]#union_ident;
                });

                (union_ident, left_location_id)
            }

            HfPlusNode::CrossProduct(..) | HfPlusNode::Join(..) => {
                let operator: syn::Ident = if matches!(self, HfPlusNode::CrossProduct(..)) {
                    parse_quote!(cross_join_multiset)
                } else {
                    parse_quote!(join_multiset)
                };

                let (HfPlusNode::CrossProduct(left, right) | HfPlusNode::Join(left, right)) = self
                else {
                    unreachable!()
                };

                let (left_inner, left_was_persist) = if let HfPlusNode::Persist(left) = *left {
                    (left, true)
                } else {
                    (left, false)
                };

                let (right_inner, right_was_persist) = if let HfPlusNode::Persist(right) = *right {
                    (right, true)
                } else {
                    (right, false)
                };

                let (left_ident, left_location_id) =
                    left_inner.emit(graph_builders, built_tees, next_stmt_id);
                let (right_ident, right_location_id) =
                    right_inner.emit(graph_builders, built_tees, next_stmt_id);

                assert_eq!(
                    left_location_id, right_location_id,
                    "join / cross product inputs must be in the same location"
                );

                let stream_id = *next_stmt_id;
                *next_stmt_id += 1;

                let stream_ident =
                    syn::Ident::new(&format!("stream_{}", stream_id), Span::call_site());

                let builder = graph_builders.entry(left_location_id).or_default();

                match (left_was_persist, right_was_persist) {
                    (true, true) => {
                        builder.add_statement(parse_quote! {
                            #stream_ident = #operator::<'static, 'static>();
                        });
                    }
                    (true, false) => {
                        builder.add_statement(parse_quote! {
                            #stream_ident = #operator::<'static, 'tick>();
                        });
                    }
                    (false, true) => {
                        builder.add_statement(parse_quote! {
                            #stream_ident = #operator::<'tick, 'static>();
                        });
                    }
                    (false, false) => {
                        builder.add_statement(parse_quote! {
                            #stream_ident = #operator::<'tick, 'tick>();
                        });
                    }
                };

                builder.add_statement(parse_quote! {
                    #left_ident -> [0]#stream_ident;
                });

                builder.add_statement(parse_quote! {
                    #right_ident -> [1]#stream_ident;
                });

                (stream_ident, left_location_id)
            }

            HfPlusNode::Difference(..) | HfPlusNode::AntiJoin(..) => {
                let operator: syn::Ident = if matches!(self, HfPlusNode::Difference(..)) {
                    parse_quote!(difference_multiset)
                } else {
                    parse_quote!(anti_join_multiset)
                };

                let (HfPlusNode::Difference(left, right) | HfPlusNode::AntiJoin(left, right)) =
                    self
                else {
                    unreachable!()
                };

                let (right, right_was_persist) = if let HfPlusNode::Persist(right) = *right {
                    (right, true)
                } else {
                    (right, false)
                };

                let (left_ident, left_location_id) =
                    left.emit(graph_builders, built_tees, next_stmt_id);
                let (right_ident, right_location_id) =
                    right.emit(graph_builders, built_tees, next_stmt_id);

                assert_eq!(
                    left_location_id, right_location_id,
                    "difference / anti join inputs must be in the same location"
                );

                let stream_id = *next_stmt_id;
                *next_stmt_id += 1;

                let stream_ident =
                    syn::Ident::new(&format!("stream_{}", stream_id), Span::call_site());

                let builder = graph_builders.entry(left_location_id).or_default();

                if right_was_persist {
                    builder.add_statement(parse_quote! {
                        #stream_ident = #operator::<'tick, 'static>();
                    });
                } else {
                    builder.add_statement(parse_quote! {
                        #stream_ident = #operator::<'tick, 'tick>();
                    });
                }

                builder.add_statement(parse_quote! {
                    #left_ident -> [pos]#stream_ident;
                });

                builder.add_statement(parse_quote! {
                    #right_ident -> [neg]#stream_ident;
                });

                (stream_ident, left_location_id)
            }

            HfPlusNode::Map { f, input } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let map_id = *next_stmt_id;
                *next_stmt_id += 1;

                let map_ident = syn::Ident::new(&format!("stream_{}", map_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #map_ident = #input_ident -> map(#f);
                });

                (map_ident, input_location_id)
            }

            HfPlusNode::FlatMap { f, input } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let flat_map_id = *next_stmt_id;
                *next_stmt_id += 1;

                let flat_map_ident =
                    syn::Ident::new(&format!("stream_{}", flat_map_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #flat_map_ident = #input_ident -> flat_map(#f);
                });

                (flat_map_ident, input_location_id)
            }

            HfPlusNode::Filter { f, input } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let filter_id = *next_stmt_id;
                *next_stmt_id += 1;

                let filter_ident =
                    syn::Ident::new(&format!("stream_{}", filter_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #filter_ident = #input_ident -> filter(#f);
                });

                (filter_ident, input_location_id)
            }

            HfPlusNode::FilterMap { f, input } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let filter_map_id = *next_stmt_id;
                *next_stmt_id += 1;

                let filter_map_ident =
                    syn::Ident::new(&format!("stream_{}", filter_map_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #filter_map_ident = #input_ident -> filter_map(#f);
                });

                (filter_map_ident, input_location_id)
            }

            HfPlusNode::Sort(input) => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let sort_id = *next_stmt_id;
                *next_stmt_id += 1;

                let sort_ident = syn::Ident::new(&format!("stream_{}", sort_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #sort_ident = #input_ident -> sort();
                });

                (sort_ident, input_location_id)
            }

            HfPlusNode::DeferTick(input) => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let defer_tick_id = *next_stmt_id;
                *next_stmt_id += 1;

                let defer_tick_ident =
                    syn::Ident::new(&format!("stream_{}", defer_tick_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #defer_tick_ident = #input_ident -> defer_tick_lazy();
                });

                (defer_tick_ident, input_location_id)
            }

            HfPlusNode::GateSignal(input, signal) => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);
                let (signal_ident, signal_location_id) =
                    signal.emit(graph_builders, built_tees, next_stmt_id);

                assert_eq!(
                    input_location_id, signal_location_id,
                    "gate_signal inputs must be in the same location"
                );

                let gate_signal_id = *next_stmt_id;
                *next_stmt_id += 1;

                let gate_signal_ident =
                    syn::Ident::new(&format!("stream_{}", gate_signal_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #input_ident -> [input]#gate_signal_ident;
                });

                builder.add_statement(parse_quote! {
                    #signal_ident -> [signal]#gate_signal_ident;
                });

                builder.add_statement(parse_quote! {
                    #gate_signal_ident = defer_signal::<'tick>();
                });

                (gate_signal_ident, input_location_id)
            }

            HfPlusNode::Enumerate(input) => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let enumerate_id = *next_stmt_id;
                *next_stmt_id += 1;

                let enumerate_ident =
                    syn::Ident::new(&format!("stream_{}", enumerate_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #enumerate_ident = #input_ident -> enumerate();
                });

                (enumerate_ident, input_location_id)
            }

            HfPlusNode::Inspect { f, input } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let inspect_id = *next_stmt_id;
                *next_stmt_id += 1;

                let inspect_ident =
                    syn::Ident::new(&format!("stream_{}", inspect_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #inspect_ident = #input_ident -> inspect(#f);
                });

                (inspect_ident, input_location_id)
            }

            HfPlusNode::Unique(input) => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let unique_id = *next_stmt_id;
                *next_stmt_id += 1;

                let unique_ident =
                    syn::Ident::new(&format!("stream_{}", unique_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                builder.add_statement(parse_quote! {
                    #unique_ident = #input_ident -> unique::<'tick>();
                });

                (unique_ident, input_location_id)
            }

            HfPlusNode::Fold { .. } | HfPlusNode::FoldKeyed { .. } => {
                let operator: syn::Ident = if matches!(self, HfPlusNode::Fold { .. }) {
                    parse_quote!(fold)
                } else {
                    parse_quote!(fold_keyed)
                };

                let (HfPlusNode::Fold { init, acc, input }
                | HfPlusNode::FoldKeyed { init, acc, input }) = self
                else {
                    unreachable!()
                };

                let (input, input_was_persist) = if let HfPlusNode::Persist(input) = *input {
                    (input, true)
                } else {
                    (input, false)
                };

                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let reduce_id = *next_stmt_id;
                *next_stmt_id += 1;

                let fold_ident =
                    syn::Ident::new(&format!("stream_{}", reduce_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                if input_was_persist {
                    builder.add_statement(parse_quote! {
                        #fold_ident = #input_ident -> #operator::<'static>(#init, #acc);
                    });
                } else {
                    builder.add_statement(parse_quote! {
                        #fold_ident = #input_ident -> #operator::<'tick>(#init, #acc);
                    });
                }

                (fold_ident, input_location_id)
            }

            HfPlusNode::Reduce { .. } | HfPlusNode::ReduceKeyed { .. } => {
                let operator: syn::Ident = if matches!(self, HfPlusNode::Reduce { .. }) {
                    parse_quote!(reduce)
                } else {
                    parse_quote!(reduce_keyed)
                };

                let (HfPlusNode::Reduce { f, input } | HfPlusNode::ReduceKeyed { f, input }) = self
                else {
                    unreachable!()
                };

                let (input, input_was_persist) = if let HfPlusNode::Persist(input) = *input {
                    (input, true)
                } else {
                    (input, false)
                };

                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let reduce_id = *next_stmt_id;
                *next_stmt_id += 1;

                let reduce_ident =
                    syn::Ident::new(&format!("stream_{}", reduce_id), Span::call_site());

                let builder = graph_builders.entry(input_location_id).or_default();
                if input_was_persist {
                    builder.add_statement(parse_quote! {
                        #reduce_ident = #input_ident -> #operator::<'static>(#f);
                    });
                } else {
                    builder.add_statement(parse_quote! {
                        #reduce_ident = #input_ident -> #operator::<'tick>(#f);
                    });
                }

                (reduce_ident, input_location_id)
            }

            HfPlusNode::Network {
                to_location,
                serialize_pipeline,
                sink_expr,
                source_expr,
                deserialize_pipeline,
                input,
            } => {
                let (input_ident, input_location_id) =
                    input.emit(graph_builders, built_tees, next_stmt_id);

                let sender_builder = graph_builders.entry(input_location_id).or_default();

                if let Some(serialize_pipeline) = serialize_pipeline {
                    sender_builder.add_statement(parse_quote! {
                        #input_ident -> #serialize_pipeline -> dest_sink(#sink_expr);
                    });
                } else {
                    sender_builder.add_statement(parse_quote! {
                        #input_ident -> dest_sink(#sink_expr);
                    });
                }

                let receiver_builder = graph_builders.entry(to_location).or_default();
                let receiver_stream_id = *next_stmt_id;
                *next_stmt_id += 1;

                let receiver_stream_ident =
                    syn::Ident::new(&format!("stream_{}", receiver_stream_id), Span::call_site());

                if let Some(deserialize_pipeline) = deserialize_pipeline {
                    receiver_builder.add_statement(parse_quote! {
                        #receiver_stream_ident = source_stream(#source_expr) -> #deserialize_pipeline;
                    });
                } else {
                    receiver_builder.add_statement(parse_quote! {
                        #receiver_stream_ident = source_stream(#source_expr);
                    });
                }

                (receiver_stream_ident, to_location)
            }
        }
    }
}


/* 

    Some formatted output for better visualization.

*/

impl fmt::Display for HfPlusGraphNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HfPlusGraphNode::HfPlusLeaf(leaf) => match leaf {
                HfPlusLeaf::ForEach { f: func, .. } => {
                    write!(f, "ForEach(f: {})", func)
                }
                HfPlusLeaf::DestSink { .. } => write!(f, "DestSink"),
                HfPlusLeaf::CycleSink { .. } => write!(f, "CycleSink"),
            },
            
            HfPlusGraphNode::HfPlusNode { node, next } => {
                match node {
                    HfPlusNode::Source { source, location_id } => {
                        write!(f, "Source(source: {:?}, location_id: {})", source, location_id)?
                    }
                    HfPlusNode::Placeholder => write!(f, "Placeholder")?,
                    HfPlusNode::CycleSource { ident, location_id } => {
                        write!(f, "CycleSource(ident, location_id: {})", location_id)?
                    }
                    HfPlusNode::Tee { inner } => {
                        write!(f, "Tee(inner: {:?})", inner)?
                    }
                    HfPlusNode::Persist(inner) => {
                        write!(f, "Persist(inner)")?
                    }
                    HfPlusNode::Delta(inner) => {
                        write!(f, "Delta(inner)")?
                    }
                    HfPlusNode::CrossProduct(left, right) => {
                        write!(f, "CrossProduct(left, right)")?
                    }
                    HfPlusNode::ZipWithSingleton(left, right) => {
                        write!(f, "ZipWithSingleton(left, right)")?
                    }
                    HfPlusNode::Join(left, right) => {
                        write!(f, "Join(left, right)")?
                    }
                    HfPlusNode::Union(_left, _right) => {
                        // write!(f, "Union(left, right)", left, right)?
                        write!(f, "Union(left, right)")?
                    }
                    HfPlusNode::Difference(left, right) => {
                        write!(f, "Difference(left, right)")?
                    }
                    HfPlusNode::AntiJoin(left, right) => {
                        write!(f, "AntiJoin(left, right)")?
                    }
                    HfPlusNode::Map { f:func, input } => {
                        write!(f, "Map(f: {}, input)", func)?
                    }
                    HfPlusNode::FlatMap { f:func, input } => {
                        write!(f, "FlatMap(f: {}, input)", func)?
                    }
                    HfPlusNode::Filter { f:func, input } => {
                        write!(f, "Filter(f: {}, input)", func)?
                    }
                    HfPlusNode::FilterMap { f:func, input } => {
                        write!(f, "FilterMap(f: {}, input)", func)?
                    }
                    HfPlusNode::DeferTick(inner) => {
                        write!(f, "DeferTick(inner)")?
                    }
                    HfPlusNode::GateSignal(input, signal) => {
                        write!(f, "GateSignal(input, signal)")?
                    }
                    HfPlusNode::Enumerate(inner) => {
                        write!(f, "Enumerate(inner)")?
                    }
                    HfPlusNode::Inspect { f:func, input } => {
                        write!(f, "Inspect(f: {}, input)", func)?
                    }
                    HfPlusNode::Unique(inner) => {
                        write!(f, "Unique(inner)")?
                    }
                    HfPlusNode::Sort(inner) => {
                        write!(f, "Sort(inner)")?
                    }
                    HfPlusNode::Fold { init, acc, input } => {
                        write!(f, "Fold(init:{}, acc:{}, inner)", init, acc)?
                    }
                    HfPlusNode::FoldKeyed { init, acc, input } => {
                        write!(f, "FoldKeyed(init:{}, acc:{}, inner)", init, acc)?
                    }
                    HfPlusNode::Reduce { f:func, input } => {
                        write!(f, "Reduce(f:{}, input)", func)?
                    }
                    HfPlusNode::ReduceKeyed { f:func, input } => {
                        write!(f, "ReduceKeyed(f:{}, input)", func)?
                    }
                    HfPlusNode::Network { to_location, serialize_pipeline, sink_expr, source_expr, deserialize_pipeline, input } => {
                        write!(f, "Network(to_location: {}, serialize_pipeline: {:?}, sink_expr: {}, source_expr: {}, deserialize_pipeline: {:?}, input)", to_location, serialize_pipeline, sink_expr, source_expr, deserialize_pipeline)?
                    }
                }
                if !next.is_empty() {
                    write!(f, " -> ")?;
                    for (i, child) in next.iter().enumerate() {
                        if i > 0 {
                            write!(f, " | ")?; // multiple output. seems there's no multiple output. TODO: change the vec! to single output
                        }
                        write!(f, "{}", child.borrow())?;
                    }
                }
                Ok(())
            }
            HfPlusGraphNode::HfPlusSource { node, next } => {
                // match node {
                //     HfPlusNode::Source { source, location_id } => {
                //         write!(f, "{}", source)?
                //     }
                //     o => write!(f, "{:?}", o)?,
                // }
                write!(f, "{}", node)?;
                if !next.is_empty() {
                    write!(f, " -> ")?;
                    for (i, child) in next.iter().enumerate() {
                        if i > 0 {
                            write!(f, " | ")?; // 多个分支的情况
                        }
                        write!(f, "{}", child.borrow())?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for DebugExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_token_stream())
    }
}

impl fmt::Display for HfPlusLeaf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HfPlusLeaf::ForEach { f: func, input } => {
                write!(f, "ForEach(f: {}) -> {}", func, input)
            }
            HfPlusLeaf::DestSink { sink, input } => {
                write!(f, "DestSink(sink: {}) -> {}", sink, input)
            }
            HfPlusLeaf::CycleSink { ident, location_id, input } => {
                write!(f, "CycleSink(ident: {}, location_id: {}) -> {}", ident, location_id, input)
            }
        }
    }
}

impl fmt::Display for HfPlusSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HfPlusSource::Stream(expr) => {
                write!(f, "Stream(expr: {})", expr)
            }

            HfPlusSource::Iter(expr) => {
                write!(f, "Iter(expr: {})", expr)
            }

            HfPlusSource::Interval(expr) => {
                write!(f, "Interval(expr: {})", expr)
            }

            HfPlusSource::Spin() => {
                write!(f, "Spin()")
            }
        }
    }
}

impl fmt::Display for HfPlusNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HfPlusNode::Placeholder => write!(f, "Placeholder"),
            HfPlusNode::Source { source, location_id } => {
                write!(f, "Source(source, location_id: {}) -> {}", location_id, source)
            }
            HfPlusNode::CycleSource { ident, location_id } => {
                write!(f, "CycleSource(ident, location_id: {}) -> {}", location_id, ident)
            }
            HfPlusNode::Tee { inner } => {
                write!(f, "Tee(inner) -> {}", inner.borrow())
            }
            HfPlusNode::Persist(inner) => {
                write!(f, "Persist(inner) -> {}", inner)
            }
            HfPlusNode::Delta(inner) => {
                write!(f, "Delta(inner) -> {}", inner)
            }
            // format output for two input, separate with |. 
            HfPlusNode::Union(left, right) => {
                write!(f, "Union(left, right): left -> {} | right -> {}", left, right)
            }
            HfPlusNode::CrossProduct(left, right) => {
                write!(f, "CrossProduct(left, right): left -> {} | right -> {}", left, right)
            }
            HfPlusNode::ZipWithSingleton(left, right) => {
                write!(f, "ZipWithSingleton(left, right): left -> {} | right -> {}", left, right)
            }
            HfPlusNode::Join(left, right) => {
                write!(f, "Join(left, right): left -> {} | right -> {}", left, right)
            }
            HfPlusNode::Difference(left, right) => {
                write!(f, "Difference(left, right): left -> {} | right -> {}", left, right)
            }
            HfPlusNode::AntiJoin(left, right) => {
                write!(f, "AntiJoin(left, right): left -> {}| right -> {}", left, right)
            }
            HfPlusNode::Map { f:func, input } => {
                write!(f, "Map(f: {}, input) -> {}", func, input)
            }
            HfPlusNode::FlatMap { f:func, input } => {
                write!(f, "FlatMap(f: {}, input) -> {}", func, input)
            }
            HfPlusNode::Filter { f:func, input } => {
                write!(f, "Filter(f: {}, input) -> {}", func, input)
            }
            HfPlusNode::FilterMap { f:func, input } => {
                write!(f, "FilterMap(f: {}, input) -> {}", func, input)
            }
            HfPlusNode::DeferTick(inner) => {
                write!(f, "DeferTick(inner) -> {}", inner)
            }
            HfPlusNode::GateSignal(input, signal) => {
                write!(f, "GateSignal(input, signal): input -> {} | signal -> {}", input, signal)
            }
            HfPlusNode::Enumerate(inner) => {
                write!(f, "Enumerate(inner) -> {}", inner)
            }
            HfPlusNode::Inspect { f:func, input } => {
                write!(f, "Inspect(f: {}, input) -> {}", func, input)
            }
            HfPlusNode::Unique(inner) => {
                write!(f, "Unique(inner) -> {}", inner)
            }
            HfPlusNode::Sort(inner) => {
                write!(f, "Sort(inner) -> {}", inner)
            }
            HfPlusNode::Fold { init, acc, input } => {
                write!(f, "Fold(init:{}, acc:{}, inner) -> {}", init, acc, input)
            }
            HfPlusNode::FoldKeyed { init, acc, input } => {
                write!(f, "FoldKeyed(init:{}, acc:{}, inner) -> {}", init, acc, input)
            }
            HfPlusNode::Reduce { f:func, input } => {
                write!(f, "Reduce(f:{}, input) -> {}", func, input)
            }
            HfPlusNode::ReduceKeyed { f:func, input } => {
                write!(f, "ReduceKeyed(f:{}, input) -> {}", func, input)
            }
            HfPlusNode::Network { to_location, serialize_pipeline, sink_expr, source_expr, deserialize_pipeline, input } => {
                write!(f, "Network(to_location: {}, serialize_pipeline: {:?}, sink_expr: {}, source_expr: {}, deserialize_pipeline: {:?}, input) -> {}", to_location, serialize_pipeline, sink_expr, source_expr, deserialize_pipeline, input)
            }

        }
    }
}