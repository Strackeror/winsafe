#![allow(non_snake_case)]

use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::privs::MAKEINTRESOURCE;
use crate::user::decl::{
	ATOM, HBITMAP, HICON, HMENU, HWND, NCCALCSIZE_PARAMS, POINT, RECT,
};

/// Variant parameter for:
///
/// * [`wm::Command`](crate::msg::wm::Command).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum AccelMenuCtrl {
	/// Accelerator event. Contains the accelerator command ID.
	Accel(u16),
	/// Menu item click event. Contains the menu item command ID.
	Menu(u16),
	/// Some child control event. Contains
	/// [`AccelMenuCtrlData`](crate::AccelMenuCtrlData) data.
	Ctrl(AccelMenuCtrlData),
}

impl AccelMenuCtrl {
	/// Returns the notification code and the control ID pair.
	#[must_use]
	pub const fn code_id(&self) -> (co::CMD, u16) {
		match self {
			AccelMenuCtrl::Accel(id) => (co::CMD::Accelerator, *id),
			AccelMenuCtrl::Menu(id) => (co::CMD::Menu, *id),
			AccelMenuCtrl::Ctrl(data) => (data.notif_code, data.ctrl_id),
		}
	}
}

/// Variant parameter for:
///
/// * [`AccelMenuCtrl`](crate::AccelMenuCtrl).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub struct AccelMenuCtrlData {
	pub notif_code: co::CMD,
	pub ctrl_id: u16,
	pub ctrl_hwnd: HWND,
}

/// Variant parameter used in
/// [window class](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-classes)
/// functions:
///
/// * [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx);
/// * [`HWND::FindWindowEx`](crate::prelude::user_Hwnd::FindWindowEx);
/// * [`UnregisterClass`](crate::UnregisterClass).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum AtomStr {
	/// An [`ATOM`](crate::ATOM) returned by
	/// [`RegisterClassEx`](crate::RegisterClassEx).
	Atom(ATOM),
	/// A string.
	Str(WString),
}

impl AtomStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Atom(atom) => MAKEINTRESOURCE(atom.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`bm::GetImage`](crate::msg::bm::GetImage);
/// * [`bm::SetImage`](crate::msg::bm::SetImage).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum BmpIcon {
	/// A bitmap.
	Bmp(HBITMAP),
	/// An icon.
	Icon(HICON),
}

