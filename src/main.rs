// use cacao::input;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use dioxus::html::input;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
// use dasp::signal;
use dasp_sample::{Sample, I24};
use dasp_signal::{self as signal, Gen, Rate, ScaleAmp, Signal};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

struct Oscilator {
    // rate: Rate, // hz: f64, // sig: S,
    // hz_s: Gen<dyn Fn() -> f64, f64>,
    // sig: ScaleAmp<>
    rate: f64,
    hz: f64,
    // phase: f64,
    t: f64,
    amp: f64,
}
impl Oscilator {
    fn new(rate: f64) -> Oscilator {
        // let o = Oscilator {hz: };
        // let hz_gen = signal::gen(|| self.hz);

        // let mut rate = signal::rate(sample_rate)

        // rate
        // .const_hz(220. / 10.)
        // .hz(hz_gen)
        // .sine()
        // .scale_amp(0.1);
        // .clip_amp(0.02);

        Oscilator {
            rate,
            hz: 220_f64,
            t: 0_f64,
            amp: 0.1,
        }
    }

    #[inline]
    fn next_t_wrapped_to(&mut self, rem: f64) -> f64 {
        let t = self.t;
        self.t = (self.t + self.step()) % rem;
        t
    }

    #[inline]
    fn next_t(&mut self) -> f64 {
        self.next_t_wrapped_to(1.0)
    }

    #[inline]
    fn step(&self) -> f64 {
        return self.hz / self.rate;
    }

    #[inline]
    fn next(&mut self) -> f64 {
        // return self.rate.next().to_sample::<f32>();

        const PI_2: f64 = core::f64::consts::PI * 2.0;
        let phase = self.next_t();
        f64::sin(PI_2 * phase) * self.amp
    }
}

use std::thread::sleep;
use std::time::Duration;
fn main() {
    let host = cpal::default_host();

    // println!("{}", host)
    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");

    // println!("{:?}", supported_configs_range)
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let config: StreamConfig = supported_config.into();

    // let mut o = Arc::new(Oscilator::new(config.sample_rate.0.into()));
    let mut o = Arc::new(Mutex::new(Oscilator::new(config.sample_rate.0.into())));
    let mut o_cb = Arc::clone(&o);
    // let mut o_ui = Arc::clone(&o);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // for x in data.iter_mut() {
                //     *x = s.next().to_sample::<f32>();
                // }

                for x in data.iter_mut() {
                    *x = o_cb.lock().unwrap().next().to_sample::<f32>();
                }
            },
            move |err| {
                println!("error: {}", err)
                // react to errors here.
            },
            // None, // None=blocking, Some(Duration)=timeout
        )
        .unwrap();

    stream.play().unwrap();

    // drop(stream);

    // for _ in 0..10 {
    //     let x = s.next();
    //     println!("{}", x);
    // }

    // for i in 0..4 {
    //     println!("{}", i);
    //     sleep(Duration::from_secs(1));
    //     o.lock().unwrap().hz = 220. + ((i + 1) as f64) / 3. * 220.
    // }

    sleep(Duration::from_secs(1));
    // di::launch(app);
    dioxus_desktop::launch_with_props(app, AppProps { o }, dioxus_desktop::Config::default());
}

struct AppProps {
    o: Arc<Mutex<Oscilator>>,
}
fn app(cx: Scope<AppProps>) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
        input {
            r#type: "range",
            min: 0,
            max: 100,
            onchange: move |e| cx.props.o.lock().unwrap().hz = 220.  + (220. * e.value.parse::<f64>().unwrap() / 100.)   ,
        },
    })
}
