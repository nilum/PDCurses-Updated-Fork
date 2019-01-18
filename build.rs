extern crate cc;

fn main() {
    let mut build = cc::Build::new();
    build
        .file("src/PDCurses/pdcurses/addch.c") //Common PDCurses files
        .file("src/PDCurses/pdcurses/addchstr.c")
        .file("src/PDCurses/pdcurses/addstr.c")
        .file("src/PDCurses/pdcurses/attr.c")
        .file("src/PDCurses/pdcurses/beep.c")
        .file("src/PDCurses/pdcurses/bkgd.c")
        .file("src/PDCurses/pdcurses/border.c")
        .file("src/PDCurses/pdcurses/clear.c")
        .file("src/PDCurses/pdcurses/color.c")
        .file("src/PDCurses/pdcurses/debug.c")
        .file("src/PDCurses/pdcurses/delch.c")
        .file("src/PDCurses/pdcurses/deleteln.c")
        .file("src/PDCurses/pdcurses/deprec.c")
        .file("src/PDCurses/pdcurses/getch.c")
        .file("src/PDCurses/pdcurses/getstr.c")
        .file("src/PDCurses/pdcurses/getyx.c")
        .file("src/PDCurses/pdcurses/inch.c")
        .file("src/PDCurses/pdcurses/inchstr.c")
        .file("src/PDCurses/pdcurses/initscr.c")
        .file("src/PDCurses/pdcurses/inopts.c")
        .file("src/PDCurses/pdcurses/insch.c")
        .file("src/PDCurses/pdcurses/insstr.c")
        .file("src/PDCurses/pdcurses/instr.c")
        .file("src/PDCurses/pdcurses/kernel.c")
        .file("src/PDCurses/pdcurses/keyname.c")
        .file("src/PDCurses/pdcurses/mouse.c")
        .file("src/PDCurses/pdcurses/move.c")
        .file("src/PDCurses/pdcurses/outopts.c")
        .file("src/PDCurses/pdcurses/overlay.c")
        .file("src/PDCurses/pdcurses/pad.c")
        .file("src/PDCurses/pdcurses/panel.c")
        .file("src/PDCurses/pdcurses/printw.c")
        .file("src/PDCurses/pdcurses/refresh.c")
        .file("src/PDCurses/pdcurses/scanw.c")
        .file("src/PDCurses/pdcurses/scr_dump.c")
        .file("src/PDCurses/pdcurses/scroll.c")
        .file("src/PDCurses/pdcurses/slk.c")
        .file("src/PDCurses/pdcurses/termattr.c")
        .file("src/PDCurses/pdcurses/terminfo.c")
        .file("src/PDCurses/pdcurses/touch.c")
        .file("src/PDCurses/pdcurses/util.c")
        .file("src/PDCurses/pdcurses/window.c")
        .include("src/PDCurses")
        .define("PDC_WIDE", Some("Y")) // Build with wide-character (Unicode) support
        .define("PDC_FORCE_UTF8", Some("Y")) // Makes PDCurses ignore the system locale, and treat all narrow-character strings as UTF-8
        .define("PDC_RGB", Some("Y")); // Use RGB colors, it's what most people expect them to be

    flavor_specifics(&mut build);

    build.compile("libpdcurses.a");
}

// Use wingui if it's win32a is chosen, or if no flavor is chosen.
#[cfg(any(feature = "win32a", all(not(feature="win32"), not(feature="win32a"))))]
fn flavor_specifics(build: &mut cc::Build) {
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=gdi32");
    println!("cargo:rustc-link-lib=dylib=comdlg32");
    println!("cargo:rustc-link-lib=dylib=shell32");

    build
        .file("src/PDCurses/wingui/pdcclip.c")
        .file("src/PDCurses/wingui/pdcdisp.c")
        .file("src/PDCurses/wingui/pdcgetsc.c")
        .file("src/PDCurses/wingui/pdckbd.c")
        .file("src/PDCurses/wingui/pdcscrn.c")
        .file("src/PDCurses/wingui/pdcsetsc.c")
        .file("src/PDCurses/wingui/pdcutil.c");
}

#[cfg(feature = "win32")]
fn flavor_specifics(build: &mut cc::Build) {
    println!("cargo:rustc-link-lib=dylib=user32");

    build
        .file("src/PDCurses/wincon/pdcclip.c")
        .file("src/PDCurses/wincon/pdcdisp.c")
        .file("src/PDCurses/wincon/pdcgetsc.c")
        .file("src/PDCurses/wincon/pdckbd.c")
        .file("src/PDCurses/wincon/pdcscrn.c")
        .file("src/PDCurses/wincon/pdcsetsc.c")
        .file("src/PDCurses/wincon/pdcutil.c");
}