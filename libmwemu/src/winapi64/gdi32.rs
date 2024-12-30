use crate::emu;
use crate::serialization;
use crate::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    match api.as_str() {
        "CreateFontIndirectA" => CreateFontIndirectA(emu),
        _ => {
            if emu.cfg.skip_unimplemented == false {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump_to_file(&emu, emu.cfg.dump_filename.as_ref().unwrap());
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!("calling unimplemented API 0x{:x} {}", addr, api);
            return api;
        }
    }
    String::new()
}

/*
HFONT CreateFontIndirectA(
  [in] const LOGFONTA *lplf
);
*/
fn CreateFontIndirectA(emu: &mut emu::Emu) {
    log_red!(emu, "** {} gdi32!CreateFontIndirectA", emu.pos);
    // TODO: return a handle to a logical font?
    // TODO: don't return failure
    emu.regs.rax = 0;
}