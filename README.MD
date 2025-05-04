# YOSYS-iSim - an Interactive-Simulator

This project tries to implement **an interactive** simulator that can simulate netlists produced by yosys (https://github.com/YosysHQ/yosys)


it will read modules from the netlist.json that can be generated with Yosys via the **write_json** command.

Project Structure:
- **yosys_isim/**: the core library that implements the simulator and the json parser
- **yosys_isim_test/**: end to end tests. compiles SystemVerilog via yosys, parses it and runs simulation tests.
- **yosys_isim_napi/**: js/ts/node bindings


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
