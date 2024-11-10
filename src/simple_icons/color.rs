mod relative_luminance {
    fn hex_to_tuple(hex: &str) -> (u8, u8, u8) {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    }

    fn rgb_norm(rgb: u8) -> f32 {
        let norm = (rgb as f32) / 255.0;
        if norm <= 0.03928 {
            norm / 12.92
        } else {
            ((norm + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Get the relative luminance of a color
    /// based on the next definition of the W3C:
    /// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
    pub fn get(hex: &str) -> f32 {
        let (r, g, b) = hex_to_tuple(hex);
        0.2126 * rgb_norm(r) + 0.7152 * rgb_norm(g) + 0.0722 * rgb_norm(b)
    }
}

pub fn is_relatively_light_icon_hex(hex: &str) -> bool {
    relative_luminance::get(hex) >= 0.4
}
