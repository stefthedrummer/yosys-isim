use core::panic;
use std::collections::VecDeque;

use smallvec::smallvec;

use crate::{
    Vec4,
    common::{FindByName, Set4, SimError},
    model::{BinaryOp_Len, HCell, HWire, Module},
};

use super::{BinOpTruthTable, Edge, Logic, SimCell, SimPort};

pub struct Sim<'m> {
    frame: usize,
    module: &'m Module,
    update_order: Vec<HCell>,
    sim_state: SimState,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum WireState {
    Prev = 0,
    Cur = 1,
}

pub struct SimState {
    pub wires: [Vec<Logic>; 2],
    pub set_wires_deferred: Vec<(HWire, Logic)>,
    pub binop_truthtables: [BinOpTruthTable; BinaryOp_Len],
}

impl SimState {
    pub fn get_edges(&self, h_wires: &[HWire], edges: &mut [Edge]) {
        if h_wires.len() != edges.len() {
            panic!("lengths do not match");
        }

        for i in 0..h_wires.len() {
            edges[i] = Edge::of(
                self.wires[WireState::Prev as usize][h_wires[i]],
                self.wires[WireState::Cur as usize][h_wires[i]],
            );
        }
    }

    pub fn get_wires(&self, state: WireState, h_wires: &[HWire], logics: &mut [Logic]) {
        if h_wires.len() != logics.len() {
            panic!("lengths do not match");
        }

        for i in 0..h_wires.len() {
            logics[i] = self.wires[state as usize][h_wires[i]];
        }
    }

    pub fn set_wires(
        &mut self,
        state: WireState,
        h_wires: &[HWire],
        logics: &[impl Into<Logic> + Copy],
    ) {
        if h_wires.len() != logics.len() {
            panic!("lengths do not match");
        }

        for i in 0..h_wires.len() {
            self.wires[state as usize][h_wires[i]] = logics[i].into();
        }
    }

    pub fn set_wires_deferred(&mut self, h_wires: &[HWire], logics: &[impl Into<Logic> + Copy]) {
        if h_wires.len() != logics.len() {
            panic!("lengths do not match");
        }

        for i in 0..h_wires.len() {
            #[cfg(debug_assertions)]
            {
                println!("set: [{:?}] = {:?}", h_wires[i], logics[i].into());
            }
            self.set_wires_deferred.push((h_wires[i], logics[i].into()));
        }
    }
}

impl<'m> Sim<'m> {
    pub fn new(module: &'m Module) -> Self {
        let num_wires: usize = compute_num_wires(module);

        let wire_nodes = compute_wire_graph(module, num_wires);
        let cell_nodes = compute_cell_graph(module, &wire_nodes);
        let update_order = compute_cell_update_order(module, &wire_nodes, &cell_nodes);

        Sim {
            frame: 0,
            module,
            update_order,
            sim_state: SimState {
                wires: [vec![Logic::X; num_wires], vec![Logic::X; num_wires]],
                set_wires_deferred: Vec::new(),
                binop_truthtables: BinOpTruthTable::compile(),
            },
        }
    }

    pub fn simulate(&mut self) -> Result<(), SimError> {
        #[allow(unused)]
        let mut sub_frame: usize = 0;
        loop {
            #[cfg(debug_assertions)]
            {
                println!("----------------------------------------------------------------");
                println!("frame: {:?}.{:?}", self.frame, sub_frame);
                println!("wires: {:?}", self.sim_state.wires[WireState::Cur as usize]);
            }

            for h_cell in self.update_order.iter() {
                let cell = &self.module.cells[*h_cell];

                cell.simulate(&mut self.sim_state);
            }

            unsafe {
                let wires = self.sim_state.wires.as_mut_slice() as *mut [Vec<Logic>];
                (*wires)[WireState::Prev as usize]
                    .copy_from_slice(&(*wires)[WireState::Cur as usize]);
            }

            if self.sim_state.set_wires_deferred.len() > 0 {
                let cur_wires = &mut self.sim_state.wires[WireState::Cur as usize];
                for (h_wire, logic) in self.sim_state.set_wires_deferred.iter() {
                    cur_wires[*h_wire] = *logic;
                }
                self.sim_state.set_wires_deferred.clear();
                sub_frame += 1;
            } else {
                break;
            }
        }

        self.frame += 1;
        Ok(())
    }

    pub fn set<E: Copy + Into<Logic>, const L: usize>(
        &mut self,
        port: &SimPort<L>,
        logics: [E; L],
    ) {
        self.sim_state
            .set_wires(WireState::Cur, &port.h_wires, &logics);
    }

    pub fn set_raw(
        &mut self,
        port_name: &str,
        logics: &[impl Into<Logic> + Copy],
    ) -> Result<(), SimError> {
        let h_wires = self.get_port_raw(port_name, logics.len())?;
        self.sim_state.set_wires(WireState::Cur, &h_wires, &logics);
        Ok(())
    }

    pub fn get<const L: usize>(&mut self, port: &SimPort<L>) -> [Logic; L] {
        let mut logics: [Logic; L] = [Logic::X; L];
        self.sim_state
            .get_wires(WireState::Cur, &port.h_wires, &mut logics);
        logics
    }

