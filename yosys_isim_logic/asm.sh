#cargo asm --no-color "<yosys_isim::model::cell::DFlipFlopCell as yosys_isim::sim::cell::CellSimModel>::simulate" > asm.asm

cargo asm --lib --release --target-cpu=znver3 --rust -p yosys_isim_logic $1 > asm.asm