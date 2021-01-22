use libm::fabsf;
use stm32f1xx_hal::pwm::{Channel, Pwm, C1, C2};
use stm32f1xx_hal::timer::Tim2NoRemap;
use stm32f1xx_hal::pac::TIM2;
use stm32f1xx_hal::gpio::gpioa::{PA0, PA1};
use stm32f1xx_hal::gpio::gpiod::{PD1, PD15};
use stm32f1xx_hal::gpio::{Alternate, PushPull, Output};

use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::prelude::*;

use crate::mpu6050;

static KP: f32 = 18000.0;
static KD: f32 = 300.0;

pub struct UprightCon<'a> {
    pwm: Pwm<TIM2, Tim2NoRemap, (C1, C2), (
        PA0<Alternate<PushPull>>,
        PA1<Alternate<PushPull>>
    )>,
    dirpins: (PD1<Output<PushPull>>, PD15<Output<PushPull>>),
    data: &'a mpu6050::Data,
}

impl<'a> UprightCon<'a> {
    pub fn init(
        mut pwm: Pwm<TIM2, Tim2NoRemap, (C1, C2), (
            PA0<Alternate<PushPull>>,
            PA1<Alternate<PushPull>>
        )>,
        dirpins: (PD1<Output<PushPull>>, PD15<Output<PushPull>>),
        data: &'a mpu6050::Data,
    ) -> Self {
        pwm.set_period(1.hz());
        pwm.set_duty(Channel::C1, pwm.get_max_duty()/2);
        pwm.set_duty(Channel::C2, pwm.get_max_duty()/2);
        pwm.enable(Channel::C1);
        pwm.enable(Channel::C2);

        UprightCon {
            data,
            dirpins,
            pwm
        }
    }

    pub fn cal_to_speed(&mut self) {

        if self.data.angle > 0.0 {
            self.dirpins.0.set_high().ok();
            self.dirpins.1.set_high().ok();
        } else {
            self.dirpins.0.set_low().ok();
            self.dirpins.1.set_low().ok();
        }

        self.pwm.set_period(100.hz());
        // let angle = fabsf(self.data.angle);
        // let gyro = self.data.gyro_y;

        // let speed = (KP * angle - KD * gyro) as u16;

    }
}