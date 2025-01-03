use crate::emu;
use crate::winapi64;
use crate::serialization;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "MessageBoxA" => MessageBoxA(emu),
        "GetDesktopWindow" => GetDesktopWindow(emu),
        "GetSystemMetrics" => GetSystemMetrics(emu),
        "SystemParametersInfoA" => SystemParametersInfoA(emu),
        "LoadIconA" => LoadIconA(emu),
        "LoadCursorA" => LoadCursorA(emu),
        "RegisterClassA" => RegisterClassA(emu),
        "RegisterClassW" => RegisterClassW(emu),
        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(&emu, emu.cfg.dump_filename.as_ref().unwrap());
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!("calling unimplemented API 0x{:x} {} at 0x{:x}", addr, api, emu.regs.rip);
            return api;
        }
    }
    String::new()
}

fn MessageBoxA(emu: &mut emu::Emu) {
    let titleptr = emu.regs.rcx;
    let msgptr = emu.regs.rdx;
    let msg = emu.maps.read_string(msgptr);
    let title = emu.maps.read_string(titleptr);

    log::info!(
        "{}** {} user32!MessageBoxA {} {} {}",
        emu.colors.light_red,
        emu.pos,
        title,
        msg,
        emu.colors.nc
    );

    emu.regs.rax = 0;
}

fn GetDesktopWindow(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} user32!GetDesktopWindow {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    //emu.regs.rax = 0x11223344; // current window handle
    emu.regs.rax = 0; // no windows handler is more stealthy
}

/*
int GetSystemMetrics(
  [in] int nIndex
);
*/
fn GetSystemMetrics(emu: &mut emu::Emu) {
    let nindex = emu.regs.rcx as usize;
    log::info!(
        "{}** {} user32!GetSystemMetrics nindex: {}{}",
        emu.colors.light_red,
        emu.pos,
        nindex,
        emu.colors.nc
    );
    // TODO: do something
    emu.regs.rax = 0;
}

/*
BOOL SystemParametersInfoA(
  [in]      UINT  uiAction,
  [in]      UINT  uiParam,
  [in, out] PVOID pvParam,
  [in]      UINT  fWinIni
);
*/
fn SystemParametersInfoA(emu: &mut emu::Emu) {
    log_red!(emu, "** {} user32!SystemParametersInfoA", emu.pos);
    // TODO: write pvParam
    emu.regs.rax = 1;
}

/*
HICON LoadIconA(
[in, optional] HINSTANCE hInstance,
[in]           LPCSTR    lpIconName
);
*/
fn LoadIconA(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} user32!LoadIconA {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // TODO: do not return null
    emu.regs.rax = 0;
}

/*
HCURSOR LoadCursorA(
  [in, optional] HINSTANCE hInstance,
  [in]           LPCSTR    lpCursorName
);
*/
fn LoadCursorA(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} user32!LoadCursorA {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // TODO: do not return null
    emu.regs.rax = 0;
}

/*
ATOM RegisterClassA(
  [in] const WNDCLASSA *lpWndClass
);
*/
fn RegisterClassA(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} user32!RegisterClassA {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // TODO: do not return null
    emu.regs.rax = 0;
}

/*
ATOM RegisterClassW(
  [in] const WNDCLASSW *lpWndClass
);
*/
fn RegisterClassW(emu: &mut emu::Emu) {
    log::info!(
        "{}** {} user32!RegisterClassW {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    // TODO: do not return null
    emu.regs.rax = 0;
}