impl BmpIcon {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub const fn as_isize(&self) -> isize {
		unsafe {
			std::mem::transmute(match self {
				BmpIcon::Bmp(hbmp) => hbmp.0,
				BmpIcon::Icon(hicon) => hicon.0,
			})
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenu`](crate::prelude::user_Hmenu::AppendMenu).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum BmpPtrStr {
	/// An [`HBITMAP`](crate::HBITMAP).
	Bmp(HBITMAP),
	/// A pointer to anything.
	Ptr(*const std::ffi::c_void),
	/// A string.
	Str(WString),
	/// Nothing.
	None,
}

impl BmpPtrStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Bmp(hbmp) => hbmp.0 as _,
			Self::Ptr(lp) => *lp as _,
			Self::Str(ws) => unsafe { ws.as_ptr() },
			Self::None => std::ptr::null(),
		}
	}
}

/// Variant parameter for:
///
/// * [`DEVMODE`](crate::DEVMODE).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum DispfNup {
	/// Used for displays.
	Dispf(co::DMDISPLAYFLAGS),
	/// Used for printers.
	Nup(co::DMNUP),
}

/// Variant parameter for:
///
/// * [`EnumDisplaySettings`](crate::EnumDisplaySettings).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum GmidxEnum {
	/// Graphics mode index.
	Gmidx(u32),
	/// Predefined enumeration.
	Enum(co::ENUM_SETTINGS),
}

impl From<GmidxEnum> for u32 {
	fn from(v: GmidxEnum) -> Self {
		match v {
			GmidxEnum::Gmidx(idx) => idx,
			GmidxEnum::Enum(es) => es.0,
		}
	}
}

/// Variant parameter for:
///
/// * [`wm::NextDlgCtl`](crate::msg::wm::NextDlgCtl).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum HwndFocus {
	/// Handle to the control to receive the focus.
	Hwnd(HWND),
	/// If `true`, the next control with [`WS::TABSTOP`](crate::co::WS::TABSTOP)
	/// receives the focus; if `false`, the previous.
	FocusNext(bool),
}

/// Variant parameter for:
///
/// * [`wm::EnterIdle`](crate::msg::wm::EnterIdle);
/// * [`HELPINFO`](crate::HELPINFO).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum HwndHmenu {
	/// A window.
	Hwnd(HWND),
	/// A menu.
	Hmenu(HMENU),
}

impl HwndHmenu {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub const fn as_isize(&self) -> isize {
		unsafe {
			std::mem::transmute(match self {
				HwndHmenu::Hwnd(hwnd) => hwnd.0,
				HwndHmenu::Hmenu(hmenu) => hmenu.0,
			})
		}
	}
}

/// Variant parameter for:
///
/// * [`HWND::SetWindowPos`](crate::prelude::user_Hwnd::SetWindowPos);
/// * [`WINDOWPOS`](crate::WINDOWPOS);
/// * [`WINDOWPOS`](crate::WINDOWPOS).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum HwndPlace {
	/// A handle to the window to precede the positioned window in the Z order.
	Hwnd(HWND),
	/// A constant specifying where the window will be placed.
	Place(co::HWND_PLACE),
	/// Nothing.
	None,
}

impl HwndPlace {
	/// Returns a pointer to the raw data content.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Hwnd(hwnd) => hwnd.0,
			Self::Place(v) => v.0 as _,
			Self::None => std::ptr::null_mut(),
		}
	}
}

/// Variant parameter for:
///
/// * [`wm::ParentNotify`](crate::msg::wm::ParentNotify).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum HwndPointId {
	/// Handle to the child window.
	Hwnd(HWND),
	/// Cursor coordinates.
	Point(POINT),
	/// Pointer identifier.
	Id(u16),
}

impl HwndPointId {
	/// Converts the contents into an `isize`.
	#[must_use]
	pub fn as_isize(&self) -> isize {
		match self {
			HwndPointId::Hwnd(hwnd) => hwnd.0 as _,
			HwndPointId::Point(pt) => u32::from(*pt) as _,
			HwndPointId::Id(id) => *id as _,
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadCursor`](crate::prelude::user_Hinstance::LoadCursor).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum IdIdcStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDC`](crate::co::IDC) constant for a stock system cursor.
	Idc(co::IDC),
	/// A resource string identifier.
	Str(WString),
}

impl IdIdcStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Idc(idc) => MAKEINTRESOURCE(idc.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadIcon`](crate::prelude::user_Hinstance::LoadIcon).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum IdIdiStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::IDI`](crate::co::IDI) constant for a stock system icon.
	Idi(co::IDI),
	/// A resource string identifier.
	Str(WString),
}

impl IdIdiStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Idi(idi) => MAKEINTRESOURCE(idi.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`HMENU::AppendMenu`](crate::prelude::user_Hmenu::AppendMenu);
/// * [`HWND::CreateWindowEx`](crate::prelude::user_Hwnd::CreateWindowEx).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum IdMenu<'a> {
	/// A command ID.
	Id(u16),
	/// An [`HMENU`](crate::HMENU).
	Menu(&'a HMENU),
	/// Nothing.
	None,
}

impl<'a> IdMenu<'a> {
	/// Returns a pointer to the raw data content.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut std::ffi::c_void {
		match self {
			Self::Id(id) => *id as _,
			Self::Menu(hMenu) => hMenu.0,
			Self::None => std::ptr::null_mut(),
		}
	}

	/// Converts the raw data into an `usize`.
	#[must_use]
	pub const fn as_usize(&self) -> usize {
		match self {
			IdMenu::Id(id) => *id as _,
			IdMenu::Menu(hMenu) => unsafe { std::mem::transmute(hMenu.0) },
			IdMenu::None => 0,
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageBitmap`](crate::prelude::user_Hinstance::LoadImageBitmap).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum IdObmStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OBM`](crate::co::OBM) constant for an OEM bitmap.
	Obm(co::OBM),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdObmStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Obm(obm) => MAKEINTRESOURCE(obm.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageCursor`](crate::prelude::user_Hinstance::LoadImageCursor).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum IdOcrStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OCR`](crate::co::OCR) constant for an OEM cursor.
	Ocr(co::OCR),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdOcrStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Ocr(ocr) => MAKEINTRESOURCE(ocr.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`HINSTANCE::LoadImageIcon`](crate::prelude::user_Hinstance::LoadImageIcon).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone)]
pub enum IdOicStr {
	/// A resource ID.
	Id(u16),
	/// A [`co::OIC`](crate::co::OIC) constant for an OEM icon.
	Oic(co::OIC),
	/// A resource string identifier or file path.
	Str(WString),
}

impl IdOicStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Oic(oic) => MAKEINTRESOURCE(oic.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::CheckMenuItem`](crate::prelude::user_Hmenu::CheckMenuItem);
/// * [`HMENU::CheckMenuRadioItem`](crate::prelude::user_Hmenu::CheckMenuRadioItem);
/// * [`HMENU::DeleteMenu`](crate::prelude::user_Hmenu::DeleteMenu);
/// * [`HMENU::EnableMenuItem`](crate::prelude::user_Hmenu::EnableMenuItem);
/// * [`HMENU::GetMenuDefaultItem`](crate::prelude::user_Hmenu::GetMenuDefaultItem);
/// * [`HMENU::GetMenuItemInfo`](crate::prelude::user_Hmenu::GetMenuItemInfo);
/// * [`HMENU::GetMenuState`](crate::prelude::user_Hmenu::GetMenuState);
/// * [`HMENU::GetMenuString`](crate::prelude::user_Hmenu::GetMenuString);
/// * [`HMENU::InsertMenuItem`](crate::prelude::user_Hmenu::InsertMenuItem);
/// * [`HMENU::RemoveMenu`](crate::prelude::user_Hmenu::RemoveMenu);
/// * [`HMENU::SetMenuItemBitmaps`](crate::prelude::user_Hmenu::SetMenuItemBitmaps);
/// * [`HMENU::SetMenuItemInfo`](crate::prelude::user_Hmenu::SetMenuItemInfo);
/// * [`HWND::HiliteMenuItem`](crate::prelude::user_Hwnd::HiliteMenuItem).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
#[derive(Clone, Copy)]
pub enum IdPos {
	/// A command ID.
	Id(u16),
	/// Zero-based position.
	Pos(u32),
}

