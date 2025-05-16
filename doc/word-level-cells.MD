# Word Level Cells

`reference:`
https://yosyshq.readthedocs.io/projects/yosys/en/latest/cell/index_word.html

| Unary operators | ☐   |
| --------------- | --- |
| $buf            | ☐   |
| $logic_not      | ☐   |
| $neg            | ☐   |
| $not            | ✅  |
| $pos            | ☐   |
| $reduce_and     | ☐   |
| $reduce_bool    | ☐   |
| $reduce_or      | ☐   |
| $reduce_xnor    | ☐   |
| $reduce_xor     | ☐   |

| Binary operators | ☐   |
| ---------------- | --- |
| $add             | ✅  |
| $and             | ✅  |
| $bweqx           | ☐   |
| $div             | ☐   |
| $divfloor        | ☐   |
| $eq              | ☐   |
| $eqx             | ☐   |
| $ge              | ☐   |
| $gt              | ☐   |
| $le              | ☐   |
| $logic_and       | ☐   |
| $logic_or        | ☐   |
| $lt              | ☐   |
| $mod             | ☐   |
| $modfloor        | ☐   |
| $mul             | ☐   |
| $ne              | ☐   |
| $nex             | ☐   |
| $or              | ✅  |
| $pow             | ☐   |
| $shift           | ☐   |
| $shiftx          | ☐   |
| $shl             | ☐   |
| $shr             | ☐   |
| $sshl            | ☐   |
| $sshr            | ☐   |
| $sub             | ☐   |
| $xnor            | ☐   |
| $xor             | ☐   |

| Multiplexers | ☐   |
| ------------ | --- |
| $bmux        | ☐   |
| $bwmux       | ☐   |
| $demux       | ☐   |
| $mux         | ☐   |
| $pmux        | ☐   |
| $tribuf      | ☐   |
| Registers    | ☐   |
| $adff        | ☐   |
| $adffe       | ☐   |
| $adlatch     | ☐   |
| $aldff       | ☐   |
| $aldffe      | ☐   |
| $dff         | ✅  |
| $dffe        | ☐   |
| $dffsr       | ☐   |
| $dffsre      | ☐   |
| $dlatch      | ☐   |
| $dlatchsr    | ☐   |
| $sdff        | ☐   |
| $sdffce      | ☐   |
| $sdffe       | ☐   |
| $sr          | ☐   |

| Memories    | ☐   |
| ----------- | --- |
| $mem        | ☐   |
| $mem_v2     | ☐   |
| $meminit    | ☐   |
| $meminit_v2 | ☐   |
| $memrd      | ☐   |
| $memrd_v2   | ☐   |
| $memwr      | ☐   |
| $memwr_v2   | ☐   |

| Finite state machines | ☐   |
| --------------------- | --- |
| $fsm                  | ☐   |

| Coarse arithmetics | ☐   |
| ------------------ | --- |
| $alu               | ☐   |
| $fa                | ☐   |
| $lcu               | ☐   |
| $macc              | ☐   |
| $macc_v2           | ☐   |

| Arbitrary logic functions | ☐   |
| ------------------------- | --- |
| $lut                      | ☐   |
| $sop                      | ☐   |
| Specify rules             | ☐   |
| $specify2                 | ☐   |
| $specify3                 | ☐   |
| $specrule                 | ☐   |

| Formal verification cells | ☐   |
| ------------------------- | --- |
| $allconst                 | ☐   |
| $allseq                   | ☐   |
| $anyconst                 | ☐   |
| $anyinit                  | ☐   |
| $anyseq                   | ☐   |
| $assert                   | ☐   |
| $assume                   | ☐   |
| $cover                    | ☐   |
| $equiv                    | ☐   |
| $fair                     | ☐   |
| $ff                       | ☐   |
| $initstate                | ☐   |
| $live                     | ☐   |

| Debugging cells | ☐   |
| --------------- | --- |
| $check          | ☐   |
| $print          | ☐   |
| $scopeinfo      | ☐   |

| Wire cells | ☐   |
| ---------- | --- |
| $concat    | ☐   |
| $slice     | ☐   |
