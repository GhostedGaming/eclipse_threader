use alloc::vec::Vec;
use eclipse_framebuffer::println;

const MAX_PROCESSES: usize = 256;

const KERNEL_STACK_SIZE: u16 = 8 * 1024;
const DEFAULT_TIME_SLICE: u8 = 5;

pub static mut INITILIZED: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
	Ready,
	Blocked,
	Running,
	Waiting,
	Terminated,
}

#[derive(Debug, Clone)]
pub struct Process {
	pub pid: u32,
	pub name: &'static str,
	pub uid: u32,
	pub state: ProcessState,
	pub priority: u8,
	pub time_slice: u8,
	pub kernel_stack_base: u64,
	pub kernel_stack_pointer: u64,
	pub user_stack_base: u64,
	pub user_stack_pointer: u64,
	pub pml4_phys_addr: u64,
	pub entry_point: u64,
	pub cpu_time: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThreadContext {
	pub regs: [u64; 17],
}

impl ThreadContext {
	pub const fn new() -> Self {
		ThreadContext { regs: [0u64; 17] }
	}

	pub fn init_stack_and_ip(&mut self, stack_top: u64, entry: u64) {
		self.regs[7] = stack_top;
		self.regs[16] = entry;
	}
}

pub fn switch_context(old: &mut ThreadContext, new: &ThreadContext) {
	let old_ptr = old.regs.as_mut_ptr();
	let new_ptr = new.regs.as_ptr();

	unsafe {
		core::arch::asm!(
			"mov [{old} + 0x00], rax",
			"mov [{old} + 0x08], rbx",
			"mov [{old} + 0x10], rcx",
			"mov [{old} + 0x18], rdx",
			"mov [{old} + 0x20], rsi",
			"mov [{old} + 0x28], rdi",
			"mov [{old} + 0x30], rbp",
			"mov [{old} + 0x38], rsp",
			"mov [{old} + 0x40], r8",
			"mov [{old} + 0x48], r9",
			"mov [{old} + 0x50], r10",
			"mov [{old} + 0x58], r11",
			"mov [{old} + 0x60], r12",
			"mov [{old} + 0x68], r13",
			"mov [{old} + 0x70], r14",
			"mov [{old} + 0x78], r15",
			"lea rax, [rip + 2f]",
			"mov [{old} + 0x80], rax",
			"mov rax, [{new} + 0x00]",
			"mov rbx, [{new} + 0x08]",
			"mov rcx, [{new} + 0x10]",
			"mov rdx, [{new} + 0x18]",
			"mov rsi, [{new} + 0x20]",
			"mov rdi, [{new} + 0x28]",
			"mov rbp, [{new} + 0x30]",
			"mov rsp, [{new} + 0x38]",
			"mov r8,  [{new} + 0x40]",
			"mov r9,  [{new} + 0x48]",
			"mov r10, [{new} + 0x50]",
			"mov r11, [{new} + 0x58]",
			"mov r12, [{new} + 0x60]",
			"mov r13, [{new} + 0x68]",
			"mov r14, [{new} + 0x70]",
			"mov r15, [{new} + 0x78]",
			"mov rax, [{new} + 0x80]",
			"jmp rax",
			"2:",
			old = in(reg) old_ptr,
			new = in(reg) new_ptr,
			lateout("rax") _,
			lateout("rcx") _,
			lateout("rdx") _,
			lateout("rsi") _,
			lateout("rdi") _,
			lateout("r8") _,
			lateout("r9") _,
			lateout("r10") _,
			lateout("r11") _,
			lateout("r12") _,
			lateout("r13") _,
			lateout("r14") _,
			lateout("r15") _,
			options(preserves_flags),
		);
	}
}

fn setup_initial_stack(process: &mut Process) {
	debug_assert!(process.kernel_stack_pointer != 0);

	let stack_top = (process.kernel_stack_base as usize).wrapping_add(KERNEL_STACK_SIZE as usize);

	let mut sp = stack_top as *mut u64;

	unsafe {
		sp = sp.offset(-1);
		core::ptr::write(sp, process.entry_point);

		sp = sp.offset(-1);
		core::ptr::write(sp, 0x202u64);

		for _ in 0..12 {
			sp = sp.offset(-1);
			core::ptr::write(sp, 0u64);
		}
	}

	process.kernel_stack_pointer = sp as u64;
}

pub fn init_scheduler() {
    if !unsafe { INITILIZED } {
        println!("Warning: Scheduler not initialized!");
        return;
    }

    
}