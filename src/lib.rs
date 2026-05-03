mod r65816;

pub trait Bind {
    fn video_refresh(
        &mut self,
        _palette: &[u32],
        _data: &[u32],
        _pitch: u32,
        _width: u32,
        _height: u32,
    ) {
    }
    fn input_poll(&mut self, _port: u32, _device: u32, _id: u32) -> i16 {
        0
    }
}

pub struct Smw {
    pub bind: Box<dyn Bind>,
}

impl Smw {
    pub fn new(bind: Box<dyn Bind>) -> Self {
        Self { bind }
    }
}
