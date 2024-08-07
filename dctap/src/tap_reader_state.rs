use crate::tap_headers::TapHeaders;
use crate::TapShape;
use csv::{Position, StringRecord};

#[derive(Debug)]
pub struct TapReaderState {
    current_shape: TapShape,
    cached_next_record: Option<(StringRecord, Position)>,
    headers: TapHeaders,
    _position: Position,
}

impl TapReaderState {
    pub fn new() -> TapReaderState {
        TapReaderState {
            current_shape: TapShape::new(0),
            cached_next_record: None,
            headers: TapHeaders::new(),
            _position: Position::new(),
        }
    }

    pub fn current_shape(&mut self) -> &mut TapShape {
        &mut self.current_shape
    }

    pub fn headers(&self) -> &TapHeaders {
        &self.headers
    }

    pub fn with_headers(mut self, headers: TapHeaders) -> Self {
        self.headers = headers;
        self
    }

    pub fn set_next_record(&mut self, rcd: &StringRecord, pos: &Position) -> &mut Self {
        self.cached_next_record = Some((rcd.clone(), pos.clone()));
        self
    }

    pub fn reset_next_record(&mut self) -> &mut Self {
        self.cached_next_record = None;
        self
    }

    pub fn get_cached_next_record(&mut self) -> Option<(&StringRecord, &Position)> {
        if let Some((rcd, pos)) = &self.cached_next_record {
            Some((rcd, pos))
        } else {
            None
        }
    }
}

impl Default for TapReaderState {
    fn default() -> Self {
        Self::new()
    }
}
