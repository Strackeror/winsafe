#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HANDLE, HRESULT};
use crate::structs::IID;

/// [`IModalWindow`](crate::shell::IModalWindow) virtual table.
pub struct IModalWindowVT {
	pub IUnknownVT: IUnknownVT,
	pub Show: fn(PPVT, HANDLE) -> HRESULT,
}

/// [`IModalWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-imodalwindow)
/// COM interface over [`IModalWindowVT`](crate::shell::vt::IModalWindowVT).
/// Inherits from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IModalWindow {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IModalWindow {
	const IID: IID = IID::new(0xb4db1657, 0x70d7, 0x485e, 0x8e3e, 0x6fcb5a5c1802);
}

macro_rules! impl_IModalWindow {
	($name:ty, $vt:ty) => {
		use crate::co;
		use crate::funcs::HRESULT_FROM_WIN32;
		use crate::handles::HWND;

		impl $name {
			fn imodalwindow_vt(&self) -> &IModalWindowVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IModalWindow::Show`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-imodalwindow-show)
			/// method.
			///
			/// Returns false if user clicked Cancel.
			pub fn Show(&self, hwnd_owner: HWND) -> WinResult<bool> {
				let hr = (self.imodalwindow_vt().Show)(self.ppvt, hwnd_owner.ptr);
				match HRESULT_FROM_WIN32(hr) {
					co::ERROR::S_OK => Ok(true),
					co::ERROR::CANCELLED => Ok(false), // ordinary error, not a COM error
					_ => Err(co::ERROR(hr as _)),
				}
			}
		}
	};
}

impl_IUnknown!(IModalWindow, IModalWindowVT);
impl_IModalWindow!(IModalWindow, IModalWindowVT);
