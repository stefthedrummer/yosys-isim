import { Sim } from ".";
import { Module } from ".";

const modules = Module.parseModulesFromFile("sandbox-netlist.json")
const module = modules.find(e => e.name == "And")!!;
const sim = Sim.create(module);

sim.set("a", [1])
sim.set("b", [1])
sim.simulate();
console.log(sim.get("y", 1))
