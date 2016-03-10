extern crate libc;

use libc::{c_char, c_int, c_short, c_void};

pub type chtype = u64; // chtypes 64 bits by default
pub type attr_t = chtype;
pub type pdc_bool = u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINDOW {
    // definition of a window
    pub _cury: i32, // current pseudo-cursor
    pub _curx: i32,
    pub _maxy: i32, // max window coordinates
    pub _maxx: i32,
    pub _begy: i32, // origin on screen
    pub _begx: i32,
    pub _flags: i32, // window properties
    pub _attrs: chtype, // standard attributes and colors
    pub bkgd: chtype, // background, normally blank
    pub _clear: pdc_bool, // causes clear at next refresh
    pub _leaveit: pdc_bool, // leaves cursor where it is
    pub _scroll: pdc_bool, // allows window scrolling
    pub _nodelay: pdc_bool, // input character wait flag
    pub _immed: pdc_bool, // immediate update flag
    pub _sync: pdc_bool, // synchronise window ancestors
    pub _use_keypad: pdc_bool, // flags keypad key mode active
    pub _y: *mut *mut chtype, // pointer to line pointer array
    pub _firstch: *mut i32, // first changed character in line
    pub _lastch: *mut i32, // last changed character in line
    pub _tmarg: i32, // top of scrolling region
    pub _bmarg: i32, // bottom of scrolling region
    pub _delayms: i32, // milliseconds of delay for getch()
    pub _parx: i32, // coords relative to parent (0,0)
    pub _pary: i32,
    pub _parent: *mut WINDOW, // subwin's pointer to parent win
}

#[link(name="pdcurses",kind="static")]
extern "C" {
    // ----------------------------------------------------------------------
    //
    //  PDCurses Function Declarations
    //
    //

    // Standard

    pub fn addch(_: chtype) -> c_int;
    pub fn addchnstr(_: *const chtype, _: c_int) -> c_int;
    pub fn addchstr(_: *const chtype) -> c_int;
    pub fn addnstr(_: *const c_char, _: c_int) -> c_int;
    pub fn addstr(_: *const c_char) -> c_int;
    pub fn attroff(_: chtype) -> c_int;
    pub fn attron(_: chtype) -> c_int;
    pub fn attrset(_: chtype) -> c_int;
    pub fn attr_get(_: *mut attr_t, _: *mut c_short, _: *mut c_void) -> c_int;
    pub fn attr_off(_: attr_t, _: *mut c_void) -> c_int;
    pub fn attr_on(_: attr_t, _: *mut c_void) -> c_int;
    pub fn attr_set(_: attr_t, _: c_short, _: *mut c_void) -> c_int;

    pub fn baudrate() -> c_int;
    pub fn beep() -> c_int;
    pub fn bkgd(_: chtype) -> c_int;
    pub fn bkgdset(_: chtype);
    pub fn border(_: chtype,
                  _: chtype,
                  _: chtype,
                  _: chtype,
                  _: chtype,
                  _: chtype,
                  _: chtype,
                  _: chtype)
                  -> c_int;
    pub fn pdc_box(_: *mut WINDOW, _: chtype, _: chtype) -> c_int;

    pub fn can_change_color() -> pdc_bool;
    pub fn cbreak() -> c_int;
    pub fn chgat(_: c_int, _: attr_t, _: c_short, _: *const c_void) -> c_int;
    pub fn clearok(_: *mut WINDOW, _: pdc_bool) -> c_int;
    pub fn clear() -> c_int;
    pub fn clrtobot() -> c_int;
    pub fn clrtoeol() -> c_int;
    pub fn color_content(_: c_short, _: *mut c_short, _: *mut c_short, _: *mut c_short) -> c_int;
    pub fn color_set(_: short, _: *mut c_void) -> c_int;
    pub fn copywin(_: *const WINDOW,
                   _: *mut WINDOW,
                   _: c_int,
                   _: c_int,
                   _: c_int,
                   _: c_int,
                   _: c_int,
                   _: c_int,
                   _: c_int)
                   -> c_int;
    pub fn curs_set(_: c_int) -> c_int;

    pub fn echo() -> c_int;
    pub fn endwin() -> c_int;
    pub fn getch() -> c_int;
    pub fn initscr() -> *mut WINDOW;
    pub fn nocbreak() -> c_int;
    pub fn nodelay(_: *mut WINDOW, _: bool) -> c_int;
    pub fn printw(_: *const c_char) -> c_int;
    pub fn refresh() -> c_int;
    pub fn wgetch(_: *mut WINDOW) -> c_int;
}
