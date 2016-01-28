#![feature(lang_items)]
#![no_std]

#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

#[lang="eh_personality"]
extern fn eh_personality() {}

#[lang="panic_fmt"]
extern fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! { loop { } }

#[allow(dead_code)]
 pub enum PinMode {
	INPUT,
	OUTPUT,
	INPUT_PULLUP,
	INPUT_PULLDOWN,
	AF_OUTPUT_PUSHPULL, //Used internally for Alternate Function Output PushPull(TIM, UART, SPI etc)
	AF_OUTPUT_DRAIN,		//Used internally for Alternate Function Output Drain(I2C etc). External pullup resistors required.
	AN_INPUT,					 //Used internally for ADC Input
	AN_OUTPUT,					//Used internally for DAC Output
	PIN_MODE_NONE=0xFF,
}

pub enum Pin {
	A0=10,
	A1=11,
	A2=12,
	D0=0,
	D1=1,
	D7=7,
}

pub enum PinState {
	LOW = 0,
	HIGH = 1,
}

use Pin::*;
use PinState::*;
use PinMode::*;

extern {
	fn HAL_Pin_Mode(pin: i16, mode: u8);
	fn HAL_GPIO_Write(pin: i16, value: u8);
	fn HAL_Delay_Milliseconds(delay: u32);
}

fn pin_mode(pin: Pin, mode: PinMode) {
	unsafe {
		HAL_Pin_Mode(pin as i16, mode as u8);
	}
}

fn digital_write(pin: Pin, value: PinState) {
	unsafe {
		HAL_GPIO_Write(pin as i16, value as u8);
	}
}

fn delay(delay: u32) {
	unsafe { HAL_Delay_Milliseconds(delay); }
}

#[no_mangle]
#[export_name = "setup"]
pub extern fn main()
{
	pin_mode(D7, OUTPUT);

	loop {
		digital_write(D7, HIGH);
		delay(100);
		digital_write(D7, LOW);
		delay(100);
	}
}

#[no_mangle]
#[export_name = "loop"]
pub extern fn loopy() {}
