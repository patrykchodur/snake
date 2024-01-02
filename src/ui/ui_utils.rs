use eyre::{eyre, Result};
use ratatui::prelude::*;

pub fn calculate_ui_block_size(map_size: (isize, isize), borders: bool) -> Result<(u16, u16)> {
    if map_size.0 <= 0 || map_size.1 <= 0 {
        return Err(eyre!("The map size should be positive: {:?}", map_size));
    }
    let border_size = if borders { 2 } else { 0 };
    Ok((
        map_size.0 as u16 + border_size,
        ((map_size.1 as u16 + 1) / 2) + border_size,
    ))
}

/// helper function to create a centered rect using up fixed width inside rectangle `r`
pub fn centered_rect(width: u16, height: u16, r: Rect) -> Result<Rect> {
    if width > r.width || height > r.height {
        return Err(eyre!(
            "Inner rectangle dimentions are bigger than outer rectangle"
        ));
    }
    let horizontal_margin = r.width - width;
    let vertical_margin = r.height - height;
    let left_horizontal_margin = horizontal_margin / 2;
    let left_vertical_margin = vertical_margin / 2;
    Ok(Rect {
        x: r.x + left_horizontal_margin,
        y: r.y + left_vertical_margin,
        width,
        height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ui_block_size() {
        // Regular example: square map and borders
        let map_size = (50, 50);
        let block_size = (52, 27);
        assert_eq!(calculate_ui_block_size(map_size, true).unwrap(), block_size);

        // The height is not even - we need additional block
        // and one pixel will not be used
        let map_size = (50, 49);
        let block_size = (52, 27);
        assert_eq!(calculate_ui_block_size(map_size, true).unwrap(), block_size);

        // Example without borders
        let map_size = (50, 50);
        let block_size = (50, 25);
        assert_eq!(
            calculate_ui_block_size(map_size, false).unwrap(),
            block_size
        );

        // Without borders and uneven height
        let map_size = (50, 50);
        let block_size = (50, 25);
        assert_eq!(
            calculate_ui_block_size(map_size, false).unwrap(),
            block_size
        );
    }

    #[test]
    fn calculate_ui_block_size_negative_x() {
        let map_size = (-50, 50);
        assert!(calculate_ui_block_size(map_size, false).is_err());
    }

    #[test]
    fn calculate_ui_block_size_negative_y() {
        let map_size = (50, -50);
        assert!(calculate_ui_block_size(map_size, false).is_err());
    }

    #[test]
    fn check_centered_rect() {
        let my_block = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        let my_inner_block = Rect {
            x: 25,
            y: 25,
            height: 50,
            width: 50,
        };

        assert_eq!(centered_rect(50, 50, my_block).unwrap(), my_inner_block);
    }

    #[test]
    fn centered_rect_invalid_width() {
        let my_block = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        assert!(centered_rect(150, 50, my_block).is_err());
    }

    #[test]
    fn centered_rect_invalid_height() {
        let my_block = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        assert!(centered_rect(50, 150, my_block).is_err());
    }
}
