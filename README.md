#PWM ESC Controller
 > Control ESCs with a PWM signal

 The purpose of this repository is to control the speed of a motor using an ESC driven by a pwm signal. In particular, this pwm signal is 50Hz with a width of anywhere from 1-2ms (min to max throttle). 


 ### Electronic Speed Controller 
In order to control the speed of a brushless motor, a middleman that can deal with intricacies involved with such as a motor is necessary. ESCs are helpful as they abstract away technical details such that the end user just needs to supply a pwm signal (or other policies such as oneshot, Dshot etc.)

I used is the **LITTLEBEE 30A**, which includes the BLHeli S software. Using documentation found online (which was for a different ESC, but I believe these are of the same model), a particular behaviour can be noted.

From the image above, I can discern that the ESC needs to detect zero throttle for a normal start up. Afterwards, the motor will run to the user's specifity.  
```
// Zero signal
rprintln!("Zero signal");
ch1.set_duty(max_duty / 20);

rprintln!("Slight delay");
delay.delay_ms(10000_u32);

rprintln!("Mid-throttle signal");
ch1.set_duty(max_duty / 18);

```
In the code above, to run the ESCs normal start-up process, I send a zero signal. I introduce a slight delay so the ESC can register the signal, and then a slightly higher throttle signal. 



### Reference
- https://www.flyingtech.co.uk/sites/default/files/product_files/S_DShot600.pdf