use hydroflow::hydroflow_syntax;

fn main() {
    let mut df = hydroflow_syntax! {
        pivot = merge() -> tee();

        inn_0 = [0]pivot;
        inn_1 = [1]pivot;

        out_0 = pivot[0];
        out_1 = pivot[1];

        out_0[0] -> inn_0; // Error: `pivot[0][0]`
        out_1 -> inn_1;
    };
    df.run_available();
}
