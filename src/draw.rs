use sdl2::render::Texture;

pub fn draw(texture: &mut Texture, screen: &[u8], width: u32, height: u32, scale: u32) -> Result<(), String> {
    texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
        for y in 0 .. height {
            for x in 0 .. width {
                let idx = ((width * y) + x) as usize;
                let slot = screen[idx];
                let color = if slot == 0 { 0xFF } else { 0x00 };
                
                for i in 0..scale {
                    for j in 0..scale {
                        let ix = (x*scale) + i;
                        let iy = (y*scale) + j;
                        let tote_width = width * scale;
                        let _tote_height = height * scale;
                        let offset = ((tote_width * iy) + ix) as usize;
            
                        buffer[offset*3] = color;
                        buffer[(offset*3)+1] = color;
                        buffer[(offset*3)+2] = color;
                    }
                }
            }
        }
    })
}
