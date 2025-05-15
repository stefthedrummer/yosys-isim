module And (
    input  logic a,
    input  logic b,
    output logic y
);
  assign y = a & b;
endmodule

module And32 (
    input  logic[31:0] a,
    input  logic[31:0] b,
    output logic[31:0] y
);
  assign y = a & b;
endmodule

module Or (
    input  logic a,
    input  logic b,
    output logic y
);
  assign y = a | b;
endmodule

module Or32 (
    input  logic[31:0] a,
    input  logic[31:0] b,
    output logic[31:0] y
);
  assign y = a | b;
endmodule

module Nand (
    input  logic a,
    input  logic b,
    output logic y
);
  assign y = ~(a & b);
endmodule

module Nor (
    input  logic a,
    input  logic b,
    output logic y
);
  assign y = ~(a | b);
endmodule

module Dff (
    input  logic c,
    input  logic[1:0] d,
    output logic[1:0] q
);
  always_ff @(posedge c) begin
    q <= d;
  end
endmodule

module Add (
    input  logic[7:0] a,
    input  logic[7:0] b,
    output logic[7:0] y
);
  assign y = a + b;
endmodule