    pub fn get_raw(&mut self, port_name: &str, width: usize) -> Result<Vec4<Logic>, SimError> {
        let h_wires = self.get_port_raw(port_name, width)?;
        let mut logics = smallvec![Logic::X ; width ];
        self.sim_state
            .get_wires(WireState::Cur, &h_wires, &mut logics);
        Ok(logics)
    }

    pub fn get_port<const W: usize>(&self, name: &str) -> Result<SimPort<W>, SimError> {
        Ok(SimPort {
            h_wires: self.get_port_raw(name, W)?,
        })
    }

    fn get_port_raw(&self, name: &str, width: usize) -> Result<Vec4<HWire>, SimError> {
        let all_ports = self
            .module
            .in_ports
            .iter()
            .chain(self.module.out_ports.iter());

        let port = all_ports.find_by_name(name)?;

        if port.h_wires.len() != width {
            Err(SimError::SimError {
                msg: format!(
                    "wrong port width [{}] on [{}],  actual is [{}]",
                    width,
                    name,
                    port.h_wires.len()
                ),
            })?;
        }

        Ok(port.h_wires.iter().cloned().collect())
    }
}

fn compute_num_wires(module: &Module) -> usize {
    let mut num_wires: usize = 0;

    for cell in module.cells.iter() {
        for port in cell.in_ports().iter().chain(cell.out_ports().iter()) {
            for wire in port.h_wires.iter() {
                num_wires = usize::max(num_wires, *wire + 1);
            }
        }
    }

    num_wires
}

#[derive(Clone, Debug)]
struct WireNode {
    pub h_in_cell: Option<HCell>,
    pub h_out_cells: Set4<HCell>,
}

fn compute_wire_graph(module: &Module, num_wires: usize) -> Vec<WireNode> {
    let mut wire_nodes: Vec<WireNode> = vec![
        WireNode {
            h_in_cell: None,
            h_out_cells: Set4::new()
        };
        num_wires
    ];

    for (h_cell, cell) in module.cells.iter().enumerate() {
        for in_port in cell.in_ports().iter() {
            for wire in in_port.h_wires.iter() {
                wire_nodes[*wire].h_out_cells.insert(h_cell);
            }
        }
        for out_port in cell.out_ports().iter() {
            for wire in out_port.h_wires.iter() {
                match wire_nodes[*wire].h_in_cell {
                    Some(_) => panic!("wire driven my multiple cells"),
                    None => wire_nodes[*wire].h_in_cell = Some(h_cell),
                }
            }
        }
    }

    wire_nodes
}

#[allow(unused)]
#[derive(Clone, Debug)]
struct CellNode {
    pub name: String,
    pub h_prev_cells: Set4<HCell>,
    pub h_next_cells: Set4<HCell>,
}

fn compute_cell_graph(module: &Module, wire_nodes: &Vec<WireNode>) -> Vec<CellNode> {
    let mut cell_nodes: Vec<CellNode> = module
        .cells
        .iter()
        .map(|cell| CellNode {
            name: cell.name().to_string(),
            h_prev_cells: Set4::new(),
            h_next_cells: Set4::new(),
        })
        .collect();

    for wire_node in wire_nodes.iter() {
        match wire_node.h_in_cell {
            Some(h_in_cell) => {
                for h_out_cell in wire_node.h_out_cells.iter() {
                    cell_nodes[h_in_cell].h_next_cells.insert(*h_out_cell);
                    cell_nodes[*h_out_cell].h_prev_cells.insert(h_in_cell);
                }
            }
            None => (),
        }
    }

    cell_nodes
}

fn compute_cell_update_order(
    module: &Module,
    wire_nodes: &Vec<WireNode>,
    cell_nodes: &Vec<CellNode>,
) -> Vec<HCell> {
    let mut token_nodes: Vec<i32> = cell_nodes
        .iter()
        .map(|it| it.h_prev_cells.len() as i32)
        .collect();

    let mut queue: VecDeque<HCell> = VecDeque::new();
    for h_cell in compute_input_cells(module, wire_nodes).iter() {
        queue.push_back(*h_cell);
        token_nodes[*h_cell] += 1;
    }

    let mut update_order: Vec<HCell> = Vec::new();
    while let Some(h_cur_cell) = queue.pop_front() {
        token_nodes[h_cur_cell] -= 1;

        match token_nodes[h_cur_cell] {
            i32::MIN..=-1 => {
                panic!("illegal state")
            }
            0 => {
                update_order.push(h_cur_cell);
                for h_next_cell in cell_nodes[h_cur_cell].h_next_cells.iter() {
                    queue.push_back(*h_next_cell);
                }
            }
            _ => (),
        }
    }

    update_order
}

fn compute_input_cells(module: &Module, wire_nodes: &Vec<WireNode>) -> Set4<HCell> {
    let mut input_cells: Set4<HWire> = Set4::new();

    for in_port in module.in_ports.iter() {
        for h_wire in in_port.h_wires.iter() {
            //
            for h_out_cell in wire_nodes[*h_wire].h_out_cells.iter() {
                input_cells.insert(*h_out_cell);
            }
        }
    }
    input_cells
}
