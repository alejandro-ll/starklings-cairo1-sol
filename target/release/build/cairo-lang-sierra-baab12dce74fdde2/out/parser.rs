// auto-generated: "lalrpop 0.20.0"
// sha3: c3c7ce51e1050899ba7ea8c6cfe6c36e29790b4d2502a6ca435b517a97e31bc1
use crate::ids::*;
use crate::program::*;
use num_bigint::BigInt;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__ConcreteLibfuncLongId {

    use crate::ids::*;
    use crate::program::*;
    use num_bigint::BigInt;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(ConcreteTypeId),
        Variant2(alloc::vec::Vec<ConcreteTypeId>),
        Variant3(GenericArg),
        Variant4(alloc::vec::Vec<GenericArg>),
        Variant5(Param),
        Variant6(alloc::vec::Vec<Param>),
        Variant7(VarId),
        Variant8(alloc::vec::Vec<VarId>),
        Variant9(String),
        Variant10(BigInt),
        Variant11(bool),
        Variant12(BranchInfo),
        Variant13(alloc::vec::Vec<BranchInfo>),
        Variant14(BranchTarget),
        Variant15(Vec<ConcreteTypeId>),
        Variant16(Vec<GenericArg>),
        Variant17(Vec<Param>),
        Variant18(Vec<VarId>),
        Variant19(ConcreteLibfuncId),
        Variant20(ConcreteLibfuncLongId),
        Variant21(core::option::Option<ConcreteTypeId>),
        Variant22(ConcreteTypeLongId),
        Variant23(Option<DeclaredTypeInfo>),
        Variant24(Function),
        Variant25(alloc::vec::Vec<Function>),
        Variant26(FunctionId),
        Variant27(core::option::Option<GenericArg>),
        Variant28(GenericLibfuncId),
        Variant29(GenericTypeId),
        Variant30(i16),
        Variant31(Invocation),
        Variant32(LibfuncDeclaration),
        Variant33(alloc::vec::Vec<LibfuncDeclaration>),
        Variant34(core::option::Option<Param>),
        Variant35(Program),
        Variant36(i64),
        Variant37(Statement),
        Variant38(alloc::vec::Vec<Statement>),
        Variant39(StatementIdx),
        Variant40(TypeDeclaration),
        Variant41(alloc::vec::Vec<TypeDeclaration>),
        Variant42(u64),
        Variant43(UserTypeId),
        Variant44(core::option::Option<VarId>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 1
        5, 0, 0, 0, 0, 0, 0, 0, 0, -37, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 3
        5, 0, 0, 0, 0, 0, 0, 0, 0, -39, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 4
        5, 39, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 5
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 7
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 8
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 9
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 11
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 15
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 16
        5, 60, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 18
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 19
        0, 0, 0, 0, 0, -102, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 3, 0, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -21, -21, 0, 0, -21, 0, -21, 0, -21, 0, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -48, -48, 0, 0, -48, 0, -48, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -78, -78, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -59, -59, 0, 0, 11, 0, 12, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -77, -77, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 35, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -22, -22, 0, 0, 0, 0, 0, 0, -22, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, -103, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 49, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, -9, -9, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, -9, -9, 0, 0, 0, -9, -9,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, -85, -85, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 52, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -51, -51, 0, 0, -51, 0, -51, 0, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -60, -60, 0, 0, 11, 0, 12, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -55, -55, 0, 0, 11, 0, 12, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, -81, -81, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, -74, -74, 0, 0, 11, 0, 12, 0, -74, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -79, -79, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -134, -134, 0, 0, 11, 0, 12, 0, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -80, -80, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, -10, -10, 0, 0, 0, -10, -10,
        // State 49
        0, -52, -52, 0, 0, -52, 0, -52, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 19, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -49, -49, 0, 0, -49, 0, -49, 0, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -61, -61, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 19, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, -54, -54, 0, 0, -54, 0, -54, 0, -54, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, -86, -86, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, -50, -50, 0, 0, -50, 0, -50, 0, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -56, -56, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, -76, -76, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, -135, -135, 0, 0, 0, 0, 0, 0, -135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, -53, -53, 0, 0, -53, 0, -53, 0, -53, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, -75, -75, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 30 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -102,
        // State 20
        -141,
        // State 21
        -58,
        // State 22
        -87,
        // State 23
        -21,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        -103,
        // State 33
        0,
        // State 34
        0,
        // State 35
        -57,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            5 => 3,
            12 => match state {
                0 => 19,
                2 => 32,
                10 => 49,
                17 => 61,
                _ => 24,
            },
            13 => match state {
                6 | 12..=13 => 40,
                14 => 55,
                _ => 25,
            },
            20 => 26,
            23 => match state {
                5 => 39,
                7 => 42,
                8 => 44,
                9 => 46,
                _ => 27,
            },
            24 => 43,
            25 => 20,
            26 => 28,
            34 => 45,
            35 => match state {
                1 => 29,
                3 => 33,
                16 | 18 => 58,
                _ => 36,
            },
            37 => 30,
            38 => match state {
                11 => 50,
                15 => 56,
                _ => 37,
            },
            39 => 21,
            49 => 22,
            59 => match state {
                12 => 53,
                13 => 54,
                _ => 41,
            },
            60 => 47,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###""->""###,
        r###"":""###,
        r###""::""###,
        r###"";""###,
        r###""<""###,
        r###""=""###,
        r###"">""###,
        r###""@""###,
        r###""[""###,
        r###""]""###,
        r###""drop:""###,
        r###""dup:""###,
        r###""fallthrough""###,
        r###""false""###,
        r###""lib@""###,
        r###""libfunc""###,
        r###""return""###,
        r###""storable:""###,
        r###""true""###,
        r###""type""###,
        r###""user@""###,
        r###""ut@""###,
        r###""zero_sized:""###,
        r###""{""###,
        r###""}""###,
        r###"r#"-?[1-9][0-9]*|0"#"###,
        r###"r#"[a-zA-Z_][a-zA-Z_0-9]*"#"###,
    ];
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
    >(
        __states: &[i16],
        _: core::marker::PhantomData<(&'input ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ConcreteLibfuncLongId;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 30 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(2, _) if true => Some(0),
            Token(3, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(6, _) if true => Some(4),
            Token(7, _) if true => Some(5),
            Token(8, _) if true => Some(6),
            Token(9, _) if true => Some(7),
            Token(10, _) if true => Some(8),
            Token(11, _) if true => Some(9),
            Token(12, _) if true => Some(10),
            Token(13, _) if true => Some(11),
            Token(14, _) if true => Some(12),
            Token(15, _) if true => Some(13),
            Token(16, _) if true => Some(14),
            Token(17, _) if true => Some(15),
            Token(18, _) if true => Some(16),
            Token(19, _) if true => Some(17),
            Token(20, _) if true => Some(18),
            Token(21, _) if true => Some(19),
            Token(22, _) if true => Some(20),
            Token(23, _) if true => Some(21),
            Token(24, _) if true => Some(22),
            Token(25, _) if true => Some(23),
            Token(26, _) if true => Some(24),
            Token(27, _) if true => Some(25),
            Token(28, _) if true => Some(26),
            Token(29, _) if true => Some(27),
            Token(0, _) if true => Some(28),
            Token(1, _) if true => Some(29),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 21,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 22,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 22,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 23,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 23,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 23,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 25,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 29,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 13,
                    nonterminal_produced: 30,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 30,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 11,
                    nonterminal_produced: 31,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 32,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 33,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 34,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 34,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 36,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 9,
                    nonterminal_produced: 42,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 42,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 42,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 43,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 44,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 45,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 46,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 47,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 49,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 50,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 50,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 52,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 53,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 54,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 56,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 57,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 58,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 60,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 61,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 62,
                }
            }
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            140 => __state_machine::SimulatedReduce::Accept,
            141 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            142 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ConcreteLibfuncLongIdParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ConcreteLibfuncLongIdParser {
        pub fn new() -> ConcreteLibfuncLongIdParser {
            let __builder = super::__intern_token::new_builder();
            ConcreteLibfuncLongIdParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<ConcreteLibfuncLongId, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<ConcreteLibfuncLongId,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                __reduce83(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            84 => {
                __reduce84(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            85 => {
                __reduce85(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            86 => {
                __reduce86(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            87 => {
                __reduce87(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            88 => {
                __reduce88(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            89 => {
                __reduce89(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            90 => {
                __reduce90(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            91 => {
                __reduce91(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            92 => {
                __reduce92(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            93 => {
                __reduce93(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            94 => {
                __reduce94(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            95 => {
                __reduce95(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            96 => {
                __reduce96(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            97 => {
                __reduce97(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            98 => {
                __reduce98(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            99 => {
                __reduce99(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            100 => {
                __reduce100(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            101 => {
                __reduce101(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            102 => {
                __reduce102(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            103 => {
                __reduce103(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            104 => {
                __reduce104(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            105 => {
                __reduce105(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            106 => {
                __reduce106(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            107 => {
                __reduce107(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            108 => {
                __reduce108(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            109 => {
                __reduce109(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            110 => {
                __reduce110(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            111 => {
                __reduce111(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            112 => {
                __reduce112(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            113 => {
                __reduce113(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            114 => {
                __reduce114(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            115 => {
                __reduce115(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            116 => {
                __reduce116(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            117 => {
                __reduce117(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            118 => {
                __reduce118(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            119 => {
                __reduce119(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            120 => {
                __reduce120(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            121 => {
                __reduce121(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            122 => {
                __reduce122(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            123 => {
                __reduce123(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            124 => {
                __reduce124(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            125 => {
                __reduce125(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            126 => {
                __reduce126(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            127 => {
                __reduce127(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            128 => {
                __reduce128(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            129 => {
                __reduce129(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            130 => {
                __reduce130(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            131 => {
                __reduce131(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            132 => {
                __reduce132(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            133 => {
                __reduce133(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            134 => {
                __reduce134(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            135 => {
                __reduce135(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            136 => {
                __reduce136(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            137 => {
                __reduce137(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            138 => {
                __reduce138(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            139 => {
                __reduce139(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            140 => {
                // __ConcreteLibfuncLongId = ConcreteLibfuncLongId => ActionFn(2);
                let __sym0 = __pop_Variant20(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action2::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            141 => {
                __reduce141(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            142 => {
                __reduce142(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BigInt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BranchInfo, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BranchTarget, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteLibfuncId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteLibfuncLongId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteTypeLongId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Function, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FunctionId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericArg, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericLibfuncId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant28(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant29(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant31<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Invocation, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant31(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant32<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LibfuncDeclaration, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant32(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<DeclaredTypeInfo>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Param, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant35<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant35(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant37<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant37(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant39<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StatementIdx, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant39(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant40<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TypeDeclaration, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant40(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant43<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UserTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant43(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, VarId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<BranchInfo>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Function>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant33<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<LibfuncDeclaration>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant33(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant38<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant38(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant41<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<TypeDeclaration>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant41(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant34<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant34(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant44<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant44(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant30<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant30(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant36<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant36(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant42<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant42(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",") = ConcreteTypeId, "," => ActionFn(99);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action99::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")* =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action97::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")* = (<ConcreteTypeId> ",")+ => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action98::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")+ = ConcreteTypeId, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action115::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")+ = (<ConcreteTypeId> ",")+, ConcreteTypeId, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action116::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",") = GenericArg, "," => ActionFn(104);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action104::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")* =  => ActionFn(102);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action102::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")* = (<GenericArg> ",")+ => ActionFn(103);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action103::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")+ = GenericArg, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action119::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")+ = (<GenericArg> ",")+, GenericArg, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action120::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(89);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action89::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(87);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action87::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(88);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action88::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action123::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action124::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",") = VarId, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action94::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")* =  => ActionFn(92);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action92::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")* = (<VarId> ",")+ => ActionFn(93);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action93::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")+ = VarId, "," => ActionFn(127);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action127::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")+ = (<VarId> ",")+, VarId, "," => ActionFn(128);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action128::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BasicLabel = r#"[a-zA-Z_][a-zA-Z_0-9]*"# => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BigInt = r#"-?[1-9][0-9]*|0"# => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action57::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo = BranchTarget, "(", VarIds, ")" => ActionFn(42);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo* =  => ActionFn(63);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action63::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo* = BranchInfo+ => ActionFn(64);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo+ = BranchInfo => ActionFn(105);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action105::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo+ = BranchInfo+, BranchInfo => ActionFn(106);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action106::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchTarget = "fallthrough" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchTarget = StatementIdx => ActionFn(44);
        let __sym0 = __pop_Variant39(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = ConcreteTypeId => ActionFn(133);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action133::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> =  => ActionFn(134);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action134::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = (<ConcreteTypeId> ",")+, ConcreteTypeId => ActionFn(135);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action135::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = (<ConcreteTypeId> ",")+ => ActionFn(136);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action136::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = GenericArg => ActionFn(139);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action139::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> =  => ActionFn(140);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action140::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 20)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = (<GenericArg> ",")+, GenericArg => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action141::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = (<GenericArg> ",")+ => ActionFn(142);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action142::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(147);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action147::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(148);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action148::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(149);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action149::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(150);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action150::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = VarId => ActionFn(175);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action175::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> =  => ActionFn(176);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action176::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = (<VarId> ",")+, VarId => ActionFn(177);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action177::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = (<VarId> ",")+ => ActionFn(178);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action178::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = BasicLabel => ActionFn(50);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", GenericArgsString, ")" => ActionFn(51);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action51::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", GenericArgsString, ",", ")" => ActionFn(52);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 23)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", ")" => ActionFn(53);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action53::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "::", BasicLabel => ActionFn(54);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "::", "<", GenericArgsString, ">" => ActionFn(55);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action55::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 23)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "<", GenericArgsString, ">" => ActionFn(56);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 23)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncId = ConcreteLabel => ActionFn(16);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncId = "[", UnsignedInt, "]" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncLongId = GenericLibfuncId, "<", GenericArgs, ">" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (4, 25)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncLongId = GenericLibfuncId => ActionFn(11);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = ConcreteLabel => ActionFn(27);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = "@", ConcreteLabel => ActionFn(28);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action28::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = "[", UnsignedInt, "]" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId? = ConcreteTypeId => ActionFn(95);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action95::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId? =  => ActionFn(96);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action96::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 27)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeIds = Comma<ConcreteTypeId> => ActionFn(30);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeLongId = GenericTypeId, "<", GenericArgs, ">" => ActionFn(7);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant29(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (4, 29)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeLongId = GenericTypeId => ActionFn(8);
        let __sym0 = __pop_Variant29(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DeclaredTypeInfo = "[", "storable:", Bool, ",", "drop:", Bool, ",", "dup:", Bool, ",", "zero_sized:", Bool, "]" => ActionFn(5);
        assert!(__symbols.len() >= 13);
        let __sym12 = __pop_Variant0(__symbols);
        let __sym11 = __pop_Variant11(__symbols);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant0(__symbols);
        let __sym8 = __pop_Variant11(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym12.2;
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10, __sym11, __sym12);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (13, 30)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DeclaredTypeInfo =  => ActionFn(6);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action6::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce68<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = FunctionId, "@", StatementIdx, "(", Params, ")", "->", "(", ConcreteTypeIds, ")", ";" => ActionFn(12);
        assert!(__symbols.len() >= 11);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant0(__symbols);
        let __sym8 = __pop_Variant15(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant39(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0;
        let __end = __sym10.2;
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (11, 31)
    }
    pub(crate) fn __reduce69<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function* =  => ActionFn(69);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action69::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 32)
    }
    pub(crate) fn __reduce70<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function* = Function+ => ActionFn(70);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce71<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function+ = Function => ActionFn(83);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action83::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce72<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function+ = Function+, Function => ActionFn(84);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant24(__symbols);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action84::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (2, 33)
    }
    pub(crate) fn __reduce73<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = ConcreteLabel => ActionFn(18);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce74<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = ConcreteLabel, "[", BasicLabel, "]" => ActionFn(19);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (4, 34)
    }
    pub(crate) fn __reduce75<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = "[", UnsignedInt, "]" => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce76<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = ConcreteTypeId => ActionFn(32);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce77<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = BigInt => ActionFn(33);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action33::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce78<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "user@", FunctionId => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant26(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action34::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce79<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "ut@", UserTypeId => ActionFn(35);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant43(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action35::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce80<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "lib@", ConcreteLibfuncId => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action36::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce81<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg? = GenericArg => ActionFn(100);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce82<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg? =  => ActionFn(101);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action101::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (0, 36)
    }
    pub(crate) fn __reduce83<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgs = Comma<GenericArg> => ActionFn(37);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce84<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgsString = GenericArg => ActionFn(48);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce85<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgsString = GenericArgsString, ",", GenericArg => ActionFn(49);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 38)
    }
    pub(crate) fn __reduce86<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericLibfuncId = PathLabel => ActionFn(15);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce87<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericTypeId = PathLabel => ActionFn(26);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce88<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // I16 = BigInt => ActionFn(62);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce89<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "->", "(", VarIds, ")", ";" => ActionFn(40);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant18(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym8.2;
        let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (9, 42)
    }
    pub(crate) fn __reduce90<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "{", "}", ";" => ActionFn(131);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action131::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (7, 42)
    }
    pub(crate) fn __reduce91<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "{", BranchInfo+, "}", ";" => ActionFn(132);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant13(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym7.2;
        let __nt = super::__action132::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (8, 42)
    }
    pub(crate) fn __reduce92<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration = "libfunc", ConcreteLibfuncId, "=", ConcreteLibfuncLongId, ";" => ActionFn(9);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant20(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant32(__nt), __end));
        (5, 43)
    }
    pub(crate) fn __reduce93<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration* =  => ActionFn(73);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action73::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (0, 44)
    }
    pub(crate) fn __reduce94<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration* = LibfuncDeclaration+ => ActionFn(74);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action74::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce95<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration+ = LibfuncDeclaration => ActionFn(79);
        let __sym0 = __pop_Variant32(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action79::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce96<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration+ = LibfuncDeclaration+, LibfuncDeclaration => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant32(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action80::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (2, 45)
    }
    pub(crate) fn __reduce97<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param = VarId, ":", ConcreteTypeId => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce98<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(85);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action85::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce99<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(86);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action86::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (0, 47)
    }
    pub(crate) fn __reduce100<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = Comma<Param> => ActionFn(14);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce101<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PathLabel = BasicLabel => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce102<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PathLabel = PathLabel, "::", BasicLabel => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce103<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(159);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action159::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (0, 50)
    }
    pub(crate) fn __reduce104<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+ => ActionFn(160);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action160::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce105<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(161);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action161::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce106<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Statement+ => ActionFn(162);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action162::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce107<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+ => ActionFn(163);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action163::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce108<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+ => ActionFn(164);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action164::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce109<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Statement+ => ActionFn(165);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action165::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce110<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Statement+ => ActionFn(166);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant38(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action166::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce111<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Function+ => ActionFn(167);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action167::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce112<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Function+ => ActionFn(168);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action168::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce113<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+, Function+ => ActionFn(169);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action169::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce114<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Statement+, Function+ => ActionFn(170);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action170::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce115<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Function+ => ActionFn(171);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action171::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce116<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Function+ => ActionFn(172);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action172::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce117<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Statement+, Function+ => ActionFn(173);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action173::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce118<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Statement+, Function+ => ActionFn(174);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant38(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action174::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (4, 50)
    }
    pub(crate) fn __reduce119<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SignedInt = BigInt => ActionFn(61);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant36(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce120<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Invocation => ActionFn(38);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce121<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = "return", "(", VarIds, ")", ";" => ActionFn(39);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (5, 52)
    }
    pub(crate) fn __reduce122<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action71::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce123<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(72);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action72::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce124<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(81);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action81::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce125<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant37(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action82::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce126<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StatementIdx = UnsignedInt => ActionFn(31);
        let __sym0 = __pop_Variant42(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant39(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce127<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration = "type", ConcreteTypeId, "=", ConcreteTypeLongId, DeclaredTypeInfo, ";" => ActionFn(4);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant22(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant40(__nt), __end));
        (6, 56)
    }
    pub(crate) fn __reduce128<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration* =  => ActionFn(75);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action75::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (0, 57)
    }
    pub(crate) fn __reduce129<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration* = TypeDeclaration+ => ActionFn(76);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action76::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce130<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration+ = TypeDeclaration => ActionFn(77);
        let __sym0 = __pop_Variant40(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce131<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration+ = TypeDeclaration+, TypeDeclaration => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant40(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action78::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (2, 58)
    }
    pub(crate) fn __reduce132<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnsignedInt = BigInt => ActionFn(60);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant42(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce133<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UserTypeId = ConcreteLabel => ActionFn(21);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant43(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce134<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UserTypeId = "[", BigInt, "]" => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant43(__nt), __end));
        (3, 60)
    }
    pub(crate) fn __reduce135<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId = BasicLabel => ActionFn(23);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce136<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId = "[", UnsignedInt, "]" => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action24::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 61)
    }
    pub(crate) fn __reduce137<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId? = VarId => ActionFn(90);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action90::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant44(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce138<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId? =  => ActionFn(91);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action91::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant44(__nt), __end));
        (0, 62)
    }
    pub(crate) fn __reduce139<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarIds = Comma<VarId> => ActionFn(25);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce141<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __ConcreteTypeLongId = ConcreteTypeLongId => ActionFn(1);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 65)
    }
    pub(crate) fn __reduce142<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant35(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 66)
    }
}
pub use self::__parse__ConcreteLibfuncLongId::ConcreteLibfuncLongIdParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__ConcreteTypeLongId {

    use crate::ids::*;
    use crate::program::*;
    use num_bigint::BigInt;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(ConcreteTypeId),
        Variant2(alloc::vec::Vec<ConcreteTypeId>),
        Variant3(GenericArg),
        Variant4(alloc::vec::Vec<GenericArg>),
        Variant5(Param),
        Variant6(alloc::vec::Vec<Param>),
        Variant7(VarId),
        Variant8(alloc::vec::Vec<VarId>),
        Variant9(String),
        Variant10(BigInt),
        Variant11(bool),
        Variant12(BranchInfo),
        Variant13(alloc::vec::Vec<BranchInfo>),
        Variant14(BranchTarget),
        Variant15(Vec<ConcreteTypeId>),
        Variant16(Vec<GenericArg>),
        Variant17(Vec<Param>),
        Variant18(Vec<VarId>),
        Variant19(ConcreteLibfuncId),
        Variant20(ConcreteLibfuncLongId),
        Variant21(core::option::Option<ConcreteTypeId>),
        Variant22(ConcreteTypeLongId),
        Variant23(Option<DeclaredTypeInfo>),
        Variant24(Function),
        Variant25(alloc::vec::Vec<Function>),
        Variant26(FunctionId),
        Variant27(core::option::Option<GenericArg>),
        Variant28(GenericLibfuncId),
        Variant29(GenericTypeId),
        Variant30(i16),
        Variant31(Invocation),
        Variant32(LibfuncDeclaration),
        Variant33(alloc::vec::Vec<LibfuncDeclaration>),
        Variant34(core::option::Option<Param>),
        Variant35(Program),
        Variant36(i64),
        Variant37(Statement),
        Variant38(alloc::vec::Vec<Statement>),
        Variant39(StatementIdx),
        Variant40(TypeDeclaration),
        Variant41(alloc::vec::Vec<TypeDeclaration>),
        Variant42(u64),
        Variant43(UserTypeId),
        Variant44(core::option::Option<VarId>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 1
        5, 0, 0, 0, 0, 0, 0, 0, 0, -37, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 3
        5, 0, 0, 0, 0, 0, 0, 0, 0, -39, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 4
        5, 39, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 5
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 7
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 8
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 9
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 11
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0,
        // State 15
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 16
        5, 60, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 18
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 9, 10, 0, 0, 0, 32, 24,
        // State 19
        0, 0, 0, 0, 0, -102, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 3, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -21, -21, 0, 0, -21, 0, -21, 0, -21, 0, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -48, -48, 0, 0, -48, 0, -48, 0, -48, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -78, -78, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -59, -59, 0, 0, 11, 0, 12, 0, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -77, -77, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 35, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -22, -22, 0, 0, 0, 0, 0, 0, -22, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, -103, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 49, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, -9, -9, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, -9, -9, 0, 0, 0, -9, -9,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, -85, -85, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 52, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -51, -51, 0, 0, -51, 0, -51, 0, -51, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -60, -60, 0, 0, 11, 0, 12, 0, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, -55, -55, 0, 0, 11, 0, 12, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, -81, -81, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, -74, -74, 0, 0, 11, 0, 12, 0, -74, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -79, -79, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -134, -134, 0, 0, 11, 0, 12, 0, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -80, -80, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, -10, -10, 0, 0, 0, -10, -10,
        // State 49
        0, -52, -52, 0, 0, -52, 0, -52, 0, -52, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 19, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -49, -49, 0, 0, -49, 0, -49, 0, -49, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -61, -61, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 19, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, -54, -54, 0, 0, -54, 0, -54, 0, -54, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, -86, -86, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, -50, -50, 0, 0, -50, 0, -50, 0, -50, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, -56, -56, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, -76, -76, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, -135, -135, 0, 0, 0, 0, 0, 0, -135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, -53, -53, 0, 0, -53, 0, -53, 0, -53, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, -75, -75, 0, 0, 0, 0, 0, 0, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 30 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -102,
        // State 20
        -142,
        // State 21
        -66,
        // State 22
        -88,
        // State 23
        -21,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        -103,
        // State 33
        0,
        // State 34
        0,
        // State 35
        -65,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            5 => 3,
            12 => match state {
                0 => 19,
                2 => 32,
                10 => 49,
                17 => 61,
                _ => 24,
            },
            13 => match state {
                6 | 12..=13 => 40,
                14 => 55,
                _ => 25,
            },
            20 => 26,
            23 => match state {
                5 => 39,
                7 => 42,
                8 => 44,
                9 => 46,
                _ => 27,
            },
            24 => 43,
            26 => 28,
            29 => 20,
            34 => 45,
            35 => match state {
                1 => 29,
                3 => 33,
                16 | 18 => 58,
                _ => 36,
            },
            37 => 30,
            38 => match state {
                11 => 50,
                15 => 56,
                _ => 37,
            },
            40 => 21,
            49 => 22,
            59 => match state {
                12 => 53,
                13 => 54,
                _ => 41,
            },
            60 => 47,
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###""->""###,
        r###"":""###,
        r###""::""###,
        r###"";""###,
        r###""<""###,
        r###""=""###,
        r###"">""###,
        r###""@""###,
        r###""[""###,
        r###""]""###,
        r###""drop:""###,
        r###""dup:""###,
        r###""fallthrough""###,
        r###""false""###,
        r###""lib@""###,
        r###""libfunc""###,
        r###""return""###,
        r###""storable:""###,
        r###""true""###,
        r###""type""###,
        r###""user@""###,
        r###""ut@""###,
        r###""zero_sized:""###,
        r###""{""###,
        r###""}""###,
        r###"r#"-?[1-9][0-9]*|0"#"###,
        r###"r#"[a-zA-Z_][a-zA-Z_0-9]*"#"###,
    ];
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
    >(
        __states: &[i16],
        _: core::marker::PhantomData<(&'input ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = ConcreteTypeLongId;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 30 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(2, _) if true => Some(0),
            Token(3, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(6, _) if true => Some(4),
            Token(7, _) if true => Some(5),
            Token(8, _) if true => Some(6),
            Token(9, _) if true => Some(7),
            Token(10, _) if true => Some(8),
            Token(11, _) if true => Some(9),
            Token(12, _) if true => Some(10),
            Token(13, _) if true => Some(11),
            Token(14, _) if true => Some(12),
            Token(15, _) if true => Some(13),
            Token(16, _) if true => Some(14),
            Token(17, _) if true => Some(15),
            Token(18, _) if true => Some(16),
            Token(19, _) if true => Some(17),
            Token(20, _) if true => Some(18),
            Token(21, _) if true => Some(19),
            Token(22, _) if true => Some(20),
            Token(23, _) if true => Some(21),
            Token(24, _) if true => Some(22),
            Token(25, _) if true => Some(23),
            Token(26, _) if true => Some(24),
            Token(27, _) if true => Some(25),
            Token(28, _) if true => Some(26),
            Token(29, _) if true => Some(27),
            Token(0, _) if true => Some(28),
            Token(1, _) if true => Some(29),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 21,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 22,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 22,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 23,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 23,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 23,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 25,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 29,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 13,
                    nonterminal_produced: 30,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 30,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 11,
                    nonterminal_produced: 31,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 32,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 33,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 34,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 34,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 36,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 9,
                    nonterminal_produced: 42,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 42,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 42,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 43,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 44,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 45,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 46,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 47,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 49,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 50,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 50,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 52,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 53,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 54,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 56,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 57,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 58,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 60,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 61,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 62,
                }
            }
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            140 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            141 => __state_machine::SimulatedReduce::Accept,
            142 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 66,
                }
            }
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ConcreteTypeLongIdParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ConcreteTypeLongIdParser {
        pub fn new() -> ConcreteTypeLongIdParser {
            let __builder = super::__intern_token::new_builder();
            ConcreteTypeLongIdParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<ConcreteTypeLongId, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<ConcreteTypeLongId,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                __reduce83(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            84 => {
                __reduce84(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            85 => {
                __reduce85(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            86 => {
                __reduce86(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            87 => {
                __reduce87(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            88 => {
                __reduce88(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            89 => {
                __reduce89(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            90 => {
                __reduce90(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            91 => {
                __reduce91(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            92 => {
                __reduce92(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            93 => {
                __reduce93(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            94 => {
                __reduce94(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            95 => {
                __reduce95(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            96 => {
                __reduce96(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            97 => {
                __reduce97(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            98 => {
                __reduce98(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            99 => {
                __reduce99(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            100 => {
                __reduce100(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            101 => {
                __reduce101(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            102 => {
                __reduce102(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            103 => {
                __reduce103(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            104 => {
                __reduce104(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            105 => {
                __reduce105(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            106 => {
                __reduce106(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            107 => {
                __reduce107(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            108 => {
                __reduce108(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            109 => {
                __reduce109(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            110 => {
                __reduce110(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            111 => {
                __reduce111(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            112 => {
                __reduce112(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            113 => {
                __reduce113(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            114 => {
                __reduce114(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            115 => {
                __reduce115(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            116 => {
                __reduce116(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            117 => {
                __reduce117(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            118 => {
                __reduce118(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            119 => {
                __reduce119(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            120 => {
                __reduce120(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            121 => {
                __reduce121(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            122 => {
                __reduce122(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            123 => {
                __reduce123(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            124 => {
                __reduce124(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            125 => {
                __reduce125(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            126 => {
                __reduce126(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            127 => {
                __reduce127(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            128 => {
                __reduce128(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            129 => {
                __reduce129(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            130 => {
                __reduce130(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            131 => {
                __reduce131(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            132 => {
                __reduce132(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            133 => {
                __reduce133(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            134 => {
                __reduce134(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            135 => {
                __reduce135(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            136 => {
                __reduce136(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            137 => {
                __reduce137(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            138 => {
                __reduce138(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            139 => {
                __reduce139(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            140 => {
                __reduce140(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            141 => {
                // __ConcreteTypeLongId = ConcreteTypeLongId => ActionFn(1);
                let __sym0 = __pop_Variant22(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            142 => {
                __reduce142(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BigInt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BranchInfo, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BranchTarget, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteLibfuncId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteLibfuncLongId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteTypeLongId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Function, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FunctionId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericArg, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericLibfuncId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant28(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant29(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant31<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Invocation, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant31(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant32<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LibfuncDeclaration, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant32(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<DeclaredTypeInfo>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Param, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant35<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant35(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant37<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant37(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant39<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StatementIdx, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant39(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant40<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TypeDeclaration, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant40(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant43<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UserTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant43(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, VarId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<BranchInfo>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Function>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant33<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<LibfuncDeclaration>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant33(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant38<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant38(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant41<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<TypeDeclaration>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant41(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant34<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant34(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant44<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant44(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant30<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant30(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant36<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant36(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant42<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant42(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",") = ConcreteTypeId, "," => ActionFn(99);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action99::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")* =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action97::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")* = (<ConcreteTypeId> ",")+ => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action98::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")+ = ConcreteTypeId, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action115::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")+ = (<ConcreteTypeId> ",")+, ConcreteTypeId, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action116::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",") = GenericArg, "," => ActionFn(104);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action104::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")* =  => ActionFn(102);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action102::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")* = (<GenericArg> ",")+ => ActionFn(103);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action103::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")+ = GenericArg, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action119::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")+ = (<GenericArg> ",")+, GenericArg, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action120::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(89);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action89::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(87);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action87::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(88);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action88::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action123::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action124::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",") = VarId, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action94::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")* =  => ActionFn(92);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action92::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")* = (<VarId> ",")+ => ActionFn(93);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action93::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")+ = VarId, "," => ActionFn(127);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action127::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")+ = (<VarId> ",")+, VarId, "," => ActionFn(128);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action128::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BasicLabel = r#"[a-zA-Z_][a-zA-Z_0-9]*"# => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BigInt = r#"-?[1-9][0-9]*|0"# => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action57::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo = BranchTarget, "(", VarIds, ")" => ActionFn(42);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo* =  => ActionFn(63);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action63::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo* = BranchInfo+ => ActionFn(64);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo+ = BranchInfo => ActionFn(105);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action105::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo+ = BranchInfo+, BranchInfo => ActionFn(106);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action106::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchTarget = "fallthrough" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchTarget = StatementIdx => ActionFn(44);
        let __sym0 = __pop_Variant39(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = ConcreteTypeId => ActionFn(133);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action133::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> =  => ActionFn(134);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action134::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = (<ConcreteTypeId> ",")+, ConcreteTypeId => ActionFn(135);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action135::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = (<ConcreteTypeId> ",")+ => ActionFn(136);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action136::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = GenericArg => ActionFn(139);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action139::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> =  => ActionFn(140);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action140::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 20)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = (<GenericArg> ",")+, GenericArg => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action141::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = (<GenericArg> ",")+ => ActionFn(142);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action142::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(147);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action147::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(148);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action148::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(149);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action149::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(150);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action150::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = VarId => ActionFn(175);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action175::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> =  => ActionFn(176);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action176::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = (<VarId> ",")+, VarId => ActionFn(177);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action177::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = (<VarId> ",")+ => ActionFn(178);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action178::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = BasicLabel => ActionFn(50);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", GenericArgsString, ")" => ActionFn(51);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action51::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", GenericArgsString, ",", ")" => ActionFn(52);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 23)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", ")" => ActionFn(53);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action53::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "::", BasicLabel => ActionFn(54);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "::", "<", GenericArgsString, ">" => ActionFn(55);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action55::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 23)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "<", GenericArgsString, ">" => ActionFn(56);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 23)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncId = ConcreteLabel => ActionFn(16);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncId = "[", UnsignedInt, "]" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncLongId = GenericLibfuncId, "<", GenericArgs, ">" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (4, 25)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncLongId = GenericLibfuncId => ActionFn(11);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = ConcreteLabel => ActionFn(27);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = "@", ConcreteLabel => ActionFn(28);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action28::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = "[", UnsignedInt, "]" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId? = ConcreteTypeId => ActionFn(95);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action95::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId? =  => ActionFn(96);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action96::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 27)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeIds = Comma<ConcreteTypeId> => ActionFn(30);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeLongId = GenericTypeId, "<", GenericArgs, ">" => ActionFn(7);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant29(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (4, 29)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeLongId = GenericTypeId => ActionFn(8);
        let __sym0 = __pop_Variant29(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DeclaredTypeInfo = "[", "storable:", Bool, ",", "drop:", Bool, ",", "dup:", Bool, ",", "zero_sized:", Bool, "]" => ActionFn(5);
        assert!(__symbols.len() >= 13);
        let __sym12 = __pop_Variant0(__symbols);
        let __sym11 = __pop_Variant11(__symbols);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant0(__symbols);
        let __sym8 = __pop_Variant11(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym12.2;
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10, __sym11, __sym12);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (13, 30)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DeclaredTypeInfo =  => ActionFn(6);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action6::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce68<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = FunctionId, "@", StatementIdx, "(", Params, ")", "->", "(", ConcreteTypeIds, ")", ";" => ActionFn(12);
        assert!(__symbols.len() >= 11);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant0(__symbols);
        let __sym8 = __pop_Variant15(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant39(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0;
        let __end = __sym10.2;
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (11, 31)
    }
    pub(crate) fn __reduce69<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function* =  => ActionFn(69);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action69::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 32)
    }
    pub(crate) fn __reduce70<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function* = Function+ => ActionFn(70);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce71<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function+ = Function => ActionFn(83);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action83::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce72<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function+ = Function+, Function => ActionFn(84);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant24(__symbols);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action84::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (2, 33)
    }
    pub(crate) fn __reduce73<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = ConcreteLabel => ActionFn(18);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce74<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = ConcreteLabel, "[", BasicLabel, "]" => ActionFn(19);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (4, 34)
    }
    pub(crate) fn __reduce75<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = "[", UnsignedInt, "]" => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce76<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = ConcreteTypeId => ActionFn(32);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce77<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = BigInt => ActionFn(33);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action33::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce78<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "user@", FunctionId => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant26(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action34::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce79<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "ut@", UserTypeId => ActionFn(35);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant43(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action35::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce80<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "lib@", ConcreteLibfuncId => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action36::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce81<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg? = GenericArg => ActionFn(100);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce82<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg? =  => ActionFn(101);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action101::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (0, 36)
    }
    pub(crate) fn __reduce83<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgs = Comma<GenericArg> => ActionFn(37);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce84<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgsString = GenericArg => ActionFn(48);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce85<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgsString = GenericArgsString, ",", GenericArg => ActionFn(49);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 38)
    }
    pub(crate) fn __reduce86<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericLibfuncId = PathLabel => ActionFn(15);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce87<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericTypeId = PathLabel => ActionFn(26);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce88<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // I16 = BigInt => ActionFn(62);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce89<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "->", "(", VarIds, ")", ";" => ActionFn(40);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant18(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym8.2;
        let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (9, 42)
    }
    pub(crate) fn __reduce90<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "{", "}", ";" => ActionFn(131);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action131::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (7, 42)
    }
    pub(crate) fn __reduce91<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "{", BranchInfo+, "}", ";" => ActionFn(132);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant13(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym7.2;
        let __nt = super::__action132::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (8, 42)
    }
    pub(crate) fn __reduce92<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration = "libfunc", ConcreteLibfuncId, "=", ConcreteLibfuncLongId, ";" => ActionFn(9);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant20(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant32(__nt), __end));
        (5, 43)
    }
    pub(crate) fn __reduce93<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration* =  => ActionFn(73);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action73::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (0, 44)
    }
    pub(crate) fn __reduce94<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration* = LibfuncDeclaration+ => ActionFn(74);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action74::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce95<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration+ = LibfuncDeclaration => ActionFn(79);
        let __sym0 = __pop_Variant32(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action79::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce96<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration+ = LibfuncDeclaration+, LibfuncDeclaration => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant32(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action80::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (2, 45)
    }
    pub(crate) fn __reduce97<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param = VarId, ":", ConcreteTypeId => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce98<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(85);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action85::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce99<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(86);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action86::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (0, 47)
    }
    pub(crate) fn __reduce100<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = Comma<Param> => ActionFn(14);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce101<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PathLabel = BasicLabel => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce102<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PathLabel = PathLabel, "::", BasicLabel => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce103<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(159);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action159::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (0, 50)
    }
    pub(crate) fn __reduce104<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+ => ActionFn(160);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action160::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce105<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(161);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action161::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce106<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Statement+ => ActionFn(162);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action162::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce107<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+ => ActionFn(163);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action163::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce108<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+ => ActionFn(164);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action164::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce109<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Statement+ => ActionFn(165);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action165::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce110<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Statement+ => ActionFn(166);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant38(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action166::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce111<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Function+ => ActionFn(167);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action167::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce112<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Function+ => ActionFn(168);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action168::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce113<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+, Function+ => ActionFn(169);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action169::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce114<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Statement+, Function+ => ActionFn(170);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action170::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce115<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Function+ => ActionFn(171);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action171::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce116<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Function+ => ActionFn(172);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action172::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce117<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Statement+, Function+ => ActionFn(173);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action173::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce118<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Statement+, Function+ => ActionFn(174);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant38(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action174::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (4, 50)
    }
    pub(crate) fn __reduce119<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SignedInt = BigInt => ActionFn(61);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant36(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce120<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Invocation => ActionFn(38);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce121<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = "return", "(", VarIds, ")", ";" => ActionFn(39);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (5, 52)
    }
    pub(crate) fn __reduce122<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action71::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce123<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(72);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action72::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce124<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(81);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action81::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce125<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant37(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action82::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce126<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StatementIdx = UnsignedInt => ActionFn(31);
        let __sym0 = __pop_Variant42(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant39(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce127<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration = "type", ConcreteTypeId, "=", ConcreteTypeLongId, DeclaredTypeInfo, ";" => ActionFn(4);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant22(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant40(__nt), __end));
        (6, 56)
    }
    pub(crate) fn __reduce128<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration* =  => ActionFn(75);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action75::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (0, 57)
    }
    pub(crate) fn __reduce129<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration* = TypeDeclaration+ => ActionFn(76);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action76::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce130<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration+ = TypeDeclaration => ActionFn(77);
        let __sym0 = __pop_Variant40(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce131<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration+ = TypeDeclaration+, TypeDeclaration => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant40(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action78::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (2, 58)
    }
    pub(crate) fn __reduce132<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnsignedInt = BigInt => ActionFn(60);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant42(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce133<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UserTypeId = ConcreteLabel => ActionFn(21);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant43(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce134<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UserTypeId = "[", BigInt, "]" => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant43(__nt), __end));
        (3, 60)
    }
    pub(crate) fn __reduce135<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId = BasicLabel => ActionFn(23);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce136<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId = "[", UnsignedInt, "]" => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action24::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 61)
    }
    pub(crate) fn __reduce137<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId? = VarId => ActionFn(90);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action90::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant44(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce138<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId? =  => ActionFn(91);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action91::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant44(__nt), __end));
        (0, 62)
    }
    pub(crate) fn __reduce139<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarIds = Comma<VarId> => ActionFn(25);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce140<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __ConcreteLibfuncLongId = ConcreteLibfuncLongId => ActionFn(2);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce142<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant35(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 66)
    }
}
pub use self::__parse__ConcreteTypeLongId::ConcreteTypeLongIdParser;

#[rustfmt::skip]
#[allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]
mod __parse__Program {

    use crate::ids::*;
    use crate::program::*;
    use num_bigint::BigInt;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(ConcreteTypeId),
        Variant2(alloc::vec::Vec<ConcreteTypeId>),
        Variant3(GenericArg),
        Variant4(alloc::vec::Vec<GenericArg>),
        Variant5(Param),
        Variant6(alloc::vec::Vec<Param>),
        Variant7(VarId),
        Variant8(alloc::vec::Vec<VarId>),
        Variant9(String),
        Variant10(BigInt),
        Variant11(bool),
        Variant12(BranchInfo),
        Variant13(alloc::vec::Vec<BranchInfo>),
        Variant14(BranchTarget),
        Variant15(Vec<ConcreteTypeId>),
        Variant16(Vec<GenericArg>),
        Variant17(Vec<Param>),
        Variant18(Vec<VarId>),
        Variant19(ConcreteLibfuncId),
        Variant20(ConcreteLibfuncLongId),
        Variant21(core::option::Option<ConcreteTypeId>),
        Variant22(ConcreteTypeLongId),
        Variant23(Option<DeclaredTypeInfo>),
        Variant24(Function),
        Variant25(alloc::vec::Vec<Function>),
        Variant26(FunctionId),
        Variant27(core::option::Option<GenericArg>),
        Variant28(GenericLibfuncId),
        Variant29(GenericTypeId),
        Variant30(i16),
        Variant31(Invocation),
        Variant32(LibfuncDeclaration),
        Variant33(alloc::vec::Vec<LibfuncDeclaration>),
        Variant34(core::option::Option<Param>),
        Variant35(Program),
        Variant36(i64),
        Variant37(Statement),
        Variant38(alloc::vec::Vec<Statement>),
        Variant39(StatementIdx),
        Variant40(TypeDeclaration),
        Variant41(alloc::vec::Vec<TypeDeclaration>),
        Variant42(u64),
        Variant43(UserTypeId),
        Variant44(core::option::Option<VarId>),
    }
    const __ACTION: &[i16] = &[
        // State 0
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 8, 70, 0, 0, 9, 0, 0, 0, 0, 0, 0, 71,
        // State 1
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 2
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 8, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 3
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 4
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 8, 70, 0, 0, 9, 0, 0, 0, 0, 0, 0, 71,
        // State 5
        6, 82, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 7
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 8
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 10
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 12
        0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 15
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 16
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 17
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 18
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 19
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 8, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 20
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 21
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 23
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 24
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 25
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 27
        0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 28
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 29
        0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 31
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 32
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 33
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 34
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 35
        6, 118, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 39
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 40
        0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 41
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 42
        0, 0, 0, 0, 0, 0, -68, 0, 0, 0, 0, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 146, 83, 0,
        // State 44
        0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 45
        6, 0, 0, 0, 0, 0, 0, 0, 0, -37, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 47
        6, 0, 0, 0, 0, 0, 0, 0, 0, -37, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 48
        0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 83, 0,
        // State 50
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 51
        6, 0, 0, 0, 0, 0, 0, 0, 0, -39, 22, 23, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 25, 26, 0, 0, 0, 83, 71,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 167, 0, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 54
        6, -33, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 55
        6, -35, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 167, 0, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 167, 0, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 167, 0, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        -48, -48, -48, 0, 0, -48, 0, -48, -48, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        -55, 0, 0, 0, 0, 10, 0, 11, 0, 0, -74, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -72,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        -121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121, 0, 0, 0, 0, 0, 0, 0, -121, 0, 0, 0, 0, 0, 0, 0, 0, 0, -121,
        // State 65
        -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96, 0, 0, 0, 0, 0, 0, -96, -96, 0, 0, 0, 0, 0, 0, 0, 0, 0, -96,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        -125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -125, 0, 0, 0, 0, 0, 0, 0, -125, 0, 0, 0, 0, 0, 0, 0, 0, 0, -125,
        // State 68
        -131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -131, 0, 0, 0, 0, 0, 0, -131, -131, 0, 0, -131, 0, 0, 0, 0, 0, 0, -131,
        // State 69
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        -21, -21, -21, 0, -21, -21, -21, -21, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, -74, -74, 0, 0, 10, 0, 11, 0, -74, -74, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -73,
        // State 73
        -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97, 0, 0, 0, 0, 0, 0, -97, -97, 0, 0, 0, 0, 0, 0, 0, 0, 0, -97,
        // State 74
        -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, -126, 0, 0, 0, 0, 0, 0, 0, 0, 0, -126,
        // State 75
        -132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -132, 0, 0, 0, 0, 0, 0, -132, -132, 0, 0, -132, 0, 0, 0, 0, 0, 0, -132,
        // State 76
        0, -78, -78, 0, 0, 0, 0, 0, 0, -78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, -59, -59, 0, 0, 10, 0, 11, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 78
        0, -77, -77, 0, 0, 0, 0, 0, 0, -77, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 79
        0, -85, -85, 0, 0, 0, 0, 0, 0, -85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 80
        0, 99, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 81
        -51, -51, -51, 0, 0, -51, 0, -51, -51, -51, -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        -22, -22, -22, 0, 0, 0, 0, 0, 0, -22, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 83
        -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -133, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 84
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 85
        0, -55, -55, 0, 0, 10, 0, 11, -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 88
        -52, -52, -52, 0, 0, -52, 0, -52, -52, -52, -52, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 89
        0, 0, 40, 0, 0, 0, 0, 0, 0, 110, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 90
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 91
        0, -136, -136, 0, -136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 92
        0, -140, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 93
        0, -44, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 94
        0, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 95
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 96
        41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 97
        -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 98
        -49, -49, -49, 0, 0, -49, 0, -49, -49, -49, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 99
        0, -60, -60, 0, 0, 10, 0, 11, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 100
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 101
        0, -81, -81, 0, 0, 0, 0, 0, 0, -81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 102
        0, -79, -79, 0, 0, 0, 0, 0, 0, -79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 103
        0, -134, -134, 0, 0, 10, 0, 11, 0, -134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 104
        0, -80, -80, 0, 0, 0, 0, 0, 0, -80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 105
        -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 106
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 107
        0, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 108
        0, 0, 40, 0, 0, 0, 0, 0, 0, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 109
        -54, -54, -54, 0, 0, -54, 0, -54, -54, -54, -54, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 110
        0, -75, -75, 0, 0, 0, 0, 0, 0, -75, -75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 111
        0, -46, 130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 112
        0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 113
        0, 0, 0, 131, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0,
        // State 114
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 115
        0, -76, -76, 0, 0, 0, 0, 0, 0, -76, -76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 116
        0, -86, -86, 0, 0, 0, 0, 0, 0, -86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 117
        -50, -50, -50, 0, 0, -50, 0, -50, -50, -50, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 118
        0, -61, -61, 0, 0, 0, 0, 0, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 119
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 120
        0, 0, 0, 0, 0, -102, -102, -102, 0, 0, 0, -102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 121
        0, 0, 0, 0, 0, 0, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 122
        0, 0, 0, 0, 0, 0, -58, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 123
        0, 0, 0, 0, 0, 47, -87, -87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 124
        0, -56, -56, 0, 0, 0, 0, 0, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 125
        0, 0, 0, 0, 0, 0, 139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 126
        0, 0, 0, 0, 0, 0, -66, 48, 0, 0, 0, -66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 127
        0, 0, 0, 0, 0, 47, -88, -88, 0, 0, 0, -88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 128
        -53, -53, -53, 0, 0, -53, 0, -53, -53, -53, -53, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 129
        0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20,
        // State 130
        49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 131
        0, -137, -137, 0, -137, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 132
        0, -101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 133
        0, -40, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 134
        0, 149, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 135
        0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 136
        0, -135, -135, 0, 0, 0, 0, 0, 0, -135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 137
        -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93, 0, 0, 0, 0, 0, 0, -93, -93, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93,
        // State 138
        -122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 0, -122, 0, 0, 0, 0, 0, 0, 0, 0, 0, -122,
        // State 139
        0, 0, 0, 0, 0, 0, 154, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 140
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 141
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, 0,
        // State 142
        54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 143
        -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 144
        -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 145
        0, 0, 0, 0, 0, 0, 159, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 146
        0, -42, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 147
        0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14,
        // State 148
        0, 0, 0, 161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 149
        0, 0, 0, 0, 0, 0, 0, 0, 0, -84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 150
        0, 0, 164, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 151
        0, 0, 0, 0, 0, 0, 0, 0, 0, 165, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 152
        0, 0, 0, 0, 0, -103, -103, -103, 0, 0, 0, -103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 153
        -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, -128, -128, 0, 0, -128, 0, 0, 0, 0, 0, 0, -128,
        // State 154
        0, 0, 0, 0, 0, 0, 0, 0, 0, 169, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 155
        0, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 156
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, -29, 0,
        // State 157
        0, 0, 0, 0, 0, 0, 171, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 158
        -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, -91, 0, 0, 0, 0, 0, 0, 0, 0, 0, -91,
        // State 159
        0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15,
        // State 160
        55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 161
        0, -98, -98, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 162
        0, 0, 173, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 163
        -9, 0, 0, 0, 0, 0, 0, 0, 0, -9, -9, -9, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, -9, -9, 0, 0, 0, -9, -9,
        // State 164
        0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 165
        0, 0, 174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 166
        0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 167
        0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 168
        0, 0, 0, 0, 0, 0, -65, 0, 0, 0, 0, -65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 169
        0, 0, 0, 0, 0, 0, 175, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 170
        -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, -92, 0, 0, 0, 0, 0, 0, 0, 0, 0, -92,
        // State 171
        0, 176, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 172
        -10, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, -10, -10, 0, 0, 0, -10, -10,
        // State 173
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 174
        -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, -90, 0, 0, 0, 0, 0, 0, 0, 0, 0, -90,
        // State 175
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, -25, 0,
        // State 176
        0, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 177
        0, -32, 181, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 178
        0, 182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 179
        0, -34, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 180
        -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4,
        // State 181
        0, 0, 0, 0, 0, 0, 185, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 182
        0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 183
        -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5,
        // State 184
        -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -69,
        // State 185
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 186
        0, 0, 188, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 187
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0,
        // State 188
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 190, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 189
        0, 0, 0, 0, 0, 0, -67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i16, integer: usize) -> i16 {
        __ACTION[(state as usize) * 30 + integer]
    }
    const __EOF_ACTION: &[i16] = &[
        // State 0
        -104,
        // State 1
        -112,
        // State 2
        -108,
        // State 3
        -106,
        // State 4
        -105,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        -116,
        // State 16
        -110,
        // State 17
        -114,
        // State 18
        -113,
        // State 19
        -109,
        // State 20
        -107,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        -118,
        // State 32
        -117,
        // State 33
        -111,
        // State 34
        -115,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        -119,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        -72,
        // State 63
        0,
        // State 64
        -121,
        // State 65
        -96,
        // State 66
        -143,
        // State 67
        -125,
        // State 68
        -131,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        -73,
        // State 73
        -97,
        // State 74
        -126,
        // State 75
        -132,
        // State 76
        0,
        // State 77
        0,
        // State 78
        0,
        // State 79
        0,
        // State 80
        0,
        // State 81
        0,
        // State 82
        0,
        // State 83
        0,
        // State 84
        0,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
        // State 90
        0,
        // State 91
        0,
        // State 92
        0,
        // State 93
        0,
        // State 94
        0,
        // State 95
        0,
        // State 96
        0,
        // State 97
        0,
        // State 98
        0,
        // State 99
        0,
        // State 100
        0,
        // State 101
        0,
        // State 102
        0,
        // State 103
        0,
        // State 104
        0,
        // State 105
        0,
        // State 106
        0,
        // State 107
        0,
        // State 108
        0,
        // State 109
        0,
        // State 110
        0,
        // State 111
        0,
        // State 112
        0,
        // State 113
        0,
        // State 114
        0,
        // State 115
        0,
        // State 116
        0,
        // State 117
        0,
        // State 118
        0,
        // State 119
        0,
        // State 120
        0,
        // State 121
        0,
        // State 122
        0,
        // State 123
        0,
        // State 124
        0,
        // State 125
        0,
        // State 126
        0,
        // State 127
        0,
        // State 128
        0,
        // State 129
        0,
        // State 130
        0,
        // State 131
        0,
        // State 132
        0,
        // State 133
        0,
        // State 134
        0,
        // State 135
        0,
        // State 136
        0,
        // State 137
        -93,
        // State 138
        -122,
        // State 139
        0,
        // State 140
        0,
        // State 141
        0,
        // State 142
        0,
        // State 143
        0,
        // State 144
        0,
        // State 145
        0,
        // State 146
        0,
        // State 147
        0,
        // State 148
        0,
        // State 149
        0,
        // State 150
        0,
        // State 151
        0,
        // State 152
        0,
        // State 153
        -128,
        // State 154
        0,
        // State 155
        0,
        // State 156
        0,
        // State 157
        0,
        // State 158
        -91,
        // State 159
        0,
        // State 160
        0,
        // State 161
        0,
        // State 162
        0,
        // State 163
        0,
        // State 164
        0,
        // State 165
        0,
        // State 166
        0,
        // State 167
        0,
        // State 168
        0,
        // State 169
        0,
        // State 170
        -92,
        // State 171
        0,
        // State 172
        0,
        // State 173
        0,
        // State 174
        -90,
        // State 175
        0,
        // State 176
        0,
        // State 177
        0,
        // State 178
        0,
        // State 179
        0,
        // State 180
        0,
        // State 181
        0,
        // State 182
        0,
        // State 183
        0,
        // State 184
        -69,
        // State 185
        0,
        // State 186
        0,
        // State 187
        0,
        // State 188
        0,
        // State 189
        0,
    ];
    fn __goto(state: i16, nt: usize) -> i16 {
        match nt {
            2 => 55,
            5 => 51,
            8 => 44,
            11 => 29,
            12 => match state {
                9 => 88,
                11 => 90,
                12 | 27 | 29 | 40 | 44 | 48 | 53 => 91,
                37..=38 => 120,
                46 => 152,
                _ => 59,
            },
            13 => match state {
                6 | 13..=14 | 22 | 26 | 30 | 43 | 49 => 83,
                36 => 119,
                _ => 76,
            },
            14 => match state {
                56 => 182,
                57 => 186,
                58 => 188,
                _ => 165,
            },
            15 => match state {
                49 => 156,
                _ => 141,
            },
            17 => 49,
            18 => 142,
            19 => 176,
            20 => 149,
            21 => 132,
            22 => 92,
            23 => match state {
                0 | 2..=4 | 16 | 19..=20 | 33 => 60,
                1 | 15 | 17..=18 | 24 | 31..=32 | 34 | 41 => 71,
                7 | 23 => 85,
                21 => 99,
                25 => 103,
                _ => 77,
            },
            24 => match state {
                7 => 86,
                23 => 101,
                _ => 61,
            },
            25 => 121,
            26 => match state {
                8 => 87,
                50 => 161,
                54 => 177,
                55 => 179,
                _ => 78,
            },
            28 => 178,
            29 => 42,
            30 => 139,
            31 => match state {
                0 | 2..=4 | 16 | 19..=20 | 33 => 62,
                _ => 72,
            },
            33 => match state {
                2 => 15,
                3 => 17,
                4 => 18,
                16 => 31,
                19 => 32,
                20 => 34,
                33 => 41,
                _ => 1,
            },
            34 => match state {
                24 => 102,
                _ => 63,
            },
            35 => match state {
                35 | 39 => 116,
                45 | 47 => 150,
                51 => 162,
                _ => 79,
            },
            37 => match state {
                47 => 154,
                _ => 151,
            },
            38 => match state {
                10 => 89,
                28 => 108,
                _ => 80,
            },
            39 => 122,
            40 => 126,
            42 => 64,
            43 => match state {
                2 | 19 => 73,
                _ => 65,
            },
            45 => match state {
                4 => 19,
                _ => 2,
            },
            46 => match state {
                44 => 146,
                _ => 133,
            },
            48 => 134,
            49 => match state {
                38 => 127,
                _ => 123,
            },
            50 => 66,
            52 => match state {
                3 | 16 | 20 | 33 => 74,
                _ => 67,
            },
            54 => match state {
                2 => 16,
                4 => 20,
                19 => 33,
                _ => 3,
            },
            55 => match state {
                14 => 96,
                _ => 143,
            },
            56 => match state {
                4 => 75,
                _ => 68,
            },
            58 => 4,
            59 => match state {
                6 => 84,
                13 => 95,
                22 => 100,
                26 => 106,
                30 => 114,
                _ => 97,
            },
            60 => 104,
            61 => match state {
                29 => 111,
                40 | 44 => 135,
                _ => 93,
            },
            63 => match state {
                27 => 107,
                48 => 155,
                53 => 171,
                _ => 94,
            },
            _ => 0,
        }
    }
    const __TERMINAL: &[&str] = &[
        r###""(""###,
        r###"")""###,
        r###"",""###,
        r###""->""###,
        r###"":""###,
        r###""::""###,
        r###"";""###,
        r###""<""###,
        r###""=""###,
        r###"">""###,
        r###""@""###,
        r###""[""###,
        r###""]""###,
        r###""drop:""###,
        r###""dup:""###,
        r###""fallthrough""###,
        r###""false""###,
        r###""lib@""###,
        r###""libfunc""###,
        r###""return""###,
        r###""storable:""###,
        r###""true""###,
        r###""type""###,
        r###""user@""###,
        r###""ut@""###,
        r###""zero_sized:""###,
        r###""{""###,
        r###""}""###,
        r###"r#"-?[1-9][0-9]*|0"#"###,
        r###"r#"[a-zA-Z_][a-zA-Z_0-9]*"#"###,
    ];
    fn __expected_tokens(__state: i16) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
        'input,
    >(
        __states: &[i16],
        _: core::marker::PhantomData<(&'input ())>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<(&())>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Program;
        type StateIndex = i16;
        type Action = i16;
        type ReduceIndex = i16;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i16, integer: usize) -> i16 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i16) -> i16 {
            __action(state, 30 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i16) -> i16 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i16, nt: usize) -> i16 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i16) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i16]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i16,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i16>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i16) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(2, _) if true => Some(0),
            Token(3, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(6, _) if true => Some(4),
            Token(7, _) if true => Some(5),
            Token(8, _) if true => Some(6),
            Token(9, _) if true => Some(7),
            Token(10, _) if true => Some(8),
            Token(11, _) if true => Some(9),
            Token(12, _) if true => Some(10),
            Token(13, _) if true => Some(11),
            Token(14, _) if true => Some(12),
            Token(15, _) if true => Some(13),
            Token(16, _) if true => Some(14),
            Token(17, _) if true => Some(15),
            Token(18, _) if true => Some(16),
            Token(19, _) if true => Some(17),
            Token(20, _) if true => Some(18),
            Token(21, _) if true => Some(19),
            Token(22, _) if true => Some(20),
            Token(23, _) if true => Some(21),
            Token(24, _) if true => Some(22),
            Token(25, _) if true => Some(23),
            Token(26, _) if true => Some(24),
            Token(27, _) if true => Some(25),
            Token(28, _) if true => Some(26),
            Token(29, _) if true => Some(27),
            Token(0, _) if true => Some(28),
            Token(1, _) if true => Some(29),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
    >(
        __reduce_index: i16,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 7,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 8,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 10,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 11,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 15,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 16,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 17,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 19,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 19,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 20,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 20,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 21,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 21,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 22,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 22,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 23,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 23,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 23,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 23,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 23,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 24,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 25,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 26,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 26,
                }
            }
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 27,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 29,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 13,
                    nonterminal_produced: 30,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 30,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 11,
                    nonterminal_produced: 31,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 32,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 32,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 33,
                }
            }
            72 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 33,
                }
            }
            73 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 34,
                }
            }
            74 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 34,
                }
            }
            75 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 34,
                }
            }
            76 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            77 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 35,
                }
            }
            78 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            79 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            80 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 35,
                }
            }
            81 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 36,
                }
            }
            82 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 36,
                }
            }
            83 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 37,
                }
            }
            84 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 38,
                }
            }
            85 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 38,
                }
            }
            86 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 39,
                }
            }
            87 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 40,
                }
            }
            88 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 41,
                }
            }
            89 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 9,
                    nonterminal_produced: 42,
                }
            }
            90 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 42,
                }
            }
            91 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 8,
                    nonterminal_produced: 42,
                }
            }
            92 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 43,
                }
            }
            93 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 44,
                }
            }
            94 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 44,
                }
            }
            95 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 45,
                }
            }
            96 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 45,
                }
            }
            97 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 46,
                }
            }
            98 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 47,
                }
            }
            99 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 47,
                }
            }
            100 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 48,
                }
            }
            101 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 49,
                }
            }
            102 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 49,
                }
            }
            103 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 50,
                }
            }
            104 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            105 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            106 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            107 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            108 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            109 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            110 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            111 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 50,
                }
            }
            112 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            113 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            114 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            115 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 50,
                }
            }
            116 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            117 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 50,
                }
            }
            118 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 50,
                }
            }
            119 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 51,
                }
            }
            120 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 52,
                }
            }
            121 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 52,
                }
            }
            122 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 53,
                }
            }
            123 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 53,
                }
            }
            124 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 54,
                }
            }
            125 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 54,
                }
            }
            126 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 55,
                }
            }
            127 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 56,
                }
            }
            128 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 57,
                }
            }
            129 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 57,
                }
            }
            130 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 58,
                }
            }
            131 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 58,
                }
            }
            132 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 59,
                }
            }
            133 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 60,
                }
            }
            134 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 60,
                }
            }
            135 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 61,
                }
            }
            136 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 61,
                }
            }
            137 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 62,
                }
            }
            138 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 62,
                }
            }
            139 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 63,
                }
            }
            140 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 64,
                }
            }
            141 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 65,
                }
            }
            142 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Program, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'input,
    >(
        __error_state: Option<i16>,
        __states: &[i16],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<(&())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i16,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i16>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Program,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                __reduce52(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            53 => {
                __reduce53(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            54 => {
                __reduce54(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            55 => {
                __reduce55(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            56 => {
                __reduce56(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            57 => {
                __reduce57(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            58 => {
                __reduce58(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            59 => {
                __reduce59(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            60 => {
                __reduce60(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            61 => {
                __reduce61(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            62 => {
                __reduce62(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            63 => {
                __reduce63(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            64 => {
                __reduce64(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            65 => {
                __reduce65(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            66 => {
                __reduce66(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            67 => {
                __reduce67(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            68 => {
                __reduce68(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            69 => {
                __reduce69(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            70 => {
                __reduce70(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            71 => {
                __reduce71(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            72 => {
                __reduce72(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            73 => {
                __reduce73(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            74 => {
                __reduce74(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            75 => {
                __reduce75(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            76 => {
                __reduce76(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            77 => {
                __reduce77(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            78 => {
                __reduce78(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            79 => {
                __reduce79(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            80 => {
                __reduce80(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            81 => {
                __reduce81(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            82 => {
                __reduce82(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            83 => {
                __reduce83(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            84 => {
                __reduce84(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            85 => {
                __reduce85(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            86 => {
                __reduce86(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            87 => {
                __reduce87(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            88 => {
                __reduce88(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            89 => {
                __reduce89(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            90 => {
                __reduce90(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            91 => {
                __reduce91(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            92 => {
                __reduce92(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            93 => {
                __reduce93(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            94 => {
                __reduce94(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            95 => {
                __reduce95(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            96 => {
                __reduce96(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            97 => {
                __reduce97(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            98 => {
                __reduce98(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            99 => {
                __reduce99(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            100 => {
                __reduce100(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            101 => {
                __reduce101(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            102 => {
                __reduce102(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            103 => {
                __reduce103(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            104 => {
                __reduce104(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            105 => {
                __reduce105(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            106 => {
                __reduce106(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            107 => {
                __reduce107(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            108 => {
                __reduce108(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            109 => {
                __reduce109(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            110 => {
                __reduce110(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            111 => {
                __reduce111(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            112 => {
                __reduce112(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            113 => {
                __reduce113(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            114 => {
                __reduce114(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            115 => {
                __reduce115(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            116 => {
                __reduce116(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            117 => {
                __reduce117(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            118 => {
                __reduce118(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            119 => {
                __reduce119(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            120 => {
                __reduce120(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            121 => {
                __reduce121(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            122 => {
                __reduce122(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            123 => {
                __reduce123(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            124 => {
                __reduce124(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            125 => {
                __reduce125(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            126 => {
                __reduce126(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            127 => {
                __reduce127(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            128 => {
                __reduce128(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            129 => {
                __reduce129(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            130 => {
                __reduce130(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            131 => {
                __reduce131(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            132 => {
                __reduce132(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            133 => {
                __reduce133(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            134 => {
                __reduce134(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            135 => {
                __reduce135(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            136 => {
                __reduce136(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            137 => {
                __reduce137(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            138 => {
                __reduce138(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            139 => {
                __reduce139(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            140 => {
                __reduce140(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            141 => {
                __reduce141(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            142 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant35(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BigInt, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BranchInfo, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BranchTarget, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant14(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant19<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteLibfuncId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant19(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant20<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteLibfuncLongId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant20(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant22<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ConcreteTypeLongId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant22(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant24<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Function, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant24(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant26<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FunctionId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant26(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericArg, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant28<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericLibfuncId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant28(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant29<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, GenericTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant29(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant31<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Invocation, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant31(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant32<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, LibfuncDeclaration, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant32(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant23<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Option<DeclaredTypeInfo>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant23(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Param, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant35<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Program, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant35(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant37<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant37(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant39<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, StatementIdx, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant39(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant40<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TypeDeclaration, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant40(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant43<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UserTypeId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant43(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, VarId, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant15(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant16(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant17(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant18<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant18(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<BranchInfo>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant25<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Function>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant25(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant33<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<LibfuncDeclaration>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant33(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant38<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Statement>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant38(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant41<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<TypeDeclaration>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant41(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant21<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<ConcreteTypeId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant21(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant27<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<GenericArg>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant27(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant34<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Param>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant34(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant44<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<VarId>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant44(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant30<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i16, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant30(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant36<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant36(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant42<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, u64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant42(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",") = ConcreteTypeId, "," => ActionFn(99);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action99::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")* =  => ActionFn(97);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action97::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")* = (<ConcreteTypeId> ",")+ => ActionFn(98);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action98::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")+ = ConcreteTypeId, "," => ActionFn(115);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action115::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<ConcreteTypeId> ",")+ = (<ConcreteTypeId> ",")+, ConcreteTypeId, "," => ActionFn(116);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action116::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",") = GenericArg, "," => ActionFn(104);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action104::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")* =  => ActionFn(102);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action102::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")* = (<GenericArg> ",")+ => ActionFn(103);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action103::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")+ = GenericArg, "," => ActionFn(119);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action119::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<GenericArg> ",")+ = (<GenericArg> ",")+, GenericArg, "," => ActionFn(120);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action120::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(89);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action89::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(87);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action87::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(88);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action88::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(123);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action123::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(124);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action124::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",") = VarId, "," => ActionFn(94);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action94::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")* =  => ActionFn(92);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action92::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 10)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")* = (<VarId> ",")+ => ActionFn(93);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action93::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")+ = VarId, "," => ActionFn(127);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action127::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<VarId> ",")+ = (<VarId> ",")+, VarId, "," => ActionFn(128);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action128::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BasicLabel = r#"[a-zA-Z_][a-zA-Z_0-9]*"# => ActionFn(45);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BigInt = r#"-?[1-9][0-9]*|0"# => ActionFn(57);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action57::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "false" => ActionFn(58);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action58::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Bool = "true" => ActionFn(59);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo = BranchTarget, "(", VarIds, ")" => ActionFn(42);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo* =  => ActionFn(63);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action63::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (0, 16)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo* = BranchInfo+ => ActionFn(64);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action64::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 16)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo+ = BranchInfo => ActionFn(105);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action105::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 17)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchInfo+ = BranchInfo+, BranchInfo => ActionFn(106);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant12(__symbols);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action106::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (2, 17)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchTarget = "fallthrough" => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // BranchTarget = StatementIdx => ActionFn(44);
        let __sym0 = __pop_Variant39(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action44::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant14(__nt), __end));
        (1, 18)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = ConcreteTypeId => ActionFn(133);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action133::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> =  => ActionFn(134);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action134::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (0, 19)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = (<ConcreteTypeId> ",")+, ConcreteTypeId => ActionFn(135);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action135::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (2, 19)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<ConcreteTypeId> = (<ConcreteTypeId> ",")+ => ActionFn(136);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action136::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 19)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = GenericArg => ActionFn(139);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action139::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> =  => ActionFn(140);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action140::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (0, 20)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = (<GenericArg> ",")+, GenericArg => ActionFn(141);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action141::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (2, 20)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<GenericArg> = (<GenericArg> ",")+ => ActionFn(142);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action142::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 20)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(147);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action147::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(148);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action148::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (0, 21)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(149);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action149::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (2, 21)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(150);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action150::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 21)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = VarId => ActionFn(175);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action175::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> =  => ActionFn(176);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action176::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (0, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = (<VarId> ",")+, VarId => ActionFn(177);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action177::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (2, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comma<VarId> = (<VarId> ",")+ => ActionFn(178);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action178::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 22)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = BasicLabel => ActionFn(50);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 23)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", GenericArgsString, ")" => ActionFn(51);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action51::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", GenericArgsString, ",", ")" => ActionFn(52);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 23)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = "(", ")" => ActionFn(53);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action53::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 23)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "::", BasicLabel => ActionFn(54);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 23)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "::", "<", GenericArgsString, ">" => ActionFn(55);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant9(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action55::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (5, 23)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLabel = ConcreteLabel, "<", GenericArgsString, ">" => ActionFn(56);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 23)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncId = ConcreteLabel => ActionFn(16);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (1, 24)
    }
    pub(crate) fn __reduce55<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncId = "[", UnsignedInt, "]" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant19(__nt), __end));
        (3, 24)
    }
    pub(crate) fn __reduce56<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncLongId = GenericLibfuncId, "<", GenericArgs, ">" => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (4, 25)
    }
    pub(crate) fn __reduce57<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteLibfuncLongId = GenericLibfuncId => ActionFn(11);
        let __sym0 = __pop_Variant28(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 25)
    }
    pub(crate) fn __reduce58<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = ConcreteLabel => ActionFn(27);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 26)
    }
    pub(crate) fn __reduce59<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = "@", ConcreteLabel => ActionFn(28);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action28::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 26)
    }
    pub(crate) fn __reduce60<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId = "[", UnsignedInt, "]" => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 26)
    }
    pub(crate) fn __reduce61<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId? = ConcreteTypeId => ActionFn(95);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action95::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (1, 27)
    }
    pub(crate) fn __reduce62<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeId? =  => ActionFn(96);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action96::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant21(__nt), __end));
        (0, 27)
    }
    pub(crate) fn __reduce63<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeIds = Comma<ConcreteTypeId> => ActionFn(30);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant15(__nt), __end));
        (1, 28)
    }
    pub(crate) fn __reduce64<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeLongId = GenericTypeId, "<", GenericArgs, ">" => ActionFn(7);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant16(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant29(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (4, 29)
    }
    pub(crate) fn __reduce65<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ConcreteTypeLongId = GenericTypeId => ActionFn(8);
        let __sym0 = __pop_Variant29(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 29)
    }
    pub(crate) fn __reduce66<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DeclaredTypeInfo = "[", "storable:", Bool, ",", "drop:", Bool, ",", "dup:", Bool, ",", "zero_sized:", Bool, "]" => ActionFn(5);
        assert!(__symbols.len() >= 13);
        let __sym12 = __pop_Variant0(__symbols);
        let __sym11 = __pop_Variant11(__symbols);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant0(__symbols);
        let __sym8 = __pop_Variant11(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant11(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym12.2;
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10, __sym11, __sym12);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (13, 30)
    }
    pub(crate) fn __reduce67<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // DeclaredTypeInfo =  => ActionFn(6);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action6::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant23(__nt), __end));
        (0, 30)
    }
    pub(crate) fn __reduce68<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = FunctionId, "@", StatementIdx, "(", Params, ")", "->", "(", ConcreteTypeIds, ")", ";" => ActionFn(12);
        assert!(__symbols.len() >= 11);
        let __sym10 = __pop_Variant0(__symbols);
        let __sym9 = __pop_Variant0(__symbols);
        let __sym8 = __pop_Variant15(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant17(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant39(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant26(__symbols);
        let __start = __sym0.0;
        let __end = __sym10.2;
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10);
        __symbols.push((__start, __Symbol::Variant24(__nt), __end));
        (11, 31)
    }
    pub(crate) fn __reduce69<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function* =  => ActionFn(69);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action69::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (0, 32)
    }
    pub(crate) fn __reduce70<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function* = Function+ => ActionFn(70);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 32)
    }
    pub(crate) fn __reduce71<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function+ = Function => ActionFn(83);
        let __sym0 = __pop_Variant24(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action83::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (1, 33)
    }
    pub(crate) fn __reduce72<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function+ = Function+, Function => ActionFn(84);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant24(__symbols);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action84::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant25(__nt), __end));
        (2, 33)
    }
    pub(crate) fn __reduce73<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = ConcreteLabel => ActionFn(18);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (1, 34)
    }
    pub(crate) fn __reduce74<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = ConcreteLabel, "[", BasicLabel, "]" => ActionFn(19);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (4, 34)
    }
    pub(crate) fn __reduce75<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // FunctionId = "[", UnsignedInt, "]" => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant26(__nt), __end));
        (3, 34)
    }
    pub(crate) fn __reduce76<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = ConcreteTypeId => ActionFn(32);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce77<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = BigInt => ActionFn(33);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action33::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 35)
    }
    pub(crate) fn __reduce78<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "user@", FunctionId => ActionFn(34);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant26(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action34::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce79<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "ut@", UserTypeId => ActionFn(35);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant43(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action35::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce80<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg = "lib@", ConcreteLibfuncId => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action36::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 35)
    }
    pub(crate) fn __reduce81<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg? = GenericArg => ActionFn(100);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action100::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (1, 36)
    }
    pub(crate) fn __reduce82<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArg? =  => ActionFn(101);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action101::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant27(__nt), __end));
        (0, 36)
    }
    pub(crate) fn __reduce83<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgs = Comma<GenericArg> => ActionFn(37);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant16(__nt), __end));
        (1, 37)
    }
    pub(crate) fn __reduce84<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgsString = GenericArg => ActionFn(48);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 38)
    }
    pub(crate) fn __reduce85<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericArgsString = GenericArgsString, ",", GenericArg => ActionFn(49);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 38)
    }
    pub(crate) fn __reduce86<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericLibfuncId = PathLabel => ActionFn(15);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant28(__nt), __end));
        (1, 39)
    }
    pub(crate) fn __reduce87<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // GenericTypeId = PathLabel => ActionFn(26);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant29(__nt), __end));
        (1, 40)
    }
    pub(crate) fn __reduce88<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // I16 = BigInt => ActionFn(62);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant30(__nt), __end));
        (1, 41)
    }
    pub(crate) fn __reduce89<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "->", "(", VarIds, ")", ";" => ActionFn(40);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant18(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym8.2;
        let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (9, 42)
    }
    pub(crate) fn __reduce90<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "{", "}", ";" => ActionFn(131);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym6.2;
        let __nt = super::__action131::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (7, 42)
    }
    pub(crate) fn __reduce91<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Invocation = ConcreteLibfuncId, "(", VarIds, ")", "{", BranchInfo+, "}", ";" => ActionFn(132);
        assert!(__symbols.len() >= 8);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant13(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant19(__symbols);
        let __start = __sym0.0;
        let __end = __sym7.2;
        let __nt = super::__action132::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        __symbols.push((__start, __Symbol::Variant31(__nt), __end));
        (8, 42)
    }
    pub(crate) fn __reduce92<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration = "libfunc", ConcreteLibfuncId, "=", ConcreteLibfuncLongId, ";" => ActionFn(9);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant20(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant19(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant32(__nt), __end));
        (5, 43)
    }
    pub(crate) fn __reduce93<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration* =  => ActionFn(73);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action73::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (0, 44)
    }
    pub(crate) fn __reduce94<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration* = LibfuncDeclaration+ => ActionFn(74);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action74::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 44)
    }
    pub(crate) fn __reduce95<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration+ = LibfuncDeclaration => ActionFn(79);
        let __sym0 = __pop_Variant32(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action79::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (1, 45)
    }
    pub(crate) fn __reduce96<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LibfuncDeclaration+ = LibfuncDeclaration+, LibfuncDeclaration => ActionFn(80);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant32(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action80::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant33(__nt), __end));
        (2, 45)
    }
    pub(crate) fn __reduce97<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param = VarId, ":", ConcreteTypeId => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 46)
    }
    pub(crate) fn __reduce98<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(85);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action85::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (1, 47)
    }
    pub(crate) fn __reduce99<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(86);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action86::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant34(__nt), __end));
        (0, 47)
    }
    pub(crate) fn __reduce100<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = Comma<Param> => ActionFn(14);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant17(__nt), __end));
        (1, 48)
    }
    pub(crate) fn __reduce101<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PathLabel = BasicLabel => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 49)
    }
    pub(crate) fn __reduce102<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PathLabel = PathLabel, "::", BasicLabel => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (3, 49)
    }
    pub(crate) fn __reduce103<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(159);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action159::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (0, 50)
    }
    pub(crate) fn __reduce104<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+ => ActionFn(160);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action160::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce105<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+ => ActionFn(161);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action161::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce106<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Statement+ => ActionFn(162);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action162::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce107<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+ => ActionFn(163);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action163::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce108<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+ => ActionFn(164);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action164::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce109<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Statement+ => ActionFn(165);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action165::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce110<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Statement+ => ActionFn(166);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant38(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action166::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce111<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Function+ => ActionFn(167);
        let __sym0 = __pop_Variant25(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action167::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (1, 50)
    }
    pub(crate) fn __reduce112<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Function+ => ActionFn(168);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action168::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce113<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Statement+, Function+ => ActionFn(169);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action169::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce114<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, Statement+, Function+ => ActionFn(170);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action170::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce115<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Function+ => ActionFn(171);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant25(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action171::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (2, 50)
    }
    pub(crate) fn __reduce116<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Function+ => ActionFn(172);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action172::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce117<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = LibfuncDeclaration+, Statement+, Function+ => ActionFn(173);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant25(__symbols);
        let __sym1 = __pop_Variant38(__symbols);
        let __sym0 = __pop_Variant33(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action173::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (3, 50)
    }
    pub(crate) fn __reduce118<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = TypeDeclaration+, LibfuncDeclaration+, Statement+, Function+ => ActionFn(174);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant25(__symbols);
        let __sym2 = __pop_Variant38(__symbols);
        let __sym1 = __pop_Variant33(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action174::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant35(__nt), __end));
        (4, 50)
    }
    pub(crate) fn __reduce119<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SignedInt = BigInt => ActionFn(61);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action61::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant36(__nt), __end));
        (1, 51)
    }
    pub(crate) fn __reduce120<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Invocation => ActionFn(38);
        let __sym0 = __pop_Variant31(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (1, 52)
    }
    pub(crate) fn __reduce121<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = "return", "(", VarIds, ")", ";" => ActionFn(39);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant18(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym4.2;
        let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant37(__nt), __end));
        (5, 52)
    }
    pub(crate) fn __reduce122<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action71::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (0, 53)
    }
    pub(crate) fn __reduce123<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement* = Statement+ => ActionFn(72);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action72::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 53)
    }
    pub(crate) fn __reduce124<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement => ActionFn(81);
        let __sym0 = __pop_Variant37(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action81::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (1, 54)
    }
    pub(crate) fn __reduce125<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement+ = Statement+, Statement => ActionFn(82);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant37(__symbols);
        let __sym0 = __pop_Variant38(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action82::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant38(__nt), __end));
        (2, 54)
    }
    pub(crate) fn __reduce126<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // StatementIdx = UnsignedInt => ActionFn(31);
        let __sym0 = __pop_Variant42(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant39(__nt), __end));
        (1, 55)
    }
    pub(crate) fn __reduce127<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration = "type", ConcreteTypeId, "=", ConcreteTypeLongId, DeclaredTypeInfo, ";" => ActionFn(4);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant23(__symbols);
        let __sym3 = __pop_Variant22(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym5.2;
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant40(__nt), __end));
        (6, 56)
    }
    pub(crate) fn __reduce128<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration* =  => ActionFn(75);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action75::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (0, 57)
    }
    pub(crate) fn __reduce129<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration* = TypeDeclaration+ => ActionFn(76);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action76::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (1, 57)
    }
    pub(crate) fn __reduce130<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration+ = TypeDeclaration => ActionFn(77);
        let __sym0 = __pop_Variant40(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (1, 58)
    }
    pub(crate) fn __reduce131<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TypeDeclaration+ = TypeDeclaration+, TypeDeclaration => ActionFn(78);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant40(__symbols);
        let __sym0 = __pop_Variant41(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action78::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant41(__nt), __end));
        (2, 58)
    }
    pub(crate) fn __reduce132<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnsignedInt = BigInt => ActionFn(60);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action60::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant42(__nt), __end));
        (1, 59)
    }
    pub(crate) fn __reduce133<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UserTypeId = ConcreteLabel => ActionFn(21);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant43(__nt), __end));
        (1, 60)
    }
    pub(crate) fn __reduce134<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UserTypeId = "[", BigInt, "]" => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant43(__nt), __end));
        (3, 60)
    }
    pub(crate) fn __reduce135<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId = BasicLabel => ActionFn(23);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 61)
    }
    pub(crate) fn __reduce136<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId = "[", UnsignedInt, "]" => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant42(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action24::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 61)
    }
    pub(crate) fn __reduce137<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId? = VarId => ActionFn(90);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action90::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant44(__nt), __end));
        (1, 62)
    }
    pub(crate) fn __reduce138<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarId? =  => ActionFn(91);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action91::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant44(__nt), __end));
        (0, 62)
    }
    pub(crate) fn __reduce139<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // VarIds = Comma<VarId> => ActionFn(25);
        let __sym0 = __pop_Variant18(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant18(__nt), __end));
        (1, 63)
    }
    pub(crate) fn __reduce140<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __ConcreteLibfuncLongId = ConcreteLibfuncLongId => ActionFn(2);
        let __sym0 = __pop_Variant20(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant20(__nt), __end));
        (1, 64)
    }
    pub(crate) fn __reduce141<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // __ConcreteTypeLongId = ConcreteTypeLongId => ActionFn(1);
        let __sym0 = __pop_Variant22(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant22(__nt), __end));
        (1, 65)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ids::*;
    use crate::program::*;
    use num_bigint::BigInt;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^((?:(?:\\-?[1-9][0-9]*)|0))", false),
            ("^((?:[A-Z_a-z][0-9A-Z_a-z]*))", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(,)", false),
            ("^((?:\\->))", false),
            ("^(:)", false),
            ("^((?:::))", false),
            ("^(;)", false),
            ("^(<)", false),
            ("^(=)", false),
            ("^(>)", false),
            ("^(@)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^((?:drop:))", false),
            ("^((?:dup:))", false),
            ("^((?:fallthrough))", false),
            ("^((?:false))", false),
            ("^((?:lib@))", false),
            ("^((?:libfunc))", false),
            ("^((?:return))", false),
            ("^((?:storable:))", false),
            ("^((?:true))", false),
            ("^((?:type))", false),
            ("^((?:user@))", false),
            ("^((?:ut@))", false),
            ("^((?:zero_sized:))", false),
            ("^(\\{)", false),
            ("^(\\})", false),
            ("^((?:(?://)[\0-\t\u{b}\u{c}\u{e}-\u{10ffff}]*[\n\r]))", true),
            ("^([\t-\r ]*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Program, usize),
) -> Program
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ConcreteTypeLongId, usize),
) -> ConcreteTypeLongId
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ConcreteLibfuncLongId, usize),
) -> ConcreteLibfuncLongId
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, type_declarations, _): (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    (_, libfunc_declarations, _): (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    (_, statements, _): (usize, alloc::vec::Vec<Statement>, usize),
    (_, funcs, _): (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    Program{type_declarations, libfunc_declarations, statements, funcs}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, ConcreteTypeId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, long_id, _): (usize, ConcreteTypeLongId, usize),
    (_, declared_type_info, _): (usize, Option<DeclaredTypeInfo>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> TypeDeclaration
{
    {
        TypeDeclaration{id, long_id, declared_type_info}
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, storable, _): (usize, bool, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, droppable, _): (usize, bool, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, duplicatable, _): (usize, bool, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, zero_sized, _): (usize, bool, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Option<DeclaredTypeInfo>
{
    Some(DeclaredTypeInfo {
        storable,
        droppable,
        duplicatable,
        zero_sized,
    })
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action6<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Option<DeclaredTypeInfo>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, generic_id, _): (usize, GenericTypeId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, generic_args, _): (usize, Vec<GenericArg>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ConcreteTypeLongId
{
    ConcreteTypeLongId{generic_id, generic_args}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, generic_id, _): (usize, GenericTypeId, usize),
) -> ConcreteTypeLongId
{
    ConcreteTypeLongId{generic_id, generic_args: vec![]}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, ConcreteLibfuncId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, long_id, _): (usize, ConcreteLibfuncLongId, usize),
    (_, _, _): (usize, &'input str, usize),
) -> LibfuncDeclaration
{
    LibfuncDeclaration{id, long_id}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, generic_id, _): (usize, GenericLibfuncId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, generic_args, _): (usize, Vec<GenericArg>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ConcreteLibfuncLongId
{
    ConcreteLibfuncLongId{generic_id, generic_args}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, generic_id, _): (usize, GenericLibfuncId, usize),
) -> ConcreteLibfuncLongId
{
    ConcreteLibfuncLongId{generic_id, generic_args: vec![]}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, FunctionId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, entry, _): (usize, StatementIdx, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, params, _): (usize, Vec<Param>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ret_types, _): (usize, Vec<ConcreteTypeId>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Function
{
    Function::new(id, params, ret_types, entry)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, VarId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ty, _): (usize, ConcreteTypeId, usize),
) -> Param
{
    Param{id, ty}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Param>, usize),
) -> Vec<Param>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
) -> GenericLibfuncId
{
    GenericLibfuncId::from_string(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
) -> ConcreteLibfuncId
{
    ConcreteLibfuncId::from_string(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, u64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ConcreteLibfuncId
{
    ConcreteLibfuncId::new(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
) -> FunctionId
{
    FunctionId::from_string(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, extra, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> FunctionId
{
    FunctionId::from_string(format!("{id}[{extra}]"))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, u64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> FunctionId
{
    FunctionId::new(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
) -> UserTypeId
{
    UserTypeId::from_string(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, BigInt, usize),
    (_, _, _): (usize, &'input str, usize),
) -> UserTypeId
{
    UserTypeId { id: id.try_into().unwrap(), debug_name: None }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
) -> VarId
{
    VarId::from_string(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, u64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> VarId
{
    VarId::new(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<VarId>, usize),
) -> Vec<VarId>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
) -> GenericTypeId
{
    GenericTypeId::from_string(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, String, usize),
) -> ConcreteTypeId
{
    ConcreteTypeId::from_string(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, String, usize),
) -> ConcreteTypeId
{
    ConcreteTypeId::from_string(format!("@{id}"))
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, u64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ConcreteTypeId
{
    ConcreteTypeId::new(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<ConcreteTypeId>, usize),
) -> Vec<ConcreteTypeId>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, u64, usize),
) -> StatementIdx
{
    StatementIdx(id as usize)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ConcreteTypeId, usize),
) -> GenericArg
{
    GenericArg::Type(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, BigInt, usize),
) -> GenericArg
{
    GenericArg::Value(v)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, FunctionId, usize),
) -> GenericArg
{
    GenericArg::UserFunc(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, UserTypeId, usize),
) -> GenericArg
{
    GenericArg::UserType(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, id, _): (usize, ConcreteLibfuncId, usize),
) -> GenericArg
{
    GenericArg::Libfunc(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<GenericArg>, usize),
) -> Vec<GenericArg>
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, invocation, _): (usize, Invocation, usize),
) -> Statement
{
    Statement::Invocation(invocation)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, Vec<VarId>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Return(args)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, libfunc_id, _): (usize, ConcreteLibfuncId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, Vec<VarId>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, results, _): (usize, Vec<VarId>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Invocation
{
    Invocation{
            libfunc_id,
            args,
            branches: vec![BranchInfo{ target: BranchTarget::Fallthrough , results }]
        }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, libfunc_id, _): (usize, ConcreteLibfuncId, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, args, _): (usize, Vec<VarId>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, branches, _): (usize, alloc::vec::Vec<BranchInfo>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Invocation
{
    Invocation{libfunc_id, args, branches}
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, target, _): (usize, BranchTarget, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, results, _): (usize, Vec<VarId>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> BranchInfo
{
    BranchInfo{ target, results }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BranchTarget
{
    BranchTarget::Fallthrough
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action44<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, StatementIdx, usize),
) -> BranchTarget
{
    BranchTarget::Statement(id)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action46<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action47<
    'input,
>(
    input: &'input str,
    (_, path_label, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, basic_label, _): (usize, String, usize),
) -> String
{
    format!("{path_label}::{basic_label}")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action48<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, GenericArg, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action49<
    'input,
>(
    input: &'input str,
    (_, head, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, tail, _): (usize, GenericArg, usize),
) -> String
{
    format!("{head}, {tail}")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action50<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action51<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, generic_args, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("({generic_args})")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action52<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, generic_args, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("({generic_args})")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action53<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> String
{
    format!("()")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action54<
    'input,
>(
    input: &'input str,
    (_, base, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, next, _): (usize, String, usize),
) -> String
{
    format!("{base}::{next}")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action55<
    'input,
>(
    input: &'input str,
    (_, base, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, generic_args, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("{base}::<{generic_args}>")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action56<
    'input,
>(
    input: &'input str,
    (_, base, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, generic_args, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("{base}<{generic_args}>")
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action57<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BigInt
{
    __0.parse().unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action58<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    false
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action59<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    true
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action60<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, BigInt, usize),
) -> u64
{
    u64::try_from(__0).unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action61<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, BigInt, usize),
) -> i64
{
    i64::try_from(__0).unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action62<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, BigInt, usize),
) -> i16
{
    i16::try_from(__0).unwrap()
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action63<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<BranchInfo>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action64<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<BranchInfo>, usize),
) -> alloc::vec::Vec<BranchInfo>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action65<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<GenericArg>, usize),
    (_, e, _): (usize, core::option::Option<GenericArg>, usize),
) -> Vec<GenericArg>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action66<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<ConcreteTypeId>, usize),
    (_, e, _): (usize, core::option::Option<ConcreteTypeId>, usize),
) -> Vec<ConcreteTypeId>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action67<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<VarId>, usize),
    (_, e, _): (usize, core::option::Option<VarId>, usize),
) -> Vec<VarId>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action68<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<Param>, usize),
    (_, e, _): (usize, core::option::Option<Param>, usize),
) -> Vec<Param>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action69<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Function>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action70<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Function>, usize),
) -> alloc::vec::Vec<Function>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action71<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Statement>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action72<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Statement>, usize),
) -> alloc::vec::Vec<Statement>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action73<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<LibfuncDeclaration>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action74<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
) -> alloc::vec::Vec<LibfuncDeclaration>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action75<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<TypeDeclaration>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action76<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<TypeDeclaration>, usize),
) -> alloc::vec::Vec<TypeDeclaration>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action77<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TypeDeclaration, usize),
) -> alloc::vec::Vec<TypeDeclaration>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action78<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    (_, e, _): (usize, TypeDeclaration, usize),
) -> alloc::vec::Vec<TypeDeclaration>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action79<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, LibfuncDeclaration, usize),
) -> alloc::vec::Vec<LibfuncDeclaration>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action80<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    (_, e, _): (usize, LibfuncDeclaration, usize),
) -> alloc::vec::Vec<LibfuncDeclaration>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action81<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> alloc::vec::Vec<Statement>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action82<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Statement>, usize),
    (_, e, _): (usize, Statement, usize),
) -> alloc::vec::Vec<Statement>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action83<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Function, usize),
) -> alloc::vec::Vec<Function>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action84<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Function>, usize),
    (_, e, _): (usize, Function, usize),
) -> alloc::vec::Vec<Function>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action85<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Param, usize),
) -> core::option::Option<Param>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action86<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Param>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action87<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Param>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action88<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Param>, usize),
) -> alloc::vec::Vec<Param>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action89<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Param, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Param
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action90<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, VarId, usize),
) -> core::option::Option<VarId>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action91<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<VarId>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action92<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<VarId>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action93<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<VarId>, usize),
) -> alloc::vec::Vec<VarId>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action94<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, VarId, usize),
    (_, _, _): (usize, &'input str, usize),
) -> VarId
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action95<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ConcreteTypeId, usize),
) -> core::option::Option<ConcreteTypeId>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action96<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<ConcreteTypeId>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action97<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<ConcreteTypeId>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action98<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ConcreteTypeId>, usize),
) -> alloc::vec::Vec<ConcreteTypeId>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action99<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ConcreteTypeId, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ConcreteTypeId
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action100<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, GenericArg, usize),
) -> core::option::Option<GenericArg>
{
    Some(__0)
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action101<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<GenericArg>
{
    None
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action102<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<GenericArg>
{
    alloc::vec![]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action103<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<GenericArg>, usize),
) -> alloc::vec::Vec<GenericArg>
{
    v
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action104<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, GenericArg, usize),
    (_, _, _): (usize, &'input str, usize),
) -> GenericArg
{
    __0
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action105<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, BranchInfo, usize),
) -> alloc::vec::Vec<BranchInfo>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action106<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<BranchInfo>, usize),
    (_, e, _): (usize, BranchInfo, usize),
) -> alloc::vec::Vec<BranchInfo>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action107<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, GenericArg, usize),
) -> alloc::vec::Vec<GenericArg>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action108<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<GenericArg>, usize),
    (_, e, _): (usize, GenericArg, usize),
) -> alloc::vec::Vec<GenericArg>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action109<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ConcreteTypeId, usize),
) -> alloc::vec::Vec<ConcreteTypeId>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action110<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<ConcreteTypeId>, usize),
    (_, e, _): (usize, ConcreteTypeId, usize),
) -> alloc::vec::Vec<ConcreteTypeId>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action111<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, VarId, usize),
) -> alloc::vec::Vec<VarId>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action112<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<VarId>, usize),
    (_, e, _): (usize, VarId, usize),
) -> alloc::vec::Vec<VarId>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action113<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Param, usize),
) -> alloc::vec::Vec<Param>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action114<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Param>, usize),
    (_, e, _): (usize, Param, usize),
) -> alloc::vec::Vec<Param>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action115<
    'input,
>(
    input: &'input str,
    __0: (usize, ConcreteTypeId, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<ConcreteTypeId>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action99(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action109(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action116<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<ConcreteTypeId>, usize),
    __1: (usize, ConcreteTypeId, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<ConcreteTypeId>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action99(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action110(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action117<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<ConcreteTypeId>, usize),
) -> Vec<ConcreteTypeId>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action97(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action118<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<ConcreteTypeId>, usize),
    __1: (usize, core::option::Option<ConcreteTypeId>, usize),
) -> Vec<ConcreteTypeId>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action98(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action119<
    'input,
>(
    input: &'input str,
    __0: (usize, GenericArg, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<GenericArg>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action104(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action107(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action120<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<GenericArg>, usize),
    __1: (usize, GenericArg, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<GenericArg>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action104(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action108(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action121<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<GenericArg>, usize),
) -> Vec<GenericArg>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action102(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action122<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<GenericArg>, usize),
    __1: (usize, core::option::Option<GenericArg>, usize),
) -> Vec<GenericArg>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action103(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action123<
    'input,
>(
    input: &'input str,
    __0: (usize, Param, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action89(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action113(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action124<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Param>, usize),
    __1: (usize, Param, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Param>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action89(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action114(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action125<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<Param>, usize),
) -> Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action87(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action126<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Param>, usize),
    __1: (usize, core::option::Option<Param>, usize),
) -> Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action88(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action127<
    'input,
>(
    input: &'input str,
    __0: (usize, VarId, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<VarId>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action94(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action111(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action128<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<VarId>, usize),
    __1: (usize, VarId, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<VarId>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action94(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action112(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action129<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<VarId>, usize),
) -> Vec<VarId>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action92(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action130<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<VarId>, usize),
    __1: (usize, core::option::Option<VarId>, usize),
) -> Vec<VarId>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action93(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action131<
    'input,
>(
    input: &'input str,
    __0: (usize, ConcreteLibfuncId, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<VarId>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, &'input str, usize),
) -> Invocation
{
    let __start0 = __4.2;
    let __end0 = __5.0;
    let __temp0 = __action63(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
        __6,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action132<
    'input,
>(
    input: &'input str,
    __0: (usize, ConcreteLibfuncId, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Vec<VarId>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, &'input str, usize),
    __5: (usize, alloc::vec::Vec<BranchInfo>, usize),
    __6: (usize, &'input str, usize),
    __7: (usize, &'input str, usize),
) -> Invocation
{
    let __start0 = __5.0;
    let __end0 = __5.2;
    let __temp0 = __action64(
        input,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
        __7,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action133<
    'input,
>(
    input: &'input str,
    __0: (usize, ConcreteTypeId, usize),
) -> Vec<ConcreteTypeId>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action95(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action134<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ConcreteTypeId>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action96(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action117(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action135<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<ConcreteTypeId>, usize),
    __1: (usize, ConcreteTypeId, usize),
) -> Vec<ConcreteTypeId>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action95(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action136<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<ConcreteTypeId>, usize),
) -> Vec<ConcreteTypeId>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action96(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action118(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action137<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __2.2;
    let __end0 = __2.2;
    let __temp0 = __action69(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action138<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
    __3: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __3.0;
    let __end0 = __3.2;
    let __temp0 = __action70(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action139<
    'input,
>(
    input: &'input str,
    __0: (usize, GenericArg, usize),
) -> Vec<GenericArg>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action100(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action121(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action140<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<GenericArg>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action101(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action121(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action141<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<GenericArg>, usize),
    __1: (usize, GenericArg, usize),
) -> Vec<GenericArg>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action100(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action122(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action142<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<GenericArg>, usize),
) -> Vec<GenericArg>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action101(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action122(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action143<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action73(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action137(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action144<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action74(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action137(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action145<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
    __2: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action73(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action138(
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action146<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
    __3: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action74(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action138(
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action147<
    'input,
>(
    input: &'input str,
    __0: (usize, Param, usize),
) -> Vec<Param>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action85(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action125(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action148<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Param>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action86(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action125(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action149<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Param>, usize),
    __1: (usize, Param, usize),
) -> Vec<Param>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action85(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action126(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action150<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Param>, usize),
) -> Vec<Param>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action86(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action126(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action151<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
) -> Program
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action71(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action143(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action152<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action72(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action143(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action153<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
) -> Program
{
    let __start0 = __1.2;
    let __end0 = __1.2;
    let __temp0 = __action71(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action154<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action72(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action144(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action155<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.2;
    let __end0 = __1.0;
    let __temp0 = __action71(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action145(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action156<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
    __2: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action72(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action145(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action157<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __1.2;
    let __end0 = __2.0;
    let __temp0 = __action71(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action146(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action158<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
    __3: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __2.0;
    let __end0 = __2.2;
    let __temp0 = __action72(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action146(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action159<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Program
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action151(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action160<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action151(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action161<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action152(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action162<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action152(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action163<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action153(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action164<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action153(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action165<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action154(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action166<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action154(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action167<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action155(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action168<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action155(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action169<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Statement>, usize),
    __1: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action156(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action170<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
    __2: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action156(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action171<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action157(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action172<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action157(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action173<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<Statement>, usize),
    __2: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action75(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action158(
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action174<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<TypeDeclaration>, usize),
    __1: (usize, alloc::vec::Vec<LibfuncDeclaration>, usize),
    __2: (usize, alloc::vec::Vec<Statement>, usize),
    __3: (usize, alloc::vec::Vec<Function>, usize),
) -> Program
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action76(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action158(
        input,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action175<
    'input,
>(
    input: &'input str,
    __0: (usize, VarId, usize),
) -> Vec<VarId>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action90(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action129(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action176<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<VarId>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action91(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action129(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action177<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<VarId>, usize),
    __1: (usize, VarId, usize),
) -> Vec<VarId>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action90(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action130(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
fn __action178<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<VarId>, usize),
) -> Vec<VarId>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action91(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action130(
        input,
        __0,
        __temp0,
    )
}
#[allow(clippy::type_complexity)]

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
