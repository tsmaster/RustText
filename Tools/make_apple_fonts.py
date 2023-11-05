from makefont import makefont as mf

from PIL import Image, ImageDraw, ImageFont

fontname = "PrintChar21.ttf"
outname = "40col.png"
fontsize = 8
cell_width = 6
cell_height = 8
cell_count_x = 16
cell_count_y = 6
begin_cell = 32
end_cell = 127
x_offset = -1

mf(fontname, outname, fontsize,
   cell_width, cell_height,
   cell_count_x, cell_count_y,
   begin_cell, end_cell,
   x_offset = x_offset)

outname = "40col_hi.png"
begin_cell = 0x2500
end_cell   = 0x259f
cell_count_y = 10


mf(fontname, outname, fontsize,
   cell_width, cell_height,
   cell_count_x, cell_count_y,
   begin_cell, end_cell)


fontname = "PRNumber3.ttf"
outname = "80col.png"
fontsize = 16
cell_width = 6
cell_height = 17
cell_count_x = 16
cell_count_y = 6
begin_cell = 32
end_cell = 127



mf(fontname, outname, fontsize,
         cell_width, cell_height,
         cell_count_x, cell_count_y,
         begin_cell, end_cell)


