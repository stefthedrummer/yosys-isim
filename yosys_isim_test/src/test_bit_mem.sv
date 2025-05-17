module test_bit_mem (
    input  logic        clk,
    input  logic        write_enable,  // write enable signal
    input  logic [4:0]  addr,          // 5-bit address (0 to 31)
    input  logic        data_in,       // bit to write
    output logic        data_out       // bit read
);

    // 32-bit memory, each bit is one storage cell
    logic [31:0] mem;

    always_ff @(posedge clk) begin
        if (write_enable) begin
            mem[addr] <= data_in;
        end
    end

    assign data_out = mem[addr];
endmodule