impl IdPos {
	/// Returns whether value is `Pos`.
	#[must_use]
	pub const fn is_by_pos(self) -> bool {
		match self {
			IdPos::Id(_) => false,
			IdPos::Pos(_) => true,
		}
	}

	/// Returns the ID or the position as a plain `u32`.
	#[must_use]
	pub const fn id_or_pos_u32(self) -> u32 {
		match self {
			IdPos::Id(id) => id as _,
			IdPos::Pos(pos) => pos,
		}
	}

	/// Returns [`MF::BYCOMMAND`](crate::co::MF::BYCOMMAND) if value is `Id`, or
	/// [`MF::BYPOSITION`](crate::co::MF::BYPOSITION) if value is `Pos`.
	#[must_use]
	pub const fn mf_flag(self) -> co::MF {
		match self {
			IdPos::Id(_) => co::MF::BYCOMMAND,
			IdPos::Pos(_) => co::MF::BYPOSITION,
		}
	}
}

/// Variant parameter for:
///
/// * [`HMENU::AppendMenuEnum`](crate::prelude::user_Hmenu::AppendMenuEnum).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum MenuEnum<'a> {
	/// A selectable entry item, with command ID and text.
	Entry(u16, &'a str),
	/// A separator.
	Separator,
	/// A submenu, with its entry text.
	Submenu(&'a HMENU, &'a str),
}

/// Variant parameter for:
///
/// * [`wm::NcCalcSize`](crate::msg::wm::NcCalcSize).
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub enum NccspRect<'a, 'b> {
	/// Mutable reference to [`NCCALCSIZE_PARAMS`](crate::NCCALCSIZE_PARAMS).
	Nccsp(&'b mut NCCALCSIZE_PARAMS<'a>),
	/// Mutable reference to [`RECT`](crate::RECT).
	Rect(&'b mut RECT),
}
