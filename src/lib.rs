use ilhook::x86::{CallbackOption, HookFlags, HookType, Hooker, Registers};
use std::error::Error;
use winapi::{
  shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
  um::winnt::DLL_PROCESS_ATTACH,
};

// FlightRecorder * __cdecl GetFlightRecorder(void)
#[no_mangle]
extern "cdecl" fn get_flight_recorder(_: *mut Registers, _: usize) {}

fn setup_console() {
  unsafe { winapi::um::consoleapi::AllocConsole() };

  println!("[setup_console]");
}

fn main() -> Result<(), Box<dyn Error>> {
  setup_console();

  let hooker = Hooker::new(
    0x00568000,
    HookType::JmpBack(get_flight_recorder),
    CallbackOption::None,
    HookFlags::empty(),
  );

  let hook = unsafe { hooker.hook()? };

  std::mem::forget(hook);

  Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
  _module: HINSTANCE,
  call_reason: DWORD,
  _reserved: LPVOID,
) -> BOOL {
  if call_reason == DLL_PROCESS_ATTACH {
    // Preferably a thread should be created here instead, since as few
    // operations as possible should be performed within `DllMain`.
    let res = main();

    if let Err(err) = &res {
      println!("{}", err);
    } else {
      println!("Hooked successfully!");
    }

    res.is_ok() as BOOL
  } else {
    TRUE
  }
}
