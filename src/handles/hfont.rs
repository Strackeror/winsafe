#![allow(non_snake_case)]

use crate::co;
use crate::ffi::gdi32;
use crate::internal_defs::const_void;
use crate::structs::LOGFONT;
use crate::Utf16;

hgdiobj_type! {
	/// Handle to a
	/// [font](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont)
	/// GDI object. Exposes methods.
	HFONT
}

impl HFONT {
	/// [`CreateFont`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontw)
	/// static method.
	pub fn CreateFont(
		cHeight: i32, cWidth: i32, cEscapement: i32, cOrientation: i32,
		cWeight: co::FW, bItalic: bool, bUnderline: bool, bStrikeOut: bool,
		iCharSet: co::CHARSET,
		iOutPrecision: co::OUT_PRECIS, iClipPrecision: co::CLIP,
		iQuality: co::QUALITY, iPitchAndFamily: co::PITCH,
		pszFaceName: &str) -> Result<HFONT, ()>
	{
		match ptr_as_opt!(
			gdi32::CreateFontW(
				cHeight, cWidth, cEscapement, cOrientation,
				u32::from(cWeight) as i32,
				bItalic as u32, bUnderline as u32, bStrikeOut as u32,
				u8::from(iCharSet) as u32,
				u8::from(iOutPrecision) as u32, u8::from(iClipPrecision) as u32,
				u8::from(iQuality) as u32, u8::from(iPitchAndFamily) as u32,
				Utf16::from_str(pszFaceName).as_ptr(),
			)
		) {
			Some(p) => Ok(Self(p)),
			None => Err(()),
		}
	}

	/// [`CreateFontIndirect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	pub fn CreateFontIndirect(lplf: &LOGFONT) -> Result<HFONT, ()> {
		match ptr_as_opt!(gdi32::CreateFontIndirectW(const_void(lplf))) {
			Some(p) => Ok(Self(p)),
			None => Err(()),
		}
	}
}