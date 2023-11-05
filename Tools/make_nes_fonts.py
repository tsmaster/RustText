from makefont import makefont as mf

fontname = "nes-arcade-font-2-1-monospaced.ttf"
outname = "nes.png"
#fontsize = 21
#cell_width = 24
#cell_height = 24

fontsize = 7
cell_width = 8
cell_height = 8

cell_count_x = 16
cell_count_y = 6
begin_cell = 32
end_cell = 127
x_offset = 0
y_offset = -4




mf(fontname, outname, fontsize,
   cell_width, cell_height,
   cell_count_x, cell_count_y,
   begin_cell, end_cell,
   y_offset = y_offset,
   )


