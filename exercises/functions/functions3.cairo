// functions3.cairo
// Execute `starklings hint functions3` or use the `hint` watch subcommand for a hint.


use debug::PrintTrait;

fn main() {
    call_me(18446744073709551615);
}

fn call_me(num: u64) {
    num.print();
}
