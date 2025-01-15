extern crate sdl2;

use sdl2::{pixels::Color, rect::Rect, render::{Canvas, TextureCreator}, ttf::Sdl2TtfContext, video::{Window, WindowContext}};

#[derive(Clone)]
pub enum TextAlignment {
    Centered,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
    MidLeft,
    MidRight,
    MidTop,
    ModBottom
}

pub struct TextInstance<'a>
{
    font: sdl2::ttf::Font<'a, 'static>,
    color: Color,
    texture_creator: Option<TextureCreator<WindowContext>>,
    canvas_size: (u32, u32),
    position: (i32, i32),
    size: (u32, u32),
    alignment: TextAlignment
}

impl<'a> TextInstance<'a> 
{
    pub fn new(context: & 'a Sdl2TtfContext, font_path: &str, font_px: u16) -> Result<TextInstance<'a>, String>
    {
        let font = context.load_font(font_path, font_px)?;

        let instace = TextInstance
        {
            font,
            color: Color::WHITE,
            texture_creator: None,
            canvas_size: (0, 0),
            position: (0, 0),
            size: (1, 1),
            alignment: TextAlignment::Centered
            
        };

        Ok(instace)
    }

    pub fn init(&mut self, canvas: &Canvas<Window>, )
    {
        self.texture_creator = Some(canvas.texture_creator());
        self.canvas_size = canvas.window().size();
    }

    pub fn draw_to_canvas(&self, text: &str, canvas: &mut Canvas<Window>) -> Result<(), String>
    {
        match &self.texture_creator {
            Some(creator) => 
            {
                let surface = self.font
                    .render(text)
                    .blended(self.color)
                    .map_err(|e| e.to_string())?;

                let texture = creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                let text_width = surface.width() * self.size.0;
                let text_height = surface.height() * self.size.1;
                
                let offset = self.get_offset((text_width, text_height));

                let target = Rect::new(
                    self.position.0 + offset.0 as i32,
                    self.position.1 + offset.1 as i32,
                    text_width * self.size.0,
                    text_height * self.size.1,
                );
                
                canvas.copy(&texture, None, Some(target))?;

                Ok(())
            },
            None => Err("Texture creator no initiated, try init() before this".to_string()),
        }
    }
    
    fn get_offset(&self, size: (u32, u32)) -> (i32, i32)
    {
        let size = (size.0 as i32, size.1 as i32);

        match self.alignment() {
            TextAlignment::Centered => (size.0 / 2, size.1 / 2),
            TextAlignment::BottomLeft => (0, 0),
            TextAlignment::BottomRight => (size.0, 0),
            TextAlignment::TopLeft => (0, size.1),
            TextAlignment::TopRight => size,
            TextAlignment::MidLeft => (0 , size.1 / 2),
            TextAlignment::MidRight => (size.0, size.1 / 2),
            TextAlignment::MidTop => (size.0 / 2, size.1),
            TextAlignment::ModBottom => (size.0 / 2, 0),
        }
    }

    pub fn set_position(&mut self, position: (i32, i32)) {
        self.position = position;
    }
    
    pub fn set_size(&mut self, size: (u32, u32)) {
        self.size = size;
    }
    pub fn set_scale(&mut self, size: u32){
        self.size = (size, size);
    }
    
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }
    
    pub fn color(&self) -> Color {
        self.color
    }
    
    pub fn position(&self) -> (i32, i32) {
        self.position
    }
    
    pub fn size(&self) -> (u32, u32) {
        self.size
    }
    
    pub fn alignment(&self) -> &TextAlignment {
        &self.alignment
    }
}

pub struct TextSettings
{
    color: Option<Color>,
    position: Option<(i32, i32)>,
    size: Option<(u32, u32)>,
    alignment: Option<TextAlignment>
}

impl TextSettings 
{
    pub fn new() -> TextSettings
    {
        TextSettings
        {
            color: None,
            position: None,
            size: None,
            alignment: None
        }
    }
    
    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
    }
    
    pub fn set_size(&mut self, size: Option<(u32, u32)>) {
        self.size = size;
    }

    pub fn set_scale(&mut self, size: u32){
        self.size = Some((size, size));
    }
    
    
    pub fn set_position(&mut self, position: Option<(i32, i32)>) {
        self.position = position;
    }

    pub fn set_alignment(&mut self, alignment: Option<TextAlignment>) {
        self.alignment = alignment;
    } 

    pub fn load_to_instance(&self, instance: &mut TextInstance)
    {
        match self.color {
            Some(color) => instance.color = color,
            None => (),
        }

        match self.size {
            Some(size) => instance.size = size,
            None => (),
        }

        match self.position {
            Some(position) => instance.position = position,
            None => (),
        }

        match &self.alignment {
            Some(alignment) => instance.alignment = alignment.clone(),
            None => (),
        }
    }

    pub fn load_from_instance(instance: &TextInstance) -> TextSettings
    {
        TextSettings
        {
            color: Some(instance.color()),
            position: Some(instance.position()),
            size: Some(instance.size()),
            alignment: Some(instance.alignment().clone())
        }
    }
}