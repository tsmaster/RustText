from PIL import Image, ImageDraw, ImageFont

def makefont(fontname, outname, fontsize,
             cell_width, cell_height,
             cell_count_x, cell_count_y,
             begin_cell, end_cell,
             x_offset = 0,
             y_offset = 0):
                 
    im_width = cell_width * cell_count_x
    im_height = cell_height * cell_count_y
    
    im = Image.new("RGBA", (im_width, im_height))
    draw = ImageDraw.Draw(im)
    
    fnt = ImageFont.truetype(fontname, fontsize)
    
    for x in range(0, cell_count_x + 1):
        for y in range(0, cell_count_y + 1):
            c = cell_count_x * y + x + begin_cell
            if ((c >= begin_cell) and
                (c <= end_cell)):
                draw.text((x * cell_width + x_offset,
                           y*cell_height + y_offset),
                          chr(c), font=fnt, fill=(255,255,255))
    
    im.save(outname)
