# YOSYS-iSim - an Interactive-Verilog-Simulator

### ... for Javascript/Typescript and Rust

This project tries to implement `an interactive` simulator that can simulate netlists produced by Yosys (https://github.com/YosysHQ/yosys)

it will read modules from the netlist.json that can be generated with Yosys via the **write_json** command.

### Project Structure:

- `**yosys_isim/**`: the core library that implements the simulator and the json parser
- `**yosys_isim_test/**`: end to end tests. compiles SystemVerilog via yosys, parses it and runs simulation tests.
- `**yosys_isim_napi/**`: js/ts/node bindings

## Javascript / Typescript

```javascript
const modules = Module.parseModulesFromFile("<netlist.json>")
const module = modules.find(e => e.name == "And")!!;
const sim = Sim.create(module);

sim.set("a", [1])
sim.set("b", [1])
sim.simulate(); // simulates 1 tick
console.log(sim.get("b", 1))
```

## Rust

```rust
let module = TEST_GATES_SV.deref().iter().find_by_name("Dff")?;

let mut sim: Sim<'_> = Sim::new(&module);
let port_c = sim.get_port::<1>("c")?;
let port_d = sim.get_port::<2>("d")?;
let port_q = sim.get_port::<2>("q")?;

assert_eq!(sim.get(&port_q), [Logic::X; 2]);

sim.set(&port_c, [0]);
sim.set(&port_d, [1, 0]);
sim.simulate()?; // simulates 1 tick

assert_eq!(sim.get(&port_q), [Logic::X; 2]);

sim.set(&port_c, [1]);
sim.simulate()?;

assert_eq!(sim.get(&port_q), [Logic::_1, Logic::_0]);

sim.set(&port_c, [0]);
sim.set(&port_d, [0, 1]);
sim.simulate()?;

sim.set(&port_c, [1]);
sim.simulate()?;

assert_eq!(sim.get(&port_q), [Logic::_0, Logic::_1]);
```

### Features :

Implementing `Gate-Level-Cells` is actually not hard, since their logic gets mostly auto generated.
For the AND gate - for instance - we define the behavour just for the logic states `0` and `1`. Then the simulator creates a `Truth-Table` automatically including the `X` state.


|                                |                  |
| ------------------------------ | ---------------- |
| Basic Logic Gates, FlipFlops   | ✅               |
| Low-Level Gates ($_XNOR_, ...) | 🟡 (some)        |
| Tests                          | 🟡 (only simple) |
| Outputs, Dumps, Traces         | ❌               |

| Combinatorial cells (simple) | ☐   |
| ---------------------------- | --- |
| $_AND_                       | ✅  |
| $_BUF_                       | ☐   |
| $_MUX_                       | ☐   |
| $_NAND_                      | ✅  |
| $_NOR_                       | ✅  |
| $_NOT_                       | ✅  |
| $_OR_                        | ✅  |
| $_XNOR_                      | ✅  |
| $_XOR_                       | ✅  |

| Combinatorial cells (combined) | ☐   |
| ------------------------------ | --- |
| $_ANDNOT_                      | ✅  |
| $_AOI3_                        | ✅  |
| $_AOI4_                        | ☐   |
| $_MUX16_                       | ☐   |
| $_MUX4_                        | ☐   |
| $_MUX8_                        | ☐   |
| $_NMUX_                        | ☐   |
| $_OAI3_                        | ✅  |
| $_OAI4_                        | ☐   |
| $_ORNOT_                       | ✅  |

| Flip-flop cells | ☐   |
| --------------- | --- |
| $_ALDFFE_NNN_   | ☐   |
| $_ALDFFE_NNP_   | ☐   |
| $_ALDFFE_NPN_   | ☐   |
| $_ALDFFE_NPP_   | ☐   |
| $_ALDFFE_PNN_   | ☐   |
| $_ALDFFE_PNP_   | ☐   |
| $_ALDFFE_PPN_   | ☐   |
| $_ALDFFE_PPP_   | ☐   |
| $_ALDFF_NN_     | ☐   |
| $_ALDFF_NP_     | ☐   |
| $_ALDFF_PN_     | ☐   |
| $_ALDFF_PP_     | ☐   |
| $_DFFE_NN0N_    | ☐   |
| $_DFFE_NN0P_    | ☐   |
| $_DFFE_NN1N_    | ☐   |
| $_DFFE_NN1P_    | ☐   |
| $_DFFE_NN_      | ☐   |
| $_DFFE_NP0N_    | ☐   |
| $_DFFE_NP0P_    | ☐   |
| $_DFFE_NP1N_    | ☐   |
| $_DFFE_NP1P_    | ☐   |
| $_DFFE_NP_      | ☐   |
| $_DFFE_PN0N_    | ☐   |
| $_DFFE_PN0P_    | ☐   |
| $_DFFE_PN1N_    | ☐   |
| $_DFFE_PN1P_    | ☐   |
| $_DFFE_PN_      | ☐   |
| $_DFFE_PP0N_    | ☐   |
| $_DFFE_PP0P_    | ☐   |
| $_DFFE_PP1N_    | ☐   |
| $_DFFE_PP1P_    | ☐   |
| $_DFFE_PP_      | ☐   |
| $_DFFSRE_NNNN_  | ☐   |
| $_DFFSRE_NNNP_  | ☐   |
| $_DFFSRE_NNPN_  | ☐   |
| $_DFFSRE_NNPP_  | ☐   |
| $_DFFSRE_NPNN_  | ☐   |
| $_DFFSRE_NPNP_  | ☐   |
| $_DFFSRE_NPPN_  | ☐   |
| $_DFFSRE_NPPP_  | ☐   |
| $_DFFSRE_PNNN_  | ☐   |
| $_DFFSRE_PNNP_  | ☐   |
| $_DFFSRE_PNPN_  | ☐   |
| $_DFFSRE_PNPP_  | ☐   |
| $_DFFSRE_PPNN_  | ☐   |
| $_DFFSRE_PPNP_  | ☐   |
| $_DFFSRE_PPPN_  | ☐   |
| $_DFFSRE_PPPP_  | ☐   |
| $_DFFSR_NNN_    | ☐   |
| $_DFFSR_NNP_    | ☐   |
| $_DFFSR_NPN_    | ☐   |
| $_DFFSR_NPP_    | ☐   |
| $_DFFSR_PNN_    | ☐   |
| $_DFFSR_PNP_    | ☐   |
| $_DFFSR_PPN_    | ☐   |
| $_DFFSR_PPP_    | ☐   |
| $_DFF_NN0_      | ☐   |
| $_DFF_NN1_      | ☐   |
| $_DFF_NP0_      | ☐   |
| $_DFF_NP1_      | ☐   |
| $_DFF_N_        | ☐   |
| $_DFF_PN0_      | ☐   |
| $_DFF_PN1_      | ☐   |
| $_DFF_PP0_      | ☐   |
| $_DFF_PP1_      | ☐   |
| $_DFF_P_        | ✅  |
| $_FF_           | ☐   |
| $_SDFFCE_NN0N_  | ☐   |
| $_SDFFCE_NN0P_  | ☐   |
| $_SDFFCE_NN1N_  | ☐   |
| $_SDFFCE_NN1P_  | ☐   |
| $_SDFFCE_NP0N_  | ☐   |
| $_SDFFCE_NP0P_  | ☐   |
| $_SDFFCE_NP1N_  | ☐   |
| $_SDFFCE_NP1P_  | ☐   |
| $_SDFFCE_PN0N_  | ☐   |
| $_SDFFCE_PN0P_  | ☐   |
| $_SDFFCE_PN1N_  | ☐   |
| $_SDFFCE_PN1P_  | ☐   |
| $_SDFFCE_PP0N_  | ☐   |
| $_SDFFCE_PP0P_  | ☐   |
| $_SDFFCE_PP1N_  | ☐   |
| $_SDFFCE_PP1P_  | ☐   |
| $_SDFFE_NN0N_   | ☐   |
| $_SDFFE_NN0P_   | ☐   |
| $_SDFFE_NN1N_   | ☐   |
| $_SDFFE_NN1P_   | ☐   |
| $_SDFFE_NP0N_   | ☐   |
| $_SDFFE_NP0P_   | ☐   |
| $_SDFFE_NP1N_   | ☐   |
| $_SDFFE_NP1P_   | ☐   |
| $_SDFFE_PN0N_   | ☐   |
| $_SDFFE_PN0P_   | ☐   |
| $_SDFFE_PN1N_   | ☐   |
| $_SDFFE_PN1P_   | ☐   |
| $_SDFFE_PP0N_   | ☐   |
| $_SDFFE_PP0P_   | ☐   |
| $_SDFFE_PP1N_   | ☐   |
| $_SDFFE_PP1P_   | ☐   |
| $_SDFF_NN0_     | ☐   |
| $_SDFF_NN1_     | ☐   |
| $_SDFF_NP0_     | ☐   |
| $_SDFF_NP1_     | ☐   |
| $_SDFF_PN0_     | ☐   |
| $_SDFF_PN1_     | ☐   |
| $_SDFF_PP0_     | ☐   |
| $_SDFF_PP1_     | ☐   |

| Latch cells     | ☐   |
| --------------- | --- |
| $_DLATCHSR_NNN_ | ☐   |
| $_DLATCHSR_NNP_ | ☐   |
| $_DLATCHSR_NPN_ | ☐   |
| $_DLATCHSR_NPP_ | ☐   |
| $_DLATCHSR_PNN_ | ☐   |
| $_DLATCHSR_PNP_ | ☐   |
| $_DLATCHSR_PPN_ | ☐   |
| $_DLATCHSR_PPP_ | ☐   |
| $_DLATCH_NN0_   | ☐   |
| $_DLATCH_NN1_   | ☐   |
| $_DLATCH_NP0_   | ☐   |
| $_DLATCH_NP1_   | ☐   |
| $_DLATCH_N_     | ☐   |
| $_DLATCH_PN0_   | ☐   |
| $_DLATCH_PN1_   | ☐   |
| $_DLATCH_PP0_   | ☐   |
| $_DLATCH_PP1_   | ☐   |
| $_DLATCH_P_     | ☐   |
| $_SR_NN_        | ☐   |
| $_SR_NP_        | ☐   |
| $_SR_PN_        | ☐   |
| $_SR_PP_        | ☐   |
