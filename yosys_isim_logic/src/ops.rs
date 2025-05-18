use yosys_isim_macros::define_enum;

define_enum![enum BinaryOp repr(u8) derive(Copy, Clone, Debug, Eq, PartialEq) {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
    AND_NOT,
    OR_NOT,
}];


