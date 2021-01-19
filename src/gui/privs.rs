//! Global objects used within `gui` module.

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::{GetSystemMetrics, SystemParametersInfo};
use crate::handles::{HFONT, HTHEME, HWND};
use crate::msg::WmNcPaint;
use crate::structs::{NONCLIENTMETRICS, POINT, RECT, SIZE};

/// Global UI font object.
static mut UI_HFONT: Option<HFONT> = None;

/// Creates the global UI font object.
pub fn create_ui_font() -> WinResult<()> {
	let mut ncm = NONCLIENTMETRICS::default();
	unsafe {
		SystemParametersInfo(
			co::SPI::GETNONCLIENTMETRICS,
			std::mem::size_of::<NONCLIENTMETRICS>() as u32,
			&mut ncm, co::SPIF::ZERO,
		)?;
		UI_HFONT = Some(HFONT::CreateFontIndirect(&ncm.lfMenuFont)?);
	}
	Ok(())
}

/// Frees the global UI font object.
pub fn delete_ui_font() {
	unsafe {
		if let Some(hfont) = UI_HFONT {
			hfont.DeleteObject().ok();
			UI_HFONT = None;
		}
	}
}

/// Retrieves the global UI font object, or panics if not created yet.
pub fn ui_font() -> HFONT {
	unsafe {
		match UI_HFONT {
			Some(hfont) => hfont,
			None => panic!("Global UI font not created."),
		}
	}
}

//------------------------------------------------------------------------------

static mut BASE_CTRL_ID: u16 = 20_000; // in-between Visual Studio Resource Editor values

pub fn auto_ctrl_id() -> u16 {
	unsafe {
		let new_id = BASE_CTRL_ID;
		BASE_CTRL_ID += 1;
		new_id
	}
}

//------------------------------------------------------------------------------

static mut DPI: POINT = POINT { x: 0, y: 0 };

/// Multiplies the given coordinates by current system DPI.
pub fn multiply_dpi(
	pt: Option<&mut POINT>, sz: Option<&mut SIZE>) -> WinResult<()>
{
	unsafe {
		if (pt.is_some() || sz.is_some()) && DPI.x == 0 { // DPI not cached yet?
			let screen_dc = HWND::null_handle().GetDC()?;
			DPI.x = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSX); // cache
			DPI.y = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSY);
		}

		if let Some(pt) = pt {
			pt.x = kernel32::MulDiv(pt.x, DPI.x, 96);
			pt.y = kernel32::MulDiv(pt.y, DPI.y, 96);
		}

		if let Some(sz) = sz {
			sz.cx = kernel32::MulDiv(sz.cx, DPI.x, 96);
			sz.cy = kernel32::MulDiv(sz.cy, DPI.y, 96);
		}
	}
	Ok(())
}

//------------------------------------------------------------------------------

/// Calculates the bound rectangle to fit the text with current system font.
/// Texts with `&` will have the `&` width added to the final length, but for
/// now that's something we can live with.
pub fn calc_text_bound_box(text: &str) -> WinResult<SIZE> {
	let desktop_hwnd = HWND::GetDesktopWindow();
	let desktop_hdc = desktop_hwnd.GetDC()?;
	let clone_dc = desktop_hdc.CreateCompatibleDC()?;
	let prev_hfont = clone_dc.SelectObjectFont(ui_font())?;

	let mut bounds = clone_dc.GetTextExtentPoint32(
		if text.is_empty() {
			"Pj" // just a placeholder to get the text height
		} else {
			text
		}
	)?;

	if text.is_empty() {
		bounds.cx = 0; // if no text was given, return just the height
	}

	clone_dc.SelectObjectFont(prev_hfont)?;
	clone_dc.DeleteDC()?;
	desktop_hwnd.ReleaseDC(desktop_hdc)?;
	Ok(bounds)
}

/// Calculates the bound rectangle to fit the text with current system font,
/// adding a check box.
pub fn calc_text_bound_box_check(text: &str) -> WinResult<SIZE> {
	let mut bound_box = calc_text_bound_box(text)?;
	bound_box.cx += GetSystemMetrics(co::SM::CXMENUCHECK) // https://stackoverflow.com/a/1165052/6923555
		+ GetSystemMetrics(co::SM::CXEDGE);

	let cy_check = GetSystemMetrics(co::SM::CYMENUCHECK);
	if cy_check > bound_box.cy {
		bound_box.cy = cy_check; // if the check is taller than the font, use its height
	}

	Ok(bound_box)
}

/// Paints the themed border of an user control, if it has the proper styles.
pub fn paint_control_borders(hwnd: HWND, wm_ncp: WmNcPaint) -> WinResult<()> {
	hwnd.DefWindowProc(wm_ncp); // let the system draw the scrollbar for us

	let ex_style = co::WS_EX::from(hwnd.GetWindowLongPtr(co::GWLP::EXSTYLE) as u32);
	if !ex_style.has(co::WS_EX::CLIENTEDGE) // no border
		|| !HTHEME::IsThemeActive()
		|| !HTHEME::IsAppThemed()
	{
		return Ok(());
	}

	let mut rc = hwnd.GetWindowRect()?; // window outmost coordinates, including margins
	hwnd.ScreenToClientRc(&mut rc)?;
	rc.left += 2; rc.top += 2; rc.right += 2; rc.bottom += 2; // because it comes up anchored at -2,-2

	let hdc = hwnd.GetWindowDC()?;

	if let Some(htheme) = hwnd.OpenThemeData("LISTVIEW") {
		// Draw only the borders to avoid flickering.
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.left, top: rc.top, right: rc.left + 2, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.left, top: rc.top, right: rc.right, bottom: rc.top + 2 })?;
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.right - 2, top: rc.top, right: rc.right, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.left, top: rc.bottom - 2, right: rc.right, bottom: rc.bottom })?;

		htheme.CloseThemeData()?;
	}

	hwnd.ReleaseDC(hdc)
}
